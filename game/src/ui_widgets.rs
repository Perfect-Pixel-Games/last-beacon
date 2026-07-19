//! Last Beacon UI widget composition support.
//!
//! Last Beacon's authored UI scenes can place reusable `.bsn` widget assets under
//! lightweight widget slots. This keeps scene files focused on layout while common
//! visual pieces live under `assets/ui/widgets/`.

use std::sync::Arc;

use bevy::{
    prelude::*,
    scene::{ResolvedSceneRoot, ScenePatch},
    text::FontSource,
};

/// Requests that a reusable Last Beacon BSN widget asset be applied to this entity.
#[derive(Clone, Debug, Default, Component, Reflect)]
#[reflect(Component, Default)]
pub struct LastBeaconBsnWidget {
    /// Asset-relative path to the `.bsn` widget that should be applied to this slot.
    pub asset_path: String,
}

#[derive(Clone, Debug, Component)]
struct LastBeaconBsnWidgetPending {
    asset_path: String,
    scene_handle: Handle<ScenePatch>,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Component)]
struct LastBeaconBsnWidgetFailed {
    reason: String,
}

/// Starts loading newly-authored widget slots.
pub fn queue_last_beacon_bsn_widgets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    widget_slots: Query<(Entity, &LastBeaconBsnWidget), Added<LastBeaconBsnWidget>>,
) {
    for (widget_slot_entity, widget_slot) in &widget_slots {
        if widget_slot.asset_path.is_empty() {
            warn!("LastBeaconBsnWidget on {widget_slot_entity:?} has an empty asset path.");
            continue;
        }

        // Store the handle on the slot so the exclusive apply system can patch this entity later.
        let scene_handle = asset_server.load(widget_slot.asset_path.clone());
        commands
            .entity(widget_slot_entity)
            .insert(LastBeaconBsnWidgetPending {
                asset_path: widget_slot.asset_path.clone(),
                scene_handle,
            });
    }
}

/// Applies loaded widget scene patches onto their slot entities.
/// Applies Last Beacon's current UI font to newly spawned text.
pub fn apply_last_beacon_ui_font(
    asset_server: Res<AssetServer>,
    mut text_fonts: Query<&mut TextFont, Added<TextFont>>,
) {
    let ui_font = asset_server.load("fonts/NotoSans-Regular.ttf");
    for mut text_font in &mut text_fonts {
        text_font.font = FontSource::Handle(ui_font.clone());
    }
}

pub fn apply_pending_last_beacon_bsn_widgets(world: &mut World) {
    let pending_widgets = {
        let mut pending_query = world.query::<(Entity, &LastBeaconBsnWidgetPending)>();
        pending_query
            .iter(world)
            .map(|(widget_slot_entity, pending_widget)| {
                (
                    widget_slot_entity,
                    pending_widget.asset_path.clone(),
                    pending_widget.scene_handle.clone(),
                )
            })
            .collect::<Vec<_>>()
    };

    for (widget_slot_entity, asset_path, scene_handle) in pending_widgets {
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

                // Resolve dependencies using the same Bevy scene-patch path as Foundation's BSN loader.
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
                let failure_reason = format!(
                    "Failed to resolve Last Beacon BSN widget `{asset_path}`: {resolve_error}"
                );
                mark_widget_failed(world, widget_slot_entity, failure_reason);
                continue;
            }
        };

        if !scene_is_ready {
            continue;
        }

        let apply_result = world.resource_scope(
            |world, scene_patches: Mut<Assets<ScenePatch>>| -> Result<(), String> {
                let Some(scene_patch) = scene_patches.get(scene_patch_id) else {
                    return Err("ScenePatch asset disappeared before widget apply".to_string());
                };
                let Ok(mut widget_slot_entity_mut) = world.get_entity_mut(widget_slot_entity)
                else {
                    return Err("Widget slot entity disappeared before apply".to_string());
                };

                scene_patch
                    .apply(&mut widget_slot_entity_mut)
                    .map_err(|apply_error| apply_error.to_string())
            },
        );

        match apply_result {
            Ok(()) => {
                if let Ok(mut widget_slot_entity_mut) = world.get_entity_mut(widget_slot_entity) {
                    widget_slot_entity_mut.remove::<LastBeaconBsnWidgetPending>();
                }
            }
            Err(apply_error) => {
                let failure_reason = format!(
                    "Failed to apply Last Beacon BSN widget `{asset_path}` to {widget_slot_entity:?}: {apply_error}"
                );
                mark_widget_failed(world, widget_slot_entity, failure_reason);
            }
        }
    }
}

fn mark_widget_failed(world: &mut World, widget_slot_entity: Entity, failure_reason: String) {
    error!("{failure_reason}");
    if let Ok(mut widget_slot_entity_mut) = world.get_entity_mut(widget_slot_entity) {
        widget_slot_entity_mut.remove::<LastBeaconBsnWidgetPending>();
        widget_slot_entity_mut.insert(LastBeaconBsnWidgetFailed {
            reason: failure_reason,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn widget_asset_path_is_authored_explicitly() {
        let widget = LastBeaconBsnWidget {
            asset_path: "ui/widgets/main_menu/title.bsn".to_string(),
        };

        assert_eq!(widget.asset_path, "ui/widgets/main_menu/title.bsn");
    }
}
