//! Last Beacon UI widget composition support.
//!
//! Last Beacon's authored UI scenes can place reusable `.bsn` widget assets under
//! lightweight widget slots. This keeps scene files focused on layout while common
//! visual pieces live under `assets/ui/widgets/`.

use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use bevy::{
    ecs::system::SystemParam,
    input::{
        keyboard::{Key, KeyboardInput},
        mouse::{MouseScrollUnit, MouseWheel},
    },
    input_focus::{
        tab_navigation::{TabGroup, TabIndex},
        FocusCause, InputFocus, InputFocusVisible,
    },
    prelude::*,
    scene::{ResolvedSceneRoot, ScenePatch},
    text::{
        EditableText, EditableTextFilter, EditableTextGeneration, FontCx, FontSource, LayoutCx,
        LineBreak, LineHeight, TextCursorStyle, TextEdit, TextLayout, TextLayoutInfo,
    },
    ui::{
        widget::TextScroll, ComputedUiRenderTargetInfo, GridAutoFlow, GridPlacement, Outline,
        RelativeCursorPosition, RepeatedGridTrack, UiGlobalTransform,
    },
    window::PrimaryWindow,
};
use foundation_runtime_library::scene_stack::{SceneContentLoading, SceneOwner};
use foundation_runtime_library::ui_theme::{FoundationUiColorToken, FoundationUiTheme};

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

/// Marks a Beacon primary action that should keep the prototype's legacy cyan style.
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

/// Shows an authored content pane only when its matching reusable tab is selected.
#[derive(Clone, Debug, Component, Reflect)]
#[reflect(Component, Default)]
pub struct LastBeaconUiTabPanel {
    /// Selection group this panel belongs to.
    pub group: String,
    /// Stable tab identifier this panel represents.
    pub tab: String,
    /// Whether this panel should be shown before the user interacts with the group.
    pub selected: bool,
}

impl Default for LastBeaconUiTabPanel {
    fn default() -> Self {
        Self {
            group: "default".to_string(),
            tab: "default".to_string(),
            selected: false,
        }
    }
}

/// Makes an authored text value editable when clicked.
#[derive(Clone, Debug, Default, Component, Reflect)]
#[reflect(Component, Default)]
pub struct LastBeaconUiTextInput {
    /// Initial value shown in the input.
    pub value: String,
    /// Whether the input should allow newline entry.
    pub multiline: bool,
}

/// Marks the draggable scroll track for a multiline text input.
#[derive(Clone, Copy, Debug, Default, Component, Reflect)]
#[reflect(Component, Default)]
pub struct LastBeaconUiTextScrollTrack;

/// Marks the visual scroll thumb for a multiline text input.
#[derive(Clone, Copy, Debug, Default, Component, Reflect)]
#[reflect(Component, Default)]
pub struct LastBeaconUiTextScrollThumb;

/// Marks the horizontal draggable scroll track for a multiline text input.
#[derive(Clone, Copy, Debug, Default, Component, Reflect)]
#[reflect(Component, Default)]
pub struct LastBeaconUiTextHorizontalScrollTrack;

/// Marks the horizontal visual scroll thumb for a multiline text input.
#[derive(Clone, Copy, Debug, Default, Component, Reflect)]
#[reflect(Component, Default)]
pub struct LastBeaconUiTextHorizontalScrollThumb;

/// Marks text that should render with the bundled Noto Sans Symbols 2 font.
#[derive(Clone, Copy, Debug, Default, Component, Reflect)]
#[reflect(Component, Default)]
pub struct LastBeaconUiSymbolIcon;

/// Marks a keyboard/gamepad-focusable Last Beacon widget that should display a
/// visible focus outline while it holds input focus.
///
/// `apply_last_beacon_ui_focusability` inserts this alongside `TabIndex` and an
/// `Outline` on every interactive widget marker; `apply_last_beacon_ui_focus_outline`
/// then mutates that `Outline`'s color in place as focus moves, rather than
/// inserting/removing the component (which `bevy_ui::Outline`'s own docs warn
/// causes archetype table moves).
#[derive(Clone, Copy, Debug, Default, Component, Reflect)]
#[reflect(Component, Default)]
pub struct LastBeaconUiFocusIndicator;

/// Updates a radio icon from reusable tab selection state.
#[derive(Clone, Debug, Component, Reflect)]
#[reflect(Component, Default)]
pub struct LastBeaconUiRadioIcon {
    /// Selection group this icon follows.
    pub group: String,
    /// Tab identifier represented by this icon.
    pub tab: String,
    /// Whether this option is initially selected.
    pub selected: bool,
}

impl Default for LastBeaconUiRadioIcon {
    fn default() -> Self {
        Self {
            group: "default".to_string(),
            tab: "default".to_string(),
            selected: false,
        }
    }
}

/// Updates a combo-box arrow icon from dropdown open state.
#[derive(Clone, Debug, Default, Component, Reflect)]
#[reflect(Component, Default)]
pub struct LastBeaconUiDropdownIcon {
    /// Dropdown key this icon follows.
    pub target: String,
}

/// Makes an editable text input feed a numeric value.
#[derive(Clone, Debug, Component, Reflect)]
#[reflect(Component, Default)]
pub struct LastBeaconUiNumberInput {
    /// Shared value key this input writes to.
    pub target: String,
    /// Minimum accepted value.
    pub min: f32,
    /// Maximum accepted value.
    pub max: f32,
}

impl Default for LastBeaconUiNumberInput {
    fn default() -> Self {
        Self {
            target: String::new(),
            min: 0.0,
            max: 100.0,
        }
    }
}

/// Applies simple clickable value behavior to authored input examples.
#[derive(Clone, Debug, Component, Reflect)]
#[reflect(Component, Default)]
pub struct LastBeaconUiValueButton {
    /// Shared value key this control writes to.
    pub target: String,
    /// Absolute value to set when pressed. If empty, `delta` is applied instead.
    pub set_value: String,
    /// Numeric delta applied to the current value when `set_value` is empty.
    pub delta: f32,
    /// Minimum numeric value for delta updates.
    pub min: f32,
    /// Maximum numeric value for delta updates.
    pub max: f32,
}

impl Default for LastBeaconUiValueButton {
    fn default() -> Self {
        Self {
            target: String::new(),
            set_value: String::new(),
            delta: 0.0,
            min: 0.0,
            max: 100.0,
        }
    }
}

/// Displays a value from [`LastBeaconUiInputValues`] in an authored text entity.
#[derive(Clone, Debug, Default, Component, Reflect)]
#[reflect(Component, Default)]
pub struct LastBeaconUiValueText {
    /// Shared value key this text mirrors.
    pub target: String,
    /// Text prepended before the value.
    pub prefix: String,
    /// Text appended after the value.
    pub suffix: String,
}

/// Toggles a reusable combo-box option panel.
#[derive(Clone, Debug, Default, Component, Reflect)]
#[reflect(Component, Default)]
pub struct LastBeaconUiDropdownToggle {
    /// Shared dropdown key this control opens or closes.
    pub target: String,
}

/// Marks a reusable combo-box option panel whose display follows dropdown state.
#[derive(Clone, Debug, Default, Component, Reflect)]
#[reflect(Component, Default)]
pub struct LastBeaconUiDropdownPanel {
    /// Shared dropdown key this panel belongs to.
    pub target: String,
}

/// Makes an authored slider update a value from cursor position.
#[derive(Clone, Debug, Component, Reflect)]
#[reflect(Component, Default)]
pub struct LastBeaconUiSlider {
    /// Shared value key this slider writes to.
    pub target: String,
    /// Minimum slider value.
    pub min: f32,
    /// Maximum slider value.
    pub max: f32,
}

impl Default for LastBeaconUiSlider {
    fn default() -> Self {
        Self {
            target: String::new(),
            min: 0.0,
            max: 100.0,
        }
    }
}

/// Marks a slider fill node whose width mirrors a stored slider value.
#[derive(Clone, Debug, Component, Reflect)]
#[reflect(Component, Default)]
pub struct LastBeaconUiSliderFill {
    /// Shared value key this fill visual reads from.
    pub target: String,
    /// Minimum slider value.
    pub min: f32,
    /// Maximum slider value.
    pub max: f32,
}

impl Default for LastBeaconUiSliderFill {
    fn default() -> Self {
        Self {
            target: String::new(),
            min: 0.0,
            max: 100.0,
        }
    }
}

/// Turns this entity into a uniform-column CSS grid container: `column_count`
/// equal-width columns, with children placed left-to-right and wrapping onto
/// new rows automatically.
///
/// A plain field like this is authored directly in `.bsn`; the equal-width
/// column tracks themselves (`RepeatedGridTrack::flex`) are constructed in
/// Rust by `apply_last_beacon_ui_uniform_grid`, because Last Beacon's `.bsn`
/// grammar only resolves expressions into registered tuple-struct/enum-variant
/// fields via reflection, not arbitrary associated functions.
#[derive(Clone, Copy, Debug, Component, Reflect)]
#[reflect(Component, Default)]
pub struct LastBeaconUiUniformGrid {
    /// Number of equal-width columns. Values below `1` are treated as `1`.
    pub column_count: u16,
}

impl Default for LastBeaconUiUniformGrid {
    fn default() -> Self {
        Self { column_count: 1 }
    }
}

/// Marks a grid cell that should span more than one column and/or row of its
/// parent grid container (for example a `LastBeaconUiUniformGrid`).
#[derive(Clone, Copy, Debug, Component, Reflect)]
#[reflect(Component, Default)]
pub struct LastBeaconUiGridItem {
    /// Number of grid columns this item should span. Values below `1` are
    /// treated as `1` (`GridPlacement::span` panics on `0`).
    pub column_span: u16,
    /// Number of grid rows this item should span. Values below `1` are
    /// treated as `1`.
    pub row_span: u16,
}

impl Default for LastBeaconUiGridItem {
    fn default() -> Self {
        Self {
            column_span: 1,
            row_span: 1,
        }
    }
}

/// Constrains this widget's own size to a clamped aspect ratio band based on
/// its parent's available content space, so authored UI (for example a HUD
/// sized around a particular ratio) neither stretches edge-to-edge on an
/// ultrawide monitor nor squashes below a legible minimum on a narrow one.
///
/// Setting `min_aspect_ratio == max_aspect_ratio` locks to a single fixed
/// ratio; this is the same widget in both cases, not a separate variant.
/// The immediate parent must center this widget (`align_items: Center,
/// justify_content: Center`) and must not size itself from this widget's own
/// size (pin the parent to a stable area, such as the window), or the
/// system's write-back could feed into a layout oscillation.
#[derive(Clone, Copy, Debug, Component, Reflect)]
#[reflect(Component, Default)]
pub struct LastBeaconUiAspectRatioBounds {
    /// Minimum allowed width-over-height ratio.
    pub min_aspect_ratio: f32,
    /// Maximum allowed width-over-height ratio.
    pub max_aspect_ratio: f32,
}

impl Default for LastBeaconUiAspectRatioBounds {
    fn default() -> Self {
        // Inert by default -- a widget left unconfigured constrains nothing.
        // `0.0..=INFINITY` can never actually clamp a real (positive, finite)
        // aspect ratio.
        Self {
            min_aspect_ratio: 0.0,
            max_aspect_ratio: f32::INFINITY,
        }
    }
}

/// Remembers selected tabs for authored reusable tab groups.
#[derive(Clone, Debug, Default, Resource)]
pub struct LastBeaconUiTabSelections {
    selected_tabs: HashMap<String, String>,
}

/// Stores lightweight example input values for the reusable UI playground widgets.
#[derive(Clone, Debug, Default, Resource)]
pub struct LastBeaconUiInputValues {
    values: HashMap<String, String>,
}

/// Stores open state for lightweight reusable dropdown examples.
#[derive(Clone, Debug, Default, Resource)]
pub struct LastBeaconUiDropdownStates {
    open_dropdowns: HashMap<String, bool>,
}

/// Stores the currently dragged multiline text box scrollbar.
#[derive(Clone, Copy, Debug, Default, Resource)]
pub struct LastBeaconUiTextBoxScrollDrag {
    active_text_box: Option<Entity>,
    axis: LastBeaconUiTextBoxScrollAxis,
}

/// Identifies which authored scrollbar axis is currently being dragged.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum LastBeaconUiTextBoxScrollAxis {
    #[default]
    Vertical,
    Horizontal,
}

/// Stores user-requested multiline text scroll after Bevy's native cursor scroll runs.
#[derive(Clone, Debug, Default, Resource)]
pub struct LastBeaconUiTextBoxScrollOverrides {
    scroll_by_text_entity: HashMap<Entity, Vec2>,
}

/// Tracks focused text boxes whose keyboard input should make the caret visible again.
#[derive(Clone, Debug, Default, Resource)]
pub struct LastBeaconUiTextBoxCaretScrollRequests {
    text_entities: HashSet<Entity>,
}

const TEXT_BOX_SCROLL_LINE_STEP: f32 = 16.0;
const TEXT_BOX_SCROLLBAR_THICKNESS_RATIO: f32 = 0.065;
const TEXT_BOX_SCROLLBAR_MIN_THICKNESS: f32 = 4.0;
const TEXT_BOX_SCROLLBAR_MAX_THICKNESS: f32 = 8.0;
const TEXT_BOX_SCROLLBAR_MIN_THUMB_RATIO: f32 = 0.25;

// Reuses the same amber accent already used for selected tabs and slider
// fills elsewhere in this file, so the focus outline reads as part of the
// same visual language rather than an unrelated new color.
const LAST_BEACON_FOCUS_OUTLINE_COLOR: Color = Color::srgb(0.984, 0.749, 0.141);
const LAST_BEACON_FOCUS_OUTLINE_WIDTH: f32 = 2.0;
const LAST_BEACON_FOCUS_OUTLINE_OFFSET: f32 = 2.0;

type LastBeaconUiTextInputFocusQuery<'w, 's> = Query<
    'w,
    's,
    (
        Entity,
        &'static LastBeaconUiTextInput,
        &'static Interaction,
        Option<&'static Children>,
    ),
    (Changed<Interaction>, With<LastBeaconUiTextInput>),
>;

type LastBeaconUiTextInputScrollQuery<'w, 's> = Query<
    'w,
    's,
    (
        Entity,
        &'static LastBeaconUiTextInput,
        Option<&'static Children>,
        Option<&'static Interaction>,
        Option<&'static RelativeCursorPosition>,
    ),
>;

type LastBeaconUiValueButtonInteractionQuery<'w, 's> = Query<
    'w,
    's,
    (&'static LastBeaconUiValueButton, &'static Interaction),
    (Changed<Interaction>, With<Button>),
>;

type LastBeaconUiDropdownToggleQuery<'w, 's> = Query<
    'w,
    's,
    (&'static LastBeaconUiDropdownToggle, &'static Interaction),
    (Changed<Interaction>, With<Button>),
>;

type LastBeaconUiSliderInteractionQuery<'w, 's> = Query<
    'w,
    's,
    (
        Entity,
        &'static LastBeaconUiSlider,
        &'static Interaction,
        &'static RelativeCursorPosition,
    ),
    With<Button>,
>;

type LastBeaconUiTabInteractionQuery<'w, 's> = Query<
    'w,
    's,
    (&'static LastBeaconUiTab, &'static Interaction),
    (Changed<Interaction>, With<Button>),
>;

type LastBeaconUiTabPanelQuery<'w, 's> =
    Query<'w, 's, (&'static LastBeaconUiTabPanel, &'static mut Node)>;

// These style queries deliberately run unconditionally every frame, NOT
// gated by `Changed<Interaction>`. `enforce_last_beacon_button_styles` runs
// in `PostUpdate`, after Foundation's own generic
// `update_foundation_menu_button_interactions` (which every one of these
// buttons also matches, since they all carry `FoundationMenuButton`) has
// already run in `Update` for this frame -- but only for entities that
// system's own last-run tick considers changed. BSN scenes spawn widgets
// through deferred commands, so a freshly created button can become visible
// to Foundation's `Update`-scheduled system one or more frames later than it
// becomes visible to this `PostUpdate`-scheduled one. When that happens,
// Foundation's system sees `Changed<Interaction>` as true on a LATER frame
// than we do, and overwrites our authoritative color with its own generic
// default well after we already applied ours -- a frame we can no longer
// detect via `Changed<Interaction>` ourselves, since it didn't change again.
// The result is a stray color that never gets corrected, exactly the kind of
// permanent-desync bug documented on `initialize_last_beacon_ui_text_inputs`.
// Running every frame regardless of `Changed<Interaction>` guarantees this
// system always has the last word, since `PostUpdate` always runs after
// `Update` within the same frame no matter how the two systems' change-tick
// histories have drifted.
type LastBeaconUiButtonStyleQuery<'w, 's> = Query<
    'w,
    's,
    (
        &'static LastBeaconUiButton,
        &'static Interaction,
        &'static mut BackgroundColor,
        &'static mut BorderColor,
        Option<&'static Children>,
    ),
    (With<Button>, Without<LastBeaconUiTab>),
>;

type LastBeaconUiTabStyleQuery<'w, 's> = Query<
    'w,
    's,
    (
        &'static LastBeaconUiTab,
        &'static Interaction,
        &'static mut BackgroundColor,
        &'static mut BorderColor,
        Option<&'static Children>,
    ),
    (With<Button>, Without<LastBeaconUiButton>),
>;

type MainMenuPrimaryButtonStyleQuery<'w, 's> = Query<
    'w,
    's,
    &'static mut BackgroundColor,
    (
        With<LastBeaconMainMenuPrimaryButton>,
        Without<LastBeaconBeaconPrimaryButton>,
        Without<LastBeaconBeaconTabButton>,
        Without<LastBeaconUiButton>,
        Without<LastBeaconUiTab>,
    ),
>;

type BeaconPrimaryButtonStyleQuery<'w, 's> = Query<
    'w,
    's,
    &'static mut BackgroundColor,
    (
        With<LastBeaconBeaconPrimaryButton>,
        Without<LastBeaconMainMenuPrimaryButton>,
        Without<LastBeaconBeaconTabButton>,
        Without<LastBeaconUiButton>,
        Without<LastBeaconUiTab>,
    ),
>;

type BeaconTabButtonStyleQuery<'w, 's> = Query<
    'w,
    's,
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
///
/// This must run after Foundation's `propagate_loaded_bsn_scene_owners` so a
/// widget slot already carries [`SceneOwner`] before it gains
/// [`SceneContentLoading`] here — otherwise the marker could briefly apply to
/// no scene, letting the parent scene reveal for one frame before hiding
/// again once ownership catches up.
pub fn queue_last_beacon_bsn_widgets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    widget_slots: Query<
        (Entity, &LastBeaconBsnWidget, Option<&SceneOwner>),
        Added<LastBeaconBsnWidget>,
    >,
) {
    for (widget_slot_entity, widget_slot, scene_owner) in &widget_slots {
        if widget_slot.asset_path.is_empty() {
            warn!("LastBeaconBsnWidget on {widget_slot_entity:?} has an empty asset path.");
            continue;
        }

        // Store the handle on the slot so the exclusive apply system can patch this entity later.
        let scene_handle = asset_server.load(widget_slot.asset_path.clone());
        let mut widget_entity_commands = commands.entity(widget_slot_entity);
        widget_entity_commands.insert(LastBeaconBsnWidgetPending {
            asset_path: widget_slot.asset_path.clone(),
            scene_handle,
        });

        if scene_owner.is_some() {
            // Keep the owning scene hidden until this nested widget also
            // finishes applying, so it can't pop in after the rest of the
            // scene is already visible.
            widget_entity_commands.insert(SceneContentLoading);
        }
    }
}

/// Holds a permanent strong reference to Last Beacon's shared UI fonts.
///
/// Without this, the only strong references to these font handles would live
/// on `TextFont` components, which despawn along with their scene. If every
/// visible piece of text happens to despawn at once (for example, closing a
/// Beacon page and opening the next one in the same frame, with a gap before
/// the new page's text exists), the font's reference count can hit zero and
/// Bevy unloads it — the next scene's text then has to reload it from
/// scratch, causing a visible pop from fallback glyphs to the real font.
/// Loading both fonts once here and holding the handles for the whole
/// session keeps them resident permanently.
#[derive(Resource)]
pub struct LastBeaconUiFontHandles {
    /// Shared handle for regular UI text.
    pub ui_font: Handle<Font>,
    /// Shared handle for symbol/icon glyphs.
    pub symbol_font: Handle<Font>,
}

impl FromWorld for LastBeaconUiFontHandles {
    fn from_world(world: &mut World) -> Self {
        // Some minimal test apps build LastBeaconPlugin without asset
        // infrastructure at all (mirroring FoundationBsnAssetPlugin's own
        // AssetServer-presence check). Degrade to default handles rather
        // than panicking; production always has AssetServer available
        // before this plugin builds.
        let Some(asset_server) = world.get_resource::<AssetServer>() else {
            return Self {
                ui_font: Handle::default(),
                symbol_font: Handle::default(),
            };
        };
        Self {
            ui_font: asset_server.load("fonts/NotoSans-Regular.ttf"),
            symbol_font: asset_server.load("fonts/NotoSansSymbols2-Regular.ttf"),
        }
    }
}

/// Applies Last Beacon's shared UI font to newly spawned text.
///
/// Text whose font has not finished loading yet is marked
/// `SceneContentLoading` so its owning scene stays hidden until the swap
/// from fallback glyphs to the real font has already happened — the font
/// should never visibly pop after a scene is shown.
/// `reveal_last_beacon_text_once_fonts_load` clears the marker once the
/// fonts are ready.
pub fn apply_last_beacon_ui_font(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    font_handles: Res<LastBeaconUiFontHandles>,
    mut text_fonts: Query<
        (Entity, &mut TextFont, Option<&LastBeaconUiSymbolIcon>),
        Added<TextFont>,
    >,
) {
    for (text_entity, mut text_font, symbol_icon) in &mut text_fonts {
        let font_handle = if symbol_icon.is_some() {
            font_handles.symbol_font.clone()
        } else {
            font_handles.ui_font.clone()
        };
        let font_is_loaded = matches!(
            asset_server.get_load_state(font_handle.id()),
            Some(bevy::asset::LoadState::Loaded)
        );
        text_font.font = FontSource::Handle(font_handle);

        if !font_is_loaded {
            commands.entity(text_entity).insert(SceneContentLoading);
        }
    }
}

/// Clears the loading marker `apply_last_beacon_ui_font` left on text whose
/// font was not yet loaded, once both shared fonts finish loading.
///
/// The fonts load once, early in the session, and `LastBeaconUiFontHandles`
/// keeps them resident afterward, so this only ever has work to do during
/// the first moments of a session.
///
/// Also force-marks each such entity's `TextFont` as changed. `TextFont`
/// already held the right `FontSource::Handle` from the moment
/// `apply_last_beacon_ui_font` ran, but if the underlying font asset hadn't
/// finished loading yet on that exact frame, `bevy_ui`'s
/// `update_editable_text_styles` (gated on `Changed<TextFont>`) failed to
/// resolve it and silently skipped applying `FontFamily`/weight/etc. to the
/// entity's editor styles -- permanently, since nothing else ever touches
/// `TextFont` again afterward. For an `EditableText` entity, that leaves its
/// `PlainEditor` stuck building layouts with no font family: structurally
/// valid (one line) but with zero glyphs/size, which stays invisible behind
/// this entity's last-known-good `TextLayoutInfo` until the first edit
/// forces a real recompute -- at which point it collapses to a permanently
/// blank input with no visible cursor. Marking `TextFont` changed here,
/// once the font is actually loaded, gives that resolution a genuine retry.
pub fn reveal_last_beacon_text_once_fonts_load(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    font_handles: Res<LastBeaconUiFontHandles>,
    mut loading_text: Query<(Entity, &mut TextFont), With<SceneContentLoading>>,
) {
    let fonts_are_loaded = matches!(
        asset_server.get_load_state(font_handles.ui_font.id()),
        Some(bevy::asset::LoadState::Loaded)
    ) && matches!(
        asset_server.get_load_state(font_handles.symbol_font.id()),
        Some(bevy::asset::LoadState::Loaded)
    );
    if !fonts_are_loaded {
        return;
    }

    for (text_entity, mut text_font) in &mut loading_text {
        commands.entity(text_entity).remove::<SceneContentLoading>();
        text_font.set_changed();
    }
}

/// Fixed width for a Number Field's value text, in pixels. Comfortably fits
/// this widget's authored range (0-250) without needing to grow past 3
/// digits. Kept fixed rather than auto-sized -- see the comment where it's
/// applied in `initialize_last_beacon_ui_text_inputs`.
const NUMBER_INPUT_VALUE_TEXT_WIDTH_PX: f32 = 48.0;

/// Turns authored text-input containers into focusable editable text widgets.
pub fn initialize_last_beacon_ui_text_inputs(
    mut commands: Commands,
    text_inputs: Query<
        (Entity, &LastBeaconUiTextInput, Option<&Children>),
        Added<LastBeaconUiTextInput>,
    >,
    children_query: Query<&Children>,
    text_query: Query<(), With<Text>>,
    mut node_query: Query<&mut Node>,
    mut text_font_query: Query<&mut TextFont>,
    number_input_query: Query<(), With<LastBeaconUiNumberInput>>,
) {
    for (input_entity, text_input, input_children) in &text_inputs {
        let text_entity = if text_query.contains(input_entity) {
            input_entity
        } else {
            let Some(text_entity) =
                first_descendant_with_text(input_children, &children_query, &text_query)
            else {
                warn!("LastBeaconUiTextInput on {input_entity:?} has no child Text entity.");
                continue;
            };
            text_entity
        };

        let mut editable_text = if text_input.multiline {
            let mut editable_text = EditableText::default();
            editable_text.editor_mut().set_text(&text_input.value);
            editable_text.queue_edit(TextEdit::TextStart(false));
            editable_text
        } else {
            EditableText::new(&text_input.value)
        };
        editable_text.allow_newlines = text_input.multiline;
        if text_input.multiline {
            editable_text.visible_width = None;
            editable_text.visible_lines = None;
        } else {
            editable_text.visible_width = Some(24.0);
            editable_text.visible_lines = Some(1.0);
        }

        if text_input.multiline {
            commands
                .entity(input_entity)
                .insert(RelativeCursorPosition::default());
        }

        let line_height = if text_input.multiline {
            LineHeight::Px(16.0)
        } else {
            LineHeight::default()
        };
        let text_layout = if text_input.multiline {
            TextLayout {
                linebreak: LineBreak::NoWrap,
                ..default()
            }
        } else if number_input_query.contains(text_entity) {
            TextLayout {
                justify: Justify::Center,
                ..default()
            }
        } else {
            TextLayout::default()
        };

        commands.entity(text_entity).insert((
            editable_text,
            TextScroll::default(),
            TextCursorStyle::default(),
            text_layout,
            line_height,
        ));

        // `bevy_ui`'s `update_editable_text_styles` only ever syncs a fresh
        // `PlainEditor`'s font family/size/weight/etc. from `TextFont` on a
        // frame where it reads `Changed<TextFont>` as true. Bevy's change
        // detection is tracked per system against that system's own
        // last-run tick, not per entity -- so if `TextFont` was authored and
        // changed (by `apply_last_beacon_ui_font`) on an earlier frame than
        // this one, and this is the first frame `update_editable_text_styles`
        // sees this entity at all (because it only starts matching once
        // `EditableText` exists, which this loop is inserting right now),
        // that earlier change is invisible to it -- permanently, since
        // nothing else ever touches `TextFont` again. The freshly-created
        // `PlainEditor` above is then stuck with parley's bare defaults
        // (no font family, 100px default size): a structurally valid but
        // zero-glyph, zero-size layout that stays hidden behind whatever
        // `TextLayoutInfo` this entity already had until the first edit
        // forces a real recompute -- at which point it collapses to a
        // permanently blank, cursor-less input. Marking `TextFont` changed
        // again here, in the same frame `EditableText` is added, guarantees
        // `update_editable_text_styles` gets a real chance to apply it.
        //
        // Confirmed live in the running game (Text Field, Text Box, and the
        // Number Field's value box all recovered). Not hermetically
        // reproducible in a headless unit test, the same as the Number
        // Field's earlier stuck-glyph defect (see
        // `heal_last_beacon_ui_value_text_stuck_glyphs`): a test built with
        // `TextFont::default()` never shows the defect at all, because
        // fontique silently falls back to a system-installed font when no
        // specific family was ever requested, and a test that instead
        // spawns a real `apply_last_beacon_ui_font`-routed `TextFont` inside
        // a minimal single-`Update`-schedule `App` also fails to reproduce
        // it, for reasons not fully understood -- most likely because the
        // real game's fuller `PreUpdate`/`Update`/`PostUpdate` system
        // ordering and archetype churn differ enough from this reduced
        // setup to change how `Changed<TextFont>` is evaluated.
        if let Ok(mut text_font) = text_font_query.get_mut(text_entity) {
            text_font.set_changed();
        }

        if let Ok(mut node) = node_query.get_mut(text_entity) {
            if text_input.multiline {
                node.width = Val::Percent(100.0);
                node.height = Val::Percent(100.0);
            }
            if number_input_query.contains(text_entity) {
                // A fixed width instead of `Val::Auto` keeps this node from
                // needing to be re-measured every time the digit text
                // changes. Auto-sizing here made the number field's glyph
                // layout intermittently race Bevy UI's layout pass: right
                // after a value change, the auto-width node could briefly
                // be measured at width 0 before converging on its real
                // size, and `update_editable_text_layout` would compute a
                // zero-glyph, zero-size text layout from that transient
                // width -- visible as the value disappearing for a few
                // frames until the next layout pass corrected it.
                node.width = Val::Px(NUMBER_INPUT_VALUE_TEXT_WIDTH_PX);
                node.align_items = AlignItems::Center;
                node.justify_content = JustifyContent::Center;
            }
        } else if text_input.multiline {
            commands.entity(text_entity).insert(Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            });
        } else if number_input_query.contains(text_entity) {
            commands.entity(text_entity).insert(Node {
                width: Val::Px(NUMBER_INPUT_VALUE_TEXT_WIDTH_PX),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            });
        } else {
            commands.entity(text_entity).insert(Node::default());
        }

        if number_input_query.contains(text_entity) {
            commands
                .entity(text_entity)
                .insert(EditableTextFilter::new(number_input_allows_character));
        }
    }
}

/// Focuses editable text when its authored input container is clicked.
#[allow(clippy::too_many_arguments)]
pub fn focus_last_beacon_ui_text_inputs(
    mut input_focus: ResMut<InputFocus>,
    text_inputs: LastBeaconUiTextInputFocusQuery,
    children_query: Query<&Children>,
    editable_text_query: Query<(), With<EditableText>>,
    mut editable_text_position_query: Query<(
        &mut EditableText,
        &ComputedNode,
        &ComputedUiRenderTargetInfo,
        &UiGlobalTransform,
        &TextScroll,
    )>,
    windows: Query<&Window, With<PrimaryWindow>>,
    ui_scale: Res<UiScale>,
) {
    for (input_entity, _text_input, interaction, input_children) in &text_inputs {
        if *interaction != Interaction::Pressed {
            continue;
        }
        let text_entity = if editable_text_query.contains(input_entity) {
            input_entity
        } else {
            let Some(text_entity) = first_descendant_with_editable_text(
                input_children,
                &children_query,
                &editable_text_query,
            ) else {
                continue;
            };
            text_entity
        };
        input_focus.set(text_entity, FocusCause::Pressed);
        queue_text_input_click_placement(
            text_entity,
            &mut editable_text_position_query,
            &windows,
            ui_scale.0,
        );
    }
}

/// Seeds stored example input values from authored labels.
pub fn initialize_last_beacon_ui_value_text(
    mut input_values: ResMut<LastBeaconUiInputValues>,
    value_texts: Query<(&LastBeaconUiValueText, &Text), Added<LastBeaconUiValueText>>,
) {
    for (value_text, text) in &value_texts {
        if value_text.target.is_empty() || input_values.values.contains_key(&value_text.target) {
            continue;
        }

        let Some(initial_value) = text
            .0
            .strip_prefix(&value_text.prefix)
            .and_then(|value| value.strip_suffix(&value_text.suffix))
        else {
            input_values
                .values
                .insert(value_text.target.clone(), text.0.clone());
            continue;
        };

        input_values
            .values
            .insert(value_text.target.clone(), initial_value.to_string());
    }
}

/// Synchronizes edited numeric text into shared widget values.
pub fn update_last_beacon_ui_number_inputs(
    mut input_values: ResMut<LastBeaconUiInputValues>,
    number_inputs: Query<
        (&LastBeaconUiNumberInput, &EditableText),
        Changed<EditableTextGeneration>,
    >,
) {
    for (number_input, editable_text) in &number_inputs {
        if number_input.target.is_empty() {
            continue;
        }
        let raw_text = editable_text.value().to_string();
        let Ok(value) = raw_text.trim().parse::<f32>() else {
            continue;
        };
        let clamped_value = value.clamp(number_input.min, number_input.max);
        let clamped_value_string = format_value(clamped_value);
        insert_input_value_if_changed(&mut input_values, &number_input.target, clamped_value_string);
    }
}

/// Applies a single reusable value-button press to shared widget state.
///
/// Factored out of `update_last_beacon_ui_value_buttons` so
/// `activate_last_beacon_ui_focused_widget_on_keyboard_input` can trigger the
/// exact same effect from Enter/Space on a focused widget, without touching
/// the `Interaction` component itself (see that system's doc comment for why).
fn apply_value_button_activation(
    button: &LastBeaconUiValueButton,
    input_values: &mut LastBeaconUiInputValues,
    dropdown_states: &mut LastBeaconUiDropdownStates,
) {
    if button.target.is_empty() {
        return;
    }

    if !button.set_value.is_empty() {
        input_values
            .values
            .insert(button.target.clone(), button.set_value.clone());
        dropdown_states
            .open_dropdowns
            .insert(button.target.clone(), false);
        return;
    }

    let current_value = input_values
        .values
        .get(&button.target)
        .and_then(|value| value.parse::<f32>().ok())
        .unwrap_or(0.0);
    let next_value = (current_value + button.delta).clamp(button.min, button.max);
    let next_value_string = format_value(next_value);
    insert_input_value_if_changed(input_values, &button.target, next_value_string);
}

/// Applies simple value changes for authored reusable input examples.
pub fn update_last_beacon_ui_value_buttons(
    mut input_values: ResMut<LastBeaconUiInputValues>,
    mut dropdown_states: ResMut<LastBeaconUiDropdownStates>,
    buttons: LastBeaconUiValueButtonInteractionQuery,
) {
    for (button, interaction) in &buttons {
        if *interaction != Interaction::Pressed {
            continue;
        }
        apply_value_button_activation(button, &mut input_values, &mut dropdown_states);
    }
}

/// Applies a single reusable dropdown-toggle press to shared widget state.
///
/// Factored out of `toggle_last_beacon_ui_dropdowns` so
/// `activate_last_beacon_ui_focused_widget_on_keyboard_input` can trigger the
/// exact same effect from Enter/Space on a focused widget.
fn apply_dropdown_toggle_activation(
    toggle: &LastBeaconUiDropdownToggle,
    dropdown_states: &mut LastBeaconUiDropdownStates,
) {
    if toggle.target.is_empty() {
        return;
    }

    let next_open_state = !dropdown_states
        .open_dropdowns
        .get(&toggle.target)
        .copied()
        .unwrap_or(false);
    dropdown_states
        .open_dropdowns
        .insert(toggle.target.clone(), next_open_state);
}

/// Opens or closes authored reusable dropdown panels.
pub fn toggle_last_beacon_ui_dropdowns(
    mut dropdown_states: ResMut<LastBeaconUiDropdownStates>,
    toggles: LastBeaconUiDropdownToggleQuery,
) {
    for (toggle, interaction) in &toggles {
        if *interaction != Interaction::Pressed {
            continue;
        }
        apply_dropdown_toggle_activation(toggle, &mut dropdown_states);
    }
}

/// Mirrors reusable radio selection into symbol icon text.
pub fn refresh_last_beacon_ui_radio_icons(
    tab_selections: Res<LastBeaconUiTabSelections>,
    mut radio_icons: Query<(&LastBeaconUiRadioIcon, &mut Text)>,
    newly_spawned_radio_icons: Query<(), Added<LastBeaconUiRadioIcon>>,
) {
    // Newly-spawned icons (for example from a scene reopened after selection
    // state was already set elsewhere) must still be synced even on a frame
    // where nobody just changed the selection, or they keep showing their
    // authored default until the next unrelated selection change.
    if !tab_selections.is_changed() && newly_spawned_radio_icons.is_empty() {
        return;
    }

    for (radio_icon, mut text) in &mut radio_icons {
        let selected_tab = tab_selections.selected_tabs.get(&radio_icon.group);
        let option_is_selected = selected_tab
            .map(|selected_tab| selected_tab == &radio_icon.tab)
            .unwrap_or(radio_icon.selected);
        text.0 = if option_is_selected {
            "●".to_string()
        } else {
            "○".to_string()
        };
    }
}

/// Mirrors reusable dropdown open state into symbol arrow text.
pub fn refresh_last_beacon_ui_dropdown_icons(
    dropdown_states: Res<LastBeaconUiDropdownStates>,
    mut dropdown_icons: Query<(&LastBeaconUiDropdownIcon, &mut Text)>,
    newly_spawned_dropdown_icons: Query<(), Added<LastBeaconUiDropdownIcon>>,
) {
    // See `refresh_last_beacon_ui_radio_icons` for why newly-spawned icons
    // must also be synced when the resource itself didn't change this frame.
    if !dropdown_states.is_changed() && newly_spawned_dropdown_icons.is_empty() {
        return;
    }

    for (dropdown_icon, mut text) in &mut dropdown_icons {
        let dropdown_is_open = dropdown_states
            .open_dropdowns
            .get(&dropdown_icon.target)
            .copied()
            .unwrap_or(false);
        text.0 = if dropdown_is_open {
            "▴".to_string()
        } else {
            "▾".to_string()
        };
    }
}

/// Applies dropdown open state to authored option panels.
pub fn refresh_last_beacon_ui_dropdown_panels(
    dropdown_states: Res<LastBeaconUiDropdownStates>,
    mut panels: Query<(&LastBeaconUiDropdownPanel, &mut Node)>,
    newly_spawned_panels: Query<(), Added<LastBeaconUiDropdownPanel>>,
) {
    // See `refresh_last_beacon_ui_radio_icons` for why newly-spawned panels
    // must also be synced when the resource itself didn't change this frame.
    if !dropdown_states.is_changed() && newly_spawned_panels.is_empty() {
        return;
    }

    for (panel, mut node) in &mut panels {
        let panel_is_open = dropdown_states
            .open_dropdowns
            .get(&panel.target)
            .copied()
            .unwrap_or(false);
        node.display = if panel_is_open {
            Display::Flex
        } else {
            Display::None
        };
    }
}

/// Enables cursor-position tracking for authored multiline text-box scroll tracks.
pub fn initialize_last_beacon_ui_text_scroll_tracks(
    mut commands: Commands,
    vertical_scroll_tracks: Query<Entity, Added<LastBeaconUiTextScrollTrack>>,
    horizontal_scroll_tracks: Query<Entity, Added<LastBeaconUiTextHorizontalScrollTrack>>,
) {
    for scroll_track_entity in &vertical_scroll_tracks {
        commands
            .entity(scroll_track_entity)
            .insert(RelativeCursorPosition::default());
    }

    for scroll_track_entity in &horizontal_scroll_tracks {
        commands
            .entity(scroll_track_entity)
            .insert(RelativeCursorPosition::default());
    }
}

/// Enables cursor-position tracking for authored reusable sliders.
pub fn initialize_last_beacon_ui_sliders(
    mut commands: Commands,
    sliders: Query<Entity, Added<LastBeaconUiSlider>>,
) {
    for slider_entity in &sliders {
        commands
            .entity(slider_entity)
            .insert(RelativeCursorPosition::default());
    }
}

/// Marks every newly loaded Last Beacon scene root as its own modal
/// tab-navigation group, so `Tab`/`Shift+Tab` cycles within that scene (or
/// overlay) instead of leaking focus into whatever scene is underneath it.
///
/// `TabIndex` only participates in tab cycling when an ancestor carries
/// `TabGroup`; a top-level scene root (an entity with `SceneOwner` and no
/// parent) is the natural place to attach one. Using `TabGroup::modal()`
/// keeps a pause overlay's tab cycle from including the paused scene beneath
/// it.
pub fn apply_last_beacon_ui_tab_groups_to_scene_roots(
    mut commands: Commands,
    scene_root_entities: Query<Entity, (Added<SceneOwner>, Without<ChildOf>)>,
) {
    for scene_root_entity in &scene_root_entities {
        commands.entity(scene_root_entity).insert(TabGroup::modal());
    }
}

/// Makes every newly spawned interactive Last Beacon widget reachable by
/// keyboard/gamepad tab navigation and ready to show a focus outline.
///
/// The `Outline` is inserted once with a transparent color rather than being
/// inserted/removed as focus moves, because `bevy_ui::Outline`'s own
/// documentation warns that repeated insertion and removal causes archetype
/// table moves; `apply_last_beacon_ui_focus_outline` only ever mutates its
/// `color` field afterward.
pub fn apply_last_beacon_ui_focusability(
    mut commands: Commands,
    buttons: Query<Entity, Added<LastBeaconUiButton>>,
    tabs: Query<Entity, Added<LastBeaconUiTab>>,
    value_buttons: Query<Entity, Added<LastBeaconUiValueButton>>,
    dropdown_toggles: Query<Entity, Added<LastBeaconUiDropdownToggle>>,
    sliders: Query<Entity, Added<LastBeaconUiSlider>>,
) {
    let newly_focusable_entities = buttons
        .iter()
        .chain(tabs.iter())
        .chain(value_buttons.iter())
        .chain(dropdown_toggles.iter())
        .chain(sliders.iter());

    for focusable_entity in newly_focusable_entities {
        commands.entity(focusable_entity).insert((
            TabIndex(0),
            LastBeaconUiFocusIndicator,
            Outline::new(
                Val::Px(LAST_BEACON_FOCUS_OUTLINE_WIDTH),
                Val::Px(LAST_BEACON_FOCUS_OUTLINE_OFFSET),
                Color::NONE,
            ),
        ));
    }
}

/// Mirrors keyboard/gamepad focus state into the visible focus outline.
///
/// Only writes `Outline.color` when it actually needs to change, both to
/// avoid unnecessary change-detection churn and because repeatedly inserting
/// a fresh `Outline` instead would cause the archetype table moves that
/// `bevy_ui::Outline`'s documentation warns against.
pub fn apply_last_beacon_ui_focus_outline(
    input_focus: Res<InputFocus>,
    input_focus_visible: Res<InputFocusVisible>,
    mut focus_indicators: Query<(Entity, &mut Outline), With<LastBeaconUiFocusIndicator>>,
) {
    if !input_focus.is_changed() && !input_focus_visible.is_changed() {
        return;
    }

    let visibly_focused_entity = input_focus.get().filter(|_| input_focus_visible.0);

    for (focus_indicator_entity, mut outline) in &mut focus_indicators {
        let target_outline_color = if Some(focus_indicator_entity) == visibly_focused_entity {
            LAST_BEACON_FOCUS_OUTLINE_COLOR
        } else {
            Color::NONE
        };

        if outline.color != target_outline_color {
            outline.color = target_outline_color;
        }
    }
}

/// Applies scrollbar track dragging to multiline text boxes.
#[allow(clippy::too_many_arguments)]
pub fn drag_last_beacon_ui_text_box_scrollbars(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut scroll_drag: ResMut<LastBeaconUiTextBoxScrollDrag>,
    mut scroll_overrides: ResMut<LastBeaconUiTextBoxScrollOverrides>,
    text_inputs: Query<(Entity, &LastBeaconUiTextInput, Option<&Children>)>,
    children_query: Query<&Children>,
    vertical_scroll_track_query: Query<
        (&Interaction, &RelativeCursorPosition),
        With<LastBeaconUiTextScrollTrack>,
    >,
    horizontal_scroll_track_query: Query<
        (&Interaction, &RelativeCursorPosition),
        With<LastBeaconUiTextHorizontalScrollTrack>,
    >,
    editable_text_marker_query: Query<(), With<EditableText>>,
    mut editable_text_query: Query<
        (&mut TextScroll, &TextLayoutInfo, &ComputedNode),
        With<EditableText>,
    >,
) {
    if !mouse_buttons.pressed(MouseButton::Left) {
        scroll_drag.active_text_box = None;
        return;
    }

    for (input_entity, text_input, input_children) in &text_inputs {
        if !text_input.multiline {
            continue;
        }

        let vertical_scroll_track = first_descendant_with_scroll_track(
            input_children,
            &children_query,
            &vertical_scroll_track_query,
        )
        .and_then(|scroll_track_entity| vertical_scroll_track_query.get(scroll_track_entity).ok());
        let horizontal_scroll_track = first_descendant_with_horizontal_scroll_track(
            input_children,
            &children_query,
            &horizontal_scroll_track_query,
        )
        .and_then(|scroll_track_entity| {
            horizontal_scroll_track_query.get(scroll_track_entity).ok()
        });

        if vertical_scroll_track
            .is_some_and(|(interaction, _)| *interaction == Interaction::Pressed)
        {
            scroll_drag.active_text_box = Some(input_entity);
            scroll_drag.axis = LastBeaconUiTextBoxScrollAxis::Vertical;
        }
        if horizontal_scroll_track
            .is_some_and(|(interaction, _)| *interaction == Interaction::Pressed)
        {
            scroll_drag.active_text_box = Some(input_entity);
            scroll_drag.axis = LastBeaconUiTextBoxScrollAxis::Horizontal;
        }
        if scroll_drag.active_text_box != Some(input_entity) {
            continue;
        }

        let relative_cursor_position = match scroll_drag.axis {
            LastBeaconUiTextBoxScrollAxis::Vertical => {
                vertical_scroll_track.map(|(_, relative_cursor_position)| relative_cursor_position)
            }
            LastBeaconUiTextBoxScrollAxis::Horizontal => horizontal_scroll_track
                .map(|(_, relative_cursor_position)| relative_cursor_position),
        };
        let Some(normalized_cursor_position) =
            relative_cursor_position.and_then(|position| position.normalized)
        else {
            continue;
        };
        let Some(editable_text_entity) = first_descendant_with_editable_text(
            input_children,
            &children_query,
            &editable_text_marker_query,
        ) else {
            continue;
        };
        let Ok((mut text_scroll, text_layout, computed_node)) =
            editable_text_query.get_mut(editable_text_entity)
        else {
            continue;
        };
        let current_scroll = scroll_overrides
            .scroll_by_text_entity
            .get(&editable_text_entity)
            .copied()
            .unwrap_or(text_scroll.0);
        let next_scroll = match scroll_drag.axis {
            LastBeaconUiTextBoxScrollAxis::Vertical => {
                let scroll_progress = (normalized_cursor_position.y + 0.5).clamp(0.0, 1.0);
                let next_scroll_y =
                    scroll_progress * text_box_max_scroll_y(text_layout, computed_node);
                Vec2::new(current_scroll.x, next_scroll_y)
            }
            LastBeaconUiTextBoxScrollAxis::Horizontal => {
                let scroll_progress = (normalized_cursor_position.x + 0.5).clamp(0.0, 1.0);
                let next_scroll_x =
                    scroll_progress * text_box_max_scroll_x(text_layout, computed_node);
                Vec2::new(next_scroll_x, current_scroll.y)
            }
        };
        text_scroll.0 = clamp_text_box_scroll(next_scroll, text_layout, computed_node);
        scroll_overrides
            .scroll_by_text_entity
            .insert(editable_text_entity, text_scroll.0);
    }
}

/// Applies mouse-wheel scrolling to hovered multiline text boxes.
pub fn scroll_last_beacon_ui_text_inputs(
    mut mouse_wheel_messages: MessageReader<MouseWheel>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut scroll_overrides: ResMut<LastBeaconUiTextBoxScrollOverrides>,
    mut text_inputs: LastBeaconUiTextInputScrollQuery,
    children_query: Query<&Children>,
    editable_text_marker_query: Query<(), With<EditableText>>,
    mut editable_text_query: Query<
        (&mut TextScroll, &TextLayoutInfo, &ComputedNode),
        With<EditableText>,
    >,
) {
    let shift_is_pressed =
        keyboard_input.pressed(KeyCode::ShiftLeft) || keyboard_input.pressed(KeyCode::ShiftRight);
    let mut vertical_scroll_delta = 0.0;
    let mut horizontal_scroll_delta = 0.0;

    for mouse_wheel_message in mouse_wheel_messages.read() {
        let message_scroll_delta = match mouse_wheel_message.unit {
            MouseScrollUnit::Line => Vec2::new(
                mouse_wheel_message.x * TEXT_BOX_SCROLL_LINE_STEP,
                mouse_wheel_message.y * TEXT_BOX_SCROLL_LINE_STEP,
            ),
            MouseScrollUnit::Pixel => Vec2::new(mouse_wheel_message.x, mouse_wheel_message.y),
        };

        if shift_is_pressed {
            horizontal_scroll_delta += message_scroll_delta.x - message_scroll_delta.y;
        } else {
            horizontal_scroll_delta += message_scroll_delta.x;
            vertical_scroll_delta += message_scroll_delta.y;
        }
    }

    if vertical_scroll_delta.abs() < f32::EPSILON && horizontal_scroll_delta.abs() < f32::EPSILON {
        return;
    }

    for (_input_entity, text_input, input_children, interaction, relative_cursor_position) in
        &mut text_inputs
    {
        let cursor_is_over = relative_cursor_position
            .map(RelativeCursorPosition::cursor_over)
            .unwrap_or_else(|| {
                interaction.is_some_and(|interaction| *interaction != Interaction::None)
            });
        if !text_input.multiline || !cursor_is_over {
            continue;
        }
        let Some(editable_text_entity) = first_descendant_with_editable_text(
            input_children,
            &children_query,
            &editable_text_marker_query,
        ) else {
            continue;
        };
        let Ok((mut text_scroll, text_layout, computed_node)) =
            editable_text_query.get_mut(editable_text_entity)
        else {
            continue;
        };
        let requested_scroll = Vec2::new(
            text_scroll.0.x + horizontal_scroll_delta,
            text_scroll.0.y - vertical_scroll_delta,
        );
        text_scroll.0 = clamp_text_box_scroll(requested_scroll, text_layout, computed_node);
        scroll_overrides
            .scroll_by_text_entity
            .insert(editable_text_entity, text_scroll.0);
    }
}

/// Records keyboard edits/navigation that should make the native caret visible again.
pub fn request_last_beacon_ui_text_box_caret_scroll_for_keyboard_input(
    mut keyboard_input_messages: MessageReader<KeyboardInput>,
    input_focus: Res<InputFocus>,
    editable_text_query: Query<(), With<EditableText>>,
    mut caret_scroll_requests: ResMut<LastBeaconUiTextBoxCaretScrollRequests>,
) {
    let Some(focused_text_entity) = input_focus.get() else {
        keyboard_input_messages.read().for_each(drop);
        return;
    };
    if !editable_text_query.contains(focused_text_entity) {
        keyboard_input_messages.read().for_each(drop);
        return;
    }

    for keyboard_input_message in keyboard_input_messages.read() {
        if !keyboard_input_message.state.is_pressed() {
            continue;
        }
        if keyboard_input_should_reveal_text_caret(&keyboard_input_message.logical_key) {
            caret_scroll_requests
                .text_entities
                .insert(focused_text_entity);
        }
    }
}

/// Reapplies user text-box scroll after Bevy's native cursor-visibility scroll runs.
pub fn apply_last_beacon_ui_text_box_scroll_overrides(
    mut scroll_overrides: ResMut<LastBeaconUiTextBoxScrollOverrides>,
    mut caret_scroll_requests: ResMut<LastBeaconUiTextBoxCaretScrollRequests>,
    mut editable_text_query: Query<
        (&mut TextScroll, &TextLayoutInfo, &ComputedNode),
        With<EditableText>,
    >,
) {
    for (text_entity, stored_scroll) in &mut scroll_overrides.scroll_by_text_entity {
        let Ok((mut text_scroll, text_layout, computed_node)) =
            editable_text_query.get_mut(*text_entity)
        else {
            continue;
        };

        // Mouse wheel and scrollbar dragging are allowed to move the caret offscreen.
        // Keyboard typing/navigation intentionally returns control to Bevy's native
        // caret-visible scroll, then stores that position as the new user override.
        if caret_scroll_requests.text_entities.remove(text_entity) {
            text_scroll.0 = clamp_text_box_scroll(text_scroll.0, text_layout, computed_node);
            *stored_scroll = text_scroll.0;
            continue;
        }

        text_scroll.0 = clamp_text_box_scroll(*stored_scroll, text_layout, computed_node);
    }
}

/// Query bundle used to refresh authored multiline text-box scrollbars.
#[derive(SystemParam)]
pub struct LastBeaconTextBoxScrollbarQueries<'w, 's> {
    text_inputs: Query<
        'w,
        's,
        (
            &'static LastBeaconUiTextInput,
            &'static ComputedNode,
            Option<&'static Children>,
        ),
    >,
    children_query: Query<'w, 's, &'static Children>,
    editable_text_marker_query: Query<'w, 's, (), With<EditableText>>,
    editable_text_query: Query<
        'w,
        's,
        (
            &'static TextScroll,
            &'static TextLayoutInfo,
            &'static ComputedNode,
        ),
        With<EditableText>,
    >,
    vertical_scroll_track_query: Query<
        'w,
        's,
        (&'static Interaction, &'static RelativeCursorPosition),
        With<LastBeaconUiTextScrollTrack>,
    >,
    vertical_scroll_thumb_query: Query<'w, 's, (), With<LastBeaconUiTextScrollThumb>>,
    horizontal_scroll_track_query: Query<
        'w,
        's,
        (&'static Interaction, &'static RelativeCursorPosition),
        With<LastBeaconUiTextHorizontalScrollTrack>,
    >,
    horizontal_scroll_thumb_query: Query<'w, 's, (), With<LastBeaconUiTextHorizontalScrollThumb>>,
    node_query: Query<'w, 's, &'static mut Node>,
}

/// Mirrors Bevy's native text scroll state into the authored scrollbar thumb.
pub fn refresh_last_beacon_ui_text_box_scrollbars(
    mut scrollbar_queries: LastBeaconTextBoxScrollbarQueries,
) {
    for (text_input, text_box_node, input_children) in &scrollbar_queries.text_inputs {
        if !text_input.multiline {
            continue;
        }
        let Some(editable_text_entity) = first_descendant_with_editable_text(
            input_children,
            &scrollbar_queries.children_query,
            &scrollbar_queries.editable_text_marker_query,
        ) else {
            continue;
        };
        let Ok((text_scroll, text_layout, computed_node)) = scrollbar_queries
            .editable_text_query
            .get(editable_text_entity)
        else {
            continue;
        };
        let has_vertical_overflow =
            text_box_max_scroll_y(text_layout, computed_node) > f32::EPSILON;
        let has_horizontal_overflow =
            text_box_max_scroll_x(text_layout, computed_node) > f32::EPSILON;
        let scrollbar_layout = text_box_scrollbar_layout(
            text_box_node,
            text_layout,
            computed_node,
            has_vertical_overflow,
            has_horizontal_overflow,
        );

        if let Some(scroll_track_entity) = first_descendant_with_scroll_track(
            input_children,
            &scrollbar_queries.children_query,
            &scrollbar_queries.vertical_scroll_track_query,
        ) {
            if let Ok(mut scroll_track_node) =
                scrollbar_queries.node_query.get_mut(scroll_track_entity)
            {
                scroll_track_node.display = if has_vertical_overflow {
                    Display::Flex
                } else {
                    Display::None
                };
                scroll_track_node.right = Val::Px(scrollbar_layout.inset);
                scroll_track_node.top = Val::Px(scrollbar_layout.vertical_track_top);
                scroll_track_node.width = Val::Px(scrollbar_layout.thickness);
                scroll_track_node.height = Val::Px(scrollbar_layout.vertical_track_length);
            }
        }

        if let Some(scroll_thumb_entity) = first_descendant_with_scroll_thumb(
            input_children,
            &scrollbar_queries.children_query,
            &scrollbar_queries.vertical_scroll_thumb_query,
        ) {
            let max_scroll_y = text_box_max_scroll_y(text_layout, computed_node);
            let scroll_progress = if max_scroll_y <= f32::EPSILON {
                0.0
            } else {
                (text_scroll.0.y / max_scroll_y).clamp(0.0, 1.0)
            };
            if let Ok(mut scroll_thumb_node) =
                scrollbar_queries.node_query.get_mut(scroll_thumb_entity)
            {
                scroll_thumb_node.width = Val::Px(scrollbar_layout.thickness);
                scroll_thumb_node.height = Val::Px(scrollbar_layout.vertical_thumb_length);
                scroll_thumb_node.top =
                    Val::Px(scroll_progress * scrollbar_layout.vertical_thumb_travel());
            }
        }

        if let Some(scroll_track_entity) = first_descendant_with_horizontal_scroll_track(
            input_children,
            &scrollbar_queries.children_query,
            &scrollbar_queries.horizontal_scroll_track_query,
        ) {
            if let Ok(mut scroll_track_node) =
                scrollbar_queries.node_query.get_mut(scroll_track_entity)
            {
                scroll_track_node.display = if has_horizontal_overflow {
                    Display::Flex
                } else {
                    Display::None
                };
                scroll_track_node.left = Val::Px(scrollbar_layout.horizontal_track_left);
                scroll_track_node.bottom = Val::Px(scrollbar_layout.inset);
                scroll_track_node.width = Val::Px(scrollbar_layout.horizontal_track_length);
                scroll_track_node.height = Val::Px(scrollbar_layout.thickness);
            }
        }

        if let Some(scroll_thumb_entity) = first_descendant_with_horizontal_scroll_thumb(
            input_children,
            &scrollbar_queries.children_query,
            &scrollbar_queries.horizontal_scroll_thumb_query,
        ) {
            let max_scroll_x = text_box_max_scroll_x(text_layout, computed_node);
            let scroll_progress = if max_scroll_x <= f32::EPSILON {
                0.0
            } else {
                (text_scroll.0.x / max_scroll_x).clamp(0.0, 1.0)
            };
            if let Ok(mut scroll_thumb_node) =
                scrollbar_queries.node_query.get_mut(scroll_thumb_entity)
            {
                scroll_thumb_node.width = Val::Px(scrollbar_layout.horizontal_thumb_length);
                scroll_thumb_node.height = Val::Px(scrollbar_layout.thickness);
                scroll_thumb_node.left =
                    Val::Px(scroll_progress * scrollbar_layout.horizontal_thumb_travel());
            }
        }
    }
}

/// Updates slider values from cursor position while pressed or dragged.
pub fn update_last_beacon_ui_sliders(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut input_values: ResMut<LastBeaconUiInputValues>,
    sliders: LastBeaconUiSliderInteractionQuery,
) {
    for (_slider_entity, slider, interaction, relative_cursor_position) in &sliders {
        let slider_is_active = *interaction == Interaction::Pressed
            || (*interaction == Interaction::Hovered && mouse_buttons.pressed(MouseButton::Left));
        if !slider_is_active || slider.target.is_empty() {
            continue;
        }

        let Some(normalized_cursor_position) = relative_cursor_position.normalized else {
            continue;
        };
        let normalized_x = (normalized_cursor_position.x + 0.5).clamp(0.0, 1.0);
        let range = slider.max - slider.min;
        if range.abs() < f32::EPSILON {
            continue;
        }

        let next_value = slider.min + normalized_x * range;
        input_values
            .values
            .insert(slider.target.clone(), format_value(next_value));
    }
}

/// Mirrors stored slider values into fill widths.
pub fn refresh_last_beacon_ui_slider_fills(
    input_values: Res<LastBeaconUiInputValues>,
    mut slider_fills: Query<(&LastBeaconUiSliderFill, &mut Node)>,
    newly_spawned_slider_fills: Query<(), Added<LastBeaconUiSliderFill>>,
) {
    // See `refresh_last_beacon_ui_value_text` for why newly-spawned fills
    // must also be synced when the resource itself didn't change this frame.
    if !input_values.is_changed() && newly_spawned_slider_fills.is_empty() {
        return;
    }

    for (slider_fill, mut node) in &mut slider_fills {
        let Some(value) = input_values
            .values
            .get(&slider_fill.target)
            .and_then(|value| value.parse::<f32>().ok())
        else {
            continue;
        };
        let range = slider_fill.max - slider_fill.min;
        if range.abs() < f32::EPSILON {
            continue;
        }
        let percent = ((value - slider_fill.min) / range * 100.0).clamp(0.0, 100.0);
        node.width = Val::Percent(percent);
    }
}

/// Mirrors stored example input values into authored text labels.
pub fn refresh_last_beacon_ui_value_text(
    input_values: Res<LastBeaconUiInputValues>,
    mut value_texts: Query<(&LastBeaconUiValueText, &mut Text, Option<&mut EditableText>)>,
    newly_spawned_value_texts: Query<(), Added<LastBeaconUiValueText>>,
    mut font_cx: ResMut<FontCx>,
    mut layout_cx: ResMut<LayoutCx>,
) {
    // `input_values.is_changed()` only reflects mutations made *this frame*.
    // A widget that spawns on a later frame (async BSN widget loading, or a
    // scene reopened after the value was already set elsewhere) would never
    // pick up the existing stored value and would keep showing its authored
    // default until an unrelated value happened to change -- e.g. a Number
    // Field showing its `.bsn`-authored placeholder until the +/- buttons
    // were clicked. Also sync whenever a value text has just appeared.
    if !input_values.is_changed() && newly_spawned_value_texts.is_empty() {
        return;
    }

    for (value_text, mut text, editable_text) in &mut value_texts {
        let Some(value) = input_values.values.get(&value_text.target) else {
            continue;
        };
        let rendered_value = format!("{}{}{}", value_text.prefix, value, value_text.suffix);
        if text.0 != rendered_value {
            text.0 = rendered_value.clone();
        }
        if let Some(mut editable_text) = editable_text {
            if editable_text.value().to_string() != rendered_value {
                // Move the cursor to the new text's end synchronously, in
                // the same call as `set_text`, rather than queuing a
                // `TextEnd` edit for `apply_text_edits` to process later.
                // The queued approach left the cursor's stale byte offset
                // (from the *old* text) unresolved until that system's next
                // run; if the new text was shorter than the old one, that
                // stale offset could momentarily point past the end of the
                // buffer, producing an out-of-bounds scroll that blanked
                // the rendered glyphs for a frame. `move_to_text_end`
                // refreshes the layout for the text we just set and then
                // clamps the selection to its true end, so there is no gap
                // where the cursor is inconsistent with the buffer.
                editable_text.editor_mut().set_text(&rendered_value);
                editable_text
                    .editor_mut()
                    .driver(&mut font_cx, &mut layout_cx)
                    .move_to_text_end();
            }
        }
    }
}

/// Recovers a value-text `EditableText` whose computed glyph layout has
/// become stuck empty despite holding non-empty content, by despawning and
/// respawning the text entity from scratch.
///
/// This is a defensive recovery, not a root-cause fix. `TextLayoutInfo` can
/// get stuck with zero glyphs and zero size after a Number Field's value
/// changes, with no further edit ever triggering a successful retry -- this
/// is the reported "Number Field value sometimes disappears" bug. The exact
/// internal trigger inside Bevy's text stack is unconfirmed, but live
/// repro logging (comparing `EditableTextGeneration` against the editor's
/// own `generation()` on both healthy and stuck transitions) ruled out a
/// stale-generation bookkeeping mismatch as the cause: the two stay
/// perfectly in sync in *both* cases. Whatever is actually failing lives
/// deeper in the layout-build step itself, and it does not self-correct:
/// forcing repeated rebuilds via `set_text` on the same entity (an earlier
/// version of this recovery) produced a rebuild-refuses-to-produce-glyphs
/// loop that fired every frame indefinitely without ever recovering.
///
/// Two earlier, less drastic versions of this recovery were tried and
/// rejected before this one:
///
/// - Queuing a select-all-then-insert edit against the existing (broken)
///   `EditableText` made the bug *worse*: `TextEdit::TextEnd(true)` resolves
///   to parley's `select_to_text_end`, which extends the selection by
///   walking `self.editor.layout`'s lines (`Selection::move_lines` ->
///   `move_to_line` -> `layout.get(line_index)`). When that cached layout
///   genuinely has zero lines -- our exact stuck state -- `move_to_line`
///   finds no line and returns the selection *unchanged* (a silent no-op),
///   so "select all" doesn't select anything, and the follow-up `Insert`
///   inserts at the stale cursor position instead of replacing the buffer --
///   *prepending* the current value to itself every frame the bug
///   persisted, observed in practice as the field settling on "250"
///   (`number_input`'s `max`) once the prepended digits parsed as a number
///   far outside the valid range and got clamped.
/// - Replacing just the `EditableText` component with a freshly constructed
///   one (to get a guaranteed-clean `PlainEditor`) did not recover anything:
///   a fresh `EditableText::new` always nudges its internal generation
///   through the same small, deterministic sequence (construct -> queue
///   `TextEnd` -> apply -> nudge), landing on the exact same value the
///   entity's separate, *not reset* `EditableTextGeneration` component was
///   already sitting at. `layout_changed` then read false forever.
/// - Forcing `layout_dirty` back to `true` on the existing editor via
///   `set_text` (same value) does trigger a genuine rebuild and a genuine
///   generation nudge every time -- confirmed live, `generation_matches_editor`
///   stayed in sync exactly as designed -- but the rebuild itself kept
///   producing zero glyphs anyway, every single frame, forever. Whatever is
///   broken is upstream of the dirty/generation bookkeeping entirely.
///
/// Since neither editing the existing `EditableText` nor swapping it out in
/// place recovers a stuck entity, this despawns the value-text entity
/// outright and spawns a full replacement with a brand new entity ID,
/// mirroring exactly what `initialize_last_beacon_ui_text_inputs` builds
/// for a non-multiline Number Field value text (fixed-width, centered,
/// digit-filtered `EditableText`). A new entity ID guarantees every piece
/// of state tied to the old one -- `EditableTextGeneration`, the cached
/// `TextLayoutInfo`, the `PlainEditor`'s internal layout cache, and
/// whatever else is actually stuck -- is gone rather than merely reset in
/// place, at the cost of a one-frame identity change (any input focus on
/// this exact entity is not preserved).
///
/// Scoped to `LastBeaconUiNumberInput` entities specifically because that is
/// currently the only widget shape that pairs `EditableText` with
/// `LastBeaconUiValueText`; other value-text widgets (sliders, combo boxes)
/// use plain, non-editable `Text` and never hit this stuck state.
pub fn heal_last_beacon_ui_value_text_stuck_glyphs(
    mut commands: Commands,
    value_texts: Query<
        (
            Entity,
            &ChildOf,
            &EditableText,
            &TextLayoutInfo,
            &LastBeaconUiValueText,
            &LastBeaconUiNumberInput,
            &TextFont,
            &TextColor,
        ),
        With<LastBeaconUiValueText>,
    >,
) {
    for (
        entity,
        child_of,
        editable_text,
        layout_info,
        value_text,
        number_input,
        text_font,
        text_color,
    ) in &value_texts
    {
        if !layout_info.glyphs.is_empty() {
            continue;
        }
        let current_value = editable_text.value().to_string();
        if current_value.is_empty() {
            continue;
        }

        warn!(
            "[heal] value-text target={:?} got stuck with an empty glyph layout while \
             holding {current_value:?}; despawning and respawning it to recover",
            value_text.target
        );

        let parent = child_of.parent();
        let mut fresh_editable_text = EditableText::new(&current_value);
        fresh_editable_text.visible_width = Some(24.0);
        fresh_editable_text.visible_lines = Some(1.0);
        fresh_editable_text.allow_newlines = false;

        commands.entity(entity).despawn();

        let replacement_entity = commands
            .spawn((
                value_text.clone(),
                number_input.clone(),
                Text::new(current_value),
                text_font.clone(),
                *text_color,
                fresh_editable_text,
                TextScroll::default(),
                TextCursorStyle::default(),
                TextLayout {
                    justify: Justify::Center,
                    ..default()
                },
                LineHeight::default(),
                Node {
                    width: Val::Px(NUMBER_INPUT_VALUE_TEXT_WIDTH_PX),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                EditableTextFilter::new(number_input_allows_character),
            ))
            .id();
        commands.entity(parent).add_child(replacement_entity);
    }
}

/// Selects a single reusable tab within its shared selection group.
///
/// Factored out of `update_last_beacon_ui_tab_selection` so
/// `activate_last_beacon_ui_focused_widget_on_keyboard_input` can trigger the
/// exact same effect from Enter/Space on a focused tab.
fn apply_tab_selection_activation(
    tab: &LastBeaconUiTab,
    tab_selections: &mut LastBeaconUiTabSelections,
) {
    tab_selections
        .selected_tabs
        .insert(tab.group.clone(), tab.tab.clone());
}

/// Updates remembered tab selection when a reusable tab is clicked.
pub fn update_last_beacon_ui_tab_selection(
    mut tab_selections: ResMut<LastBeaconUiTabSelections>,
    tabs: LastBeaconUiTabInteractionQuery,
) {
    for (tab, tab_interaction) in &tabs {
        if *tab_interaction == Interaction::Pressed {
            apply_tab_selection_activation(tab, &mut tab_selections);
        }
    }
}

/// Activates the currently focused widget when Enter or Space is pressed.
///
/// This deliberately never touches the `Interaction` component to simulate a
/// mouse press: `bevy_ui::focus::ui_focus_system` only ever resets a
/// non-hovered node's `Interaction` from `Hovered` back to `None`, never from
/// `Pressed`, so a keyboard-forced `Pressed` on a widget the cursor is not
/// over would get stuck indefinitely. Instead this calls the same small
/// activation helper the matching mouse-driven system already calls.
#[allow(clippy::too_many_arguments)]
pub fn activate_last_beacon_ui_focused_widget_on_keyboard_input(
    mut keyboard_input_messages: MessageReader<KeyboardInput>,
    input_focus: Res<InputFocus>,
    value_buttons: Query<&LastBeaconUiValueButton>,
    dropdown_toggles: Query<&LastBeaconUiDropdownToggle>,
    tabs: Query<&LastBeaconUiTab>,
    mut input_values: ResMut<LastBeaconUiInputValues>,
    mut dropdown_states: ResMut<LastBeaconUiDropdownStates>,
    mut tab_selections: ResMut<LastBeaconUiTabSelections>,
) {
    let Some(focused_entity) = input_focus.get() else {
        keyboard_input_messages.read().for_each(drop);
        return;
    };

    let activation_key_was_pressed = keyboard_input_messages
        .read()
        .any(|keyboard_input_message| {
            keyboard_input_message.state.is_pressed()
                && matches!(keyboard_input_message.logical_key, Key::Enter | Key::Space)
        });
    if !activation_key_was_pressed {
        return;
    }

    if let Ok(value_button) = value_buttons.get(focused_entity) {
        apply_value_button_activation(value_button, &mut input_values, &mut dropdown_states);
    }
    if let Ok(dropdown_toggle) = dropdown_toggles.get(focused_entity) {
        apply_dropdown_toggle_activation(dropdown_toggle, &mut dropdown_states);
    }
    if let Ok(tab) = tabs.get(focused_entity) {
        apply_tab_selection_activation(tab, &mut tab_selections);
    }
}

/// Shows only the reusable tab panel that matches the current tab selection.
pub fn refresh_last_beacon_ui_tab_panels(
    tab_selections: Res<LastBeaconUiTabSelections>,
    mut tab_panels: LastBeaconUiTabPanelQuery,
    newly_spawned_tab_panels: Query<(), Added<LastBeaconUiTabPanel>>,
) {
    // See `refresh_last_beacon_ui_value_text` for why newly-spawned panels
    // must also be synced when the resource itself didn't change this frame.
    if !tab_selections.is_changed() && newly_spawned_tab_panels.is_empty() {
        return;
    }

    for (tab_panel, mut node) in &mut tab_panels {
        let selected_tab = tab_selections.selected_tabs.get(&tab_panel.group);
        let panel_is_selected = selected_tab
            .map(|selected_tab| selected_tab == &tab_panel.tab)
            .unwrap_or(tab_panel.selected);
        node.display = if panel_is_selected {
            Display::Flex
        } else {
            Display::None
        };
    }
}

/// Restores prototype-authored button colors after generic Foundation interaction styling.
pub fn enforce_last_beacon_button_styles(
    theme: Res<FoundationUiTheme>,
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
        let button_style = reusable_button_style(&theme, &button.variant, *button_interaction);
        *button_background = BackgroundColor(button_style.background_color);
        *button_border = BorderColor::all(button_style.border_color);
        apply_text_color(button_children, button_style.text_color, &mut text_colors);
    }

    for (tab, tab_interaction, mut tab_background, mut tab_border, tab_children) in &mut ui_tabs {
        let selected_tab = tab_selections.selected_tabs.get(&tab.group);
        let is_selected = selected_tab
            .map(|selected_tab| selected_tab == &tab.tab)
            .unwrap_or(tab.selected);
        let tab_style = reusable_tab_style(&theme, is_selected, *tab_interaction);
        *tab_background = BackgroundColor(tab_style.background_color);
        *tab_border = BorderColor::all(tab_style.border_color);
        apply_text_color(tab_children, tab_style.text_color, &mut text_colors);
    }

    for mut button_background in &mut main_menu_primary_buttons {
        *button_background = BackgroundColor(theme.color(FoundationUiColorToken::Accent));
    }

    for mut button_background in &mut beacon_primary_buttons {
        *button_background = BackgroundColor(theme.color(FoundationUiColorToken::SecondaryAccent));
    }

    for mut button_background in &mut beacon_tab_buttons {
        *button_background = BackgroundColor(theme.color(FoundationUiColorToken::Transparent));
    }
}

#[derive(Clone, Copy, Debug)]
struct LastBeaconWidgetStyle {
    background_color: Color,
    border_color: Color,
    text_color: Color,
}

fn reusable_button_style(
    theme: &FoundationUiTheme,
    variant: &str,
    interaction: Interaction,
) -> LastBeaconWidgetStyle {
    let normalized_variant = variant.trim().to_ascii_lowercase();
    match (normalized_variant.as_str(), interaction) {
        ("primary", Interaction::Pressed) => LastBeaconWidgetStyle {
            background_color: theme.color(FoundationUiColorToken::AccentPressed),
            border_color: theme.color(FoundationUiColorToken::AccentPressed),
            text_color: theme.color(FoundationUiColorToken::TextOnAccent),
        },
        ("primary", Interaction::Hovered) => LastBeaconWidgetStyle {
            background_color: theme.color(FoundationUiColorToken::AccentHover),
            border_color: theme.color(FoundationUiColorToken::AccentHover),
            text_color: theme.color(FoundationUiColorToken::TextOnAccent),
        },
        ("primary", _) => LastBeaconWidgetStyle {
            background_color: theme.color(FoundationUiColorToken::Accent),
            border_color: theme.color(FoundationUiColorToken::Accent),
            text_color: theme.color(FoundationUiColorToken::TextOnAccent),
        },
        ("tertiary", Interaction::Pressed) => LastBeaconWidgetStyle {
            background_color: theme.color_alpha(FoundationUiColorToken::Accent, 0.18),
            border_color: theme.color(FoundationUiColorToken::Accent),
            text_color: theme.color(FoundationUiColorToken::Accent),
        },
        ("tertiary", Interaction::Hovered) => LastBeaconWidgetStyle {
            background_color: theme.color_alpha(FoundationUiColorToken::Accent, 0.1),
            border_color: theme.color(FoundationUiColorToken::Accent),
            text_color: theme.color(FoundationUiColorToken::AccentHover),
        },
        ("tertiary", _) => LastBeaconWidgetStyle {
            background_color: theme.color(FoundationUiColorToken::Transparent),
            border_color: theme.color(FoundationUiColorToken::Border),
            text_color: theme.color(FoundationUiColorToken::TextMuted),
        },
        (_, Interaction::Pressed) => LastBeaconWidgetStyle {
            background_color: theme.color(FoundationUiColorToken::SurfaceStrong),
            border_color: theme.color(FoundationUiColorToken::TextMuted),
            text_color: theme.color(FoundationUiColorToken::TextPrimary),
        },
        (_, Interaction::Hovered) => LastBeaconWidgetStyle {
            background_color: theme.color(FoundationUiColorToken::SurfaceStrong),
            border_color: theme.color(FoundationUiColorToken::BorderHover),
            text_color: theme.color(FoundationUiColorToken::TextPrimary),
        },
        (_, _) => LastBeaconWidgetStyle {
            background_color: theme.color(FoundationUiColorToken::Surface),
            border_color: theme.color(FoundationUiColorToken::Border),
            text_color: theme.color(FoundationUiColorToken::TextPrimary),
        },
    }
}

fn reusable_tab_style(
    theme: &FoundationUiTheme,
    is_selected: bool,
    interaction: Interaction,
) -> LastBeaconWidgetStyle {
    match (is_selected, interaction) {
        (true, Interaction::Pressed) => LastBeaconWidgetStyle {
            background_color: theme.color_alpha(FoundationUiColorToken::Accent, 0.22),
            border_color: theme.color(FoundationUiColorToken::Accent),
            text_color: theme.color(FoundationUiColorToken::Accent),
        },
        (true, _) => LastBeaconWidgetStyle {
            background_color: theme.color(FoundationUiColorToken::AccentSoft),
            border_color: theme.color(FoundationUiColorToken::Accent),
            text_color: theme.color(FoundationUiColorToken::Accent),
        },
        (false, Interaction::Pressed) => LastBeaconWidgetStyle {
            background_color: theme.color_alpha(FoundationUiColorToken::Accent, 0.16),
            border_color: theme.color(FoundationUiColorToken::Accent),
            text_color: theme.color(FoundationUiColorToken::Accent),
        },
        (false, Interaction::Hovered) => LastBeaconWidgetStyle {
            background_color: theme.color_alpha(FoundationUiColorToken::Accent, 0.08),
            border_color: theme.color(FoundationUiColorToken::Border),
            text_color: theme.color(FoundationUiColorToken::TextPrimary),
        },
        (false, _) => LastBeaconWidgetStyle {
            background_color: theme.color(FoundationUiColorToken::Transparent),
            border_color: theme.color(FoundationUiColorToken::Transparent),
            text_color: theme.color(FoundationUiColorToken::TextMuted),
        },
    }
}

fn format_value(value: f32) -> String {
    if value.fract().abs() < f32::EPSILON {
        format!("{value:.0}")
    } else {
        format!("{value:.1}")
    }
}

fn queue_text_input_click_placement(
    text_entity: Entity,
    editable_text_query: &mut Query<(
        &mut EditableText,
        &ComputedNode,
        &ComputedUiRenderTargetInfo,
        &UiGlobalTransform,
        &TextScroll,
    )>,
    windows: &Query<&Window, With<PrimaryWindow>>,
    ui_scale: f32,
) {
    let Ok(window) = windows.single() else {
        return;
    };
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };
    let Ok((mut editable_text, computed_node, render_target_info, global_transform, text_scroll)) =
        editable_text_query.get_mut(text_entity)
    else {
        return;
    };
    let Some(local_position) = global_transform.try_inverse().map(|inverse| {
        inverse.transform_point2(cursor_position * render_target_info.scale_factor() / ui_scale)
            - computed_node.content_box().min
            + text_scroll.0
    }) else {
        return;
    };

    editable_text.queue_edit(TextEdit::MoveToPoint(local_position));
}

fn insert_input_value_if_changed(
    input_values: &mut LastBeaconUiInputValues,
    target: &str,
    value: String,
) {
    if input_values
        .values
        .get(target)
        .is_some_and(|current_value| current_value == &value)
    {
        return;
    }

    input_values.values.insert(target.to_string(), value);
}

fn number_input_allows_character(character: char) -> bool {
    character.is_ascii_digit() || character == '.' || character == '-'
}

fn keyboard_input_should_reveal_text_caret(key: &Key) -> bool {
    match key {
        Key::Character(text) => !text.is_empty(),
        Key::Enter
        | Key::Space
        | Key::Backspace
        | Key::Delete
        | Key::ArrowLeft
        | Key::ArrowRight
        | Key::ArrowUp
        | Key::ArrowDown
        | Key::Home
        | Key::End
        | Key::PageUp
        | Key::PageDown => true,
        _ => false,
    }
}

fn clamp_text_box_scroll(
    scroll: Vec2,
    text_layout: &TextLayoutInfo,
    computed_node: &ComputedNode,
) -> Vec2 {
    Vec2::new(
        scroll
            .x
            .clamp(0.0, text_box_max_scroll_x(text_layout, computed_node)),
        scroll
            .y
            .clamp(0.0, text_box_max_scroll_y(text_layout, computed_node)),
    )
}

fn text_box_max_scroll_x(text_layout: &TextLayoutInfo, computed_node: &ComputedNode) -> f32 {
    (text_layout.size.x - computed_node.content_box().width()).max(0.0)
}

fn text_box_max_scroll_y(text_layout: &TextLayoutInfo, computed_node: &ComputedNode) -> f32 {
    (text_layout.size.y - computed_node.content_box().height()).max(0.0)
}

#[derive(Clone, Copy, Debug)]
struct LastBeaconUiTextBoxScrollbarLayout {
    thickness: f32,
    inset: f32,
    vertical_track_top: f32,
    vertical_track_length: f32,
    vertical_thumb_length: f32,
    horizontal_track_left: f32,
    horizontal_track_length: f32,
    horizontal_thumb_length: f32,
}

impl LastBeaconUiTextBoxScrollbarLayout {
    fn vertical_thumb_travel(self) -> f32 {
        (self.vertical_track_length - self.vertical_thumb_length).max(0.0)
    }

    fn horizontal_thumb_travel(self) -> f32 {
        (self.horizontal_track_length - self.horizontal_thumb_length).max(0.0)
    }
}

fn text_box_scrollbar_layout(
    text_box_node: &ComputedNode,
    text_layout: &TextLayoutInfo,
    text_node: &ComputedNode,
    has_vertical_overflow: bool,
    has_horizontal_overflow: bool,
) -> LastBeaconUiTextBoxScrollbarLayout {
    let text_box_size = text_box_node.size() * text_box_node.inverse_scale_factor;
    let shorter_text_box_axis = text_box_size.x.min(text_box_size.y).max(0.0);
    let thickness = (shorter_text_box_axis * TEXT_BOX_SCROLLBAR_THICKNESS_RATIO).clamp(
        TEXT_BOX_SCROLLBAR_MIN_THICKNESS,
        TEXT_BOX_SCROLLBAR_MAX_THICKNESS,
    );
    let inset = thickness * 0.67;
    let top_inset = inset * 2.0;
    // Only reserve the shared corner when both scrollbar axes are visible. If one
    // axis is hidden, the visible scrollbar should use that corner space too.
    let right_lane_width = if has_vertical_overflow && has_horizontal_overflow {
        thickness + inset * 2.0
    } else {
        inset
    };
    let bottom_lane_height = if has_vertical_overflow && has_horizontal_overflow {
        thickness + inset * 2.0
    } else {
        inset
    };
    let horizontal_track_left = thickness * 2.0;
    let vertical_track_length = (text_box_size.y - top_inset - bottom_lane_height).max(thickness);
    let horizontal_track_length =
        (text_box_size.x - horizontal_track_left - right_lane_width).max(thickness);
    let viewport_size = text_node.content_box().size();
    let vertical_thumb_length = proportional_scroll_thumb_length(
        vertical_track_length,
        viewport_size.y,
        text_layout.size.y,
    );
    let horizontal_thumb_length = proportional_scroll_thumb_length(
        horizontal_track_length,
        viewport_size.x,
        text_layout.size.x,
    );

    LastBeaconUiTextBoxScrollbarLayout {
        thickness,
        inset,
        vertical_track_top: top_inset,
        vertical_track_length,
        vertical_thumb_length,
        horizontal_track_left,
        horizontal_track_length,
        horizontal_thumb_length,
    }
}

fn proportional_scroll_thumb_length(
    track_length: f32,
    viewport_length: f32,
    content_length: f32,
) -> f32 {
    if content_length <= f32::EPSILON || content_length <= viewport_length {
        return track_length;
    }

    let visible_content_ratio =
        (viewport_length / content_length).clamp(TEXT_BOX_SCROLLBAR_MIN_THUMB_RATIO, 1.0);
    track_length * visible_content_ratio
}

fn first_descendant_with_text(
    children: Option<&Children>,
    children_query: &Query<&Children>,
    text_query: &Query<(), With<Text>>,
) -> Option<Entity> {
    first_matching_descendant(children, children_query, |entity| {
        text_query.contains(entity)
    })
}

fn first_descendant_with_editable_text(
    children: Option<&Children>,
    children_query: &Query<&Children>,
    editable_text_query: &Query<(), With<EditableText>>,
) -> Option<Entity> {
    first_matching_descendant(children, children_query, |entity| {
        editable_text_query.contains(entity)
    })
}

fn first_descendant_with_scroll_thumb(
    children: Option<&Children>,
    children_query: &Query<&Children>,
    scroll_thumb_query: &Query<(), With<LastBeaconUiTextScrollThumb>>,
) -> Option<Entity> {
    first_matching_descendant(children, children_query, |entity| {
        scroll_thumb_query.contains(entity)
    })
}

fn first_descendant_with_horizontal_scroll_thumb(
    children: Option<&Children>,
    children_query: &Query<&Children>,
    scroll_thumb_query: &Query<(), With<LastBeaconUiTextHorizontalScrollThumb>>,
) -> Option<Entity> {
    first_matching_descendant(children, children_query, |entity| {
        scroll_thumb_query.contains(entity)
    })
}

fn first_descendant_with_scroll_track(
    children: Option<&Children>,
    children_query: &Query<&Children>,
    scroll_track_query: &Query<
        (&Interaction, &RelativeCursorPosition),
        With<LastBeaconUiTextScrollTrack>,
    >,
) -> Option<Entity> {
    first_matching_descendant(children, children_query, |entity| {
        scroll_track_query.contains(entity)
    })
}

fn first_descendant_with_horizontal_scroll_track(
    children: Option<&Children>,
    children_query: &Query<&Children>,
    scroll_track_query: &Query<
        (&Interaction, &RelativeCursorPosition),
        With<LastBeaconUiTextHorizontalScrollTrack>,
    >,
) -> Option<Entity> {
    first_matching_descendant(children, children_query, |entity| {
        scroll_track_query.contains(entity)
    })
}

fn first_matching_descendant(
    children: Option<&Children>,
    children_query: &Query<&Children>,
    matches_entity: impl Fn(Entity) -> bool + Copy,
) -> Option<Entity> {
    let children = children?;
    for child_entity in children.iter() {
        if matches_entity(child_entity) {
            return Some(child_entity);
        }
        if let Ok(grandchildren) = children_query.get(child_entity) {
            if let Some(descendant) =
                first_matching_descendant(Some(grandchildren), children_query, matches_entity)
            {
                return Some(descendant);
            }
        }
    }
    None
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
                    widget_slot_entity_mut.remove::<SceneContentLoading>();
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
        // A failed widget load is still a settled outcome: reveal the parent
        // scene instead of hiding it forever because one widget broke.
        widget_slot_entity_mut.remove::<SceneContentLoading>();
        widget_slot_entity_mut.insert(LastBeaconBsnWidgetFailed {
            reason: failure_reason,
        });
    }
}

/// Configures a newly authored uniform grid container's `Node` for CSS Grid
/// layout with `column_count` equal-width columns.
pub fn apply_last_beacon_ui_uniform_grid(
    mut uniform_grids: Query<(&LastBeaconUiUniformGrid, &mut Node), Added<LastBeaconUiUniformGrid>>,
) {
    for (uniform_grid, mut node) in &mut uniform_grids {
        node.display = Display::Grid;
        node.grid_auto_flow = GridAutoFlow::Row;
        node.grid_template_columns = RepeatedGridTrack::flex(uniform_grid.column_count.max(1), 1.0);
    }
}

/// Translates an authored span into Bevy's native grid placement.
///
/// `GridPlacement::span` panics on a span of `0`, so a `.bsn`-authored `0`
/// (or default-initialized value) is clamped up to `1` -- Bevy's own default
/// placement already behaves as "span 1," so this never changes behavior for
/// an unset value.
pub fn apply_last_beacon_ui_grid_item_span(
    mut grid_items: Query<(&LastBeaconUiGridItem, &mut Node), Added<LastBeaconUiGridItem>>,
) {
    for (grid_item, mut node) in &mut grid_items {
        node.grid_column = GridPlacement::span(grid_item.column_span.max(1));
        node.grid_row = GridPlacement::span(grid_item.row_span.max(1));
    }
}

/// Keeps a `LastBeaconUiAspectRatioBounds` widget's own size within its
/// configured aspect-ratio band, derived from its parent's available content
/// space.
///
/// Runs after layout (`bevy::ui::UiSystems::PostLayout`) so the parent's
/// `ComputedNode` reflects this frame's actual size, matching the same
/// "read `ComputedNode`, write a derived `Node` value" shape already used by
/// `refresh_last_beacon_ui_text_box_scrollbars`.
pub fn apply_last_beacon_ui_aspect_ratio_bounds(
    mut aspect_ratio_widgets: Query<(&LastBeaconUiAspectRatioBounds, &ChildOf, &mut Node)>,
    parent_computed_nodes: Query<&ComputedNode>,
) {
    for (aspect_ratio_bounds, child_of, mut node) in &mut aspect_ratio_widgets {
        let Ok(parent_computed_node) = parent_computed_nodes.get(child_of.parent()) else {
            continue;
        };
        let available_size = parent_computed_node.content_box().size();
        if available_size.x <= 0.0 || available_size.y <= 0.0 {
            continue;
        }

        // A typo'd `min > max` degrades gracefully to the correctly-ordered
        // clamp instead of producing an inverted (always-empty) range.
        let (min_aspect_ratio, max_aspect_ratio) =
            if aspect_ratio_bounds.min_aspect_ratio <= aspect_ratio_bounds.max_aspect_ratio {
                (
                    aspect_ratio_bounds.min_aspect_ratio,
                    aspect_ratio_bounds.max_aspect_ratio,
                )
            } else {
                (
                    aspect_ratio_bounds.max_aspect_ratio,
                    aspect_ratio_bounds.min_aspect_ratio,
                )
            };

        let available_ratio = available_size.x / available_size.y;
        let clamped_ratio = available_ratio.clamp(min_aspect_ratio, max_aspect_ratio);

        // "Contain fit": the largest size at `clamped_ratio` that still fits
        // inside the parent's available content space.
        let width_at_available_height = available_size.y * clamped_ratio;
        let target_size = if width_at_available_height <= available_size.x {
            Vec2::new(width_at_available_height, available_size.y)
        } else {
            Vec2::new(available_size.x, available_size.x / clamped_ratio)
        };

        let target_width = Val::Px(target_size.x);
        let target_height = Val::Px(target_size.y);
        if node.width != target_width {
            node.width = target_width;
        }
        if node.height != target_height {
            node.height = target_height;
        }
    }
}

/// Plugin which registers Last Beacon's grid-span and aspect-ratio layout
/// widgets.
///
/// Grouped into its own plugin, rather than appended directly to
/// `LastBeaconPlugin`'s builder chain, to mirror how `bevy_feathers` composes
/// its own controls: one plugin per widget family, added together by a
/// parent plugin (`ControlsPlugin` there, `LastBeaconPlugin` here).
pub struct LastBeaconUiLayoutWidgetsPlugin;

impl Plugin for LastBeaconUiLayoutWidgetsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LastBeaconUiUniformGrid>()
            .register_type::<LastBeaconUiGridItem>()
            .register_type::<LastBeaconUiAspectRatioBounds>()
            .add_systems(
                Update,
                (
                    apply_last_beacon_ui_uniform_grid,
                    apply_last_beacon_ui_grid_item_span,
                ),
            )
            .add_systems(
                PostUpdate,
                apply_last_beacon_ui_aspect_ratio_bounds.after(bevy::ui::UiSystems::PostLayout),
            );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use foundation_runtime_library::scene_stack::SceneId;

    #[test]
    fn widget_asset_path_is_authored_explicitly() {
        let widget = LastBeaconBsnWidget {
            asset_path: "ui/widgets/main_menu/title.bsn".to_string(),
        };

        assert_eq!(widget.asset_path, "ui/widgets/main_menu/title.bsn");
    }

    #[test]
    fn queueing_a_scene_owned_widget_slot_marks_the_scene_as_still_loading() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(bevy::asset::AssetPlugin {
            file_path: ".".to_string(),
            ..default()
        });
        app.init_asset::<ScenePatch>();
        app.add_systems(Update, queue_last_beacon_bsn_widgets);

        let scene_owner = SceneOwner {
            scene_id: SceneId(1),
        };
        let widget_slot_entity = app
            .world_mut()
            .spawn((
                LastBeaconBsnWidget {
                    asset_path: "ui/widgets/common/divider.bsn".to_string(),
                },
                scene_owner,
            ))
            .id();

        app.update();

        assert!(
            app.world()
                .get::<SceneContentLoading>(widget_slot_entity)
                .is_some(),
            "a scene-owned widget slot must mark its scene as still loading while pending"
        );
        assert!(app
            .world()
            .get::<LastBeaconBsnWidgetPending>(widget_slot_entity)
            .is_some());
    }

    #[test]
    fn text_with_an_unloaded_font_is_marked_loading() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        // "." has no font files under it, so the load never completes —
        // exactly the "not loaded yet" state this test needs to observe.
        app.add_plugins(bevy::asset::AssetPlugin {
            file_path: ".".to_string(),
            ..default()
        });
        app.init_asset::<Font>();
        app.init_resource::<LastBeaconUiFontHandles>();
        app.add_systems(Update, apply_last_beacon_ui_font);

        let text_entity = app.world_mut().spawn(TextFont::default()).id();
        app.update();

        assert!(
            app.world()
                .get::<SceneContentLoading>(text_entity)
                .is_some(),
            "text must stay marked loading until its font finishes loading"
        );
    }

    #[test]
    fn text_finishing_its_font_load_clears_the_loading_marker() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(bevy::asset::AssetPlugin {
            file_path: crate::asset_root().to_string_lossy().to_string(),
            ..default()
        });
        // The real font loader (not just the Assets<Font> collection) is
        // required for a load to ever reach LoadState::Loaded.
        app.add_plugins(bevy::text::TextPlugin);
        app.init_resource::<LastBeaconUiFontHandles>();
        app.add_systems(
            Update,
            (
                apply_last_beacon_ui_font,
                reveal_last_beacon_text_once_fonts_load,
            )
                .chain(),
        );

        let text_entity = app.world_mut().spawn(TextFont::default()).id();

        // Real font files load asynchronously from disk; give it many frames.
        for _frame_number in 0..600 {
            app.update();
        }

        assert!(
            app.world()
                .get::<SceneContentLoading>(text_entity)
                .is_none(),
            "the loading marker must clear once the shared fonts finish loading"
        );
    }

    #[test]
    fn font_handles_are_reused_across_calls_instead_of_reloaded() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(bevy::asset::AssetPlugin {
            file_path: ".".to_string(),
            ..default()
        });
        app.init_asset::<Font>();
        app.init_resource::<LastBeaconUiFontHandles>();
        app.add_systems(Update, apply_last_beacon_ui_font);

        let first_text_entity = app.world_mut().spawn(TextFont::default()).id();
        app.update();
        let second_text_entity = app.world_mut().spawn(TextFont::default()).id();
        app.update();

        let get_font_handle_id = |world: &World, entity: Entity| {
            let FontSource::Handle(handle) = &world.get::<TextFont>(entity).unwrap().font else {
                panic!("expected a font handle");
            };
            handle.id()
        };
        assert_eq!(
            get_font_handle_id(app.world(), first_text_entity),
            get_font_handle_id(app.world(), second_text_entity),
            "every text entity should share the same persistent font handle, not a freshly loaded one"
        );
    }

    #[test]
    fn queueing_an_unowned_widget_slot_does_not_mark_anything_loading() {
        // Standalone widgets (no SceneOwner yet) have no scene to hide, so
        // they must not gain a marker that nothing will ever clear correctly.
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(bevy::asset::AssetPlugin {
            file_path: ".".to_string(),
            ..default()
        });
        app.init_asset::<ScenePatch>();
        app.add_systems(Update, queue_last_beacon_bsn_widgets);

        let widget_slot_entity = app
            .world_mut()
            .spawn(LastBeaconBsnWidget {
                asset_path: "ui/widgets/common/divider.bsn".to_string(),
            })
            .id();

        app.update();

        assert!(app
            .world()
            .get::<SceneContentLoading>(widget_slot_entity)
            .is_none());
    }

    #[test]
    fn applying_a_widget_clears_the_scene_loading_marker() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(bevy::asset::AssetPlugin {
            file_path: ".".to_string(),
            ..default()
        });
        app.init_asset::<ScenePatch>();
        app.add_systems(
            Update,
            (
                queue_last_beacon_bsn_widgets,
                apply_pending_last_beacon_bsn_widgets,
            )
                .chain(),
        );

        let scene_owner = SceneOwner {
            scene_id: SceneId(1),
        };
        let widget_slot_entity = app
            .world_mut()
            .spawn((
                LastBeaconBsnWidget {
                    asset_path: "ui/widgets/common/divider.bsn".to_string(),
                },
                scene_owner,
            ))
            .id();

        // Replace the asset-server-loaded handle with an inline scene patch
        // so the widget can actually resolve/apply without touching disk.
        let scene_patch = {
            let asset_server = app.world().resource::<AssetServer>();
            ScenePatch::load(asset_server, bevy::scene::bsn! { LastBeaconUiSymbolIcon })
        };
        let scene_handle = app
            .world_mut()
            .resource_mut::<Assets<ScenePatch>>()
            .add(scene_patch);
        app.update();
        app.world_mut()
            .entity_mut(widget_slot_entity)
            .insert(LastBeaconBsnWidgetPending {
                asset_path: "ui/widgets/common/divider.bsn".to_string(),
                scene_handle,
            });

        app.update();
        app.update();

        assert!(
            app.world()
                .get::<SceneContentLoading>(widget_slot_entity)
                .is_none(),
            "the scene loading marker must clear once the widget finishes applying"
        );
        assert!(app
            .world()
            .get::<LastBeaconBsnWidgetPending>(widget_slot_entity)
            .is_none());
    }

    struct FailingWidgetScene;

    impl bevy::scene::Scene for FailingWidgetScene {
        fn resolve(
            self,
            _context: &mut bevy::scene::ResolveContext,
            _scene: &mut bevy::scene::ResolvedScene,
        ) -> Result<(), bevy::scene::ResolveSceneError> {
            Err(bevy::scene::ResolveSceneError::MissingScene)
        }

        fn register_dependencies(&self, _dependencies: &mut bevy::scene::SceneDependencies) {}
    }

    #[test]
    fn a_failed_widget_load_still_clears_the_scene_loading_marker() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(bevy::asset::AssetPlugin {
            file_path: ".".to_string(),
            ..default()
        });
        app.init_asset::<ScenePatch>();
        app.add_systems(Update, apply_pending_last_beacon_bsn_widgets);

        let scene_handle = app
            .world_mut()
            .resource_mut::<Assets<ScenePatch>>()
            .add(ScenePatch {
                scene: Some(Box::new(FailingWidgetScene)),
                dependencies: Vec::new(),
                resolved: None,
            });
        let scene_owner = SceneOwner {
            scene_id: SceneId(1),
        };
        let widget_slot_entity = app
            .world_mut()
            .spawn((
                LastBeaconBsnWidget {
                    asset_path: "ui/widgets/common/missing.bsn".to_string(),
                },
                scene_owner,
                SceneContentLoading,
                LastBeaconBsnWidgetPending {
                    asset_path: "ui/widgets/common/missing.bsn".to_string(),
                    scene_handle,
                },
            ))
            .id();

        app.update();

        assert!(
            app.world()
                .get::<SceneContentLoading>(widget_slot_entity)
                .is_none(),
            "a failed widget load must not hide its parent scene forever"
        );
        assert!(app
            .world()
            .get::<LastBeaconBsnWidgetFailed>(widget_slot_entity)
            .is_some());
    }

    #[test]
    fn button_style_corrects_a_stray_overwrite_even_when_interaction_did_not_change() {
        // Regression test for a real bug: Foundation's generic
        // `update_foundation_menu_button_interactions` (Update) matches every
        // Last Beacon button too (they all carry `FoundationMenuButton`), and
        // can overwrite this system's authoritative color on a frame where,
        // from THIS system's own change-tick perspective, `Interaction`
        // never changed -- e.g. because Foundation's system only caught up to
        // a BSN-deferred-spawned entity a frame or more after this one did.
        // The Primary button in the UI Playground showed exactly this: it
        // rendered with Foundation's generic dark navy resting color instead
        // of its intended gold, leaving its dark text unreadable. Since
        // `enforce_last_beacon_button_styles` runs in `PostUpdate` -- always
        // after `Update` in the same frame -- it must win by simply running
        // unconditionally every frame, not by trying to detect the stray
        // write via change detection.
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.init_resource::<LastBeaconUiTabSelections>();
        app.insert_resource(crate::ui_theme::load_last_beacon_ui_theme());
        app.add_systems(Update, enforce_last_beacon_button_styles);

        let button_entity = app
            .world_mut()
            .spawn((
                Button,
                Interaction::None,
                BackgroundColor(Color::NONE),
                BorderColor::all(Color::NONE),
                LastBeaconUiButton {
                    variant: "secondary".to_string(),
                },
            ))
            .id();

        app.update();
        let correct_background_color = app.world().get::<BackgroundColor>(button_entity).unwrap().0;

        // Simulate a foreign system (like Foundation's) clobbering the color
        // on a later frame without touching `Interaction` at all.
        let stray_background_color = Color::srgb(1.0, 0.0, 1.0);
        app.world_mut()
            .get_mut::<BackgroundColor>(button_entity)
            .unwrap()
            .0 = stray_background_color;

        app.update();

        assert_eq!(
            app.world()
                .get::<BackgroundColor>(button_entity)
                .unwrap()
                .0,
            correct_background_color,
            "the style system must correct a stray color overwrite on the very next frame, \
             even though its own Interaction never changed"
        );
    }

    #[test]
    fn reusable_button_and_tab_styles_resolve_to_last_beacons_exact_shipped_colors() {
        // Snapshot regression test for the theme migration: these are the
        // exact literal `Color::srgb(...)`/`Color::srgba(...)` values
        // `reusable_button_style`/`reusable_tab_style` hardcoded before they
        // were rewritten to read `FoundationUiTheme` token lookups. Asserting
        // against the real shipped `theme.toml` (not a synthetic test theme)
        // proves both that every token mapping in the rewrite is correct AND
        // that the shipped theme file actually contains the values the
        // rewrite depends on.
        let theme = crate::ui_theme::load_last_beacon_ui_theme();

        let cases = [
            ("primary", Interaction::Pressed, Color::srgb(0.854, 0.55, 0.08), Color::srgb(0.854, 0.55, 0.08), Color::srgb(0.008, 0.024, 0.09)),
            ("primary", Interaction::Hovered, Color::srgb(1.0, 0.827, 0.32), Color::srgb(1.0, 0.827, 0.32), Color::srgb(0.008, 0.024, 0.09)),
            ("primary", Interaction::None, Color::srgb(0.984, 0.749, 0.141), Color::srgb(0.984, 0.749, 0.141), Color::srgb(0.008, 0.024, 0.09)),
            ("tertiary", Interaction::Pressed, Color::srgba(0.984, 0.749, 0.141, 0.18), Color::srgb(0.984, 0.749, 0.141), Color::srgb(0.984, 0.749, 0.141)),
            ("tertiary", Interaction::Hovered, Color::srgba(0.984, 0.749, 0.141, 0.1), Color::srgb(0.984, 0.749, 0.141), Color::srgb(1.0, 0.827, 0.32)),
            ("tertiary", Interaction::None, Color::srgba(0.0, 0.0, 0.0, 0.0), Color::srgb(0.278, 0.333, 0.412), Color::srgb(0.58, 0.639, 0.722)),
            ("secondary", Interaction::Pressed, Color::srgb(0.2, 0.255, 0.333), Color::srgb(0.58, 0.639, 0.722), Color::srgb(0.945, 0.961, 0.976)),
            ("secondary", Interaction::Hovered, Color::srgb(0.2, 0.255, 0.333), Color::srgb(0.796, 0.835, 0.882), Color::srgb(0.945, 0.961, 0.976)),
            ("secondary", Interaction::None, Color::srgb(0.118, 0.161, 0.231), Color::srgb(0.278, 0.333, 0.412), Color::srgb(0.945, 0.961, 0.976)),
        ];
        for (variant, interaction, expected_background, expected_border, expected_text) in cases {
            let style = reusable_button_style(&theme, variant, interaction);
            assert_eq!(style.background_color, expected_background, "{variant} {interaction:?} background");
            assert_eq!(style.border_color, expected_border, "{variant} {interaction:?} border");
            assert_eq!(style.text_color, expected_text, "{variant} {interaction:?} text");
        }

        let tab_cases = [
            (true, Interaction::Pressed, Color::srgba(0.984, 0.749, 0.141, 0.22), Color::srgb(0.984, 0.749, 0.141), Color::srgb(0.984, 0.749, 0.141)),
            (true, Interaction::None, Color::srgba(0.984, 0.749, 0.141, 0.12), Color::srgb(0.984, 0.749, 0.141), Color::srgb(0.984, 0.749, 0.141)),
            (false, Interaction::Pressed, Color::srgba(0.984, 0.749, 0.141, 0.16), Color::srgb(0.984, 0.749, 0.141), Color::srgb(0.984, 0.749, 0.141)),
            (false, Interaction::Hovered, Color::srgba(0.984, 0.749, 0.141, 0.08), Color::srgb(0.278, 0.333, 0.412), Color::srgb(0.945, 0.961, 0.976)),
            (false, Interaction::None, Color::srgba(0.0, 0.0, 0.0, 0.0), Color::srgba(0.0, 0.0, 0.0, 0.0), Color::srgb(0.58, 0.639, 0.722)),
        ];
        for (is_selected, interaction, expected_background, expected_border, expected_text) in tab_cases {
            let style = reusable_tab_style(&theme, is_selected, interaction);
            assert_eq!(style.background_color, expected_background, "selected={is_selected} {interaction:?} background");
            assert_eq!(style.border_color, expected_border, "selected={is_selected} {interaction:?} border");
            assert_eq!(style.text_color, expected_text, "selected={is_selected} {interaction:?} text");
        }
    }

    #[test]
    fn button_style_updates_when_interaction_changes() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.init_resource::<LastBeaconUiTabSelections>();
        app.insert_resource(crate::ui_theme::load_last_beacon_ui_theme());
        app.add_systems(Update, enforce_last_beacon_button_styles);

        let button_entity = app
            .world_mut()
            .spawn((
                Button,
                Interaction::None,
                BackgroundColor(Color::NONE),
                BorderColor::all(Color::NONE),
                LastBeaconUiButton {
                    variant: "primary".to_string(),
                },
            ))
            .id();

        app.update();
        let resting_background_color = app.world().get::<BackgroundColor>(button_entity).unwrap().0;

        *app.world_mut()
            .get_mut::<Interaction>(button_entity)
            .unwrap() = Interaction::Hovered;
        app.update();

        let hovered_background_color = app.world().get::<BackgroundColor>(button_entity).unwrap().0;
        assert_ne!(
            hovered_background_color, resting_background_color,
            "the style system must restyle a button on the frame its Interaction changes"
        );
    }

    #[test]
    fn both_tabs_in_a_group_restyle_when_only_the_shared_selection_changes() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.init_resource::<LastBeaconUiTabSelections>();
        app.insert_resource(crate::ui_theme::load_last_beacon_ui_theme());
        app.add_systems(Update, enforce_last_beacon_button_styles);

        let selected_tab_entity = app
            .world_mut()
            .spawn((
                Button,
                Interaction::None,
                BackgroundColor(Color::NONE),
                BorderColor::all(Color::NONE),
                LastBeaconUiTab {
                    group: "group".to_string(),
                    tab: "a".to_string(),
                    selected: true,
                },
            ))
            .id();
        let unselected_tab_entity = app
            .world_mut()
            .spawn((
                Button,
                Interaction::None,
                BackgroundColor(Color::NONE),
                BorderColor::all(Color::NONE),
                LastBeaconUiTab {
                    group: "group".to_string(),
                    tab: "b".to_string(),
                    selected: false,
                },
            ))
            .id();

        app.update();
        let tab_a_selected_background_color = app
            .world()
            .get::<BackgroundColor>(selected_tab_entity)
            .unwrap()
            .0;
        let tab_b_unselected_background_color = app
            .world()
            .get::<BackgroundColor>(unselected_tab_entity)
            .unwrap()
            .0;

        // Select tab B through the shared resource without touching either
        // tab's own `Interaction`, mirroring what a real tab click does to a
        // sibling tab it did not click.
        app.world_mut()
            .resource_mut::<LastBeaconUiTabSelections>()
            .selected_tabs
            .insert("group".to_string(), "b".to_string());
        app.update();

        let tab_a_background_color_after_selection_change = app
            .world()
            .get::<BackgroundColor>(selected_tab_entity)
            .unwrap()
            .0;
        let tab_b_background_color_after_selection_change = app
            .world()
            .get::<BackgroundColor>(unselected_tab_entity)
            .unwrap()
            .0;

        assert_ne!(
            tab_a_background_color_after_selection_change, tab_a_selected_background_color,
            "tab A must restyle to its unselected look once tab B becomes selected"
        );
        assert_ne!(
            tab_b_background_color_after_selection_change, tab_b_unselected_background_color,
            "tab B must restyle to its selected look even though its own Interaction never changed"
        );
    }

    #[test]
    fn scene_root_gains_a_modal_tab_group() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_systems(Update, apply_last_beacon_ui_tab_groups_to_scene_roots);

        let scene_root_entity = app
            .world_mut()
            .spawn(SceneOwner {
                scene_id: SceneId(1),
            })
            .id();

        app.update();

        assert!(
            app.world().get::<TabGroup>(scene_root_entity).is_some(),
            "a top-level scene root must become its own tab group so Tab navigation works inside it"
        );
    }

    #[test]
    fn nested_scene_owned_entity_does_not_get_its_own_tab_group() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_systems(Update, apply_last_beacon_ui_tab_groups_to_scene_roots);

        let scene_root_entity = app.world_mut().spawn_empty().id();
        let nested_scene_owned_entity = app
            .world_mut()
            .spawn((
                SceneOwner {
                    scene_id: SceneId(1),
                },
                ChildOf(scene_root_entity),
            ))
            .id();

        app.update();

        assert!(
            app.world()
                .get::<TabGroup>(nested_scene_owned_entity)
                .is_none(),
            "a nested scene-owned entity is not a scene root and must not get its own tab group"
        );
    }

    #[test]
    fn focusable_widget_markers_gain_tab_index_and_a_hidden_focus_outline() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_systems(Update, apply_last_beacon_ui_focusability);

        let button_entity = app.world_mut().spawn(LastBeaconUiButton::default()).id();
        let tab_entity = app.world_mut().spawn(LastBeaconUiTab::default()).id();
        let value_button_entity = app
            .world_mut()
            .spawn(LastBeaconUiValueButton::default())
            .id();
        let dropdown_toggle_entity = app
            .world_mut()
            .spawn(LastBeaconUiDropdownToggle::default())
            .id();
        let slider_entity = app.world_mut().spawn(LastBeaconUiSlider::default()).id();

        app.update();

        for focusable_entity in [
            button_entity,
            tab_entity,
            value_button_entity,
            dropdown_toggle_entity,
            slider_entity,
        ] {
            assert_eq!(
                app.world().get::<TabIndex>(focusable_entity),
                Some(&TabIndex(0)),
                "every interactive widget marker must become keyboard-focusable"
            );
            assert!(
                app.world()
                    .get::<LastBeaconUiFocusIndicator>(focusable_entity)
                    .is_some(),
                "every interactive widget marker must be able to show a focus outline"
            );
            let outline = app
                .world()
                .get::<Outline>(focusable_entity)
                .expect("focusable widgets must have an Outline component to mutate later");
            assert_eq!(
                outline.color,
                Color::NONE,
                "the outline must start hidden until this widget actually gains focus"
            );
        }
    }

    #[test]
    fn focus_outline_color_follows_input_focus_and_visibility() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.init_resource::<InputFocus>();
        app.init_resource::<InputFocusVisible>();
        app.add_systems(Update, apply_last_beacon_ui_focus_outline);

        let focus_indicator_entity = app
            .world_mut()
            .spawn((
                LastBeaconUiFocusIndicator,
                Outline::new(Val::Px(2.0), Val::Px(2.0), Color::NONE),
            ))
            .id();

        app.update();
        assert_eq!(
            app.world()
                .get::<Outline>(focus_indicator_entity)
                .unwrap()
                .color,
            Color::NONE,
            "the outline must stay hidden while nothing is focused"
        );

        app.world_mut()
            .resource_mut::<InputFocus>()
            .set(focus_indicator_entity, FocusCause::Pressed);
        app.world_mut().resource_mut::<InputFocusVisible>().0 = true;
        app.update();

        assert_eq!(
            app.world()
                .get::<Outline>(focus_indicator_entity)
                .unwrap()
                .color,
            LAST_BEACON_FOCUS_OUTLINE_COLOR,
            "the outline must become visible once this entity is focused and focus is visible"
        );

        app.world_mut().resource_mut::<InputFocus>().clear();
        app.update();

        assert_eq!(
            app.world()
                .get::<Outline>(focus_indicator_entity)
                .unwrap()
                .color,
            Color::NONE,
            "the outline must clear once focus moves away from this entity"
        );
    }

    fn spawn_test_keyboard_activation_message(
        app: &mut App,
        logical_key: Key,
        key_code: bevy::input::keyboard::KeyCode,
    ) {
        app.world_mut().write_message(KeyboardInput {
            key_code,
            logical_key,
            state: bevy::input::ButtonState::Pressed,
            text: None,
            repeat: false,
            window: Entity::PLACEHOLDER,
        });
    }

    #[test]
    fn keyboard_enter_activates_focused_value_button() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_message::<KeyboardInput>();
        app.init_resource::<InputFocus>();
        app.init_resource::<LastBeaconUiInputValues>();
        app.init_resource::<LastBeaconUiDropdownStates>();
        app.init_resource::<LastBeaconUiTabSelections>();
        app.add_systems(
            Update,
            activate_last_beacon_ui_focused_widget_on_keyboard_input,
        );

        let value_button_entity = app
            .world_mut()
            .spawn(LastBeaconUiValueButton {
                target: "brightness".to_string(),
                set_value: String::new(),
                delta: 5.0,
                min: 0.0,
                max: 100.0,
            })
            .id();
        app.world_mut()
            .resource_mut::<InputFocus>()
            .set(value_button_entity, FocusCause::Pressed);

        spawn_test_keyboard_activation_message(
            &mut app,
            Key::Enter,
            bevy::input::keyboard::KeyCode::Enter,
        );
        app.update();

        let stored_brightness_value = app
            .world()
            .resource::<LastBeaconUiInputValues>()
            .values
            .get("brightness")
            .cloned();
        assert_eq!(
            stored_brightness_value,
            Some("5".to_string()),
            "Enter on a focused value button must apply its delta the same way a mouse press does"
        );
    }

    #[test]
    fn keyboard_space_activates_focused_dropdown_toggle() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_message::<KeyboardInput>();
        app.init_resource::<InputFocus>();
        app.init_resource::<LastBeaconUiInputValues>();
        app.init_resource::<LastBeaconUiDropdownStates>();
        app.init_resource::<LastBeaconUiTabSelections>();
        app.add_systems(
            Update,
            activate_last_beacon_ui_focused_widget_on_keyboard_input,
        );

        let dropdown_toggle_entity = app
            .world_mut()
            .spawn(LastBeaconUiDropdownToggle {
                target: "combo-mode".to_string(),
            })
            .id();
        app.world_mut()
            .resource_mut::<InputFocus>()
            .set(dropdown_toggle_entity, FocusCause::Pressed);

        spawn_test_keyboard_activation_message(
            &mut app,
            Key::Space,
            bevy::input::keyboard::KeyCode::Space,
        );
        app.update();

        let dropdown_is_open = app
            .world()
            .resource::<LastBeaconUiDropdownStates>()
            .open_dropdowns
            .get("combo-mode")
            .copied();
        assert_eq!(
            dropdown_is_open,
            Some(true),
            "Space on a focused dropdown toggle must open it the same way a mouse press does"
        );
    }

    #[test]
    fn keyboard_enter_activates_focused_tab() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_message::<KeyboardInput>();
        app.init_resource::<InputFocus>();
        app.init_resource::<LastBeaconUiInputValues>();
        app.init_resource::<LastBeaconUiDropdownStates>();
        app.init_resource::<LastBeaconUiTabSelections>();
        app.add_systems(
            Update,
            activate_last_beacon_ui_focused_widget_on_keyboard_input,
        );

        let tab_entity = app
            .world_mut()
            .spawn(LastBeaconUiTab {
                group: "radio-power".to_string(),
                tab: "standby".to_string(),
                selected: false,
            })
            .id();
        app.world_mut()
            .resource_mut::<InputFocus>()
            .set(tab_entity, FocusCause::Pressed);

        spawn_test_keyboard_activation_message(
            &mut app,
            Key::Enter,
            bevy::input::keyboard::KeyCode::Enter,
        );
        app.update();

        let selected_tab = app
            .world()
            .resource::<LastBeaconUiTabSelections>()
            .selected_tabs
            .get("radio-power")
            .cloned();
        assert_eq!(
            selected_tab,
            Some("standby".to_string()),
            "Enter on a focused tab must select it the same way a mouse press does"
        );
    }

    #[test]
    fn uniform_grid_configures_the_requested_column_count() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_systems(Update, apply_last_beacon_ui_uniform_grid);

        let uniform_grid_entity = app
            .world_mut()
            .spawn((Node::default(), LastBeaconUiUniformGrid { column_count: 3 }))
            .id();

        app.update();

        let uniform_grid_node = app.world().get::<Node>(uniform_grid_entity).unwrap();
        assert_eq!(uniform_grid_node.display, Display::Grid);
        assert_eq!(
            uniform_grid_node.grid_template_columns,
            RepeatedGridTrack::flex::<Vec<RepeatedGridTrack>>(3, 1.0)
        );
    }

    #[test]
    fn uniform_grid_column_count_of_zero_still_produces_a_single_column() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_systems(Update, apply_last_beacon_ui_uniform_grid);

        let uniform_grid_entity = app
            .world_mut()
            .spawn((Node::default(), LastBeaconUiUniformGrid { column_count: 0 }))
            .id();

        app.update();

        let uniform_grid_node = app.world().get::<Node>(uniform_grid_entity).unwrap();
        assert_eq!(
            uniform_grid_node.grid_template_columns,
            RepeatedGridTrack::flex::<Vec<RepeatedGridTrack>>(1, 1.0),
            "an authored column_count of 0 must still produce a usable single-column grid"
        );
    }

    #[test]
    fn grid_item_span_translates_into_native_grid_placement() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_systems(Update, apply_last_beacon_ui_grid_item_span);

        let grid_item_entity = app
            .world_mut()
            .spawn((
                Node::default(),
                LastBeaconUiGridItem {
                    column_span: 2,
                    row_span: 3,
                },
            ))
            .id();

        app.update();

        let grid_item_node = app.world().get::<Node>(grid_item_entity).unwrap();
        assert_eq!(grid_item_node.grid_column, GridPlacement::span(2));
        assert_eq!(grid_item_node.grid_row, GridPlacement::span(3));
    }

    #[test]
    fn grid_item_span_of_zero_does_not_panic_and_yields_span_one() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_systems(Update, apply_last_beacon_ui_grid_item_span);

        let grid_item_entity = app
            .world_mut()
            .spawn((
                Node::default(),
                LastBeaconUiGridItem {
                    column_span: 0,
                    row_span: 0,
                },
            ))
            .id();

        app.update();

        let grid_item_node = app.world().get::<Node>(grid_item_entity).unwrap();
        assert_eq!(grid_item_node.grid_column, GridPlacement::span(1));
        assert_eq!(grid_item_node.grid_row, GridPlacement::span(1));
    }

    fn spawn_aspect_ratio_test_widget(
        app: &mut App,
        parent_size: Vec2,
        aspect_ratio_bounds: LastBeaconUiAspectRatioBounds,
    ) -> Entity {
        let parent_entity = app
            .world_mut()
            .spawn(ComputedNode {
                size: parent_size,
                ..default()
            })
            .id();
        app.world_mut()
            .spawn((Node::default(), ChildOf(parent_entity), aspect_ratio_bounds))
            .id()
    }

    #[test]
    fn aspect_ratio_bounds_forces_a_fixed_square_ratio() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_systems(Update, apply_last_beacon_ui_aspect_ratio_bounds);

        let widget_entity = spawn_aspect_ratio_test_widget(
            &mut app,
            Vec2::new(1600.0, 900.0),
            LastBeaconUiAspectRatioBounds {
                min_aspect_ratio: 1.0,
                max_aspect_ratio: 1.0,
            },
        );

        app.update();

        let widget_node = app.world().get::<Node>(widget_entity).unwrap();
        assert_eq!(widget_node.width, Val::Px(900.0));
        assert_eq!(widget_node.height, Val::Px(900.0));
    }

    #[test]
    fn aspect_ratio_bounds_passes_through_a_ratio_already_inside_the_band() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_systems(Update, apply_last_beacon_ui_aspect_ratio_bounds);

        let widget_entity = spawn_aspect_ratio_test_widget(
            &mut app,
            Vec2::new(1600.0, 900.0),
            LastBeaconUiAspectRatioBounds {
                min_aspect_ratio: 0.5,
                max_aspect_ratio: 5.0,
            },
        );

        app.update();

        let widget_node = app.world().get::<Node>(widget_entity).unwrap();
        assert_eq!(widget_node.width, Val::Px(1600.0));
        assert_eq!(widget_node.height, Val::Px(900.0));
    }

    #[test]
    fn aspect_ratio_bounds_clamps_a_narrow_parent_up_to_the_minimum() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_systems(Update, apply_last_beacon_ui_aspect_ratio_bounds);

        // A 400x900 parent has a ratio of ~0.44, below the configured minimum.
        let widget_entity = spawn_aspect_ratio_test_widget(
            &mut app,
            Vec2::new(400.0, 900.0),
            LastBeaconUiAspectRatioBounds {
                min_aspect_ratio: 1.0,
                max_aspect_ratio: 2.0,
            },
        );

        app.update();

        let widget_node = app.world().get::<Node>(widget_entity).unwrap();
        assert_eq!(widget_node.width, Val::Px(400.0));
        assert_eq!(widget_node.height, Val::Px(400.0));
    }

    #[test]
    fn aspect_ratio_bounds_swaps_a_backwards_min_and_max() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_systems(Update, apply_last_beacon_ui_aspect_ratio_bounds);

        // A 1600x900 parent has a ratio of ~1.778, which falls inside the
        // *correctly-ordered* [1.0, 2.0] band. Authoring the fields backwards
        // (`min: 2.0, max: 1.0`) must still swap to that same band internally
        // -- `f32::clamp` always panics if `min > max` is passed
        // through unswapped, so this also guards against that panic.
        let widget_entity = spawn_aspect_ratio_test_widget(
            &mut app,
            Vec2::new(1600.0, 900.0),
            LastBeaconUiAspectRatioBounds {
                min_aspect_ratio: 2.0,
                max_aspect_ratio: 1.0,
            },
        );

        app.update();

        let widget_node = app.world().get::<Node>(widget_entity).unwrap();
        assert_eq!(
            widget_node.width,
            Val::Px(1600.0),
            "the parent's ratio is already inside the (correctly-ordered) band, so it must pass through unchanged"
        );
        assert_eq!(widget_node.height, Val::Px(900.0));
    }

    #[test]
    fn aspect_ratio_bounds_default_is_inert() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_systems(Update, apply_last_beacon_ui_aspect_ratio_bounds);

        let widget_entity = spawn_aspect_ratio_test_widget(
            &mut app,
            Vec2::new(1600.0, 900.0),
            LastBeaconUiAspectRatioBounds::default(),
        );

        app.update();

        let widget_node = app.world().get::<Node>(widget_entity).unwrap();
        assert_eq!(
            widget_node.width,
            Val::Px(1600.0),
            "an unconfigured (Default) aspect-ratio widget must not constrain anything"
        );
        assert_eq!(widget_node.height, Val::Px(900.0));
    }

    #[test]
    fn value_text_spawned_after_the_stored_value_already_exists_still_shows_it() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.init_resource::<LastBeaconUiInputValues>();
        app.init_resource::<FontCx>();
        app.init_resource::<LayoutCx>();
        app.add_systems(Update, refresh_last_beacon_ui_value_text);

        app.world_mut()
            .resource_mut::<LastBeaconUiInputValues>()
            .values
            .insert("number-field".to_string(), "99".to_string());
        // Let the resource's change tick age out with an update that has
        // nothing new to sync, exactly like a widget asset that finishes
        // streaming in on a later frame after the value was already set
        // elsewhere in the session (e.g. a scene reopened, or a nested BSN
        // widget that loads asynchronously).
        app.update();

        let value_text_entity = app
            .world_mut()
            .spawn((
                LastBeaconUiValueText {
                    target: "number-field".to_string(),
                    prefix: String::new(),
                    suffix: String::new(),
                },
                Text::new("42"),
            ))
            .id();

        app.update();

        assert_eq!(
            app.world().get::<Text>(value_text_entity).unwrap().0,
            "99",
            "a value text spawned after its stored value already exists must show that value immediately, not its authored placeholder"
        );
    }

    #[test]
    fn slider_fill_spawned_after_the_stored_value_already_exists_still_shows_it() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.init_resource::<LastBeaconUiInputValues>();
        app.add_systems(Update, refresh_last_beacon_ui_slider_fills);

        app.world_mut()
            .resource_mut::<LastBeaconUiInputValues>()
            .values
            .insert("slider-volume".to_string(), "75".to_string());
        app.update();

        let slider_fill_entity = app
            .world_mut()
            .spawn((
                LastBeaconUiSliderFill {
                    target: "slider-volume".to_string(),
                    min: 0.0,
                    max: 100.0,
                },
                Node::default(),
            ))
            .id();

        app.update();

        assert_eq!(
            app.world().get::<Node>(slider_fill_entity).unwrap().width,
            Val::Percent(75.0),
            "a slider fill spawned after its stored value already exists must show that value immediately, not its authored default width"
        );
    }

    /// Builds a headless app with the real bevy_text/bevy_ui plugins wired
    /// up (not mocked), spawns a Number Field's exact entity shape, lets the
    /// container's `Added<LastBeaconUiTextInput>` go unnoticed for
    /// `frames_before_text_input_is_added` frames (mirroring the delay
    /// between a `.bsn` scene's initial spawn and
    /// `initialize_last_beacon_ui_text_inputs` actually observing it, which
    /// in the real game depends on scene-loading/command-flush timing), then
    /// returns the glyph count read back from `TextLayoutInfo` after each of
    /// the next 60 frames.
    fn number_field_glyph_counts_after_text_input_delay(
        frames_before_text_input_is_added: usize,
    ) -> Vec<usize> {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(bevy::asset::AssetPlugin {
            file_path: ".".to_string(),
            ..default()
        });
        app.add_plugins(bevy::text::TextPlugin);
        app.add_plugins(bevy::input::InputPlugin::default());
        app.add_plugins(bevy::a11y::AccessibilityPlugin);
        app.add_plugins(bevy::window::WindowPlugin::default());
        app.add_plugins(bevy::image::ImagePlugin::default());
        app.add_plugins(bevy::picking::DefaultPickingPlugins);
        app.init_asset::<bevy::image::TextureAtlasLayout>();
        app.add_plugins(bevy::ui::UiPlugin::default());
        app.add_systems(Update, initialize_last_beacon_ui_text_inputs);

        // Mirror the Number Field's authored structure: a container that
        // will carry `LastBeaconUiTextInput`, with a child `Text` entity
        // carrying `LastBeaconUiNumberInput` -- the child starts out as
        // plain `Text` (matching the `.bsn`-authored "42") and only gains
        // `EditableText` once `initialize_last_beacon_ui_text_inputs`
        // observes the container's `LastBeaconUiTextInput`.
        let text_entity = app
            .world_mut()
            .spawn((
                LastBeaconUiNumberInput {
                    target: "number-field".to_string(),
                    min: 0.0,
                    max: 250.0,
                },
                LastBeaconUiValueText {
                    target: "number-field".to_string(),
                    prefix: String::new(),
                    suffix: String::new(),
                },
                Text::new("42"),
                TextFont::default(),
                Node::default(),
            ))
            .id();
        let container_entity = app.world_mut().spawn(Node::default()).add_child(text_entity).id();

        for _ in 0..frames_before_text_input_is_added {
            app.update();
        }

        app.world_mut()
            .entity_mut(container_entity)
            .insert(LastBeaconUiTextInput {
                value: "42".to_string(),
                multiline: false,
            });

        (0..60)
            .map(|_| {
                app.update();
                app.world()
                    .get::<TextLayoutInfo>(text_entity)
                    .map(|info| info.glyphs.len())
                    .unwrap_or(0)
            })
            .collect()
    }

    #[test]
    fn number_field_editable_text_glyphs_populate_immediately_after_first_layout() {
        // The real game's scene-loading/command-flush timing determines how
        // many frames elapse between the Number Field's container spawning
        // and `initialize_last_beacon_ui_text_inputs` actually observing its
        // `LastBeaconUiTextInput`. Sweep a range of delays to find whether
        // any specific timing leaves `TextLayoutInfo` stuck with zero
        // glyphs -- the reported "Number Field value disappears" bug, seen
        // both on some scene loads and after some +/- button clicks.
        for delay in 0..12 {
            let glyphs_by_frame = number_field_glyph_counts_after_text_input_delay(delay);
            assert!(
                glyphs_by_frame.iter().skip(2).all(|&count| count > 0),
                "with {delay} frame(s) between spawn and LastBeaconUiTextInput being added, \
                 TextLayoutInfo glyphs must not go to (and stay at) zero once EditableText holds \"42\"; \
                 per-frame glyph counts were {glyphs_by_frame:?}"
            );
        }
    }

    /// Finds whichever entity currently carries `LastBeaconUiValueText` for
    /// `target`. `heal_last_beacon_ui_value_text_stuck_glyphs` despawns and
    /// respawns the stuck entity under a brand new ID, so tests can't hold
    /// on to the original `Entity` across a heal and must look it up fresh
    /// each time instead.
    fn find_value_text_entity(app: &mut App, target: &str) -> Entity {
        let mut query = app.world_mut().query::<(Entity, &LastBeaconUiValueText)>();
        query
            .iter(app.world())
            .find(|(_, value_text)| value_text.target == target)
            .map(|(entity, _)| entity)
            .unwrap_or_else(|| panic!("no LastBeaconUiValueText entity found for target {target:?}"))
    }

    #[test]
    fn stuck_zero_glyph_value_text_self_heals_within_a_few_frames() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(bevy::asset::AssetPlugin {
            file_path: ".".to_string(),
            ..default()
        });
        app.add_plugins(bevy::text::TextPlugin);
        app.add_plugins(bevy::input::InputPlugin::default());
        app.add_plugins(bevy::a11y::AccessibilityPlugin);
        app.add_plugins(bevy::window::WindowPlugin::default());
        app.add_plugins(bevy::image::ImagePlugin::default());
        app.add_plugins(bevy::picking::DefaultPickingPlugins);
        app.init_asset::<bevy::image::TextureAtlasLayout>();
        app.add_plugins(bevy::ui::UiPlugin::default());
        app.add_systems(
            Update,
            (
                initialize_last_beacon_ui_text_inputs,
                heal_last_beacon_ui_value_text_stuck_glyphs,
            ),
        );

        let text_entity = app
            .world_mut()
            .spawn((
                LastBeaconUiNumberInput {
                    target: "number-field".to_string(),
                    min: 0.0,
                    max: 250.0,
                },
                LastBeaconUiValueText {
                    target: "number-field".to_string(),
                    prefix: String::new(),
                    suffix: String::new(),
                },
                Text::new("42"),
                TextFont::default(),
                Node::default(),
            ))
            .id();
        app.world_mut()
            .spawn((
                LastBeaconUiTextInput {
                    value: "42".to_string(),
                    multiline: false,
                },
                Node::default(),
            ))
            .add_child(text_entity);

        for _ in 0..10 {
            app.update();
        }
        let glyphs_before_corruption = app
            .world()
            .get::<TextLayoutInfo>(text_entity)
            .unwrap()
            .glyphs
            .len();
        assert!(
            glyphs_before_corruption > 0,
            "test setup: expected a healthy, non-empty layout before simulating the stuck-glyph defect"
        );

        // Simulate the exact defect observed in the real game: `TextLayoutInfo`
        // stuck at zero glyphs despite `EditableText` still holding "42",
        // with no further edit queued to trigger a relayout.
        {
            let mut layout_info = app
                .world_mut()
                .get_mut::<TextLayoutInfo>(text_entity)
                .unwrap();
            layout_info.glyphs.clear();
            layout_info.size = Vec2::ZERO;
        }

        for _ in 0..3 {
            app.update();
        }

        // The healer despawns and respawns the stuck entity, so look up
        // whichever entity now holds the "number-field" value text.
        let healed_entity = find_value_text_entity(&mut app, "number-field");
        let glyphs_after_healing = app
            .world()
            .get::<TextLayoutInfo>(healed_entity)
            .unwrap()
            .glyphs
            .len();
        assert!(
            glyphs_after_healing > 0,
            "heal_last_beacon_ui_value_text_stuck_glyphs must recover a stuck zero-glyph EditableText within a few frames"
        );
        assert_eq!(
            app.world()
                .get::<EditableText>(healed_entity)
                .unwrap()
                .value()
                .to_string(),
            "42",
            "healing must restore exactly the pre-corruption value, not duplicate or corrupt it"
        );
    }

    #[test]
    fn healing_repeatedly_never_duplicates_the_value() {
        // Earlier versions of `heal_last_beacon_ui_value_text_stuck_glyphs`
        // edited the existing `EditableText` in place. That looked fine in
        // isolated testing, but in the real game it turned a rare, cosmetic
        // "value goes blank" bug into a much worse one: repeatedly
        // re-triggering every frame the underlying layout stayed broken,
        // in one version *prepending* "42" to itself ("4242", "424242", ...)
        // because `TextEdit::TextEnd(true)`'s selection math silently
        // no-ops against a zero-line cached layout instead of selecting
        // anything. This drives the real system, called many times in a row
        // against a `TextLayoutInfo` that (unlike the real bug, but exactly
        // like the real bug's *worst case*) never stops looking "stuck", to
        // prove the current despawn-and-respawn healer cannot duplicate or
        // grow the value no matter how many times it fires.
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_systems(Update, heal_last_beacon_ui_value_text_stuck_glyphs);

        let text_entity = app
            .world_mut()
            .spawn((
                LastBeaconUiNumberInput {
                    target: "number-field".to_string(),
                    min: 0.0,
                    max: 250.0,
                },
                LastBeaconUiValueText {
                    target: "number-field".to_string(),
                    prefix: String::new(),
                    suffix: String::new(),
                },
                EditableText::new("42"),
                TextLayoutInfo::default(),
                TextFont::default(),
                TextColor::default(),
            ))
            .id();
        app.world_mut()
            .spawn(Node::default())
            .add_child(text_entity);

        for _ in 0..20 {
            // `TextLayoutInfo::default()` always has zero glyphs, so a
            // freshly-respawned replacement entity looks exactly as "stuck"
            // as the one it replaced -- the worst case for a healer that
            // might otherwise compound edits over time. Every iteration
            // therefore triggers another despawn-and-respawn cycle.
            app.update();

            let current_entity = find_value_text_entity(&mut app, "number-field");
            assert_eq!(
                app.world()
                    .get::<EditableText>(current_entity)
                    .unwrap()
                    .value()
                    .to_string(),
                "42",
                "repeated healing must never change, duplicate, or grow the value"
            );
        }
    }

    /// `reveal_last_beacon_text_once_fonts_load` must mark `TextFont` as
    /// changed for every entity it un-marks `SceneContentLoading` on, once
    /// the shared fonts finish loading.
    ///
    /// This is the fix for the "Text Field / Text Box goes permanently
    /// blank on first click/edit" bug: `apply_last_beacon_ui_font` assigns
    /// the real font handle to `TextFont` exactly once, on the frame it
    /// first sees an entity. Real font assets load asynchronously from
    /// disk; if that load hasn't finished yet at that exact moment,
    /// `bevy_ui`'s `update_editable_text_styles` (gated on
    /// `Changed<TextFont>`) fails to resolve the font and silently skips
    /// applying `FontFamily` to that entity's `PlainEditor` styles --
    /// forever, since nothing else ever touched `TextFont` again before
    /// this fix. Without a font family, `PlainEditor` permanently builds
    /// structurally valid but zero-size, zero-glyph layouts, which stay
    /// hidden behind the entity's last-good `TextLayoutInfo` until the
    /// first edit forces a real recompute and it collapses to a
    /// permanently blank, cursor-less input.
    ///
    /// The real async font-loading race that triggers this is not
    /// hermetically reproducible here: Last Beacon's actual font files are
    /// small enough that a real `AssetServer` load completes within a
    /// single `app.update()` in this test environment, so it can never be
    /// observed still `Loading` on the one frame `apply_last_beacon_ui_font`
    /// runs (confirmed by probing `AssetServer::get_load_state` immediately
    /// after that frame during development of this test). Instead, this
    /// test verifies the actual fix mechanism directly and deterministically:
    /// once the shared fonts are loaded, does `reveal_last_beacon_text_once_fonts_load`
    /// give every previously-`SceneContentLoading` entity's `TextFont` a
    /// fresh `Changed` tick? Manual, live QA confirmed the fix resolves the
    /// real bug in the running game.
    #[test]
    fn reveal_text_once_fonts_load_marks_text_font_changed() {
        // Deliberately does NOT register `reveal_last_beacon_text_once_fonts_load`
        // (or anything else) in `Update`: `bevy_text::TextPlugin` alone was
        // found, during development of this test, to touch `TextFont`'s
        // change tick on its own for unrelated reasons (confirmed by
        // observing `Changed<TextFont>` fire even with no Last Beacon
        // systems registered at all). Watching for `Changed<TextFont>` via a
        // query is therefore too noisy a signal here; instead, the system
        // under test is invoked directly with `run_system_once`, and its
        // effect is checked precisely via the component's raw change tick.
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(bevy::asset::AssetPlugin {
            file_path: crate::asset_root().to_string_lossy().to_string(),
            ..default()
        });
        app.add_plugins(bevy::text::TextPlugin);
        app.init_resource::<LastBeaconUiFontHandles>();

        let text_entity = app
            .world_mut()
            .spawn((TextFont::default(), SceneContentLoading))
            .id();

        // Real font files load asynchronously from disk; give it many
        // frames to actually finish loading (matches the established
        // pattern in `text_finishing_its_font_load_clears_the_loading_marker`).
        // Both shared fonts must finish, not just the one this entity uses --
        // `reveal_last_beacon_text_once_fonts_load` gates on both.
        for _ in 0..600 {
            let asset_server = app.world().resource::<AssetServer>();
            let font_handles = app.world().resource::<LastBeaconUiFontHandles>();
            let both_loaded = matches!(
                asset_server.get_load_state(font_handles.ui_font.id()),
                Some(bevy::asset::LoadState::Loaded)
            ) && matches!(
                asset_server.get_load_state(font_handles.symbol_font.id()),
                Some(bevy::asset::LoadState::Loaded)
            );
            if both_loaded {
                break;
            }
            app.update();
        }
        assert!(
            matches!(
                app.world()
                    .resource::<AssetServer>()
                    .get_load_state(app.world().resource::<LastBeaconUiFontHandles>().ui_font.id()),
                Some(bevy::asset::LoadState::Loaded)
            ),
            "test setup: the shared UI font never finished loading"
        );

        let tick_before = app
            .world()
            .entity(text_entity)
            .get_change_ticks::<TextFont>()
            .unwrap()
            .changed;

        use bevy::ecs::system::RunSystemOnce as _;
        app.world_mut()
            .run_system_once(reveal_last_beacon_text_once_fonts_load)
            .unwrap();
        app.world_mut().flush();

        let tick_after = app
            .world()
            .entity(text_entity)
            .get_change_ticks::<TextFont>()
            .unwrap()
            .changed;

        assert_ne!(
            tick_before, tick_after,
            "reveal_last_beacon_text_once_fonts_load must mark TextFont changed so \
             Changed<TextFont>-gated font-resolution systems (like bevy_ui's \
             update_editable_text_styles) get a genuine retry once the font is loaded"
        );
        assert!(
            app.world().get::<SceneContentLoading>(text_entity).is_none(),
            "the loading marker must still clear once the shared fonts finish loading"
        );
    }
}
