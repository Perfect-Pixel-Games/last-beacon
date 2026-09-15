//! Regression test for the modular-vehicle "Neither body ... is in an
//! island" Avian3D panic.
//!
//! Every other vehicle-system test (in `game/src/shared/vehicle/*.rs`)
//! exercises `LastBeaconVehiclePlugin` without ever installing real Avian3D
//! physics or stepping a physics tick -- which is exactly why the underlying
//! system-ordering bug shipped past three separate task reviews undetected.
//! `materialize_last_beacon_vehicle_module_bodies`/`_wheel_module_bodies`
//! used to run *before* `apply_pending_last_beacon_vehicle_module_instances`
//! in `LastBeaconVehiclePlugin`'s `Update` chain. On the exact frame a module
//! instance's `.bsn` content resolved, `apply_pending_*` (an exclusive system
//! that mutates the `World` directly, no command buffering) inserted that
//! module's `LastBeaconVehicleModuleBody`/`WheelModuleBody` -- but
//! `materialize_*` had already run earlier in that same chain execution, so
//! its `Added<>` query missed the just-inserted component for a full frame,
//! deferring the `RigidBody`/`Collider` insertion to the *next* frame.
//! Meanwhile `wire_last_beacon_vehicle_connections` (later in the same
//! chain) only checks whether a module is still "pending" load, not whether
//! it has a `RigidBody` yet, so it could spawn a joint referencing a module
//! entity that would not become a rigid body until the following frame. That
//! joint got flushed to the `World` a whole physics tick before its
//! referenced entity became a rigid body, and Avian3D's island builder
//! panics when a joint references a body it doesn't recognize as being in
//! any island.
//!
//! This test spawns the real `vehicle/vehicle_module_testbed.bsn` through
//! the full production pipeline -- real file-backed BSN asset loading AND
//! real Avian3D physics ticking -- and asserts it settles into its expected
//! steady state without panicking. It fails (panics) against the ordering
//! that shipped, and passes once `queue_*`/`apply_pending_*` are ordered
//! before the `materialize_*` systems.

use std::time::{Duration, Instant};

use avian3d::prelude::{FixedJoint, RevoluteJoint, RigidBody};
use bevy::prelude::*;
use foundation_runtime_library::prelude::*;
use last_beacon::{asset_root, shared::vehicle::LastBeaconVehiclePlugin};

/// Expected steady-state rigid body count for the test wagon: `CoreBlock`,
/// `BeamCenter`, `BeamFront`, and `BeamBack` fuse into one compound chassis
/// body (see `connection.rs`'s doc comment for why welds no longer produce
/// their own `RigidBody`), plus the four wheels.
const EXPECTED_RIGID_BODY_COUNT: usize = 5;
/// Expected steady-state fixed-joint count: zero -- welds fuse modules into
/// one compound body instead of becoming a physics joint.
const EXPECTED_FIXED_JOINT_COUNT: usize = 0;
/// Expected steady-state revolute-joint count: one hinge per wheel.
const EXPECTED_REVOLUTE_JOINT_COUNT: usize = 4;

#[test]
fn vehicle_module_testbed_settles_under_real_physics_without_panicking() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(bevy::asset::AssetPlugin {
        file_path: asset_root().to_string_lossy().to_string(),
        ..default()
    });
    app.add_plugins(bevy::scene::ScenePlugin);
    app.add_message::<SceneLoadRequested>();
    app.add_message::<SceneAdded>();
    app.add_message::<SceneFocused>();
    app.add_plugins(FoundationBsnAssetPlugin);
    // `materialize_last_beacon_vehicle_module_bodies`/`_wheel_module_bodies`
    // create mesh/material assets, and Avian3D's own collider cache
    // (`avian3d::collision::collider::cache::clear_unused_colliders`) reads
    // `AssetEvent<Mesh>` -- both need `Assets<Mesh>`/`Assets<StandardMaterial>`
    // initialized, which `RenderPlugin`/`PbrPlugin` normally do as part of
    // `DefaultPlugins` in the real game.
    app.init_asset::<Mesh>();
    app.init_asset::<StandardMaterial>();
    app.add_plugins(LastBeaconVehiclePlugin);
    app.add_plugins(FoundationPhysicsPlugin);

    // Avian3D registers some of its resources (e.g. collider-tree
    // diagnostics) from `Plugin::finish` rather than `Plugin::build`, which
    // only runs via `App::finish`/`App::cleanup` -- normally called
    // internally by `App::run`. A bare `app.update()` loop never calls
    // those, so they must be invoked explicitly here, matching this crate's
    // existing pattern for tests exercising plugins with a `finish` hook
    // (see `game/src/world/free_fly_camera.rs` and
    // `foundation-runtime-library/src/menu.rs`).
    app.finish();
    app.cleanup();

    app.add_systems(Startup, |mut commands: Commands| {
        commands.spawn_bsn_asset("vehicle/vehicle_module_testbed.bsn");
    });

    // Real disk-backed asset loading is asynchronous and its timing varies
    // under load, so poll with a generous wall-clock deadline (mirroring
    // `game/src/shared/vehicle/instance.rs`'s own test pattern) instead of a
    // fixed frame count. Sleeping a little each frame also lets Bevy's
    // `Time<Virtual>` accumulate enough real elapsed time for Avian's
    // `FixedPostUpdate` physics schedule to actually run -- its default
    // 64 Hz timestep needs about 15.6ms of accumulated real time per tick,
    // and this is exactly the schedule whose island builder panics on the
    // bug this test guards against.
    let steady_state_deadline = Instant::now() + Duration::from_secs(30);
    loop {
        app.update();
        std::thread::sleep(Duration::from_millis(5));

        if vehicle_component_counts(&mut app) == expected_counts() {
            break;
        }

        assert!(
            Instant::now() < steady_state_deadline,
            "vehicle module testbed never reached its expected steady state \
             ({EXPECTED_RIGID_BODY_COUNT} RigidBody, {EXPECTED_FIXED_JOINT_COUNT} FixedJoint, \
             {EXPECTED_REVOLUTE_JOINT_COUNT} RevoluteJoint) within the timeout; saw {:?}",
            vehicle_component_counts(&mut app)
        );
    }

    // Keep stepping real physics ticks well past first reaching steady
    // state: the panic this guards against fires from inside Avian's own
    // physics schedule, not from the vehicle systems themselves, so a test
    // that stopped the instant counts first matched could get lucky and
    // return before that schedule ever ran this session. Continuing to step
    // proves physics actually kept running against the resolved vehicle
    // without panicking.
    for _ in 0..120 {
        app.update();
        std::thread::sleep(Duration::from_millis(5));
    }

    assert_eq!(
        vehicle_component_counts(&mut app),
        expected_counts(),
        "vehicle module testbed should still be at its expected steady state \
         after continued physics stepping"
    );
}

/// Named pairs of modules that a [`LastBeaconVehicleConnection`] rigidly
/// joins together in `vehicle_module_testbed.bsn` (mirrors that file's
/// connection list) -- the first three pairs are welds (now fused into one
/// compound chassis body, so their distance is trivially exact; kept here
/// anyway so a future regression in the fusion math would still be caught),
/// the last four are wheel hinges (still real `RevoluteJoint`s). A
/// connection holding means the straight-line distance between each pair
/// stays essentially constant over time -- unlike linear velocity, this is
/// meaningful even though the testbed spawns in mid-air with no ground plane
/// and free-falls for the whole test, since every module falls together and
/// the distances are translation-invariant.
const RIGIDLY_CONNECTED_MODULE_PAIRS: [(&str, &str); 7] = [
    ("BeamCenter", "CoreBlock"),
    ("BeamCenter", "BeamFront"),
    ("BeamCenter", "BeamBack"),
    ("BeamFront", "WheelFrontLeft"),
    ("BeamFront", "WheelFrontRight"),
    ("BeamBack", "WheelBackLeft"),
    ("BeamBack", "WheelBackRight"),
];

/// Regression test for connected modules vibrating/tearing apart instead of
/// holding together as one rigid structure. Several separate bugs have
/// produced this over this module's history -- overlapping colliders
/// fighting their own joint, `FixedJoint`/`RevoluteJoint` frame bases not
/// accounting for a chassis module's own rotation, and numerically unstable
/// zero-compliance joints on a "hub" body with 3+ connections (see the doc
/// comments in `connection.rs` for each) -- so rather than re-deriving each
/// one's exact symptom, this test asserts the actual invariant that matters:
/// the distance between every rigidly-joined module pair stays close to its
/// initial value throughout continued physics stepping. Tearing apart (any
/// of those bugs) blows well past the tolerance; falling together under
/// gravity as one rigid structure (all fixed) does not.
#[test]
fn vehicle_module_testbed_holds_its_shape_under_continued_physics() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(bevy::asset::AssetPlugin {
        file_path: asset_root().to_string_lossy().to_string(),
        ..default()
    });
    app.add_plugins(bevy::scene::ScenePlugin);
    app.add_message::<SceneLoadRequested>();
    app.add_message::<SceneAdded>();
    app.add_message::<SceneFocused>();
    app.add_plugins(FoundationBsnAssetPlugin);
    app.init_asset::<Mesh>();
    app.init_asset::<StandardMaterial>();
    app.add_plugins(LastBeaconVehiclePlugin);
    app.add_plugins(FoundationPhysicsPlugin);

    app.finish();
    app.cleanup();

    app.add_systems(Startup, |mut commands: Commands| {
        commands.spawn_bsn_asset("vehicle/vehicle_module_testbed.bsn");
    });

    // First reach the same steady-state component counts the other test
    // waits for, so joints have actually been spawned before we start
    // measuring shape.
    let steady_state_deadline = Instant::now() + Duration::from_secs(30);
    loop {
        app.update();
        std::thread::sleep(Duration::from_millis(5));

        if vehicle_component_counts(&mut app) == expected_counts() {
            break;
        }

        assert!(
            Instant::now() < steady_state_deadline,
            "vehicle module testbed never reached its expected steady state before shape measurement could start"
        );
    }

    // A joint's very first tick or two can produce a small one-time
    // "pop" as its position/basis constraints are enforced for the first
    // time (e.g. settling out the sub-millimeter float-precision gap
    // between `beam.bsn`'s authored length and the exact anchor distance).
    // Let that transient settle before recording the baseline distances
    // shape drift is measured against, so this test asserts "does the
    // structure hold together over time" rather than "is the very first
    // physics tick perfectly seamless".
    let settle_deadline = Instant::now() + Duration::from_secs(1);
    while Instant::now() < settle_deadline {
        app.update();
        std::thread::sleep(Duration::from_millis(5));
    }

    let initial_distances = rigidly_connected_pair_distances(&mut app);

    const MAX_SHAPE_DRIFT: f32 = 0.15;
    let measuring_deadline = Instant::now() + Duration::from_secs(3);
    while Instant::now() < measuring_deadline {
        app.update();
        std::thread::sleep(Duration::from_millis(5));

        let current_distances = rigidly_connected_pair_distances(&mut app);
        for ((module_a, module_b), (initial, current)) in RIGIDLY_CONNECTED_MODULE_PAIRS
            .iter()
            .zip(initial_distances.iter().zip(current_distances.iter()))
        {
            assert!(
                (current - initial).abs() < MAX_SHAPE_DRIFT,
                "the distance between `{module_a}` and `{module_b}` drifted from {initial} to \
                 {current} (>= {MAX_SHAPE_DRIFT} tolerance) -- the vehicle is tearing itself \
                 apart instead of holding together as one rigid structure"
            );
        }
    }
}

/// The current straight-line distance between each of
/// [`RIGIDLY_CONNECTED_MODULE_PAIRS`], in the same order.
///
/// Uses `GlobalTransform` rather than Avian3D's own `Position` -- a welded
/// module no longer has `Position` at all once fused into its compound
/// body's `Collider` hierarchy (see `connection.rs`'s doc comment), but
/// `GlobalTransform` still correctly reflects its world position regardless
/// of whether it's a rigid body in its own right or a fused child.
fn rigidly_connected_pair_distances(app: &mut App) -> Vec<f32> {
    let world = app.world_mut();
    let mut positions = world.query::<(&Name, &GlobalTransform)>();
    let positions_by_name: std::collections::HashMap<&str, Vec3> = positions
        .iter(world)
        .map(|(name, transform)| (name.as_str(), transform.translation()))
        .collect();

    RIGIDLY_CONNECTED_MODULE_PAIRS
        .iter()
        .map(|(module_a, module_b)| {
            let position_a = *positions_by_name
                .get(module_a)
                .unwrap_or_else(|| panic!("module `{module_a}` should exist in the testbed"));
            let position_b = *positions_by_name
                .get(module_b)
                .unwrap_or_else(|| panic!("module `{module_b}` should exist in the testbed"));
            position_a.distance(position_b)
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq)]
struct VehicleComponentCounts {
    rigid_bodies: usize,
    fixed_joints: usize,
    revolute_joints: usize,
}

fn expected_counts() -> VehicleComponentCounts {
    VehicleComponentCounts {
        rigid_bodies: EXPECTED_RIGID_BODY_COUNT,
        fixed_joints: EXPECTED_FIXED_JOINT_COUNT,
        revolute_joints: EXPECTED_REVOLUTE_JOINT_COUNT,
    }
}

fn vehicle_component_counts(app: &mut App) -> VehicleComponentCounts {
    let world = app.world_mut();
    VehicleComponentCounts {
        rigid_bodies: world.query::<&RigidBody>().iter(world).count(),
        fixed_joints: world.query::<&FixedJoint>().iter(world).count(),
        revolute_joints: world.query::<&RevoluteJoint>().iter(world).count(),
    }
}
