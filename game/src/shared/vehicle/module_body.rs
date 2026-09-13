//! Turns authored module-body data into real Bevy meshes/materials and Avian
//! rigid bodies/colliders.

use avian3d::prelude::*;
use bevy::prelude::*;

use super::LastBeaconVehicleModuleBody;

/// Builds the mesh, material, rigid body, collider, and mass for every
/// newly-authored [`LastBeaconVehicleModuleBody`].
pub fn materialize_last_beacon_vehicle_module_bodies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    module_bodies: Query<(Entity, &LastBeaconVehicleModuleBody), Added<LastBeaconVehicleModuleBody>>,
) {
    for (module_entity, module_body) in &module_bodies {
        let mesh = meshes.add(Cuboid::new(
            module_body.size_x,
            module_body.size_y,
            module_body.size_z,
        ));
        let material = materials.add(StandardMaterial {
            base_color: last_beacon_vehicle_module_color(&module_body.color),
            ..default()
        });

        commands.entity(module_entity).insert((
            Mesh3d(mesh),
            MeshMaterial3d(material),
            RigidBody::Dynamic,
            Collider::cuboid(module_body.size_x, module_body.size_y, module_body.size_z),
            Mass(module_body.mass),
        ));
    }
}

/// Resolves a module's authored color name to a concrete color, warning and
/// falling back to `steel_blue` for an unrecognized name.
pub fn last_beacon_vehicle_module_color(color_name: &str) -> Color {
    match color_name.trim().to_ascii_lowercase().as_str() {
        "steel_blue" => Color::srgb(0.29, 0.42, 0.55),
        "charcoal" => Color::srgb(0.16, 0.16, 0.18),
        "olive" => Color::srgb(0.36, 0.38, 0.21),
        unknown_color => {
            warn!("Unknown vehicle module color `{unknown_color}`; using steel_blue");
            Color::srgb(0.29, 0.42, 0.55)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::asset::AssetPlugin;

    fn test_app() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(AssetPlugin::default());
        app.init_asset::<Mesh>();
        app.init_asset::<StandardMaterial>();
        app.add_systems(Update, materialize_last_beacon_vehicle_module_bodies);
        app
    }

    #[test]
    fn materializing_a_module_body_adds_a_dynamic_rigid_body_and_collider() {
        let mut app = test_app();
        let module_entity = app
            .world_mut()
            .spawn(LastBeaconVehicleModuleBody {
                size_x: 2.0,
                size_y: 1.0,
                size_z: 1.0,
                mass: 15.0,
                color: "olive".to_string(),
            })
            .id();

        app.update();

        assert!(matches!(
            app.world().get::<RigidBody>(module_entity),
            Some(RigidBody::Dynamic)
        ));
        assert!(app.world().get::<Collider>(module_entity).is_some());
        assert!(app.world().get::<Mass>(module_entity).is_some());
        assert!(app.world().get::<Mesh3d>(module_entity).is_some());
        assert!(app.world().get::<MeshMaterial3d<StandardMaterial>>(module_entity).is_some());
    }

    #[test]
    fn unknown_color_falls_back_to_steel_blue() {
        assert_eq!(
            last_beacon_vehicle_module_color("not-a-real-color"),
            Color::srgb(0.29, 0.42, 0.55)
        );
    }
}
