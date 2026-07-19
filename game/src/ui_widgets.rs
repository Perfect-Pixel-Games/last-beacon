//! Last Beacon UI widget composition support.
//!
//! Last Beacon's authored UI scenes can place reusable `.bsn` widget assets under
//! lightweight widget slots. This keeps scene files focused on layout while common
//! visual pieces live under `assets/ui/widgets/`.

use std::{collections::HashMap, sync::Arc};

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

/// Marks a Main Menu primary button that should keep the prototype's yellow style.
#[derive(Clone, Copy, Debug, Default, Component, Reflect)]
#[reflect(Component, Default)]
pub struct LastBeaconMainMenuPrimaryButton;

/// Marks a Beacon primary action that should keep the prototype's cyan style.
#[derive(Clone, Copy, Debug, Default, Component, Reflect)]
#[reflect(Component, Default)]
pub struct LastBeaconBeaconPrimaryButton;

/// Marks a Beacon navigation tab whose transparent background should not be overwritten.
#[derive(Clone, Copy, Debug, Default, Component, Reflect)]
#[reflect(Component, Default)]
pub struct LastBeaconBeaconTabButton;

/// Applies the reusable Last Beacon button visual treatment.
#[derive(Clone, Debug, Component, Reflect)]
#[reflect(Component, Default)]
pub struct LastBeaconUiButton {
    /// Button variant: `primary`, `secondary`, or `tertiary`.
    pub variant: String,
}

impl Default for LastBeaconUiButton {
    fn default() -> Self {
        Self {
            variant: "secondary".to_string(),
        }
    }
}

/// Applies reusable Last Beacon tab behavior and visual treatment.
#[derive(Clone, Debug, Component, Reflect)]
#[reflect(Component, Default)]
pub struct LastBeaconUiTab {
    /// Selection group this tab belongs to.
    pub group: String,
    /// Stable tab identifier within the group.
    pub tab: String,
    /// Whether this tab should be selected before the user interacts with the group.
    pub selected: bool,
}

impl Default for LastBeaconUiTab {
    fn default() -> Self {
        Self {
            group: "default".to_string(),
            tab: "default".to_string(),
            selected: false,
        }
    }
}

/// Remembers selected tabs for authored reusable tab groups.
#[derive(Clone, Debug, Default, Resource)]
pub struct LastBeaconUiTabSelections {
    selected_tabs: HashMap<String, String>,
}

type LastBeaconUiTabInteractionQuery<'world, 'state> = Query<
    'world,
    'state,
    (&'static LastBeaconUiTab, &'static Interaction),
    (Changed<Interaction>, With<Button>),
>;

type LastBeaconUiButtonStyleQuery<'world, 'state> = Query<
    'world,
    'state,
    (
        &'static LastBeaconUiButton,
        &'static Interaction,
        &'static mut BackgroundColor,
        &'static mut BorderColor,
        Option<&'static Children>,
    ),
    (With<Button>, Without<LastBeaconUiTab>),
>;

type LastBeaconUiTabStyleQuery<'world, 'state> = Query<
    'world,
    'state,
    (
        &'static LastBeaconUiTab,
        &'static Interaction,
        &'static mut BackgroundColor,
        &'static mut BorderColor,
        Option<&'static Children>,
    ),
    (With<Button>, Without<LastBeaconUiButton>),
>;

type MainMenuPrimaryButtonStyleQuery<'world, 'state> = Query<
    'world,
    'state,
    &'static mut BackgroundColor,
    (
        With<LastBeaconMainMenuPrimaryButton>,
        Without<LastBeaconBeaconPrimaryButton>,
        Without<LastBeaconBeaconTabButton>,
        Without<LastBeaconUiButton>,
        Without<LastBeaconUiTab>,
    ),
>;

type BeaconPrimaryButtonStyleQuery<'world, 'state> = Query<
    'world,
    'state,
    &'static mut BackgroundColor,
    (
        With<LastBeaconBeaconPrimaryButton>,
        Without<LastBeaconMainMenuPrimaryButton>,
        Without<LastBeaconBeaconTabButton>,
        Without<LastBeaconUiButton>,
        Without<LastBeaconUiTab>,
    ),
>;

type BeaconTabButtonStyleQuery<'world, 'state> = Query<
    'world,
    'state,
    &'static mut BackgroundColor,
    (
        With<LastBeaconBeaconTabButton>,
        Without<LastBeaconMainMenuPrimaryButton>,
        Without<LastBeaconBeaconPrimaryButton>,
        Without<LastBeaconUiButton>,
        Without<LastBeaconUiTab>,
    ),
>;

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

/// Updates remembered tab selection when a reusable tab is clicked.
pub fn update_last_beacon_ui_tab_selection(
    mut tab_selections: ResMut<LastBeaconUiTabSelections>,
    tabs: LastBeaconUiTabInteractionQuery,
) {
    for (tab, tab_interaction) in &tabs {
        if *tab_interaction == Interaction::Pressed {
            tab_selections
                .selected_tabs
                .insert(tab.group.clone(), tab.tab.clone());
        }
    }
}

/// Restores prototype-authored button colors after generic Foundation interaction styling.
pub fn enforce_last_beacon_button_styles(
    mut ui_buttons: LastBeaconUiButtonStyleQuery,
    mut ui_tabs: LastBeaconUiTabStyleQuery,
    tab_selections: Res<LastBeaconUiTabSelections>,
    mut text_colors: Query<&mut TextColor>,
    mut main_menu_primary_buttons: MainMenuPrimaryButtonStyleQuery,
    mut beacon_primary_buttons: BeaconPrimaryButtonStyleQuery,
    mut beacon_tab_buttons: BeaconTabButtonStyleQuery,
) {
    for (button, button_interaction, mut button_background, mut button_border, button_children) in
        &mut ui_buttons
    {
        let button_style = reusable_button_style(&button.variant, *button_interaction);
        *button_background = BackgroundColor(button_style.background_color);
        *button_border = BorderColor::all(button_style.border_color);
        apply_text_color(button_children, button_style.text_color, &mut text_colors);
    }

    for (tab, tab_interaction, mut tab_background, mut tab_border, tab_children) in &mut ui_tabs {
        let selected_tab = tab_selections.selected_tabs.get(&tab.group);
        let is_selected = selected_tab
            .map(|selected_tab| selected_tab == &tab.tab)
            .unwrap_or(tab.selected);
        let tab_style = reusable_tab_style(is_selected, *tab_interaction);
        *tab_background = BackgroundColor(tab_style.background_color);
        *tab_border = BorderColor::all(tab_style.border_color);
        apply_text_color(tab_children, tab_style.text_color, &mut text_colors);
    }

    let prototype_menu_accent = Color::srgb(0.984, 0.749, 0.141);
    for mut button_background in &mut main_menu_primary_buttons {
        *button_background = BackgroundColor(prototype_menu_accent);
    }

    let prototype_beacon_accent = Color::srgb(0.133, 0.827, 0.933);
    for mut button_background in &mut beacon_primary_buttons {
        *button_background = BackgroundColor(prototype_beacon_accent);
    }

    let transparent_background = Color::srgba(0.0, 0.0, 0.0, 0.0);
    for mut button_background in &mut beacon_tab_buttons {
        *button_background = BackgroundColor(transparent_background);
    }
}

#[derive(Clone, Copy, Debug)]
struct LastBeaconWidgetStyle {
    background_color: Color,
    border_color: Color,
    text_color: Color,
}

fn reusable_button_style(variant: &str, interaction: Interaction) -> LastBeaconWidgetStyle {
    let normalized_variant = variant.trim().to_ascii_lowercase();
    match (normalized_variant.as_str(), interaction) {
        ("primary", Interaction::Pressed) => LastBeaconWidgetStyle {
            background_color: Color::srgb(0.028, 0.628, 0.718),
            border_color: Color::srgb(0.028, 0.628, 0.718),
            text_color: Color::srgb(0.008, 0.024, 0.09),
        },
        ("primary", Interaction::Hovered) => LastBeaconWidgetStyle {
            background_color: Color::srgb(0.404, 0.91, 0.965),
            border_color: Color::srgb(0.404, 0.91, 0.965),
            text_color: Color::srgb(0.008, 0.024, 0.09),
        },
        ("primary", _) => LastBeaconWidgetStyle {
            background_color: Color::srgb(0.133, 0.827, 0.933),
            border_color: Color::srgb(0.133, 0.827, 0.933),
            text_color: Color::srgb(0.008, 0.024, 0.09),
        },
        ("tertiary", Interaction::Pressed) => LastBeaconWidgetStyle {
            background_color: Color::srgba(0.133, 0.827, 0.933, 0.18),
            border_color: Color::srgb(0.133, 0.827, 0.933),
            text_color: Color::srgb(0.133, 0.827, 0.933),
        },
        ("tertiary", Interaction::Hovered) => LastBeaconWidgetStyle {
            background_color: Color::srgba(0.133, 0.827, 0.933, 0.1),
            border_color: Color::srgb(0.133, 0.827, 0.933),
            text_color: Color::srgb(0.404, 0.91, 0.965),
        },
        ("tertiary", _) => LastBeaconWidgetStyle {
            background_color: Color::srgba(0.0, 0.0, 0.0, 0.0),
            border_color: Color::srgb(0.278, 0.333, 0.412),
            text_color: Color::srgb(0.58, 0.639, 0.722),
        },
        (_, Interaction::Pressed) => LastBeaconWidgetStyle {
            background_color: Color::srgb(0.2, 0.255, 0.333),
            border_color: Color::srgb(0.58, 0.639, 0.722),
            text_color: Color::srgb(0.945, 0.961, 0.976),
        },
        (_, Interaction::Hovered) => LastBeaconWidgetStyle {
            background_color: Color::srgb(0.2, 0.255, 0.333),
            border_color: Color::srgb(0.796, 0.835, 0.882),
            text_color: Color::srgb(0.945, 0.961, 0.976),
        },
        (_, _) => LastBeaconWidgetStyle {
            background_color: Color::srgb(0.118, 0.161, 0.231),
            border_color: Color::srgb(0.278, 0.333, 0.412),
            text_color: Color::srgb(0.945, 0.961, 0.976),
        },
    }
}

fn reusable_tab_style(is_selected: bool, interaction: Interaction) -> LastBeaconWidgetStyle {
    match (is_selected, interaction) {
        (true, Interaction::Pressed) => LastBeaconWidgetStyle {
            background_color: Color::srgba(0.133, 0.827, 0.933, 0.22),
            border_color: Color::srgb(0.133, 0.827, 0.933),
            text_color: Color::srgb(0.133, 0.827, 0.933),
        },
        (true, _) => LastBeaconWidgetStyle {
            background_color: Color::srgba(0.133, 0.827, 0.933, 0.12),
            border_color: Color::srgb(0.133, 0.827, 0.933),
            text_color: Color::srgb(0.133, 0.827, 0.933),
        },
        (false, Interaction::Pressed) => LastBeaconWidgetStyle {
            background_color: Color::srgba(0.133, 0.827, 0.933, 0.16),
            border_color: Color::srgb(0.133, 0.827, 0.933),
            text_color: Color::srgb(0.133, 0.827, 0.933),
        },
        (false, Interaction::Hovered) => LastBeaconWidgetStyle {
            background_color: Color::srgba(0.133, 0.827, 0.933, 0.08),
            border_color: Color::srgb(0.278, 0.333, 0.412),
            text_color: Color::srgb(0.945, 0.961, 0.976),
        },
        (false, _) => LastBeaconWidgetStyle {
            background_color: Color::srgba(0.0, 0.0, 0.0, 0.0),
            border_color: Color::srgba(0.0, 0.0, 0.0, 0.0),
            text_color: Color::srgb(0.58, 0.639, 0.722),
        },
    }
}

fn apply_text_color(
    children: Option<&Children>,
    text_color: Color,
    text_colors: &mut Query<&mut TextColor>,
) {
    let Some(children) = children else {
        return;
    };

    for child_entity in children.iter() {
        if let Ok(mut child_text_color) = text_colors.get_mut(child_entity) {
            *child_text_color = TextColor(text_color);
        }
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
