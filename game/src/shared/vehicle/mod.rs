//! Foundational modular-vehicle attachment system.
//!
//! Modules (`.bsn`-authored blocks like a core, a beam, a plate, a wheel)
//! attach to each other via Avian3D physics joints to form a larger,
//! physically simulated structure -- the basis every playable vehicle will
//! eventually be built from. This module owns the attachment mechanism
//! itself; concrete gameplay modules (powered wheels, thrusters, weapons)
//! are future work built on top of it.

mod connection;
mod instance;
mod module_body;

pub use connection::wire_last_beacon_vehicle_connections;
pub use instance::{
    apply_pending_last_beacon_vehicle_module_instances, queue_last_beacon_vehicle_module_instances,
};
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
#[require(Transform)]
pub struct LastBeaconVehicleModuleSocket {
    /// Name used by a [`LastBeaconVehicleConnection`] to reference this socket.
    pub socket_name: String,
}

/// Authored in a *vehicle* `.bsn`, one per module placed in that vehicle.
///
/// A system loads the referenced module `.bsn` and applies its content onto
/// this same entity, so this entity becomes that module's rigid body once
/// resolved -- keeping whatever `Name`/`Transform` the vehicle `.bsn` already
/// authored here for placement and for [`LastBeaconVehicleConnection`] to
/// find it by name.
#[derive(Clone, Debug, Default, Component, Reflect)]
#[reflect(Component, Default)]
pub struct LastBeaconVehicleModuleInstance {
    /// Asset-relative path to the module's `.bsn` file, e.g.
    /// `"vehicle/modules/core.bsn"`.
    pub asset_path: String,
}

/// Which kind of Avian3D joint a [`LastBeaconVehicleConnection`] spawns.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Reflect)]
#[reflect(Default)]
pub enum LastBeaconVehicleJointKind {
    /// A rigid weld -- zero relative motion between the two sockets. The
    /// default for ordinary structural connections.
    #[default]
    Fixed,
    /// A free-spinning hinge, for wheels. Always rotates about the *wheel*
    /// module's own local Y axis, matching `Collider::cylinder`'s natural
    /// rotational symmetry axis -- see `connection::LAST_BEACON_VEHICLE_HINGE_AXIS`.
    Hinge,
}

/// A sibling entity under a vehicle root, naming two module instances (by
/// `Name`) and a socket on each (by `socket_name`) to join together.
///
/// [`wire_last_beacon_vehicle_connections`] resolves this once both named
/// module instances have finished loading and spawns the corresponding
/// Avian3D joint entity.
#[derive(Clone, Debug, Default, Component, Reflect)]
#[reflect(Component, Default)]
pub struct LastBeaconVehicleConnection {
    /// `Name` of the first module instance in this vehicle.
    pub module_a: String,
    /// Socket name on `module_a` to anchor this joint to.
    pub socket_a: String,
    /// `Name` of the second module instance in this vehicle.
    pub module_b: String,
    /// Socket name on `module_b` to anchor this joint to.
    pub socket_b: String,
    /// Which joint type to spawn.
    pub joint_kind: LastBeaconVehicleJointKind,
}

/// Installs Last Beacon's modular-vehicle attachment system.
#[derive(Default)]
pub struct LastBeaconVehiclePlugin;

impl Plugin for LastBeaconVehiclePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LastBeaconVehicleModuleBody>()
            .register_type::<LastBeaconVehicleWheelModuleBody>()
            .register_type::<LastBeaconVehicleModuleSocket>()
            .register_type::<LastBeaconVehicleModuleInstance>()
            .register_type::<LastBeaconVehicleConnection>()
            .register_type::<LastBeaconVehicleJointKind>()
            .add_systems(
                Update,
                (
                    materialize_last_beacon_vehicle_module_bodies,
                    materialize_last_beacon_vehicle_wheel_module_bodies,
                    queue_last_beacon_vehicle_module_instances,
                    apply_pending_last_beacon_vehicle_module_instances,
                    wire_last_beacon_vehicle_connections,
                )
                    .chain(),
            );
    }
}
