//! Loads a [`LastBeaconVehicleModuleInstance`]'s referenced `.bsn` module
//! definition and applies it onto the same entity, mirroring
//! `game/src/ui_widgets.rs`'s `LastBeaconBsnWidget` load/apply pattern.

use std::sync::Arc;

use bevy::{
    asset::{AssetServer, Handle},
    prelude::*,
    scene::{ResolvedSceneRoot, ScenePatch},
};

use super::LastBeaconVehicleModuleInstance;

/// Tracks a module instance whose `.bsn` asset is still loading/resolving.
///
/// `pub(super)` so [`super::connection`]'s joint-wiring system can check
/// whether a referenced module instance has finished loading yet.
#[derive(Clone, Debug, Component)]
pub(super) struct LastBeaconVehicleModuleInstancePending {
    pub(super) scene_handle: Handle<ScenePatch>,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Component)]
struct LastBeaconVehicleModuleInstanceFailed {
    reason: String,
}

/// Starts loading a newly-authored module instance's `.bsn` module definition.
pub fn queue_last_beacon_vehicle_module_instances(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    module_instances: Query<
        (Entity, &LastBeaconVehicleModuleInstance),
        Added<LastBeaconVehicleModuleInstance>,
    >,
) {
    for (instance_entity, module_instance) in &module_instances {
        if module_instance.asset_path.is_empty() {
            warn!(
                "LastBeaconVehicleModuleInstance on {instance_entity:?} has an empty asset path."
            );
            continue;
        }

        let scene_handle = asset_server.load(module_instance.asset_path.clone());
        commands
            .entity(instance_entity)
            .insert(LastBeaconVehicleModuleInstancePending { scene_handle });
    }
}

/// Applies each pending module instance's loaded `.bsn` content onto its own
/// entity once the asset resolves, turning it into that module's rigid body.
pub fn apply_pending_last_beacon_vehicle_module_instances(world: &mut World) {
    let pending_instances = {
        let mut pending_query = world.query::<(Entity, &LastBeaconVehicleModuleInstancePending)>();
        pending_query
            .iter(world)
            .map(|(instance_entity, pending)| (instance_entity, pending.scene_handle.clone()))
            .collect::<Vec<_>>()
    };

    for (instance_entity, scene_handle) in pending_instances {
        let scene_patch_id = scene_handle.id();
        let resolve_result = world.resource_scope(
            |world, mut scene_patches: Mut<Assets<ScenePatch>>| -> Result<bool, String> {
                let Some(scene_patch) = scene_patches.get(scene_patch_id) else {
                    return Ok(false);
                };

                if scene_patch.resolved.is_some() {
                    return Ok(true);
                }

                let scene = scene_patches
                    .get_mut(scene_patch_id)
                    .and_then(|mut scene_patch| scene_patch.scene.take());
                let Some(scene) = scene else {
                    return Ok(false);
                };

                let asset_server = world.resource::<AssetServer>();
                let resolved_scene_root =
                    ResolvedSceneRoot::resolve(scene, asset_server, &scene_patches)
                        .map_err(|resolve_error| resolve_error.to_string())?;
                if let Some(mut scene_patch) = scene_patches.get_mut(scene_patch_id) {
                    scene_patch.resolved = Some(Arc::new(resolved_scene_root));
                }
                Ok(true)
            },
        );

        let scene_is_ready = match resolve_result {
            Ok(scene_is_ready) => scene_is_ready,
            Err(resolve_error) => {
                let failure_reason =
                    format!("Failed to resolve vehicle module instance: {resolve_error}");
                mark_module_instance_failed(world, instance_entity, failure_reason);
                continue;
            }
        };

        if !scene_is_ready {
            continue;
        }

        let apply_result = world.resource_scope(
            |world, scene_patches: Mut<Assets<ScenePatch>>| -> Result<(), String> {
                let Some(scene_patch) = scene_patches.get(scene_patch_id) else {
                    return Err("ScenePatch asset disappeared before module apply".to_string());
                };
                let Ok(mut instance_entity_mut) = world.get_entity_mut(instance_entity) else {
                    return Err("Module instance entity disappeared before apply".to_string());
                };

                scene_patch
                    .apply(&mut instance_entity_mut)
                    .map_err(|apply_error| apply_error.to_string())
            },
        );

        match apply_result {
            Ok(()) => {
                if let Ok(mut instance_entity_mut) = world.get_entity_mut(instance_entity) {
                    instance_entity_mut.remove::<LastBeaconVehicleModuleInstancePending>();
                }
            }
            Err(apply_error) => {
                let failure_reason = format!(
                    "Failed to apply vehicle module instance to {instance_entity:?}: {apply_error}"
                );
                mark_module_instance_failed(world, instance_entity, failure_reason);
            }
        }
    }
}

fn mark_module_instance_failed(world: &mut World, instance_entity: Entity, failure_reason: String) {
    error!("{failure_reason}");
    if let Ok(mut instance_entity_mut) = world.get_entity_mut(instance_entity) {
        instance_entity_mut.remove::<LastBeaconVehicleModuleInstancePending>();
        instance_entity_mut.insert(LastBeaconVehicleModuleInstanceFailed {
            reason: failure_reason,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use foundation_runtime_library::prelude::{
        FoundationBsnAssetPlugin, SceneAdded, SceneFocused, SceneLoadRequested,
    };
    use std::io::Write;

    #[derive(Clone, Debug, Default, Component, Reflect)]
    #[reflect(Component, Default)]
    struct FakeModuleMarker;

    fn test_app_with_asset_root(asset_root: &std::path::Path) -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(bevy::asset::AssetPlugin {
            file_path: asset_root.to_string_lossy().to_string(),
            ..default()
        });
        app.init_asset::<ScenePatch>();
        app.add_message::<SceneLoadRequested>();
        app.add_message::<SceneAdded>();
        app.add_message::<SceneFocused>();
        app.add_plugins(FoundationBsnAssetPlugin);
        app.register_type::<FakeModuleMarker>();
        app.add_systems(
            Update,
            (
                queue_last_beacon_vehicle_module_instances,
                apply_pending_last_beacon_vehicle_module_instances,
            )
                .chain(),
        );
        app
    }

    #[test]
    fn queuing_an_instance_with_an_empty_asset_path_does_not_panic() {
        let temp_dir = std::env::temp_dir().join("last_beacon_vehicle_instance_test_empty");
        std::fs::create_dir_all(&temp_dir).unwrap();
        let mut app = test_app_with_asset_root(&temp_dir);
        let instance_entity = app
            .world_mut()
            .spawn(LastBeaconVehicleModuleInstance {
                asset_path: String::new(),
            })
            .id();

        app.update();

        assert!(app
            .world()
            .get::<LastBeaconVehicleModuleInstancePending>(instance_entity)
            .is_none());
    }

    #[test]
    fn resolved_module_content_applies_onto_the_instance_entity() {
        let temp_dir = std::env::temp_dir().join("last_beacon_vehicle_instance_test_fixture");
        std::fs::create_dir_all(&temp_dir).unwrap();
        let fixture_path = temp_dir.join("fake_module.bsn");
        let mut fixture_file = std::fs::File::create(&fixture_path).unwrap();
        write!(
            fixture_file,
            "last_beacon::shared::vehicle::instance::tests::FakeModuleMarker"
        )
        .unwrap();
        drop(fixture_file);

        let mut app = test_app_with_asset_root(&temp_dir);
        let instance_entity = app
            .world_mut()
            .spawn((
                Name::new("TestInstance"),
                LastBeaconVehicleModuleInstance {
                    asset_path: "fake_module.bsn".to_string(),
                },
            ))
            .id();

        // Real disk I/O through the file-backed AssetServer is asynchronous
        // and its timing varies under load (e.g. running alongside the rest
        // of the test suite in parallel), so poll with a generous timeout
        // instead of a fixed frame count.
        let load_deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
        loop {
            app.update();
            if app
                .world()
                .get::<LastBeaconVehicleModuleInstancePending>(instance_entity)
                .is_none()
            {
                break;
            }
            assert!(
                std::time::Instant::now() < load_deadline,
                "vehicle module instance never finished loading/applying within the timeout"
            );
            std::thread::sleep(std::time::Duration::from_millis(5));
        }

        assert!(app
            .world()
            .get::<LastBeaconVehicleModuleInstancePending>(instance_entity)
            .is_none());
        assert!(app
            .world()
            .get::<FakeModuleMarker>(instance_entity)
            .is_some());
        // The vehicle-authored Name must survive the module apply, since
        // LastBeaconVehicleConnection resolves modules by this name.
        assert_eq!(
            app.world().get::<Name>(instance_entity).unwrap().as_str(),
            "TestInstance"
        );

        std::fs::remove_dir_all(&temp_dir).ok();
    }
}
