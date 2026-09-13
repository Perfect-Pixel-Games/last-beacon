//! Foundational modular-vehicle attachment system.
//!
//! Modules (`.bsn`-authored blocks like a core, a beam, a plate, a wheel)
//! attach to each other via Avian3D physics joints to form a larger,
//! physically simulated structure -- the basis every playable vehicle will
//! eventually be built from. This module owns the attachment mechanism
//! itself; concrete gameplay modules (powered wheels, thrusters, weapons)
//! are future work built on top of it.

mod module_body;

pub use module_body::{
    materialize_last_beacon_vehicle_module_bodies,
    materialize_last_beacon_vehicle_wheel_module_bodies,
};

use bevy::prelude::*;

/// Authored on a box-shaped module's root entity (Core, Beam, Plate). A
/// reactive system turns this into the real mesh, material, rigid body,
/// collider, and mass -- this project's `.bsn` grammar can construct plain
/// structs like this one via reflection, but can never call a constructor
/// function like `Collider::cuboid(...)` directly.
#[derive(Clone, Debug, Component, Reflect)]
#[reflect(Component, Default)]
pub struct LastBeaconVehicleModuleBody {
    /// Full box width along local X, in meters.
    pub size_x: f32,
    /// Full box height along local Y, in meters.
    pub size_y: f32,
    /// Full box depth along local Z, in meters.
    pub size_z: f32,
    /// Rigid body mass in kilograms.
    pub mass: f32,
    /// Named color, resolved by `module_body::last_beacon_vehicle_module_color`.
    pub color: String,
}

impl Default for LastBeaconVehicleModuleBody {
    fn default() -> Self {
        Self {
            size_x: 1.0,
            size_y: 1.0,
            size_z: 1.0,
            mass: 10.0,
            color: "steel_blue".to_string(),
        }
    }
}

/// Authored on a wheel module's root entity. Separate from
/// [`LastBeaconVehicleModuleBody`] because a wheel is a cylinder, not a box,
/// and needs its own collider/mesh construction.
#[derive(Clone, Debug, Component, Reflect)]
#[reflect(Component, Default)]
pub struct LastBeaconVehicleWheelModuleBody {
    /// Wheel radius in meters.
    pub radius: f32,
    /// Full wheel width (the cylinder's height, along local Y) in meters.
    pub width: f32,
    /// Rigid body mass in kilograms.
    pub mass: f32,
    /// Named color, resolved by `module_body::last_beacon_vehicle_module_color`.
    pub color: String,
}

impl Default for LastBeaconVehicleWheelModuleBody {
    fn default() -> Self {
        Self {
            radius: 0.5,
            width: 0.3,
            mass: 5.0,
            color: "charcoal".to_string(),
        }
    }
}

/// Marks a child entity of a module's root as a named attachment point.
///
/// The entity's own `Transform.translation` is that socket's local anchor
/// offset relative to the module's rigid body -- authored directly as a
/// plain `Transform` in `.bsn`, no marker/system workaround needed since
/// `Transform` is a plain public-field struct.
#[derive(Clone, Debug, Default, Component, Reflect)]
#[reflect(Component, Default)]
pub struct LastBeaconVehicleModuleSocket {
    /// Name used by a [`LastBeaconVehicleConnection`] to reference this socket.
    pub socket_name: String,
}

/// Installs Last Beacon's modular-vehicle attachment system.
#[derive(Default)]
pub struct LastBeaconVehiclePlugin;

impl Plugin for LastBeaconVehiclePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LastBeaconVehicleModuleBody>()
            .register_type::<LastBeaconVehicleWheelModuleBody>()
            .register_type::<LastBeaconVehicleModuleSocket>()
            .add_systems(
                Update,
                (
                    materialize_last_beacon_vehicle_module_bodies,
                    materialize_last_beacon_vehicle_wheel_module_bodies,
                ),
            );
    }
}
