//! LastBeacon gameplay plugin and Foundation engine integration.
//!
//! The Foundation engine launches this crate as a registered game. Concrete BSN
//! scenes live in [`scenes`], while reusable scene-stack, splash, menu, and
//! gameplay systems live in `foundation-runtime-library`.

use std::path::PathBuf;

use bevy::{
    asset::AssetPlugin,
    prelude::*,
    render::{
        settings::{Backends, InstanceFlags, RenderCreation, WgpuSettings},
        RenderPlugin,
    },
};
#[cfg(feature = "editor")]
use foundation_editor_library::prelude::*;
use foundation_runtime_library::prelude::*;

pub mod scenes;
pub mod ui_widgets;

/// Foundation game name used by the engine `--game` argument.
pub const GAME_NAME: &str = "last-beacon";

/// Returns LastBeacon's asset root.
///
/// Foundation uses this when launching LastBeacon as a statically registered game.
pub fn asset_root() -> PathBuf {
    if let Ok(explicit_asset_root) = std::env::var("FOUNDATION_ASSET_ROOT") {
        return PathBuf::from(explicit_asset_root);
    }

    if let Some(packaged_asset_root) = packaged_asset_root() {
        return packaged_asset_root;
    }

    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("assets")
}

fn packaged_asset_root() -> Option<PathBuf> {
    let executable_directory = std::env::current_exe()
        .ok()
        .and_then(|executable_path| executable_path.parent().map(std::path::Path::to_path_buf))?;
    let packaged_asset_root = executable_directory.join("assets");
    packaged_asset_root.is_dir().then_some(packaged_asset_root)
}

/// Runs LastBeacon with Foundation runtime systems installed.
///
/// This is shared by the thin Cargo binary wrapper and future packaged launchers.
pub fn run() -> AppExit {
    let asset_root = asset_root().to_string_lossy().to_string();
    let editor_enabled =
        cfg!(feature = "editor") && std::env::args().any(|argument| argument == "--editor");

    let mut app = App::new();
    app.insert_resource(ClearColor(Color::BLACK))
        .set_error_handler(bevy::ecs::error::error)
        .add_plugins(last_beacon_default_plugins(asset_root))
        .add_plugins(FoundationPlugin)
        .add_plugins(LastBeaconPlugin)
        .add_systems(Startup, spawn_default_camera);

    add_editor_plugins(&mut app, editor_enabled);

    app.run()
}

fn last_beacon_default_plugins(asset_root: String) -> impl PluginGroup {
    DefaultPlugins
        .build()
        .set(foundation_log_plugin())
        .disable::<GilrsPlugin>()
        .set(AssetPlugin {
            file_path: asset_root,
            ..default()
        })
        .set(RenderPlugin {
            render_creation: RenderCreation::Automatic(Box::new(WgpuSettings {
                backends: platform_render_backends(),
                instance_flags: InstanceFlags::empty().with_env(),
                ..default()
            })),
            ..default()
        })
}

fn platform_render_backends() -> Option<Backends> {
    // Keep the fast Windows path that made gameplay appear immediately, while disabling
    // validation layers separately so local Vulkan SDK warnings do not flood normal logs.
    #[cfg(target_os = "windows")]
    {
        Some(Backends::from_env().unwrap_or(Backends::VULKAN))
    }

    #[cfg(not(target_os = "windows"))]
    {
        Some(Backends::from_env().unwrap_or(Backends::PRIMARY))
    }
}

fn spawn_default_camera(mut commands: Commands) {
    let camera_order = 100;
    commands.spawn((
        Camera2d,
        Camera {
            order: camera_order,
            ..default()
        },
    ));
}

/// LastBeacon's Bevy plugin.
#[derive(Default)]
pub struct LastBeaconPlugin;

impl Plugin for LastBeaconPlugin {
    fn build(&self, app: &mut App) {
        // Credits JSON lives under this game's asset directory; the reusable
        // credits systems only search roots that games register here.
        app.insert_resource(FoundationCreditsAssetRoots {
            roots: vec![asset_root()],
        })
        .register_type::<SpinningCube>()
        .register_type::<LastBeaconPlaceholderCubeScene>()
        .register_type::<LastBeaconHideWhenSettingsOpen>()
        .register_type::<scenes::LastBeaconBeaconPageButton>()
        .register_type::<ui_widgets::LastBeaconBsnWidget>()
        .init_resource::<ui_widgets::LastBeaconUiTabSelections>()
        .init_resource::<ui_widgets::LastBeaconUiInputValues>()
        .init_resource::<ui_widgets::LastBeaconUiDropdownStates>()
        .init_resource::<ui_widgets::LastBeaconUiTextBoxScrollDrag>()
        .init_resource::<ui_widgets::LastBeaconUiTextBoxScrollOverrides>()
        .init_resource::<ui_widgets::LastBeaconUiTextBoxCaretScrollRequests>()
        .register_type::<ui_widgets::LastBeaconMainMenuPrimaryButton>()
        .register_type::<ui_widgets::LastBeaconBeaconPrimaryButton>()
        .register_type::<ui_widgets::LastBeaconBeaconTabButton>()
        .register_type::<ui_widgets::LastBeaconUiButton>()
        .register_type::<ui_widgets::LastBeaconUiTab>()
        .register_type::<ui_widgets::LastBeaconUiTabPanel>()
        .register_type::<ui_widgets::LastBeaconUiTextInput>()
        .register_type::<ui_widgets::LastBeaconUiTextScrollTrack>()
        .register_type::<ui_widgets::LastBeaconUiTextScrollThumb>()
        .register_type::<ui_widgets::LastBeaconUiTextHorizontalScrollTrack>()
        .register_type::<ui_widgets::LastBeaconUiTextHorizontalScrollThumb>()
        .register_type::<ui_widgets::LastBeaconUiSymbolIcon>()
        .register_type::<ui_widgets::LastBeaconUiRadioIcon>()
        .register_type::<ui_widgets::LastBeaconUiDropdownIcon>()
        .register_type::<ui_widgets::LastBeaconUiNumberInput>()
        .register_type::<ui_widgets::LastBeaconUiValueButton>()
        .register_type::<ui_widgets::LastBeaconUiValueText>()
        .register_type::<ui_widgets::LastBeaconUiDropdownToggle>()
        .register_type::<ui_widgets::LastBeaconUiDropdownPanel>()
        .register_type::<ui_widgets::LastBeaconUiSlider>()
        .register_type::<ui_widgets::LastBeaconUiSliderFill>()
        .add_systems(
            Startup,
            (
                ui_widgets::preload_last_beacon_ui_fonts,
                scenes::register_last_beacon_bsn_scenes,
                scenes::open_initial_scene,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                scenes::spawn_requested_last_beacon_scene_drivers,
                scenes::navigate_last_beacon_beacon_pages,
                // Must run after Foundation propagates SceneOwner onto newly
                // applied scene content, so newly-discovered widget slots are
                // already attributable to their owning scene before the scene
                // stack's visibility sync runs this frame. See
                // `queue_last_beacon_bsn_widgets`'s doc comment.
                ui_widgets::queue_last_beacon_bsn_widgets.after(propagate_loaded_bsn_scene_owners),
                ui_widgets::apply_last_beacon_ui_font,
                ui_widgets::initialize_last_beacon_ui_text_inputs,
                ui_widgets::focus_last_beacon_ui_text_inputs,
                ui_widgets::initialize_last_beacon_ui_text_scroll_tracks,
                ui_widgets::drag_last_beacon_ui_text_box_scrollbars,
                ui_widgets::scroll_last_beacon_ui_text_inputs,
                ui_widgets::request_last_beacon_ui_text_box_caret_scroll_for_keyboard_input,
                initialize_last_beacon_placeholder_cube_scenes,
            ),
        )
        .add_systems(
            Update,
            (
                ui_widgets::initialize_last_beacon_ui_value_text,
                ui_widgets::update_last_beacon_ui_number_inputs,
                ui_widgets::update_last_beacon_ui_value_buttons,
                ui_widgets::toggle_last_beacon_ui_dropdowns,
                ui_widgets::refresh_last_beacon_ui_dropdown_icons,
                ui_widgets::refresh_last_beacon_ui_dropdown_panels,
                ui_widgets::refresh_last_beacon_ui_radio_icons,
                ui_widgets::initialize_last_beacon_ui_sliders,
                ui_widgets::update_last_beacon_ui_sliders,
                ui_widgets::refresh_last_beacon_ui_slider_fills,
                ui_widgets::refresh_last_beacon_ui_value_text,
                ui_widgets::update_last_beacon_ui_tab_selection,
                ui_widgets::refresh_last_beacon_ui_tab_panels,
                exit_game_on_foundation_exit_request,
                spin_cube.run_if(foundation_is_not_paused),
            ),
        )
        .add_systems(
            Update,
            ui_widgets::apply_pending_last_beacon_bsn_widgets.run_if(foundation_is_not_paused),
        )
        .add_systems(
            PostUpdate,
            (
                ui_widgets::apply_last_beacon_ui_text_box_scroll_overrides,
                ui_widgets::refresh_last_beacon_ui_text_box_scrollbars,
            )
                .chain()
                .after(bevy::ui::UiSystems::PostLayout),
        )
        .add_systems(PostUpdate, ui_widgets::enforce_last_beacon_button_styles)
        .add_systems(Last, hide_last_beacon_menu_ui_behind_settings);
    }
}

fn exit_game_on_foundation_exit_request(
    mut exit_requests: MessageReader<FoundationExitRequested>,
    mut app_exit: MessageWriter<AppExit>,
) {
    for _exit_request in exit_requests.read() {
        app_exit.write(AppExit::Success);
    }
}

#[cfg(feature = "editor")]
fn add_editor_plugins(app: &mut App, editor_enabled: bool) {
    if editor_enabled {
        app.add_plugins(FoundationEditorPlugin);
        app.insert_resource(FoundationEditorMode { enabled: true });
        debug!("Foundation editor mode enabled for LastBeacon.");
    }
}

#[cfg(not(feature = "editor"))]
fn add_editor_plugins(_app: &mut App, editor_enabled: bool) {
    if editor_enabled {
        warn!("LastBeacon was built without editor support; ignoring `--editor`.");
    }
}

/// Inputs for LastBeacon's example console greeting command.
#[cfg(feature = "dev-tools")]
#[derive(Clone, Debug, ConsoleCommandInput)]
pub struct LastBeaconConsoleGreetingInputs {
    /// Name that should appear in the console greeting.
    pub name: String,
}

/// Example LastBeacon-authored console command.
#[cfg(feature = "dev-tools")]
#[console_command]
pub fn last_beacon_greeting(inputs: ConsoleInputs<LastBeaconConsoleGreetingInputs>) {
    info!("LastBeacon console greeting for {}.", inputs.name);
}

/// Inputs for LastBeacon's user-facing `example.say-hello` console command.
#[cfg(feature = "dev-tools")]
#[derive(Clone, Debug, ConsoleCommandInput)]
pub struct LastBeaconSayHelloInputs {
    /// Name that should be greeted by the example command.
    pub name: String,
}

/// Example command that demonstrates using an argument.
#[cfg(feature = "dev-tools")]
#[console_command(name = "example.say-hello")]
pub fn say_hello(inputs: ConsoleInputs<LastBeaconSayHelloInputs>) {
    info!("Hello, {}!", inputs.name);
}

/// Example simple command that has no arguments.
#[cfg(feature = "dev-tools")]
#[console_command(name = "example.say-hello-world")]
pub fn say_hello_world() {
    info!("Hello World!");
}

/// Hides menu UI while the Settings scene is open above it.
#[derive(Clone, Copy, Debug, Default, Component, Reflect)]
#[reflect(Component, Default)]
pub struct LastBeaconHideWhenSettingsOpen;

/// Placeholder rotating cube scene used until final menu, Beacon, and gameplay art exists.
#[derive(Clone, Debug, Component, Reflect)]
#[reflect(Component, Default)]
pub struct LastBeaconPlaceholderCubeScene {
    /// Cube color name: `green`, `red`, or `blue`.
    pub cube_color: String,
    /// Edge length of the generated cube in world units.
    pub cube_size: f32,
}

impl Default for LastBeaconPlaceholderCubeScene {
    fn default() -> Self {
        Self {
            cube_color: "blue".to_string(),
            cube_size: 2.0,
        }
    }
}

/// Example gameplay component used by LastBeacon-specific systems.
#[derive(Clone, Copy, Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct SpinningCube;

type PlaceholderCubeSceneInitQuery<'world, 'state> = Query<
    'world,
    'state,
    (
        Entity,
        &'static LastBeaconPlaceholderCubeScene,
        Option<&'static SceneOwner>,
        Option<&'static ChildOf>,
    ),
    Added<LastBeaconPlaceholderCubeScene>,
>;

fn initialize_last_beacon_placeholder_cube_scenes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    placeholder_scenes: PlaceholderCubeSceneInitQuery,
    scene_owners: Query<&SceneOwner>,
) {
    for (placeholder_scene_entity, placeholder_scene, scene_owner, parent_link) in
        &placeholder_scenes
    {
        let effective_scene_owner =
            effective_placeholder_scene_owner(scene_owner.copied(), parent_link, &scene_owners);
        debug!(
            "Initializing LastBeaconPlaceholderCubeScene on {placeholder_scene_entity:?} with scene_owner={effective_scene_owner:?}"
        );

        let cube_size = placeholder_scene.cube_size;
        let cube_mesh = meshes.add(Cuboid::from_size(Vec3::splat(cube_size)));
        let cube_material = materials.add(StandardMaterial {
            base_color: placeholder_cube_color(&placeholder_scene.cube_color),
            ..default()
        });
        let cube_position = Vec3::new(0.0, cube_size * 0.5, 0.0);

        let cube_entity = commands
            .spawn((
                Mesh3d(cube_mesh),
                MeshMaterial3d(cube_material),
                Transform::from_translation(cube_position),
                SpinningCube,
                Name::new("Last Beacon Placeholder Cube"),
            ))
            .id();

        let light_illuminance = 12_000.0;
        let light_position = Vec3::new(3.0, 5.0, 3.0);
        let light_target = Vec3::ZERO;

        let light_entity = commands
            .spawn((
                DirectionalLight {
                    illuminance: light_illuminance,
                    shadow_maps_enabled: true,
                    ..default()
                },
                Transform::from_translation(light_position).looking_at(light_target, Vec3::Y),
                Name::new("Last Beacon Placeholder Light"),
            ))
            .id();

        let camera_position = Vec3::new(4.0, 3.0, 6.0);
        let camera_target = Vec3::new(0.0, 0.75, 0.0);

        let camera_entity = commands
            .spawn((
                Camera3d::default(),
                Transform::from_translation(camera_position).looking_at(camera_target, Vec3::Y),
                Name::new("Last Beacon Placeholder Camera"),
            ))
            .id();

        if let Some(scene_owner) = effective_scene_owner {
            for generated_entity in [cube_entity, light_entity, camera_entity] {
                commands.entity(generated_entity).insert(scene_owner);
            }
        }
    }
}

fn effective_placeholder_scene_owner(
    scene_owner: Option<SceneOwner>,
    parent_link: Option<&ChildOf>,
    scene_owners: &Query<&SceneOwner>,
) -> Option<SceneOwner> {
    scene_owner.or_else(|| {
        parent_link.and_then(|parent_link| scene_owners.get(parent_link.parent()).ok().copied())
    })
}

fn placeholder_cube_color(cube_color: &str) -> Color {
    match cube_color.trim().to_ascii_lowercase().as_str() {
        "green" => Color::srgb(0.18, 0.78, 0.34),
        "red" => Color::srgb(0.86, 0.18, 0.16),
        "blue" => Color::srgb(0.22, 0.42, 0.95),
        unknown_color => {
            warn!("Unknown LastBeaconPlaceholderCubeScene color `{unknown_color}`; using blue");
            Color::srgb(0.22, 0.42, 0.95)
        }
    }
}

fn spin_cube(time: Res<Time>, mut spinning_entities: Query<&mut Transform, With<SpinningCube>>) {
    for mut transform in &mut spinning_entities {
        let spin_speed_radians_per_second = 0.8;
        let spin_delta = spin_speed_radians_per_second * time.delta_secs();
        transform.rotate_y(spin_delta);
    }
}

fn hide_last_beacon_menu_ui_behind_settings(
    scene_stack: Option<Res<SceneStack>>,
    mut menu_roots: Query<&mut Node, With<LastBeaconHideWhenSettingsOpen>>,
) {
    let Some(scene_stack) = scene_stack else {
        return;
    };

    let settings_scene_is_open = scene_stack.entries().iter().any(|scene_stack_entry| {
        matches!(
            &scene_stack_entry.source,
            SceneSource::BsnScene { key } if key == scenes::OPTIONS_MENU_SCENE
        )
    });

    for mut menu_node in &mut menu_roots {
        let desired_display = if settings_scene_is_open {
            // Hide only marked UI roots; generated cube/gameplay entities stay visible.
            Display::None
        } else {
            Display::Flex
        };

        if menu_node.display != desired_display {
            menu_node.display = desired_display;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_name_matches_foundation_launch_argument() {
        assert_eq!(GAME_NAME, "last-beacon");
    }

    #[test]
    fn settings_overlay_hides_marked_menu_ui_without_closing_lower_scene() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(FoundationSceneStackPlugin);
        app.add_systems(Last, hide_last_beacon_menu_ui_behind_settings);

        app.world_mut()
            .write_message(SceneCommand::open(SceneSource::runtime("main-menu")));
        app.update();

        let scene_root = app
            .world_mut()
            .spawn(SceneOwner {
                scene_id: SceneId(1),
            })
            .id();
        let menu_root = app
            .world_mut()
            .spawn((Node::default(), LastBeaconHideWhenSettingsOpen))
            .id();
        app.world_mut().entity_mut(scene_root).add_child(menu_root);

        let settings_options = OpenSceneOptions::default()
            .with_key("options-menu")
            .with_presentation(ScenePresentation::INPUT_BLOCKING_OVERLAY);
        app.world_mut()
            .write_message(SceneCommand::open_with_options(
                SceneSource::bsn_scene(scenes::OPTIONS_MENU_SCENE),
                settings_options,
            ));
        app.update();

        assert_eq!(
            app.world().get::<Node>(menu_root).map(|node| node.display),
            Some(Display::None),
            "settings should hide marked menu UI roots while leaving the lower scene stacked",
        );
        assert_eq!(
            app.world().resource::<SceneStack>().len(),
            2,
            "settings should not close the lower scene it is hiding UI from",
        );

        app.world_mut().write_message(SceneCommand::CloseCurrent);
        app.update();

        assert_eq!(
            app.world().get::<Node>(menu_root).map(|node| node.display),
            Some(Display::Flex),
            "closing settings should restore marked menu UI display",
        );
    }

    #[test]
    fn last_beacon_registers_its_credits_asset_root() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(FoundationPlugin);
        app.add_plugins(LastBeaconPlugin);

        let credits_asset_roots = app.world().resource::<FoundationCreditsAssetRoots>();
        assert!(
            credits_asset_roots.roots.contains(&asset_root()),
            "LastBeacon should search its own asset directory for credits JSON"
        );
    }

    #[test]
    #[cfg(feature = "dev-tools")]
    fn last_beacon_console_command_is_linked_into_last_beacon_binary() {
        let registry = FoundationConsoleRegistry::default();

        assert!(registry
            .commands()
            .iter()
            .any(|command| command.name == "last_beacon_greeting"));
    }

    #[test]
    #[cfg(feature = "dev-tools")]
    fn say_hello_console_command_uses_overridden_name() {
        let registry = FoundationConsoleRegistry::default();
        let say_hello_command = registry
            .commands()
            .iter()
            .find(|command| command.name == "example.say-hello")
            .expect("say_hello should register with its overridden console name");
        let parameters = (say_hello_command.parameters)();

        assert_eq!(parameters[0].name, "name");
        assert_eq!(parameters[0].type_name, "String");
    }
}
