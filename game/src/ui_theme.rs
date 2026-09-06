//! Last Beacon's concrete UI theme values.
//!
//! The theming *mechanism* -- token enums, `FoundationUiTheme`, the
//! BSN-authorable themed wrapper components, and the resolver systems that
//! apply them -- lives entirely in `foundation_runtime_library::ui_theme`
//! and is installed automatically by `FoundationPlugin`. This module only
//! supplies Last Beacon's specific palette/spacing/type-scale values, loaded
//! from `ui/theme.toml` under this game's asset root. A different game built
//! on Foundation reuses the exact same widget library and mechanism by
//! shipping its own theme file instead of this one.

use foundation_runtime_library::ui_theme::{load_ui_theme_from_file, FoundationUiTheme};

/// Relative path (under this game's asset root) to Last Beacon's UI theme file.
pub const LAST_BEACON_UI_THEME_FILE_NAME: &str = "ui/theme.toml";

/// Loads Last Beacon's concrete UI theme from `ui/theme.toml`.
///
/// Called once from [`crate::LastBeaconPlugin::build`] to override the
/// loud placeholder theme `FoundationPlugin` installs by default.
pub fn load_last_beacon_ui_theme() -> FoundationUiTheme {
    load_ui_theme_from_file(crate::asset_root().join(LAST_BEACON_UI_THEME_FILE_NAME))
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::prelude::Color;
    use foundation_runtime_library::ui_theme::FoundationUiColorToken;

    #[test]
    fn last_beacons_shipped_theme_file_loads_its_real_accent_color() {
        let theme = load_last_beacon_ui_theme();

        assert_eq!(
            theme.color(FoundationUiColorToken::Accent),
            Color::srgb(0.984, 0.749, 0.141),
            "game/assets/ui/theme.toml should parse into Last Beacon's real gold accent, \
             not the engine's loud placeholder -- if this fails, the shipped theme file is \
             missing, malformed, or has drifted from this test's expected schema"
        );
    }
}
