//! Last Beacon Hub gameplay -- the Beacon dashboard, hangar, garage, mission
//! control, fabrication, and silo-upgrades pages.
//!
//! This module owns Hub-only gameplay systems and components. It may depend on
//! [`crate::shared`] for anything that needs to reach World gameplay or the
//! options menu, but must never depend on [`crate::world`] directly. If a future
//! feature seems to need that, route the data through [`crate::shared`] instead
//! of adding a `use crate::world` here -- that one-way boundary is what keeps
//! the Hub and World gameplay spaces separate.

use bevy::prelude::*;

/// Installs Last Beacon's Hub gameplay systems.
///
/// Currently empty: this is the designated home for Hub gameplay as it is built
/// in follow-up features.
#[derive(Default)]
pub struct LastBeaconHubGameplayPlugin;

impl Plugin for LastBeaconHubGameplayPlugin {
    fn build(&self, _app: &mut App) {}
}
