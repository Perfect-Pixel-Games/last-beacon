//! Debug visualization for vehicle module sockets.
//!
//! Draws a small dot at every socket's actual world position plus a short
//! arrow along its outward normal, so a vehicle's attachment geometry can be
//! inspected visually instead of only through log/test output -- e.g. to
//! confirm two connected sockets' positions truly coincide and their normals
//! truly oppose, or to catch an unexpectedly-oriented normal at a glance.

use bevy::prelude::*;

use super::connection::LastBeaconVehicleConnectionFailed;
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

/// Radius of the marker drawn at a failed connection's best-known anchor
/// position -- smaller than [`SOCKET_DOT_RADIUS`] so it doesn't visually
/// compete with real socket markers when both happen to be near each other.
const CONNECTION_FAILURE_MARKER_RADIUS: f32 = 0.15;
/// Bright red -- deliberately outside the cyan/orange/grey palette
/// [`draw_last_beacon_vehicle_socket_gizmos`] already uses, so a failure
/// reads unambiguously as "wrong," not just "a different socket kind."
const CONNECTION_FAILURE_COLOR: Color = Color::srgb(1.0, 0.0, 0.0);

/// Draws a red marker at every `LastBeaconVehicleConnectionFailed`
/// connection's best-known anchor position, so an author can spot a bad
/// tag/socket reference in-scene, not just in logs. Draws nothing for a
/// failed connection whose `marker_position` is `None` (neither named module
/// resolved, so there's nothing spatial to anchor a marker to) -- the
/// `warn!` logged when it failed is the only signal in that case.
pub fn draw_last_beacon_vehicle_connection_failure_gizmos(
    mut gizmos: Gizmos,
    failed_connections: Query<&LastBeaconVehicleConnectionFailed>,
) {
    for failed in &failed_connections {
        let Some(marker_position) = failed.marker_position else {
            continue;
        };
        gizmos.sphere(
            marker_position,
            CONNECTION_FAILURE_MARKER_RADIUS,
            CONNECTION_FAILURE_COLOR,
        );
    }
}
