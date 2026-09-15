//! Turns a module's directly-authored Bevy shape (wrapped in
//! [`LastBeaconVehicleModuleCuboidShape`]/[`LastBeaconVehicleModuleCylinderShape`])
//! into the real mesh, material, and collider it implies.
//!
//! Generic over Bevy's own shape types rather than a Last-Beacon-specific
//! "module body" descriptor -- see `mod.rs`'s module doc comment for why --
//! so these systems only need to bridge geometry to rendering/physics, not
//! duplicate a shape's own fields.

use avian3d::prelude::*;
use bevy::prelude::*;

use super::{
    LastBeaconVehicleModuleColor, LastBeaconVehicleModuleCuboidShape,
    LastBeaconVehicleModuleCylinderShape, LastBeaconVehicleModuleInstance,
};

/// Fallback base color for a module that omits [`LastBeaconVehicleModuleColor`]
/// -- a neutral mid-grey, chosen only to make a missing color visually
/// obvious (paired with the `warn!` below) rather than to look good.
const LAST_BEACON_VEHICLE_FALLBACK_MODULE_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);

type NewlyAddedModuleCuboidsQuery<'w, 's> = Query<
    'w,
    's,
    (
        Entity,
        &'static LastBeaconVehicleModuleCuboidShape,
        Option<&'static LastBeaconVehicleModuleColor>,
    ),
    (
        Added<LastBeaconVehicleModuleCuboidShape>,
        With<LastBeaconVehicleModuleInstance>,
    ),
>;

type NewlyAddedModuleCylindersQuery<'w, 's> = Query<
    'w,
    's,
    (
        Entity,
        &'static LastBeaconVehicleModuleCylinderShape,
        Option<&'static LastBeaconVehicleModuleColor>,
    ),
    (
        Added<LastBeaconVehicleModuleCylinderShape>,
        With<LastBeaconVehicleModuleInstance>,
    ),
>;

/// Builds the mesh, material, and collider for every newly-authored
/// [`LastBeaconVehicleModuleCuboidShape`] belonging to a vehicle module
/// (scoped by [`LastBeaconVehicleModuleInstance`] so this doesn't fire for
/// an unrelated cuboid shape used elsewhere in the game).
pub fn materialize_last_beacon_vehicle_cuboid_modules(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    module_cuboids: NewlyAddedModuleCuboidsQuery,
) {
    for (module_entity, shape, color) in &module_cuboids {
        let cuboid = shape.0;
        let base_color = resolve_last_beacon_vehicle_module_color(module_entity, color);
        let mesh = meshes.add(cuboid);
        let material = materials.add(StandardMaterial {
            base_color,
            ..default()
        });
        let full_size = cuboid.half_size * 2.0;

        commands.entity(module_entity).insert((
            Mesh3d(mesh),
            MeshMaterial3d(material),
            Collider::cuboid(full_size.x, full_size.y, full_size.z),
        ));
    }
}

/// Builds the mesh, material, and collider for every newly-authored
/// [`LastBeaconVehicleModuleCylinderShape`] belonging to a vehicle module --
/// same scoping rationale as [`materialize_last_beacon_vehicle_cuboid_modules`].
pub fn materialize_last_beacon_vehicle_cylinder_modules(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    module_cylinders: NewlyAddedModuleCylindersQuery,
) {
    for (module_entity, shape, color) in &module_cylinders {
        let cylinder = shape.0;
        let base_color = resolve_last_beacon_vehicle_module_color(module_entity, color);
        let mesh = meshes.add(cylinder);
        let material = materials.add(StandardMaterial {
            base_color,
            ..default()
        });

        commands.entity(module_entity).insert((
            Mesh3d(mesh),
            MeshMaterial3d(material),
            Collider::cylinder(cylinder.radius, cylinder.half_height * 2.0),
        ));
    }
}

/// Reads a module's authored color, warning and falling back to a visually
/// obvious neutral grey if it omitted [`LastBeaconVehicleModuleColor`]
/// entirely.
fn resolve_last_beacon_vehicle_module_color(
    module_entity: Entity,
    color: Option<&LastBeaconVehicleModuleColor>,
) -> Color {
    match color {
        Some(color) => color.0,
        None => {
            warn!(
                "Vehicle module {module_entity:?} has a shape but no LastBeaconVehicleModuleColor; using a fallback grey."
            );
            LAST_BEACON_VEHICLE_FALLBACK_MODULE_COLOR
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
        app.add_systems(
            Update,
            (
                materialize_last_beacon_vehicle_cuboid_modules,
                materialize_last_beacon_vehicle_cylinder_modules,
            ),
        );
        app
    }

    #[test]
    fn a_module_cuboid_gets_a_mesh_material_and_matching_collider() {
        let mut app = test_app();
        let module_entity = app
            .world_mut()
            .spawn((
                LastBeaconVehicleModuleInstance {
                    asset_path: "unused.bsn".to_string(),
                },
                LastBeaconVehicleModuleCuboidShape(Cuboid::new(2.0, 1.0, 1.0)),
                LastBeaconVehicleModuleColor(Color::srgb(0.36, 0.38, 0.21)),
            ))
            .id();

        app.update();

        assert!(app.world().get::<Mesh3d>(module_entity).is_some());
        assert!(app
            .world()
            .get::<MeshMaterial3d<StandardMaterial>>(module_entity)
            .is_some());
        assert!(app.world().get::<Collider>(module_entity).is_some());
    }

    #[test]
    fn a_module_cylinder_gets_a_mesh_material_and_matching_collider() {
        let mut app = test_app();
        let wheel_entity = app
            .world_mut()
            .spawn((
                LastBeaconVehicleModuleInstance {
                    asset_path: "unused.bsn".to_string(),
                },
                LastBeaconVehicleModuleCylinderShape(Cylinder::new(0.6, 0.4)),
                LastBeaconVehicleModuleColor(Color::srgb(0.16, 0.16, 0.18)),
            ))
            .id();

        app.update();

        assert!(app.world().get::<Mesh3d>(wheel_entity).is_some());
        assert!(app
            .world()
            .get::<MeshMaterial3d<StandardMaterial>>(wheel_entity)
            .is_some());
        assert!(app.world().get::<Collider>(wheel_entity).is_some());
    }

    #[test]
    fn a_cuboid_shape_outside_a_vehicle_module_is_left_alone() {
        // Not tagged with `LastBeaconVehicleModuleInstance` -- e.g. some
        // unrelated cuboid shape used elsewhere in the game (UI, level
        // geometry). Must not be accidentally physics-ified.
        let mut app = test_app();
        let entity = app
            .world_mut()
            .spawn(LastBeaconVehicleModuleCuboidShape(Cuboid::new(
                1.0, 1.0, 1.0,
            )))
            .id();

        app.update();

        assert!(app.world().get::<Mesh3d>(entity).is_none());
        assert!(app.world().get::<Collider>(entity).is_none());
    }
}
