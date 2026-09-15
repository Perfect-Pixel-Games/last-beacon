//! Debug visualization for vehicle module sockets.
//!
//! Draws a small dot at every socket's actual world position plus a short
//! arrow along its outward normal, so a vehicle's attachment geometry can be
//! inspected visually instead of only through log/test output -- e.g. to
//! confirm two connected sockets' positions truly coincide and their normals
//! truly oppose, or to catch an unexpectedly-oriented normal at a glance.

use bevy::prelude::*;

use super::{
    LastBeaconVehicleFixedJoint, LastBeaconVehicleHingeJoint, LastBeaconVehicleModuleSocket,
};

/// Radius of the sphere drawn at each socket's position. Sized to read
/// clearly against the wheel (0.5m radius) and beam (0.3m cross-section)
/// modules at normal testbed viewing distance.
const SOCKET_DOT_RADIUS: f32 = 0.1;
/// Length of the arrow drawn along each socket's outward normal (local +X,
/// per the convention documented on `core.bsn`).
const SOCKET_NORMAL_ARROW_LENGTH: f32 = 0.5;

/// Draws a dot and outward-normal arrow at every [`LastBeaconVehicleModuleSocket`]
/// in the world, colored by whichever joint-type component it carries --
/// cyan for [`LastBeaconVehicleFixedJoint`] (an ordinary weld point), orange
/// for [`LastBeaconVehicleHingeJoint`] (a wheel axle), grey for a socket
/// that (invalidly) carries neither or both -- see
/// [`LastBeaconVehicleModuleSocket`] for why that's a `.bsn` authoring bug
/// [`wire_last_beacon_vehicle_connections`](super::wire_last_beacon_vehicle_connections)
/// warns about and skips, not something this gizmo silently hides.
pub fn draw_last_beacon_vehicle_socket_gizmos(
    mut gizmos: Gizmos,
    sockets: Query<(
        &GlobalTransform,
        &LastBeaconVehicleModuleSocket,
        Option<&LastBeaconVehicleFixedJoint>,
        Option<&LastBeaconVehicleHingeJoint>,
    )>,
) {
    for (global_transform, _socket, fixed_joint, hinge_joint) in &sockets {
        let position = global_transform.translation();
        let normal = global_transform.rotation() * Vec3::X;
        let color = match (fixed_joint, hinge_joint) {
            (Some(_), None) => Color::srgb(0.0, 0.85, 1.0),
            (None, Some(_)) => Color::srgb(1.0, 0.6, 0.0),
            (None, None) | (Some(_), Some(_)) => Color::srgb(0.5, 0.5, 0.5),
        };
        gizmos.sphere(position, SOCKET_DOT_RADIUS, color);
        gizmos.arrow(
            position,
            position + normal * SOCKET_NORMAL_ARROW_LENGTH,
            color,
        );
    }
}
