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
    input_focus::{FocusCause, InputFocus},
    prelude::*,
    scene::{ResolvedSceneRoot, ScenePatch},
    text::{
        EditableText, EditableTextFilter, FontSource, LineBreak, LineHeight, TextCursorStyle,
        TextEdit, TextLayout, TextLayoutInfo,
    },
    ui::{
        widget::TextScroll, ComputedUiRenderTargetInfo, RelativeCursorPosition, UiGlobalTransform,
    },
    window::PrimaryWindow,
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
    mut text_fonts: Query<(&mut TextFont, Option<&LastBeaconUiSymbolIcon>), Added<TextFont>>,
) {
    let ui_font = asset_server.load("fonts/NotoSans-Regular.ttf");
    let symbol_font = asset_server.load("fonts/NotoSansSymbols2-Regular.ttf");
    for (mut text_font, symbol_icon) in &mut text_fonts {
        let font_handle = if symbol_icon.is_some() {
            symbol_font.clone()
        } else {
            ui_font.clone()
        };
        text_font.font = FontSource::Handle(font_handle);
    }
}

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

        if let Ok(mut node) = node_query.get_mut(text_entity) {
            if text_input.multiline {
                node.width = Val::Percent(100.0);
                node.height = Val::Percent(100.0);
            }
            if number_input_query.contains(text_entity) {
                node.align_items = AlignItems::Center;
                node.justify_content = JustifyContent::Center;
            }
        } else if text_input.multiline {
            commands.entity(text_entity).insert(Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
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
    number_inputs: Query<(&LastBeaconUiNumberInput, &EditableText), Changed<EditableText>>,
) {
    for (number_input, editable_text) in &number_inputs {
        if number_input.target.is_empty() {
            continue;
        }
        let Ok(value) = editable_text.value().to_string().trim().parse::<f32>() else {
            continue;
        };
        let clamped_value = value.clamp(number_input.min, number_input.max);
        insert_input_value_if_changed(
            &mut input_values,
            &number_input.target,
            format_value(clamped_value),
        );
    }
}

/// Applies simple value changes for authored reusable input examples.
pub fn update_last_beacon_ui_value_buttons(
    mut input_values: ResMut<LastBeaconUiInputValues>,
    mut dropdown_states: ResMut<LastBeaconUiDropdownStates>,
    buttons: LastBeaconUiValueButtonInteractionQuery,
) {
    for (button, interaction) in &buttons {
        if *interaction != Interaction::Pressed || button.target.is_empty() {
            continue;
        }

        if !button.set_value.is_empty() {
            input_values
                .values
                .insert(button.target.clone(), button.set_value.clone());
            dropdown_states
                .open_dropdowns
                .insert(button.target.clone(), false);
            continue;
        }

        let current_value = input_values
            .values
            .get(&button.target)
            .and_then(|value| value.parse::<f32>().ok())
            .unwrap_or(0.0);
        let next_value = (current_value + button.delta).clamp(button.min, button.max);
        insert_input_value_if_changed(&mut input_values, &button.target, format_value(next_value));
    }
}

/// Opens or closes authored reusable dropdown panels.
pub fn toggle_last_beacon_ui_dropdowns(
    mut dropdown_states: ResMut<LastBeaconUiDropdownStates>,
    toggles: LastBeaconUiDropdownToggleQuery,
) {
    for (toggle, interaction) in &toggles {
        if *interaction != Interaction::Pressed || toggle.target.is_empty() {
            continue;
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
}

/// Mirrors reusable radio selection into symbol icon text.
pub fn refresh_last_beacon_ui_radio_icons(
    tab_selections: Res<LastBeaconUiTabSelections>,
    mut radio_icons: Query<(&LastBeaconUiRadioIcon, &mut Text)>,
) {
    if !tab_selections.is_changed() {
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
) {
    if !dropdown_states.is_changed() {
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
) {
    if !dropdown_states.is_changed() {
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
) {
    if !input_values.is_changed() {
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
) {
    if !input_values.is_changed() {
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
                editable_text.editor_mut().set_text(&rendered_value);
                editable_text.queue_edit(TextEdit::TextEnd(false));
            }
        }
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

/// Shows only the reusable tab panel that matches the current tab selection.
pub fn refresh_last_beacon_ui_tab_panels(
    tab_selections: Res<LastBeaconUiTabSelections>,
    mut tab_panels: LastBeaconUiTabPanelQuery,
) {
    if !tab_selections.is_changed() {
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
            background_color: Color::srgb(0.854, 0.55, 0.08),
            border_color: Color::srgb(0.854, 0.55, 0.08),
            text_color: Color::srgb(0.008, 0.024, 0.09),
        },
        ("primary", Interaction::Hovered) => LastBeaconWidgetStyle {
            background_color: Color::srgb(1.0, 0.827, 0.32),
            border_color: Color::srgb(1.0, 0.827, 0.32),
            text_color: Color::srgb(0.008, 0.024, 0.09),
        },
        ("primary", _) => LastBeaconWidgetStyle {
            background_color: Color::srgb(0.984, 0.749, 0.141),
            border_color: Color::srgb(0.984, 0.749, 0.141),
            text_color: Color::srgb(0.008, 0.024, 0.09),
        },
        ("tertiary", Interaction::Pressed) => LastBeaconWidgetStyle {
            background_color: Color::srgba(0.984, 0.749, 0.141, 0.18),
            border_color: Color::srgb(0.984, 0.749, 0.141),
            text_color: Color::srgb(0.984, 0.749, 0.141),
        },
        ("tertiary", Interaction::Hovered) => LastBeaconWidgetStyle {
            background_color: Color::srgba(0.984, 0.749, 0.141, 0.1),
            border_color: Color::srgb(0.984, 0.749, 0.141),
            text_color: Color::srgb(1.0, 0.827, 0.32),
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
            background_color: Color::srgba(0.984, 0.749, 0.141, 0.22),
            border_color: Color::srgb(0.984, 0.749, 0.141),
            text_color: Color::srgb(0.984, 0.749, 0.141),
        },
        (true, _) => LastBeaconWidgetStyle {
            background_color: Color::srgba(0.984, 0.749, 0.141, 0.12),
            border_color: Color::srgb(0.984, 0.749, 0.141),
            text_color: Color::srgb(0.984, 0.749, 0.141),
        },
        (false, Interaction::Pressed) => LastBeaconWidgetStyle {
            background_color: Color::srgba(0.984, 0.749, 0.141, 0.16),
            border_color: Color::srgb(0.984, 0.749, 0.141),
            text_color: Color::srgb(0.984, 0.749, 0.141),
        },
        (false, Interaction::Hovered) => LastBeaconWidgetStyle {
            background_color: Color::srgba(0.984, 0.749, 0.141, 0.08),
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
