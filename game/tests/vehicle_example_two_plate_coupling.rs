//! Proves `game/assets/vehicle/examples/two_plate_coupling.bsn` -- the
//! worked example referenced by `docs/vehicle-authoring-guide.md` -- is
//! real, loadable, non-drifting data: it settles under real physics without
//! panicking, and the weld actually pulls the two plates' sockets together.
//! Mirrors `vehicle_module_physics.rs`'s test harness pattern.

use std::time::{Duration, Instant};

use avian3d::prelude::{FixedJoint, RevoluteJoint, RigidBody};
use bevy::prelude::*;
use foundation_runtime_library::prelude::*;
use last_beacon::{asset_root, shared::vehicle::LastBeaconVehiclePlugin};

/// The two plates fuse into one compound body (see `connection.rs`'s doc
/// comment for why a weld no longer produces its own `RigidBody`).
const EXPECTED_RIGID_BODY_COUNT: usize = 1;

#[test]
fn two_plate_coupling_example_settles_under_real_physics_without_panicking() {
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
        commands.spawn_bsn_asset("vehicle/examples/two_plate_coupling.bsn");
    });

    let steady_state_deadline = Instant::now() + Duration::from_secs(30);
    loop {
        app.update();
        std::thread::sleep(Duration::from_millis(5));

        if rigid_body_count(&mut app) == EXPECTED_RIGID_BODY_COUNT {
            break;
        }

        assert!(
            Instant::now() < steady_state_deadline,
            "two_plate_coupling example never reached its expected steady state \
             ({EXPECTED_RIGID_BODY_COUNT} RigidBody) within the timeout; saw {} RigidBody",
            rigid_body_count(&mut app)
        );
    }

    // Keep stepping real physics ticks past first reaching steady state, same
    // rationale as vehicle_module_physics.rs: the panic class this guards
    // against fires from inside Avian's own physics schedule.
    for _ in 0..120 {
        app.update();
        std::thread::sleep(Duration::from_millis(5));
    }

    assert_eq!(
        rigid_body_count(&mut app),
        EXPECTED_RIGID_BODY_COUNT,
        "two_plate_coupling example should still be at its expected steady state \
         after continued physics stepping"
    );

    let world = app.world_mut();
    assert_eq!(
        world.query::<&FixedJoint>().iter(world).count(),
        0,
        "a weld must never produce a FixedJoint entity"
    );
    assert_eq!(
        world.query::<&RevoluteJoint>().iter(world).count(),
        0,
        "this example has no hinges"
    );

    // PlateOne's "a" socket (local +0.3 on X) should coincide with PlateTwo's
    // "b" socket (local -0.3 on X) once the weld has fused them -- proving
    // the example's authored socket offsets are actually correct, not just
    // "didn't panic."
    let mut named_transforms = world.query::<(&Name, &GlobalTransform)>();
    let mut plate_one_global = None;
    let mut plate_two_global = None;
    for (name, transform) in named_transforms.iter(world) {
        match name.as_str() {
            "PlateOne" => plate_one_global = Some(*transform),
            "PlateTwo" => plate_two_global = Some(*transform),
            _ => {}
        }
    }
    let plate_one_global = plate_one_global.expect("PlateOne should exist");
    let plate_two_global = plate_two_global.expect("PlateTwo should exist");
    let plate_one_socket_a = plate_one_global.transform_point(Vec3::new(0.3, 0.0, 0.0));
    let plate_two_socket_b = plate_two_global.transform_point(Vec3::new(-0.3, 0.0, 0.0));
    assert!(
        plate_one_socket_a.distance(plate_two_socket_b) < 1e-3,
        "PlateOne's socket `a` ({plate_one_socket_a:?}) should coincide with PlateTwo's socket \
         `b` ({plate_two_socket_b:?}) once the weld has settled"
    );
}

fn rigid_body_count(app: &mut App) -> usize {
    let world = app.world_mut();
    world.query::<&RigidBody>().iter(world).count()
}
