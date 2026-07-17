//! LastBeacon BSN asset scene catalog.
//!
//! Foundation owns the generic scene stack and temporary `.bsn` asset bridge.
//! LastBeacon owns stable scene keys and maps them to game-owned `.bsn` files.

use bevy::prelude::*;
use foundation_runtime_library::prelude::*;

/// Scene key for the first startup splash screen.
pub const PIXEL_PERFECT_SPLASH_SCENE: &str = "last-beacon/splash_pixel_perfect";
/// Scene key for the second startup splash screen.
pub const BEVY_SPLASH_SCENE: &str = "last-beacon/splash_bevy";
/// Scene key for the example main menu.
pub const MAIN_MENU_SCENE: &str = "last-beacon/main_menu";
/// Scene key for the stack-based options menu.
pub const OPTIONS_MENU_SCENE: &str = "last-beacon/options_menu";
/// Scene key for the scrolling credits scene.
pub const CREDITS_SCENE: &str = "last-beacon/credits";
/// Scene key for the small sample gameplay level.
pub const GAMEPLAY_LEVEL_SCENE: &str = "last-beacon/gameplay_level";
/// Scene key for the gameplay pause menu.
pub const PAUSE_MENU_SCENE: &str = "last-beacon/pause_menu";

/// Registers LastBeacon scene-stack keys with their `.bsn` asset files.
pub fn register_last_beacon_bsn_scenes(mut registry: ResMut<FoundationBsnSceneRegistry>) {
    // Keep scene keys stable while moving authored scene structure into asset files.
    registry.register_scene(
        PIXEL_PERFECT_SPLASH_SCENE,
        "scenes/pixel_perfect_splash.bsn",
    );
    registry.register_scene(BEVY_SPLASH_SCENE, "scenes/bevy_splash.bsn");
    registry.register_scene(MAIN_MENU_SCENE, "scenes/main_menu.bsn");
    registry.register_scene(OPTIONS_MENU_SCENE, "scenes/options_menu.bsn");
    registry.register_scene(CREDITS_SCENE, "scenes/credits.bsn");
    registry.register_scene(GAMEPLAY_LEVEL_SCENE, "scenes/gameplay_level.bsn");
    registry.register_scene(PAUSE_MENU_SCENE, "scenes/pause_menu.bsn");
}

/// Opens the first LastBeacon scene-stack entry.
pub fn open_initial_scene(mut scene_commands: MessageWriter<SceneCommand>) {
    let startup_scene_source = SceneSource::bsn_scene(PIXEL_PERFECT_SPLASH_SCENE);
    let startup_scene_options = OpenSceneOptions::default()
        .with_key("startup-splash")
        .with_presentation(ScenePresentation::FULLSCREEN);

    scene_commands.write(SceneCommand::Clear);
    scene_commands.write(SceneCommand::open_with_options(
        startup_scene_source,
        startup_scene_options,
    ));
}

/// Spawns LastBeacon runtime scene drivers that cannot live in static `.bsn` files.
pub fn spawn_requested_last_beacon_scene_drivers(
    mut commands: Commands,
    mut scene_requests: MessageReader<SceneLoadRequested>,
) {
    for scene_request in scene_requests.read() {
        let scene_owner = SceneOwner {
            scene_id: scene_request.scene_id,
        };
        let scene_key = scene_source_key(&scene_request.source);

        match scene_key.as_deref() {
            Some(PIXEL_PERFECT_SPLASH_SCENE) => {
                spawn_splash_driver(
                    &mut commands,
                    "Pixel Perfect",
                    BEVY_SPLASH_SCENE,
                    false,
                    true,
                    scene_owner,
                );
            }
            Some(BEVY_SPLASH_SCENE) => {
                spawn_splash_driver(
                    &mut commands,
                    "Bevy",
                    MAIN_MENU_SCENE,
                    true,
                    false,
                    scene_owner,
                );
            }
            Some(foundation_runtime_scene_key)
                if foundation_runtime_scene_key.starts_with("foundation/") => {}
            _ => {}
        }
    }
}

fn spawn_splash_driver(
    commands: &mut Commands,
    splash_name: &'static str,
    next_scene_key: &'static str,
    reset_stack_for_next_scene: bool,
    replace_current_scene: bool,
    scene_owner: SceneOwner,
) {
    let splash_timings = FoundationSplashTimings::new(0.75, 1.0, 0.75);
    let splash_screen = FoundationSplashScreen {
        timings: splash_timings,
        font_size: 72.0,
        next_scene_key: next_scene_key.to_string(),
        reset_stack_for_next_scene,
        replace_current_scene,
    };

    commands.spawn((Name::new(splash_name), splash_screen, scene_owner));
}

fn scene_source_key(scene_source: &SceneSource) -> Option<String> {
    match scene_source {
        SceneSource::BsnScene { key } => Some(key.clone()),
        SceneSource::Runtime { key } => Some(key.0.clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn required_scene_keys_are_stable() {
        assert_eq!(
            PIXEL_PERFECT_SPLASH_SCENE,
            "last-beacon/splash_pixel_perfect"
        );
        assert_eq!(BEVY_SPLASH_SCENE, "last-beacon/splash_bevy");
        assert_eq!(MAIN_MENU_SCENE, "last-beacon/main_menu");
        assert_eq!(OPTIONS_MENU_SCENE, "last-beacon/options_menu");
        assert_eq!(CREDITS_SCENE, "last-beacon/credits");
        assert_eq!(GAMEPLAY_LEVEL_SCENE, "last-beacon/gameplay_level");
        assert_eq!(PAUSE_MENU_SCENE, "last-beacon/pause_menu");
    }

    #[test]
    fn scene_registry_maps_keys_to_bsn_assets() {
        let mut registry = FoundationBsnSceneRegistry::default();
        registry.register_scene(MAIN_MENU_SCENE, "scenes/main_menu.bsn");

        assert_eq!(
            registry.resolve_scene_path(MAIN_MENU_SCENE),
            "scenes/main_menu.bsn"
        );
    }
}
