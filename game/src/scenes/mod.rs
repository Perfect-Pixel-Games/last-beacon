//! LastBeacon BSN scene catalog.
//!
//! Foundation owns the generic scene stack. LastBeacon owns these concrete scene
//! keys and maps each key to one scene-specific Rust module.

mod bevy_splash;
mod credits;
mod gameplay;
mod main_menu;
mod options_menu;
mod pause_menu;
mod pixel_perfect_splash;

use bevy::prelude::*;
use foundation_runtime_library::prelude::*;

pub use bevy_splash::spawn_bevy_splash_scene;
pub use credits::credits_scene;
pub use gameplay::gameplay_level_scene;
pub use main_menu::main_menu_scene;
pub use options_menu::options_menu_scene;
pub use pause_menu::pause_menu_scene;
pub use pixel_perfect_splash::spawn_pixel_perfect_splash_scene;

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

/// Spawns requested LastBeacon scenes from Foundation scene-load messages.
pub fn spawn_requested_last_beacon_scenes(
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
                spawn_pixel_perfect_splash_scene(&mut commands, scene_owner);
            }
            Some(BEVY_SPLASH_SCENE) => {
                spawn_bevy_splash_scene(&mut commands, scene_owner);
            }
            Some(MAIN_MENU_SCENE) => {
                commands.spawn_scene(main_menu_scene(scene_owner));
            }
            Some(OPTIONS_MENU_SCENE) => {
                commands.spawn_scene(options_menu_scene(scene_owner));
            }
            Some(CREDITS_SCENE) => {
                commands.spawn_scene(credits_scene(scene_owner));
            }
            Some(GAMEPLAY_LEVEL_SCENE) => {
                commands.spawn_scene(gameplay_level_scene(scene_owner));
            }
            Some(PAUSE_MENU_SCENE) => {
                commands.spawn_scene(pause_menu_scene(scene_owner));
            }
            Some(foundation_runtime_scene_key)
                if foundation_runtime_scene_key.starts_with("foundation/") => {}
            Some(unknown_scene_key) => {
                warn!("Unknown LastBeacon scene requested: {unknown_scene_key}");
            }
            None => {
                warn!("LastBeacon received a scene source without a scene key");
            }
        }
    }
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
}
