//! Last Beacon UI widget composition support.
//!
//! Last Beacon's authored UI scenes can place reusable `.bsn` widget assets under
//! lightweight widget slots. This keeps scene files focused on layout while common
//! visual pieces live under `assets/ui/widgets/`.

use std::{collections::HashMap, sync::Arc};

use bevy::{
    input::{
        keyboard::{Key, KeyboardInput},
        mouse::{MouseScrollUnit, MouseWheel},
        ButtonState,
    },
    input_focus::{FocusCause, InputFocus},
    prelude::*,
    scene::{ResolvedSceneRoot, ScenePatch},
    text::{EditableText, FontSource, TextCursorStyle, TextEdit, TextLayout},
    ui::{RelativeCursorPosition, ScrollPosition},
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

/// Stores custom text and wheel offsets for multiline text inputs.
#[derive(Clone, Debug, Default, Resource)]
pub struct LastBeaconUiTextBoxStates {
    boxes: HashMap<Entity, LastBeaconUiTextBoxState>,
}

/// Stores the currently dragged multiline text box scrollbar.
#[derive(Clone, Copy, Debug, Default, Resource)]
pub struct LastBeaconUiTextBoxScrollDrag {
    active_text_box: Option<Entity>,
}

/// Tracks the blink phase for custom multiline text-box carets.
#[derive(Clone, Copy, Debug, Default, Resource)]
pub struct LastBeaconUiTextBoxCaretBlink {
    caret_is_visible: bool,
}

#[derive(Clone, Debug, Default)]
struct LastBeaconUiTextBoxState {
    value: String,
    first_visible_line: usize,
}

const TEXT_BOX_VISIBLE_LINES: usize = 4;

type LastBeaconUiTextInputFocusQuery<'world, 'state> = Query<
    'world,
    'state,
    (
        Entity,
        &'static LastBeaconUiTextInput,
        &'static Interaction,
        Option<&'static Children>,
    ),
    (Changed<Interaction>, With<LastBeaconUiTextInput>),
>;

type LastBeaconUiTextInputScrollQuery<'world, 'state> = Query<
    'world,
    'state,
    (
        Entity,
        &'static LastBeaconUiTextInput,
        Option<&'static Children>,
        Option<&'static Interaction>,
        Option<&'static RelativeCursorPosition>,
        &'static mut ScrollPosition,
    ),
>;

type LastBeaconUiValueButtonInteractionQuery<'world, 'state> = Query<
    'world,
    'state,
    (&'static LastBeaconUiValueButton, &'static Interaction),
    (Changed<Interaction>, With<Button>),
>;

type LastBeaconUiDropdownToggleQuery<'world, 'state> = Query<
    'world,
    'state,
    (&'static LastBeaconUiDropdownToggle, &'static Interaction),
    (Changed<Interaction>, With<Button>),
>;

type LastBeaconUiSliderInteractionQuery<'world, 'state> = Query<
    'world,
    'state,
    (
        Entity,
        &'static LastBeaconUiSlider,
        &'static Interaction,
        &'static RelativeCursorPosition,
    ),
    With<Button>,
>;

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
    mut text_box_states: ResMut<LastBeaconUiTextBoxStates>,
    text_inputs: Query<
        (Entity, &LastBeaconUiTextInput, Option<&Children>),
        Added<LastBeaconUiTextInput>,
    >,
    children_query: Query<&Children>,
    text_query: Query<(), With<Text>>,
    mut text_values: Query<&mut Text>,
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

        if text_input.multiline {
            text_box_states.boxes.insert(
                input_entity,
                LastBeaconUiTextBoxState {
                    value: text_input.value.clone(),
                    first_visible_line: 0,
                },
            );
            if let Ok(mut text) = text_values.get_mut(text_entity) {
                text.0 = visible_text_box_lines(&text_input.value, 0, false);
            }
            commands
                .entity(input_entity)
                .insert((ScrollPosition::default(), RelativeCursorPosition::default()));
            continue;
        }

        let mut editable_text = EditableText::new(&text_input.value);
        editable_text.allow_newlines = false;
        editable_text.visible_width = Some(24.0);
        editable_text.visible_lines = Some(1.0);

        commands.entity(text_entity).insert((
            Node::default(),
            editable_text,
            TextCursorStyle::default(),
            TextLayout::default(),
        ));
    }
}

/// Focuses editable text when its authored input container is clicked.
pub fn focus_last_beacon_ui_text_inputs(
    mut input_focus: ResMut<InputFocus>,
    text_inputs: LastBeaconUiTextInputFocusQuery,
    children_query: Query<&Children>,
    editable_text_query: Query<(), With<EditableText>>,
) {
    for (input_entity, text_input, interaction, input_children) in &text_inputs {
        if *interaction != Interaction::Pressed {
            continue;
        }
        if text_input.multiline {
            input_focus.set(input_entity, FocusCause::Pressed);
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
        input_values
            .values
            .insert(number_input.target.clone(), format_value(clamped_value));
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
        input_values
            .values
            .insert(button.target.clone(), format_value(next_value));
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
    scroll_tracks: Query<Entity, Added<LastBeaconUiTextScrollTrack>>,
) {
    for scroll_track_entity in &scroll_tracks {
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

/// Refreshes the custom multiline caret blink state for focused text boxes.
pub fn refresh_last_beacon_ui_text_box_cursors(
    time: Res<Time>,
    input_focus: Res<InputFocus>,
    mut caret_blink: ResMut<LastBeaconUiTextBoxCaretBlink>,
    text_box_states: Res<LastBeaconUiTextBoxStates>,
    text_inputs: Query<(&LastBeaconUiTextInput, Option<&Children>)>,
    children_query: Query<&Children>,
    mut text_query: Query<&mut Text>,
) {
    let next_caret_is_visible = (time.elapsed_secs() * 2.0).floor() as i32 % 2 == 0;
    if caret_blink.caret_is_visible == next_caret_is_visible && !input_focus.is_changed() {
        return;
    }
    caret_blink.caret_is_visible = next_caret_is_visible;

    for (input_entity, text_box_state) in &text_box_states.boxes {
        let Ok((text_input, input_children)) = text_inputs.get(*input_entity) else {
            continue;
        };
        if !text_input.multiline {
            continue;
        }
        let Some(text_entity) =
            first_descendant_with_rendered_text(input_children, &children_query, &text_query)
        else {
            continue;
        };
        if let Ok(mut text) = text_query.get_mut(text_entity) {
            let text_box_has_focus = input_focus.get() == Some(*input_entity);
            let should_show_caret = text_box_has_focus && caret_blink.caret_is_visible;
            text.0 = visible_text_box_lines(
                &text_box_state.value,
                text_box_state.first_visible_line,
                should_show_caret,
            );
        }
    }
}

/// Applies keyboard input to focused multiline text boxes without using Bevy's internal text scroller.
#[allow(clippy::too_many_arguments)]
pub fn type_into_last_beacon_ui_text_boxes(
    input_focus: Res<InputFocus>,
    mut keyboard_inputs: MessageReader<KeyboardInput>,
    mut text_box_states: ResMut<LastBeaconUiTextBoxStates>,
    text_inputs: Query<(&LastBeaconUiTextInput, Option<&Children>)>,
    children_query: Query<&Children>,
    scroll_thumb_query: Query<(), With<LastBeaconUiTextScrollThumb>>,
    mut text_query: Query<&mut Text>,
    mut node_query: Query<&mut Node>,
) {
    let Some(focused_entity) = input_focus.get() else {
        keyboard_inputs.clear();
        return;
    };
    let Ok((text_input, input_children)) = text_inputs.get(focused_entity) else {
        keyboard_inputs.clear();
        return;
    };
    if !text_input.multiline {
        return;
    }

    let mut text_changed = false;
    let Some(text_box_state) = text_box_states.boxes.get_mut(&focused_entity) else {
        keyboard_inputs.clear();
        return;
    };

    for keyboard_input in keyboard_inputs.read() {
        if keyboard_input.state != ButtonState::Pressed {
            continue;
        }

        match &keyboard_input.logical_key {
            Key::Enter => {
                text_box_state.value.push('\n');
                text_changed = true;
            }
            Key::Backspace => {
                text_box_state.value.pop();
                text_changed = true;
            }
            Key::Character(character) => {
                text_box_state.value.push_str(character.as_str());
                text_changed = true;
            }
            _ => {}
        }
    }

    if !text_changed {
        return;
    }

    text_box_state.first_visible_line = max_first_visible_text_box_line(&text_box_state.value);
    refresh_text_box_view(
        focused_entity,
        input_children,
        text_box_state,
        &children_query,
        &scroll_thumb_query,
        &mut text_query,
        &mut node_query,
    );
}

/// Applies scrollbar track dragging to multiline text boxes.
#[allow(clippy::too_many_arguments)]
pub fn drag_last_beacon_ui_text_box_scrollbars(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut scroll_drag: ResMut<LastBeaconUiTextBoxScrollDrag>,
    mut text_box_states: ResMut<LastBeaconUiTextBoxStates>,
    text_inputs: Query<(Entity, &LastBeaconUiTextInput, Option<&Children>)>,
    children_query: Query<&Children>,
    scroll_track_query: Query<
        (&Interaction, &RelativeCursorPosition),
        With<LastBeaconUiTextScrollTrack>,
    >,
    scroll_thumb_query: Query<(), With<LastBeaconUiTextScrollThumb>>,
    mut text_query: Query<&mut Text>,
    mut node_query: Query<&mut Node>,
) {
    if !mouse_buttons.pressed(MouseButton::Left) {
        scroll_drag.active_text_box = None;
        return;
    }

    for (input_entity, text_input, input_children) in &text_inputs {
        if !text_input.multiline {
            continue;
        }

        let Some(scroll_track_entity) = first_descendant_with_scroll_track(
            input_children,
            &children_query,
            &scroll_track_query,
        ) else {
            continue;
        };
        let Ok((scroll_track_interaction, relative_cursor_position)) =
            scroll_track_query.get(scroll_track_entity)
        else {
            continue;
        };

        if *scroll_track_interaction == Interaction::Pressed {
            scroll_drag.active_text_box = Some(input_entity);
        }
        if scroll_drag.active_text_box != Some(input_entity) {
            continue;
        }

        let Some(normalized_cursor_position) = relative_cursor_position.normalized else {
            continue;
        };
        let Some(text_box_state) = text_box_states.boxes.get_mut(&input_entity) else {
            continue;
        };
        let max_first_visible_line = max_first_visible_text_box_line(&text_box_state.value);
        let scroll_progress = (normalized_cursor_position.y + 0.5).clamp(0.0, 1.0);
        text_box_state.first_visible_line =
            (scroll_progress * max_first_visible_line as f32).round() as usize;

        refresh_text_box_view(
            input_entity,
            input_children,
            text_box_state,
            &children_query,
            &scroll_thumb_query,
            &mut text_query,
            &mut node_query,
        );
    }
}

/// Applies mouse-wheel scrolling to hovered multiline text boxes.
pub fn scroll_last_beacon_ui_text_inputs(
    mut mouse_wheel_messages: MessageReader<MouseWheel>,
    mut text_box_states: ResMut<LastBeaconUiTextBoxStates>,
    mut text_inputs: LastBeaconUiTextInputScrollQuery,
    children_query: Query<&Children>,
    scroll_thumb_query: Query<(), With<LastBeaconUiTextScrollThumb>>,
    mut text_query: Query<&mut Text>,
    mut node_query: Query<&mut Node>,
) {
    let scroll_delta = mouse_wheel_messages
        .read()
        .map(|message| match message.unit {
            MouseScrollUnit::Line => message.y,
            MouseScrollUnit::Pixel => message.y / 18.0,
        })
        .sum::<f32>();
    if scroll_delta.abs() < f32::EPSILON {
        return;
    }

    for (
        input_entity,
        text_input,
        input_children,
        interaction,
        relative_cursor_position,
        mut scroll_position,
    ) in &mut text_inputs
    {
        let cursor_is_over = relative_cursor_position
            .map(RelativeCursorPosition::cursor_over)
            .unwrap_or_else(|| {
                interaction.is_some_and(|interaction| *interaction != Interaction::None)
            });
        if !text_input.multiline || !cursor_is_over {
            continue;
        }
        scroll_position.y = 0.0;

        let Some(text_box_state) = text_box_states.boxes.get_mut(&input_entity) else {
            continue;
        };
        let max_first_visible_line = max_first_visible_text_box_line(&text_box_state.value);
        let scroll_line_delta = if scroll_delta > 0.0 { -1 } else { 1 };
        text_box_state.first_visible_line = text_box_state
            .first_visible_line
            .saturating_add_signed(scroll_line_delta)
            .min(max_first_visible_line);

        refresh_text_box_view(
            input_entity,
            input_children,
            text_box_state,
            &children_query,
            &scroll_thumb_query,
            &mut text_query,
            &mut node_query,
        );
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
        text.0 = rendered_value.clone();
        if let Some(mut editable_text) = editable_text {
            editable_text.editor_mut().set_text(&rendered_value);
            editable_text.queue_edit(TextEdit::TextEnd(false));
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

fn visible_text_box_lines(
    value: &str,
    first_visible_line: usize,
    should_show_caret: bool,
) -> String {
    let total_line_count = value.lines().count().max(1);
    let mut visible_lines = value
        .lines()
        .skip(first_visible_line)
        .take(TEXT_BOX_VISIBLE_LINES)
        .map(str::to_string)
        .collect::<Vec<_>>();
    if visible_lines.is_empty() {
        visible_lines.push(String::new());
    }

    let last_visible_line = first_visible_line + visible_lines.len();
    if should_show_caret && last_visible_line >= total_line_count {
        if let Some(last_line) = visible_lines.last_mut() {
            last_line.push('|');
        }
    }

    visible_lines.join("\n")
}

fn max_first_visible_text_box_line(value: &str) -> usize {
    value.lines().count().saturating_sub(TEXT_BOX_VISIBLE_LINES)
}

fn refresh_text_box_view(
    input_entity: Entity,
    input_children: Option<&Children>,
    text_box_state: &LastBeaconUiTextBoxState,
    children_query: &Query<&Children>,
    scroll_thumb_query: &Query<(), With<LastBeaconUiTextScrollThumb>>,
    text_query: &mut Query<&mut Text>,
    node_query: &mut Query<&mut Node>,
) {
    let Some(text_entity) =
        first_descendant_with_rendered_text(input_children, children_query, text_query)
    else {
        warn!("LastBeaconUiTextInput on {input_entity:?} has no child Text entity.");
        return;
    };
    if let Ok(mut text) = text_query.get_mut(text_entity) {
        text.0 = visible_text_box_lines(
            &text_box_state.value,
            text_box_state.first_visible_line,
            false,
        );
    }

    let max_first_visible_line = max_first_visible_text_box_line(&text_box_state.value);
    let scroll_progress = if max_first_visible_line == 0 {
        0.0
    } else {
        text_box_state.first_visible_line as f32 / max_first_visible_line as f32
    };
    let text_box_scroll_track_height = 72.0;
    let text_box_scroll_thumb_height = 28.0;
    let text_box_scroll_thumb_travel = text_box_scroll_track_height - text_box_scroll_thumb_height;

    if let Some(scroll_thumb_entity) =
        first_descendant_with_scroll_thumb(input_children, children_query, scroll_thumb_query)
    {
        if let Ok(mut scroll_thumb_node) = node_query.get_mut(scroll_thumb_entity) {
            scroll_thumb_node.top = Val::Px(scroll_progress * text_box_scroll_thumb_travel);
        }
    }
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

fn first_descendant_with_rendered_text(
    children: Option<&Children>,
    children_query: &Query<&Children>,
    text_query: &Query<&mut Text>,
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
