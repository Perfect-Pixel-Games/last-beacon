//! LastBeacon BSN asset scene catalog.
//!
//! Foundation owns the generic scene stack and temporary `.bsn` asset bridge.
//! LastBeacon owns stable scene keys and maps them to game-owned `.bsn` files.

use bevy::prelude::*;
use foundation_runtime_library::prelude::*;

/// Runtime scene key for the main-menu preload and splash-sequence root.
pub const MAIN_MENU_ROOT_SCENE: &str = "last-beacon/main_menu_root";
/// Scene key for the first startup splash screen.
pub const PIXEL_PERFECT_SPLASH_SCENE: &str = "last-beacon/splash_pixel_perfect";
/// Scene key for the second startup splash screen.
pub const BEVY_SPLASH_SCENE: &str = "last-beacon/splash_bevy";
/// Scene key for the example main menu.
pub const MAIN_MENU_SCENE: &str = "last-beacon/main_menu";
/// Scene key for the stack-based settings menu.
pub const OPTIONS_MENU_SCENE: &str = "last-beacon/options_menu";
/// Scene key for the persistent Beacon shell.
pub const BEACON_SCENE: &str = "last-beacon/beacon";
/// Stable stack key for the persistent Beacon shell.
pub const BEACON_SHELL_STACK_KEY: &str = "beacon-shell";
/// Stable stack key for the currently selected Beacon page.
pub const BEACON_PAGE_STACK_KEY: &str = "beacon-page";
/// Scene key for the Beacon dashboard.
pub const DASHBOARD_SCENE: &str = "last-beacon/dashboard";
/// Scene key for the Beacon hangar.
pub const HANGAR_SCENE: &str = "last-beacon/hangar";
/// Scene key for the Beacon garage.
pub const GARAGE_SCENE: &str = "last-beacon/garage";
/// Scene key for mission control.
pub const MISSION_CONTROL_SCENE: &str = "last-beacon/mission_control";
/// Scene key for the fabrication bay.
pub const FABRICATION_SCENE: &str = "last-beacon/fabrication";
/// Scene key for the reusable UI widget playground.
pub const UI_PLAYGROUND_SCENE: &str = "last-beacon/ui_playground";
/// Scene key for silo upgrades.
pub const SILO_UPGRADES_SCENE: &str = "last-beacon/silo_upgrades";
/// Scene key for the scrolling credits scene.
pub const CREDITS_SCENE: &str = "last-beacon/credits";
/// Scene key for the small sample gameplay level.
pub const GAMEPLAY_LEVEL_SCENE: &str = "last-beacon/gameplay_level";
/// Scene key for the gameplay pause menu.
pub const PAUSE_MENU_SCENE: &str = "last-beacon/pause_menu";

/// Button data that replaces the current Beacon page while leaving the shell in place.
#[derive(Clone, Debug, Default, Component, Reflect)]
#[reflect(Component, Default)]
pub struct LastBeaconBeaconPageButton {
    /// Scene key for the page that should become the selected Beacon tab content.
    pub scene_key: String,
}

#[derive(Clone, Copy, Debug, Component, PartialEq, Eq)]
pub(crate) enum LastBeaconMainMenuRootPhase {
    Starting,
    PixelPerfectSplash,
    BevySplash,
    MainMenu,
}

/// Runtime driver for the main-menu domain's preload and splash sequence.
#[derive(Clone, Copy, Debug, Component, Reflect)]
#[reflect(Component)]
pub struct LastBeaconMainMenuRoot;

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
    registry.register_scene(BEACON_SCENE, "scenes/beacon.bsn");
    registry.register_scene(DASHBOARD_SCENE, "scenes/dashboard.bsn");
    registry.register_scene(HANGAR_SCENE, "scenes/hangar.bsn");
    registry.register_scene(GARAGE_SCENE, "scenes/garage.bsn");
    registry.register_scene(MISSION_CONTROL_SCENE, "scenes/mission_control.bsn");
    registry.register_scene(FABRICATION_SCENE, "scenes/fabrication.bsn");
    registry.register_scene(UI_PLAYGROUND_SCENE, "scenes/ui_playground.bsn");
    registry.register_scene(SILO_UPGRADES_SCENE, "scenes/silo_upgrades.bsn");
    registry.register_scene(CREDITS_SCENE, "scenes/credits.bsn");
    registry.register_scene(GAMEPLAY_LEVEL_SCENE, "scenes/gameplay_level.bsn");
    registry.register_scene(PAUSE_MENU_SCENE, "scenes/pause_menu.bsn");
}

/// Registers scene preload relationships used to keep common UI transitions warm.
pub fn register_last_beacon_scene_preloads(mut preload_registry: ResMut<ScenePreloadRegistry>) {
    preload_registry.register_preloads(
        SceneSource::runtime(MAIN_MENU_ROOT_SCENE),
        [
            SceneSource::bsn_scene(PIXEL_PERFECT_SPLASH_SCENE),
            SceneSource::bsn_scene(BEVY_SPLASH_SCENE),
            SceneSource::bsn_scene(MAIN_MENU_SCENE),
            SceneSource::bsn_scene(OPTIONS_MENU_SCENE),
        ],
    );
    preload_registry.register_preloads(
        SceneSource::bsn_scene(BEACON_SCENE),
        [
            SceneSource::bsn_scene(DASHBOARD_SCENE),
            SceneSource::bsn_scene(HANGAR_SCENE),
            SceneSource::bsn_scene(GARAGE_SCENE),
            SceneSource::bsn_scene(MISSION_CONTROL_SCENE),
            SceneSource::bsn_scene(FABRICATION_SCENE),
            SceneSource::bsn_scene(SILO_UPGRADES_SCENE),
            SceneSource::bsn_scene(OPTIONS_MENU_SCENE),
        ],
    );
    preload_registry.register_preloads(
        SceneSource::bsn_scene(GAMEPLAY_LEVEL_SCENE),
        [
            SceneSource::bsn_scene(PAUSE_MENU_SCENE),
            SceneSource::bsn_scene(OPTIONS_MENU_SCENE),
        ],
    );
    preload_registry.register_preloads(SceneSource::bsn_scene(PAUSE_MENU_SCENE), []);
}

/// Opens the first LastBeacon scene-stack entry.
pub fn open_initial_scene(mut scene_commands: MessageWriter<SceneCommand>) {
    let startup_scene_commands = startup_scene_commands_or_default(
        std::env::args().skip(1),
        default_startup_scene_commands(),
    )
    .unwrap_or_else(|startup_scene_error| {
        error!(
            "Failed to parse startup scene override; using default startup scene: {startup_scene_error}"
        );
        default_startup_scene_commands()
    });

    for startup_scene_command in startup_scene_commands {
        scene_commands.write(startup_scene_command);
    }
}

fn default_startup_scene_commands() -> Vec<SceneCommand> {
    let startup_scene_source = SceneSource::runtime(MAIN_MENU_ROOT_SCENE);
    let startup_scene_options = OpenSceneOptions::default()
        .with_key("startup-splash")
        .with_presentation(ScenePresentation::FULLSCREEN);

    vec![
        SceneCommand::Clear,
        SceneCommand::open_with_options(startup_scene_source, startup_scene_options),
    ]
}

/// Spawns LastBeacon runtime scene drivers that cannot live in static `.bsn` files.
pub fn spawn_requested_last_beacon_scene_drivers(
    mut commands: Commands,
    mut scene_requests: MessageReader<SceneLoadRequested>,
    mut scene_commands: MessageWriter<SceneCommand>,
) {
    for scene_request in scene_requests.read() {
        let scene_owner = SceneOwner {
            scene_id: scene_request.scene_id,
        };
        let scene_key = scene_source_key(&scene_request.source);

        match scene_key.as_deref() {
            Some(MAIN_MENU_ROOT_SCENE) => {
                commands.spawn((
                    Name::new("Main Menu Root"),
                    LastBeaconMainMenuRoot,
                    LastBeaconMainMenuRootPhase::Starting,
                    scene_owner,
                ));
            }
            Some(BEACON_SCENE) => {
                open_beacon_page(&mut scene_commands, DASHBOARD_SCENE);
            }
            Some(foundation_runtime_scene_key)
                if foundation_runtime_scene_key.starts_with("foundation/") => {}
            _ => {}
        }
    }
}

/// Advances the main-menu root through its splash sequence and final menu handoff.
pub(crate) fn advance_last_beacon_main_menu_roots(
    mut scene_commands: MessageWriter<SceneCommand>,
    mut splash_completed: MessageReader<FoundationSplashCompleted>,
    scene_stack: Res<SceneStack>,
    preparation_registry: Res<ScenePreparationRegistry>,
    mut main_menu_roots: Query<&mut LastBeaconMainMenuRootPhase, With<LastBeaconMainMenuRoot>>,
) {
    let main_menu_preload_sources = main_menu_preload_sources();
    let main_menu_preloads_are_ready = main_menu_preload_sources.iter().all(|scene_source| {
        preparation_registry.status(scene_source) == Some(&ScenePreparationStatus::Ready)
    });

    for preload_source in &main_menu_preload_sources {
        scene_commands.write(SceneCommand::preload(preload_source.clone()));
    }

    for mut main_menu_root_phase in &mut main_menu_roots {
        if *main_menu_root_phase == LastBeaconMainMenuRootPhase::Starting {
            if !main_menu_preloads_are_ready {
                continue;
            }

            let splash_options = OpenSceneOptions::default()
                .with_key("main-menu-pixel-perfect-splash")
                .with_presentation(ScenePresentation::INPUT_BLOCKING_OVERLAY);
            scene_commands.write(SceneCommand::open_with_options(
                SceneSource::bsn_scene(PIXEL_PERFECT_SPLASH_SCENE),
                splash_options,
            ));
            *main_menu_root_phase = LastBeaconMainMenuRootPhase::PixelPerfectSplash;
        }
    }

    for splash_completed_message in splash_completed.read() {
        let Some(scene_owner) = splash_completed_message.scene_owner else {
            continue;
        };
        let Some(completed_scene_entry) = scene_stack.get(scene_owner.scene_id) else {
            continue;
        };
        let Some(completed_scene_key) = scene_source_key(&completed_scene_entry.source) else {
            continue;
        };

        for mut main_menu_root_phase in &mut main_menu_roots {
            match (*main_menu_root_phase, completed_scene_key.as_str()) {
                (LastBeaconMainMenuRootPhase::PixelPerfectSplash, PIXEL_PERFECT_SPLASH_SCENE) => {
                    scene_commands
                        .write(SceneCommand::Close(SceneTarget::Id(scene_owner.scene_id)));
                    let splash_options = OpenSceneOptions::default()
                        .with_key("main-menu-bevy-splash")
                        .with_presentation(ScenePresentation::INPUT_BLOCKING_OVERLAY);
                    scene_commands.write(SceneCommand::open_with_options(
                        SceneSource::bsn_scene(BEVY_SPLASH_SCENE),
                        splash_options,
                    ));
                    *main_menu_root_phase = LastBeaconMainMenuRootPhase::BevySplash;
                }
                (LastBeaconMainMenuRootPhase::BevySplash, BEVY_SPLASH_SCENE) => {
                    scene_commands.write(SceneCommand::clear_and_open(SceneSource::bsn_scene(
                        MAIN_MENU_SCENE,
                    )));
                    *main_menu_root_phase = LastBeaconMainMenuRootPhase::MainMenu;
                }
                _ => {}
            }
        }
    }
}

fn main_menu_preload_sources() -> Vec<SceneSource> {
    vec![
        SceneSource::bsn_scene(PIXEL_PERFECT_SPLASH_SCENE),
        SceneSource::bsn_scene(BEVY_SPLASH_SCENE),
        SceneSource::bsn_scene(MAIN_MENU_SCENE),
        SceneSource::bsn_scene(OPTIONS_MENU_SCENE),
    ]
}

/// Replaces the current Beacon page stack entry with a new non-blocking page scene.
pub fn navigate_last_beacon_beacon_pages(
    mut scene_commands: MessageWriter<SceneCommand>,
    buttons: Query<(&LastBeaconBeaconPageButton, &Interaction), Changed<Interaction>>,
) {
    for (button, button_interaction) in &buttons {
        if *button_interaction != Interaction::Pressed {
            continue;
        }

        let page_scene_key = button.scene_key.trim();
        if page_scene_key.is_empty() {
            warn!("LastBeaconBeaconPageButton has an empty scene_key");
            continue;
        }

        open_beacon_page(&mut scene_commands, page_scene_key);
    }
}

fn open_beacon_page(scene_commands: &mut MessageWriter<SceneCommand>, page_scene_key: &str) {
    // Keep the shared Beacon shell alive while replacing only the content layer.
    scene_commands.write(SceneCommand::Close(SceneTarget::Key(SceneKey::new(
        BEACON_PAGE_STACK_KEY,
    ))));

    let page_scene_options = OpenSceneOptions::default()
        .with_key(BEACON_PAGE_STACK_KEY)
        .with_presentation(ScenePresentation::NON_BLOCKING_OVERLAY);
    scene_commands.write(SceneCommand::open_with_options(
        SceneSource::bsn_scene(page_scene_key),
        page_scene_options,
    ));
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
        assert_eq!(MAIN_MENU_ROOT_SCENE, "last-beacon/main_menu_root");
        assert_eq!(BEVY_SPLASH_SCENE, "last-beacon/splash_bevy");
        assert_eq!(MAIN_MENU_SCENE, "last-beacon/main_menu");
        assert_eq!(OPTIONS_MENU_SCENE, "last-beacon/options_menu");
        assert_eq!(BEACON_SCENE, "last-beacon/beacon");
        assert_eq!(BEACON_SHELL_STACK_KEY, "beacon-shell");
        assert_eq!(BEACON_PAGE_STACK_KEY, "beacon-page");
        assert_eq!(DASHBOARD_SCENE, "last-beacon/dashboard");
        assert_eq!(HANGAR_SCENE, "last-beacon/hangar");
        assert_eq!(GARAGE_SCENE, "last-beacon/garage");
        assert_eq!(MISSION_CONTROL_SCENE, "last-beacon/mission_control");
        assert_eq!(FABRICATION_SCENE, "last-beacon/fabrication");
        assert_eq!(UI_PLAYGROUND_SCENE, "last-beacon/ui_playground");
        assert_eq!(SILO_UPGRADES_SCENE, "last-beacon/silo_upgrades");
        assert_eq!(CREDITS_SCENE, "last-beacon/credits");
        assert_eq!(GAMEPLAY_LEVEL_SCENE, "last-beacon/gameplay_level");
        assert_eq!(PAUSE_MENU_SCENE, "last-beacon/pause_menu");
    }

    #[test]
    fn default_startup_scene_opens_main_menu_root() {
        let startup_commands = default_startup_scene_commands();

        assert_eq!(
            startup_commands,
            vec![
                SceneCommand::Clear,
                SceneCommand::open_with_options(
                    SceneSource::runtime(MAIN_MENU_ROOT_SCENE),
                    OpenSceneOptions::default()
                        .with_key("startup-splash")
                        .with_presentation(ScenePresentation::FULLSCREEN),
                ),
            ]
        );
    }

    #[test]
    fn scene_preload_registry_registers_common_ui_targets() {
        let mut app = App::new();
        app.insert_resource(ScenePreloadRegistry::default());
        app.add_systems(Startup, register_last_beacon_scene_preloads);
        app.update();

        let preload_registry = app.world().resource::<ScenePreloadRegistry>();
        assert_eq!(
            preload_registry.preload_targets(&SceneSource::runtime(MAIN_MENU_ROOT_SCENE)),
            &[
                SceneSource::bsn_scene(PIXEL_PERFECT_SPLASH_SCENE),
                SceneSource::bsn_scene(BEVY_SPLASH_SCENE),
                SceneSource::bsn_scene(MAIN_MENU_SCENE),
                SceneSource::bsn_scene(OPTIONS_MENU_SCENE),
            ]
        );
        assert_eq!(
            preload_registry.preload_targets(&SceneSource::bsn_scene(BEACON_SCENE)),
            &[
                SceneSource::bsn_scene(DASHBOARD_SCENE),
                SceneSource::bsn_scene(HANGAR_SCENE),
                SceneSource::bsn_scene(GARAGE_SCENE),
                SceneSource::bsn_scene(MISSION_CONTROL_SCENE),
                SceneSource::bsn_scene(FABRICATION_SCENE),
                SceneSource::bsn_scene(SILO_UPGRADES_SCENE),
                SceneSource::bsn_scene(OPTIONS_MENU_SCENE),
            ]
        );
        assert_eq!(
            preload_registry.preload_targets(&SceneSource::bsn_scene(GAMEPLAY_LEVEL_SCENE)),
            &[
                SceneSource::bsn_scene(PAUSE_MENU_SCENE),
                SceneSource::bsn_scene(OPTIONS_MENU_SCENE),
            ]
        );
        assert!(preload_registry
            .preload_targets(&SceneSource::bsn_scene(PAUSE_MENU_SCENE))
            .is_empty());
    }

    #[test]
    fn main_menu_root_waits_for_full_preload_group_before_first_splash() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(FoundationSceneStackPlugin);
        app.add_message::<FoundationSplashCompleted>();
        app.add_systems(Update, advance_last_beacon_main_menu_roots);

        app.world_mut().spawn((
            LastBeaconMainMenuRoot,
            LastBeaconMainMenuRootPhase::Starting,
            SceneOwner {
                scene_id: SceneId(1),
            },
        ));
        app.update();

        let mut phases = app.world_mut().query::<&LastBeaconMainMenuRootPhase>();
        assert_eq!(
            phases
                .single(app.world())
                .copied()
                .expect("test should have one main-menu root phase"),
            LastBeaconMainMenuRootPhase::Starting,
            "the splash sequence should not start until Pixel Perfect, Bevy, main menu, and options are all prepared"
        );
    }

    #[test]
    fn main_menu_root_starts_first_splash_after_full_preload_group_is_ready() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(FoundationSceneStackPlugin);
        app.add_message::<FoundationSplashCompleted>();
        app.add_systems(Update, advance_last_beacon_main_menu_roots);

        app.world_mut().spawn((
            LastBeaconMainMenuRoot,
            LastBeaconMainMenuRootPhase::Starting,
            SceneOwner {
                scene_id: SceneId(1),
            },
        ));
        for scene_source in main_menu_preload_sources() {
            app.world_mut().write_message(ScenePreloadReady {
                source: scene_source,
            });
        }
        app.update();
        app.update();

        let mut phases = app.world_mut().query::<&LastBeaconMainMenuRootPhase>();
        assert_eq!(
            phases
                .single(app.world())
                .copied()
                .expect("test should have one main-menu root phase"),
            LastBeaconMainMenuRootPhase::PixelPerfectSplash,
            "the splash sequence should begin once the full main-menu preload group is ready"
        );
    }

    #[test]
    fn scene_registry_maps_keys_to_bsn_assets() {
        let mut registry = FoundationBsnSceneRegistry::default();
        registry.register_scene(MAIN_MENU_SCENE, "scenes/main_menu.bsn");
        registry.register_scene(BEACON_SCENE, "scenes/beacon.bsn");
        registry.register_scene(DASHBOARD_SCENE, "scenes/dashboard.bsn");
        registry.register_scene(SILO_UPGRADES_SCENE, "scenes/silo_upgrades.bsn");
        registry.register_scene(UI_PLAYGROUND_SCENE, "scenes/ui_playground.bsn");

        assert_eq!(
            registry.resolve_scene_path(MAIN_MENU_SCENE),
            "scenes/main_menu.bsn"
        );
        assert_eq!(
            registry.resolve_scene_path(BEACON_SCENE),
            "scenes/beacon.bsn"
        );
        assert_eq!(
            registry.resolve_scene_path(DASHBOARD_SCENE),
            "scenes/dashboard.bsn"
        );
        assert_eq!(
            registry.resolve_scene_path(SILO_UPGRADES_SCENE),
            "scenes/silo_upgrades.bsn"
        );
        assert_eq!(
            registry.resolve_scene_path(UI_PLAYGROUND_SCENE),
            "scenes/ui_playground.bsn"
        );
    }
}
