//! Regression test for the wagon (and anything else) falling straight
//! through the World testbed's landscape: the terrain used to be a pure
//! render mesh with no Avian3D collider at all, so nothing could ever rest
//! on it. This spawns the exact same trimesh collider construction
//! `game/src/world/mod.rs` uses for the terrain, drops a dynamic body onto
//! it under real physics, and asserts its fall is arrested well short of
//! what unobstructed free-fall over the same time would produce.
//!
//! This asserts "collision happened" rather than "the body came to rest at
//! an exact height": the terrain is a proc-genned *mountain*, and a sphere
//! (chosen so the test isn't sensitive to a collider's exact shape) has zero
//! rolling resistance, so on anything but dead-flat ground it keeps rolling
//! downhill indefinitely under gravity's slope component -- never reaching
//! zero velocity even though the terrain collision is working correctly.
//! Asserting near-zero velocity or an exact resting height would make this
//! test fail on entirely correct behavior depending on the slope under the
//! drop point.

use std::time::{Duration, Instant};

use avian3d::prelude::{Collider, Position, RigidBody};
use bevy::prelude::*;
use foundation_runtime_library::prelude::FoundationPhysicsPlugin;
use last_beacon::world::landscape;

#[test]
fn a_dynamic_body_dropped_above_the_landscape_collides_with_it_instead_of_falling_through() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    // Avian3D's own collider cache (`avian3d::collision::collider::cache::clear_unused_colliders`)
    // reads `AssetEvent<Mesh>`, so `Assets<Mesh>` must be initialized even
    // though this test never loads a mesh asset from disk -- matches the
    // same requirement documented in `vehicle_module_physics.rs`.
    app.add_plugins(bevy::asset::AssetPlugin::default());
    app.init_asset::<Mesh>();
    app.add_plugins(FoundationPhysicsPlugin);
    app.finish();
    app.cleanup();

    let seed = 1;
    let settings = landscape::LandscapeGenerationSettings::default();
    let terrain_mesh = landscape::build_landscape_mesh(seed, &settings);
    let terrain_collider = Collider::trimesh_from_mesh(&terrain_mesh)
        .expect("the landscape mesh should always produce a valid trimesh collider");
    app.world_mut()
        .spawn((RigidBody::Static, terrain_collider, Transform::IDENTITY));

    let drop_x = 0.0;
    let drop_z = 0.0;
    let ground_height = landscape::landscape_height_at(seed, drop_x, drop_z, &settings);
    let drop_clearance = 20.0;
    let start_height = ground_height + drop_clearance;
    let dropped_entity = app
        .world_mut()
        .spawn((
            RigidBody::Dynamic,
            Collider::sphere(0.5),
            Transform::from_xyz(drop_x, start_height, drop_z),
        ))
        .id();

    let fall_duration_seconds = 8.0_f32;
    let fall_deadline = Instant::now() + Duration::from_secs_f32(fall_duration_seconds);
    while Instant::now() < fall_deadline {
        app.update();
        std::thread::sleep(Duration::from_millis(5));
    }

    let final_position = app
        .world()
        .get::<Position>(dropped_entity)
        .expect("dropped body should still exist");
    let actual_drop_distance = start_height - final_position.y;

    // Gravity's default magnitude (`avian3d::prelude::Gravity`'s default is
    // Earth-like, ~9.81 m/s^2); an unobstructed body would cover this
    // distance in `fall_duration_seconds` if nothing ever stopped it.
    let unobstructed_free_fall_distance = 0.5 * 9.81 * fall_duration_seconds.powi(2);
    assert!(
        actual_drop_distance < unobstructed_free_fall_distance * 0.5,
        "a body dropped {drop_clearance}m above the terrain fell {actual_drop_distance}m over \
         {fall_duration_seconds}s, close to the {unobstructed_free_fall_distance}m unobstructed \
         free-fall would cover -- it fell straight through the terrain instead of colliding with it"
    );
}
