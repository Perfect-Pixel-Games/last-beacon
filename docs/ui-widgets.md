# Last Beacon Reusable UI Widgets

This document lists the reusable common UI widgets under `game/assets/ui/widgets/common/` and the properties authors are expected to update when reusing or copying them into game scenes.

Most reusable widgets are demonstrated in `game/assets/scenes/ui_playground.bsn`. Main Menu-specific compositions are intentionally not covered here.

## Keyboard & Focus

Buttons, tabs/radio options, sliders, and dropdown toggles are all keyboard/gamepad-focusable:

- `Tab`/`Shift+Tab` cycles focus among these widgets within the current scene or overlay. A pause overlay traps Tab focus inside itself rather than leaking into the paused scene beneath it.
- A focused widget shows a visible amber outline (the same accent color already used for selected tabs and slider fills).
- `Enter`/`Space` activates the focused widget the same way a mouse click does: it presses a value button, opens/closes a dropdown, or selects a tab/radio option.
- Sliders are focusable and show the outline, but are not yet keyboard-adjustable (no arrow-key stepping). This is a known follow-up, not a bug.
- Authors do not need to add anything to `.bsn` files for this: `TabIndex`, the focus outline, and keyboard activation are all applied automatically to `LastBeaconUiButton`, `LastBeaconUiTab`, `LastBeaconUiValueButton`, `LastBeaconUiDropdownToggle`, and `LastBeaconUiSlider` entities as they spawn.

## General Authoring Notes

- Keep reusable widget assets under `game/assets/ui/widgets/common/`.
- Prefer changing authored BSN values and text content before adding new Rust behavior.
- Reusable widgets should default to `width: Percent(100)` and fill the parent slot. Override the root width only in the parent scene or wrapper when a fixed footprint is intentional.
- When a widget uses a runtime `target`, every component that shares one interactive value must use the same `target` string.
- `LastBeaconUiValueText` mirrors a stored value into text. Its `target`, `prefix`, and `suffix` must match the value source.
- Symbol glyphs such as radio dots and dropdown arrows should keep `LastBeaconUiSymbolIcon` so they use the bundled symbol font.

## Primary Button

Asset: `game/assets/ui/widgets/common/primary_button.bsn`

Use for the main action in a local widget group.

Author-updated properties:

- `bevy_ui::widget::text::Text`: visible button label.
- `FoundationMenuButton { action, scene_key, stack_key }`: button action when used for navigation.
- Root `Node { width }`: defaults to `Percent(100)`; override only when a fixed footprint is intentional.
- `Node { height, padding }`: button height and internal spacing.
- `LastBeaconUiButton { variant }`: normally keep as `"primary"`.

Avoid changing:

- Runtime-managed hover/pressed colors; `LastBeaconUiButton` refreshes these.

## Secondary Button

Asset: `game/assets/ui/widgets/common/secondary_button.bsn`

Use for normal actions that are less prominent than the primary action.

Author-updated properties:

- `bevy_ui::widget::text::Text`: visible button label.
- `FoundationMenuButton { action, scene_key, stack_key }`: navigation/action behavior.
- Root `Node { width }`: defaults to `Percent(100)`; override only when a fixed footprint is intentional.
- `Node { height, padding }`: button height and internal spacing.
- `LastBeaconUiButton { variant }`: normally keep as `"secondary"`.

## Tertiary Button

Asset: `game/assets/ui/widgets/common/tertiary_button.bsn`

Use for quiet or outline-style actions.

Author-updated properties:

- `bevy_ui::widget::text::Text`: visible button label.
- `FoundationMenuButton { action, scene_key, stack_key }`: navigation/action behavior.
- Root `Node { width }`: defaults to `Percent(100)`; override only when a fixed footprint is intentional.
- `Node { height, padding }`: button height and internal spacing.
- `LastBeaconUiButton { variant }`: normally keep as `"tertiary"`.

## Tabs

Asset: `game/assets/ui/widgets/common/tabs.bsn`

Use for stateful tab selection within a local UI area.

Author-updated properties:

- `LastBeaconUiTab { group }`: shared selection group for all tabs in one tab set.
- `LastBeaconUiTab { tab }`: stable identifier for each tab.
- `LastBeaconUiTab { selected }`: set `true` on exactly one initial tab per group.
- `bevy_ui::widget::text::Text`: visible tab label.
- Child tab `Node { flex_grow }`: tabs share available width by default; override width only for intentionally fixed tabs.

Avoid changing:

- Runtime-managed selected/hover/pressed colors.

## Stat Rows Panel

Asset: `game/assets/ui/widgets/common/stat_rows_panel.bsn`

Use for compact labelled rows in a reusable panel.

Author-updated properties:

- `#PanelHeader` text: panel title.
- Each stat row label `Text`: left-side label.
- Each stat row value `Text`: right-side value.
- Root `Node { width }`: defaults to `Percent(100)`; parent slots decide the panel width by default.
- Row count: duplicate or remove stat row blocks as needed.

## Typography Panel

Asset: `game/assets/ui/widgets/common/typography_panel.bsn`

Use to preview and copy the standard text hierarchy for reusable UI surfaces.

Author-updated properties:

- `#TextH1 Text`: page or major panel heading sample.
- `#TextH2 Text`: section heading sample.
- `#TextH3 Text`: subsection/accent heading sample.
- `#TextH4 Text`: small label heading sample.
- `#ParagraphText Text`: body/paragraph copy sample.
- `TextFont { font_size }`: typography scale for each text style.
- `TextColor`: color role for each text style.
- Root `Node { width }`: defaults to `Percent(100)`; parent slots decide the panel width by default.

## Divider

Asset: `game/assets/ui/widgets/common/divider.bsn`

Use as a horizontal division line between related widget groups.

Author-updated properties:

- Root `Node { width }`: defaults to `Percent(100)`; parent slots decide line width by default.
- Root `Node { height }`: divider thickness.
- `BackgroundColor`: divider color and opacity.

## Text Field

Asset: `game/assets/ui/widgets/common/text_field.bsn`

Use for a single-line editable text value.

Author-updated properties:

- `LastBeaconUiTextInput { value }`: initial editable text.
- `LastBeaconUiTextInput { multiline }`: keep `false`.
- Child `Text`: should match the initial `value`.
- Root `Node { width }`: defaults to `Percent(100)`; parent slots decide the field width by default.
- `Node { height, padding }`: field height and text inset.

Runtime behavior:

- Clicking focuses native Bevy `EditableText`.
- Text is left-aligned for normal text fields.

## Text Box

Asset: `game/assets/ui/widgets/common/text_box.bsn`

Use for multiline editable text with vertical and horizontal scrolling.

Author-updated properties:

- `LastBeaconUiTextInput { value }`: initial multiline text.
- `LastBeaconUiTextInput { multiline }`: keep `true`.
- Child `Text`: should match the initial `value`.
- Root `Node { width }`: defaults to `Percent(100)`; parent slots decide the text-box width by default.
- Root `Node { height, padding }`: overall box height and content inset.
- Text `TextFont { font_size }`: visual text size.

Runtime-calculated properties:

- Vertical scrollbar track position, length, and thickness.
- Horizontal scrollbar track position, length, and thickness.
- Scroll thumb lengths and travel distance.
- Scrollbar positions from native `TextScroll`.

Do not hardcode scrollbar dimensions for a specific scene. The runtime derives them from the box size and text content.

Runtime behavior:

- Mouse wheel scrolls vertically.
- Horizontal wheel or `Shift + mouse wheel` scrolls horizontally.
- Scrollbar dragging can move the caret offscreen.
- Typing or keyboard caret movement scrolls back to keep the caret visible.

## Radio Buttons

Asset: `game/assets/ui/widgets/common/radio_buttons.bsn`

Use for mutually exclusive option choices.

Author-updated properties:

- `LastBeaconUiTab { group }`: shared group for the radio set.
- `LastBeaconUiTab { tab }`: option identifier.
- `LastBeaconUiTab { selected }`: initial selected option.
- `LastBeaconUiValueButton { target }`: value key updated when clicked; usually same semantic key as the tab group.
- `LastBeaconUiValueButton { set_value }`: stored value for the option.
- `LastBeaconUiRadioIcon { group, tab, selected }`: must match the corresponding option.
- Option label `Text`: visible label.

Keep:

- Radio icon text as `●`/`○` and keep `LastBeaconUiSymbolIcon`.

## Toggle Buttons

Asset: `game/assets/ui/widgets/common/toggle_buttons.bsn`

Use for a two-option segmented control.

Author-updated properties:

- `LastBeaconUiTab { group }`: shared group for both buttons.
- `LastBeaconUiTab { tab }`: option identifier.
- `LastBeaconUiTab { selected }`: initial selected option.
- `LastBeaconUiValueButton { target }`: shared stored value key.
- `LastBeaconUiValueButton { set_value }`: value written by each option.
- Button label `Text`: visible option label.
- Button `flex_grow`: toggle buttons share available width by default; override child widths only for intentionally uneven segments.

## Combo Box

Asset: `game/assets/ui/widgets/common/combo_box.bsn`

Use for choosing one value from a small dropdown list.

Author-updated properties:

- `LastBeaconUiDropdownToggle { target }`: dropdown open-state key.
- `LastBeaconUiDropdownPanel { target }`: must match the toggle target.
- `LastBeaconUiDropdownIcon { target }`: must match the toggle target.
- `LastBeaconUiValueText { target }`: displayed selected value key.
- Option `LastBeaconUiValueButton { target }`: stored selected value key.
- Option `LastBeaconUiValueButton { set_value }`: value written by the option.
- Button/display `Text`: initial displayed label.
- Option label `Text`: visible option labels.
- `#ComboBoxOptions Node { top, width }`: popup placement if the button height changes; width should normally remain `Percent(100)`.

Keep:

- Dropdown arrow text as `▾`; runtime changes it to `▴` when open.
- `LastBeaconUiSymbolIcon` on the arrow.

## Number Field

Asset: `game/assets/ui/widgets/common/number_field.bsn`

Use for numeric values that support button increments and direct typing.

Expected order:

- `#NumberMinus`
- `#NumberValueInput`
- `#NumberPlus`

Author-updated properties:

- `LastBeaconUiValueButton { target }` on `#NumberMinus` and `#NumberPlus`: shared numeric value key.
- `LastBeaconUiValueButton { delta }`: increment/decrement amount.
- `LastBeaconUiValueButton { min, max }`: button clamp range.
- `LastBeaconUiTextInput { value }`: initial center value.
- `LastBeaconUiNumberInput { target, min, max }`: editable numeric target and clamp range.
- `LastBeaconUiValueText { target, prefix, suffix }`: center text display format.
- Center child `Text`: should match the initial value.
- Root `Node { width }`: defaults to `Percent(100)`; parent slots decide total width by default.
- Center value `Node { flex_grow }`: fills the remaining space between fixed `-` and `+` buttons.
- `-`/`+` button `Node { width }`: fixed affordance width unless a larger touch target is desired.

Runtime behavior:

- `-` and `+` only change the stored value by `delta`.
- Center value is the only editable input.
- Direct typing is filtered to numeric characters.
- Runtime preserves authored layout while adding native editable behavior.

## Slider

Asset: `game/assets/ui/widgets/common/slider.bsn`

Use for continuous numeric values.

Author-updated properties:

- `LastBeaconUiSlider { target }`: stored value key written by dragging/clicking the track.
- `LastBeaconUiSlider { min, max }`: authored numeric range.
- `LastBeaconUiSliderFill { target, min, max }`: must match the slider target and range.
- `LastBeaconUiValueText { target, prefix, suffix }`: right-hand value display.
- Initial fill `Node { width: Val::Percent(...) }`: initial visual fill matching the initial value.
- Initial value `Text`: displayed value matching the initial stored value.
- Root `Node { width }`: defaults to `Percent(100)`; parent slots decide total width by default.
- Track `Node { flex_grow }`: fills remaining space before the fixed value readout.
- Value readout `Node { width }`: fixed slot for stable percentage text.

Runtime behavior:

- Clicking or dragging the track maps cursor position directly across `min..max`.
- Fill width is refreshed from the stored value.
- The right-hand value text is refreshed from the stored value.

## Property Container

Asset: `game/assets/ui/widgets/common/property_container.bsn`

Use for playground-style property rows: a label on the left and a reusable widget on the right.

Author-updated properties:

- Property name `Text`: left-side row label.
- `LastBeaconBsnWidget { asset_path }`: widget shown in the right-side value slot.
- Left label `Node { width }`: property-name column width.
- Right value `Node { flex_grow }`: widget column fills remaining row width by default.
- Row list: duplicate/remove rows to show the desired properties.

Notes:

- This container is intentionally layout-only: no background, no border, no headers.
- It is useful for UI Playground/documentation scenes, not necessarily for final game screens.

## Uniform Grid

Asset: `game/assets/ui/widgets/common/uniform_grid.bsn`

Use for laying out children in evenly sized columns, wrapping onto new rows automatically.

Author-updated properties:

- `LastBeaconUiUniformGrid { column_count }`: number of equal-width columns. Values below `1` are treated as `1`.
- Cell contents: duplicate/remove/edit the sample cells to show the desired content.
- Root `Node { width }`: defaults to `Percent(100)`; parent slots decide the grid width by default.
- `Node { row_gap, column_gap }`: spacing between cells.

Runtime behavior:

- `LastBeaconUiUniformGrid` configures the entity for CSS Grid layout (`column_count` equal-width columns) the first time it is spawned.
- Children are placed left-to-right and wrap onto new rows automatically.
- Combine with `LastBeaconUiGridItem` (below) to let a cell span more than one column or row.

## Span Grid

`LastBeaconUiGridItem` is not its own asset; it is a component authored on a child of a grid container (such as Uniform Grid) to make that child span more than one cell.

Author-updated properties:

- `LastBeaconUiGridItem { column_span, row_span }`: number of columns/rows this cell should span. Values below `1` are treated as `1`.

Keep:

- Only use this inside an actual grid container (`display: Display::Grid`, such as Uniform Grid) — it has no effect otherwise.

## Aspect Ratio Container

Asset: `game/assets/ui/widgets/common/aspect_ratio_container.bsn`

Use to keep a widget's size within a clamped aspect-ratio band regardless of the window's actual aspect ratio — for example, keeping a HUD from stretching edge-to-edge on an ultrawide monitor or squashing below a legible minimum on a narrow one.

Author-updated properties:

- `LastBeaconUiAspectRatioBounds { min_aspect_ratio, max_aspect_ratio }`: allowed width-over-height ratio band. Set both to the same value for a single fixed ratio (for example `1.7778` for 16:9).
- Inner child contents: replace the sample text/fill with the desired content.

Runtime behavior:

- The bounded widget's own `Node.width`/`Node.height` are recalculated from its parent's available content space every time that space changes, clamped into `[min_aspect_ratio, max_aspect_ratio]`, and fit to the largest size at that ratio that still fits.
- A widget left at `LastBeaconUiAspectRatioBounds`'s default is inert (constrains nothing).

Do not:

- Size the immediate parent from this widget's own size (for example, do not give the parent an automatic/content-based height while this widget is its only child) — pin the parent to a stable area (the window, or a fixed-size panel) instead, or the two sizes can feed back into each other.
- Forget to center this widget in its parent (`align_items: Center, justify_content: Center`); the widget does not center itself.

`game/assets/scenes/ui_playground.bsn` demonstrates the real intended usage directly: the whole scene's content (header + every widget gallery) is wrapped in a `#UiPlaygroundAspectRatioFrame` carrying `LastBeaconUiAspectRatioBounds { min_aspect_ratio: 1.7777778, max_aspect_ratio: 1.7777778 }` (a fixed 16:9), so the entire playground stays letterboxed to 16:9 instead of stretching edge-to-edge on an ultrawide window. `aspect_ratio_container.bsn` remains available separately as a smaller, self-contained example for reuse in other scenes.
