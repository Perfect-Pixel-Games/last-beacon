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
pub mod landscape;

/// Installs Last Beacon's World gameplay systems.
#[derive(Default)]
pub struct LastBeaconWorldGameplayPlugin;

impl Plugin for LastBeaconWorldGameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FoundationFreeFlyCameraPlugin)
            .register_type::<LastBeaconLandscapeTestScene>()
            .add_systems(Update, initialize_last_beacon_landscape_test_scenes);
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
    /// Seed for the procedural terrain's Perlin noise.
    pub seed: u32,
}

impl Default for LastBeaconLandscapeTestScene {
    fn default() -> Self {
        Self { seed: 1337 }
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
    landscape_test_scenes: LandscapeTestSceneInitQuery,
    scene_owners: Query<&SceneOwner>,
) {
    for (scene_entity, landscape_test_scene, scene_owner, parent_link) in &landscape_test_scenes {
        let effective_scene_owner =
            effective_landscape_test_scene_owner(scene_owner.copied(), parent_link, &scene_owners);
        debug!(
            "Initializing LastBeaconLandscapeTestScene on {scene_entity:?} with scene_owner={effective_scene_owner:?}"
        );

        let terrain_mesh = meshes.add(landscape::build_landscape_mesh(landscape_test_scene.seed));
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
        );
        let camera_position = Vec3::new(
            CAMERA_SPAWN_X,
            terrain_height_at_camera + CAMERA_HEIGHT_ABOVE_TERRAIN,
            CAMERA_SPAWN_Z,
        );
        // `spawn_default_camera` (`game/src/lib.rs`) always keeps a `Camera2d`
        // at order 100 alive for UI rendering, with the default
        // `ClearColorConfig::Default` -- which clears the whole viewport to
        // `ClearColor` every frame regardless of camera order. A 3D camera at
        // the default order (0) would render *before* that UI camera and
        // then be wiped out by its clear pass. Rendering after it instead
        // (higher order) with `ClearColorConfig::None` draws on top without
        // re-clearing, which is safe here because this scene's sky+terrain
        // fill the entire frame (nothing needs to show through from the
        // otherwise-empty UI layer underneath).
        let camera_entity = commands
            .spawn((
                Camera3d::default(),
                Camera {
                    order: 200,
                    clear_color: ClearColorConfig::None,
                    ..default()
                },
                Transform::from_translation(camera_position),
                environment::landscape_camera_rendering_bundle(),
                foundation_free_fly_camera_bundle(),
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
