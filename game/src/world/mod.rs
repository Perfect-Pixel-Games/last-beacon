//! Last Beacon World gameplay -- the launched-vehicle/expedition surface loop.
//!
//! This module owns World-only gameplay systems and components. It may depend
//! on [`crate::shared`] for anything that needs to reach Hub gameplay or the
//! options menu, but must never depend on [`crate::hub`] directly. If a future
//! feature seems to need that, route the data through [`crate::shared`] instead
//! of adding a `use crate::hub` here -- that one-way boundary is what keeps the
//! Hub and World gameplay spaces separate.

use bevy::{camera::ClearColorConfig, light::atmosphere::ScatteringMedium, prelude::*};
use foundation_runtime_library::prelude::*;

pub mod environment;
pub mod free_fly_camera;
pub mod landscape;
pub mod shader_erosion;

use free_fly_camera::{last_beacon_free_fly_camera_bundle, LastBeaconFreeFlyCameraPlugin};

/// Installs Last Beacon's World gameplay systems.
#[derive(Default)]
pub struct LastBeaconWorldGameplayPlugin;

impl Plugin for LastBeaconWorldGameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LastBeaconFreeFlyCameraPlugin)
            .init_resource::<landscape::LandscapeGenerationSettings>()
            .register_type::<LastBeaconLandscapeTestScene>()
            .add_systems(Update, initialize_last_beacon_landscape_test_scenes)
            .add_systems(
                Update,
                rebuild_landscape_terrain_when_settings_change
                    .run_if(resource_changed::<landscape::LandscapeGenerationSettings>),
            )
            .add_systems(Update, keep_pause_menu_visible_over_the_world_scene);
    }
}

/// Marker for the terrain mesh entity, used by
/// [`rebuild_landscape_terrain_when_settings_change`] to find and regenerate
/// the mesh in place when [`landscape::LandscapeGenerationSettings`] changes
/// (e.g. via the `landscape.set` debug console command).
#[derive(Component)]
struct LastBeaconLandscapeTerrainMesh {
    seed: u32,
}

/// Marker for the World landscape scene's own 3D camera, used by
/// [`keep_pause_menu_visible_over_the_world_scene`] to tell whether that
/// scene is currently open.
#[derive(Component)]
struct LastBeaconLandscapeCamera;

/// The World scene's 3D camera's normal render order, above the persistent
/// UI camera's order (100, `spawn_default_camera` in `game/src/lib.rs`) so it
/// paints over an assumed-empty UI layer during ordinary gameplay -- this is
/// the same arrangement already proven to render correctly.
const LAST_BEACON_LANDSCAPE_CAMERA_ORDER: isize = 200;

/// Order the UI camera renders at instead, while paused, so the pause menu
/// draws on top of the frozen World scene instead of being painted over by
/// it every frame.
const LAST_BEACON_UI_CAMERA_PAUSED_ORDER: isize = LAST_BEACON_LANDSCAPE_CAMERA_ORDER + 100;

/// Lets the pause menu (and any other UI opened while paused) render on top
/// of the World scene, instead of being painted over by it every frame.
///
/// Normally the World scene's camera renders above the persistent UI camera
/// (see [`LAST_BEACON_LANDSCAPE_CAMERA_ORDER`]) so it can fill the screen
/// without needing to know whether any UI is present. That's backwards for
/// the pause menu, which needs to show *on top* of the (now-frozen) World
/// scene. Rather than always reordering the two cameras -- which would make
/// every frame of ordinary gameplay depend on MSAA writeback compositing
/// correctly between differently-configured cameras -- this only swaps them
/// while [`FoundationPauseState`] reports paused *and* the World scene's own
/// camera exists, keeping ordinary gameplay on the unchanged, already-proven
/// rendering path.
fn keep_pause_menu_visible_over_the_world_scene(
    pause_state: Res<FoundationPauseState>,
    world_scene_cameras: Query<(), With<LastBeaconLandscapeCamera>>,
    mut ui_cameras: Query<&mut Camera, With<Camera2d>>,
) {
    let should_render_above_world_scene = pause_state.paused && !world_scene_cameras.is_empty();
    let (desired_order, desired_clear_color) = if should_render_above_world_scene {
        (LAST_BEACON_UI_CAMERA_PAUSED_ORDER, ClearColorConfig::None)
    } else {
        (100, ClearColorConfig::Default)
    };
    for mut ui_camera in &mut ui_cameras {
        ui_camera.order = desired_order;
        ui_camera.clear_color = desired_clear_color;
    }
}

/// Regenerates the landscape terrain mesh in place whenever
/// [`landscape::LandscapeGenerationSettings`] changes, so debug-console
/// parameter tweaks are visible immediately without reopening the scene.
fn rebuild_landscape_terrain_when_settings_change(
    settings: Res<landscape::LandscapeGenerationSettings>,
    mut meshes: ResMut<Assets<Mesh>>,
    terrain_query: Query<(&LastBeaconLandscapeTerrainMesh, &Mesh3d)>,
) {
    for (terrain_marker, mesh_handle) in &terrain_query {
        if let Some(mut mesh) = meshes.get_mut(&mesh_handle.0) {
            *mesh = landscape::build_landscape_mesh(terrain_marker.seed, &settings);
        }
    }
}

/// Temporary gameplay-testbed scene: a procedurally generated mountain
/// landscape under a physically-based sky, used to give gameplay systems a
/// large, visually realistic space to be tested against.
///
/// See `docs/plans/world-landscape-testbed/plan.md`. Not a shippable level --
/// no chunking/LOD, no collision, replace before shipping a real World scene.
#[derive(Clone, Copy, Debug, Component, Reflect)]
#[reflect(Component, Default)]
pub struct LastBeaconLandscapeTestScene {
    /// Seed for the procedural terrain's Perlin noise. Randomized by
    /// [`Default`] so each run generates a different landscape; the BSN
    /// scene authors this component with no fields set so it keeps that
    /// random default instead of pinning a fixed seed.
    pub seed: u32,
}

impl Default for LastBeaconLandscapeTestScene {
    fn default() -> Self {
        Self {
            seed: rand::random(),
        }
    }
}

type LandscapeTestSceneInitQuery<'world, 'state> = Query<
    'world,
    'state,
    (
        Entity,
        &'static LastBeaconLandscapeTestScene,
        Option<&'static SceneOwner>,
        Option<&'static ChildOf>,
    ),
    Added<LastBeaconLandscapeTestScene>,
>;

#[allow(clippy::too_many_arguments)]
fn initialize_last_beacon_landscape_test_scenes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut scattering_media: ResMut<Assets<ScatteringMedium>>,
    landscape_settings: Res<landscape::LandscapeGenerationSettings>,
    landscape_test_scenes: LandscapeTestSceneInitQuery,
    scene_owners: Query<&SceneOwner>,
    existing_terrain_owners: Query<Option<&SceneOwner>, With<LastBeaconLandscapeTerrainMesh>>,
) {
    for (scene_entity, landscape_test_scene, scene_owner, parent_link) in &landscape_test_scenes {
        let effective_scene_owner =
            effective_landscape_test_scene_owner(scene_owner.copied(), parent_link, &scene_owners);
        info!(
            "Initializing LastBeaconLandscapeTestScene on {scene_entity:?} with scene_owner={effective_scene_owner:?}, seed={}",
            landscape_test_scene.seed
        );

        // The BSN scene-loading pipeline can spawn this scene's root entity
        // more than once while resolving a single `open_scene` request (e.g.
        // an initial placeholder root later replaced by the fully-resolved
        // one), each triggering `Added<LastBeaconLandscapeTestScene>`. Since
        // this system's terrain/camera/atmosphere/sun are spawned
        // imperatively rather than as BSN-authored children, they aren't
        // cleaned up when that replacement happens, leaving duplicate
        // cameras behind -- harmless when they shared one render order, but
        // a genuine ambiguity (and blank/black render) now that this scene's
        // camera renders at the default order. Skip re-initializing for a
        // scene_owner this system has already built a terrain for.
        if effective_scene_owner.is_some()
            && existing_terrain_owners
                .iter()
                .any(|existing_owner| existing_owner.copied() == effective_scene_owner)
        {
            continue;
        }

        let terrain_mesh = meshes.add(landscape::build_landscape_mesh(
            landscape_test_scene.seed,
            &landscape_settings,
        ));
        let terrain_material = materials.add(StandardMaterial {
            base_color: Color::WHITE,
            perceptual_roughness: 0.92,
            metallic: 0.0,
            ..default()
        });
        let terrain_entity = commands
            .spawn((
                Mesh3d(terrain_mesh),
                MeshMaterial3d(terrain_material),
                Transform::IDENTITY,
                LastBeaconLandscapeTerrainMesh {
                    seed: landscape_test_scene.seed,
                },
                Name::new("Last Beacon Landscape Terrain"),
            ))
            .id();

        let (atmosphere_entity, sun_entity) =
            environment::spawn_landscape_sky_and_sun(&mut commands, &mut scattering_media);

        // Set back from the origin so the free-fly camera starts with the
        // landscape's mountains already in view, and comfortably above
        // whatever the actual terrain height happens to be at that spot for
        // this seed -- a fixed Y here previously landed the camera at
        // ground level (or inside the mesh) depending on the seed's noise
        // value at that exact point.
        const CAMERA_SPAWN_X: f32 = 0.0;
        const CAMERA_SPAWN_Z: f32 = 1500.0;
        const CAMERA_HEIGHT_ABOVE_TERRAIN: f32 = 400.0;
        let terrain_height_at_camera = landscape::landscape_height_at(
            landscape_test_scene.seed,
            CAMERA_SPAWN_X,
            CAMERA_SPAWN_Z,
            &landscape_settings,
        );
        let camera_position = Vec3::new(
            CAMERA_SPAWN_X,
            terrain_height_at_camera + CAMERA_HEIGHT_ABOVE_TERRAIN,
            CAMERA_SPAWN_Z,
        );
        // `spawn_default_camera` (`game/src/lib.rs`) always keeps a `Camera2d`
        // at order 100 alive for UI rendering, with the default
        // `ClearColorConfig::Default` -- which clears the whole viewport to
        // `ClearColor` every frame regardless of camera order. Rendering
        // above it (with `ClearColorConfig::None`) draws on top without
        // re-clearing, which is safe here because this scene's sky+terrain
        // fill the entire frame. See `keep_pause_menu_visible_over_the_world_scene`
        // for how the pause menu still gets to show on top of this scene
        // despite that ordering.
        let camera_entity = commands
            .spawn((
                Camera3d::default(),
                Camera {
                    order: LAST_BEACON_LANDSCAPE_CAMERA_ORDER,
                    clear_color: ClearColorConfig::None,
                    ..default()
                },
                LastBeaconLandscapeCamera,
                Transform::from_translation(camera_position),
                environment::landscape_camera_rendering_bundle(),
                last_beacon_free_fly_camera_bundle(),
                Name::new("Last Beacon Landscape Free-Fly Camera"),
            ))
            .id();

        if let Some(scene_owner) = effective_scene_owner {
            for generated_entity in [terrain_entity, atmosphere_entity, sun_entity, camera_entity] {
                commands.entity(generated_entity).insert(scene_owner);
            }
        }
    }
}

fn effective_landscape_test_scene_owner(
    scene_owner: Option<SceneOwner>,
    parent_link: Option<&ChildOf>,
    scene_owners: &Query<&SceneOwner>,
) -> Option<SceneOwner> {
    scene_owner.or_else(|| {
        parent_link.and_then(|parent_link| scene_owners.get(parent_link.parent()).ok().copied())
    })
}

/// Inputs for the `landscape.set` debug console command.
#[cfg(feature = "dev-tools")]
#[derive(Clone, Debug, ConsoleCommandInput)]
pub struct LandscapeSetParameterInputs {
    /// Name of the terrain-generation parameter to set; see
    /// [`landscape::apply_named_parameter`] for the full list of names.
    pub name: String,
    /// New value for the named parameter.
    pub value: f32,
}

/// Sets one terrain-generation parameter and triggers a mesh rebuild, so the
/// Shadertoy source's runtime-adjustable knobs can be explored live in-game.
#[cfg(feature = "dev-tools")]
#[console_command(name = "landscape.set")]
pub fn set_landscape_parameter(
    inputs: ConsoleInputs<LandscapeSetParameterInputs>,
    mut settings: ResMut<landscape::LandscapeGenerationSettings>,
) {
    match landscape::apply_named_parameter(&mut settings, &inputs.name, inputs.value) {
        Ok(()) => info!("landscape.{} = {}", inputs.name, inputs.value),
        Err(message) => error!("{message}"),
    }
}

/// Inputs for the `landscape.get` debug console command.
#[cfg(feature = "dev-tools")]
#[derive(Clone, Debug, ConsoleCommandInput)]
pub struct LandscapeGetParameterInputs {
    /// Name of the terrain-generation parameter to read; see
    /// [`landscape::named_parameter_value`] for the full list of names.
    pub name: String,
}

/// Prints the current value of one terrain-generation parameter.
#[cfg(feature = "dev-tools")]
#[console_command(name = "landscape.get")]
pub fn get_landscape_parameter(
    inputs: ConsoleInputs<LandscapeGetParameterInputs>,
    settings: Res<landscape::LandscapeGenerationSettings>,
) {
    match landscape::named_parameter_value(&settings, &inputs.name) {
        Ok(value) => info!("landscape.{} = {value}", inputs.name),
        Err(message) => error!("{message}"),
    }
}

/// Resets every terrain-generation parameter to the shader's own defaults.
#[cfg(feature = "dev-tools")]
#[console_command(name = "landscape.reset")]
pub fn reset_landscape_parameters(mut settings: ResMut<landscape::LandscapeGenerationSettings>) {
    *settings = landscape::LandscapeGenerationSettings::default();
    info!("landscape parameters reset to defaults");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_app_with_ui_camera() -> (App, Entity) {
        let mut app = App::new();
        app.init_resource::<FoundationPauseState>();
        app.add_systems(Update, keep_pause_menu_visible_over_the_world_scene);

        let ui_camera_entity = app.world_mut().spawn((Camera2d, Camera::default())).id();

        (app, ui_camera_entity)
    }

    #[test]
    fn ui_camera_stays_at_its_normal_order_when_not_paused() {
        let (mut app, ui_camera_entity) = test_app_with_ui_camera();
        app.world_mut().spawn(LastBeaconLandscapeCamera);

        app.update();

        let ui_camera = app
            .world()
            .get::<Camera>(ui_camera_entity)
            .expect("UI camera should exist");
        assert_eq!(ui_camera.order, 100);
        assert!(matches!(ui_camera.clear_color, ClearColorConfig::Default));
    }

    #[test]
    fn ui_camera_renders_above_world_scene_while_paused() {
        let (mut app, ui_camera_entity) = test_app_with_ui_camera();
        app.world_mut().spawn(LastBeaconLandscapeCamera);
        app.world_mut()
            .resource_mut::<FoundationPauseState>()
            .paused = true;

        app.update();

        let ui_camera = app
            .world()
            .get::<Camera>(ui_camera_entity)
            .expect("UI camera should exist");
        assert_eq!(ui_camera.order, LAST_BEACON_UI_CAMERA_PAUSED_ORDER);
        assert!(ui_camera.order > LAST_BEACON_LANDSCAPE_CAMERA_ORDER);
        assert!(matches!(ui_camera.clear_color, ClearColorConfig::None));
    }

    #[test]
    fn pausing_without_a_world_scene_open_does_not_reorder_the_ui_camera() {
        let (mut app, ui_camera_entity) = test_app_with_ui_camera();
        // No `LastBeaconLandscapeCamera` spawned -- e.g. paused in a scene
        // that doesn't have a competing high-order 3D camera.
        app.world_mut()
            .resource_mut::<FoundationPauseState>()
            .paused = true;

        app.update();

        let ui_camera = app
            .world()
            .get::<Camera>(ui_camera_entity)
            .expect("UI camera should exist");
        assert_eq!(ui_camera.order, 100);
        assert!(matches!(ui_camera.clear_color, ClearColorConfig::Default));
    }
}
