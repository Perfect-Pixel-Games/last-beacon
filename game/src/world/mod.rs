//! Last Beacon World gameplay -- the launched-vehicle/expedition surface loop.
//!
//! This module owns World-only gameplay systems and components. It may depend
//! on [`crate::shared`] for anything that needs to reach Hub gameplay or the
//! options menu, but must never depend on [`crate::hub`] directly. If a future
//! feature seems to need that, route the data through [`crate::shared`] instead
//! of adding a `use crate::hub` here -- that one-way boundary is what keeps the
//! Hub and World gameplay spaces separate.

use bevy::prelude::*;

/// Installs Last Beacon's World gameplay systems.
///
/// Currently empty: this is the designated home for World/expedition gameplay
/// as it is built in follow-up features.
#[derive(Default)]
pub struct LastBeaconWorldGameplayPlugin;

impl Plugin for LastBeaconWorldGameplayPlugin {
    fn build(&self, _app: &mut App) {}
}
