//! Foundational modular-vehicle attachment system.
//!
//! Modules (`.bsn`-authored blocks like a core, a beam, a plate, a wheel)
//! attach to each other via Avian3D physics joints to form a larger,
//! physically simulated structure -- the basis every playable vehicle will
//! eventually be built from. This module owns the attachment mechanism
//! itself; concrete gameplay modules (powered wheels, thrusters, weapons)
//! are future work built on top of it.

use bevy::prelude::*;

/// Installs Last Beacon's modular-vehicle attachment system.
#[derive(Default)]
pub struct LastBeaconVehiclePlugin;

impl Plugin for LastBeaconVehiclePlugin {
    fn build(&self, _app: &mut App) {}
}
