# UI Widget Feathers Alignment Plan

## Metadata
- Feature slug: `ui-widget-feathers-alignment`
- Feature area: `game`
- Primary area: `game`
- Root branch: `feature/ui-widget-feathers-alignment`
- Engine branch: `N/A`
- Engine submodule pointer: `N/A` (no engine changes planned)
- Status: `Planned`
- Planning model: `gpt-5.5` (role fulfilled by Claude Sonnet 5, per the user's standing instruction that Claude/subagents replace GPT in this workflow)
- Implementation model: `gpt-5.4` (role fulfilled by Claude Sonnet 5)
- Review model: `gpt-5.5` (role fulfilled by Claude Sonnet 5)
- Created: `2026-08-24`
- Last updated: `2026-08-24`

## User Request
Review the UI widget library (`game/src/ui_widgets.rs` and `game/assets/ui/widgets/`) covering text fields, combo boxes, radio buttons, sliders, etc., and ensure it is well built and follows the Bevy Feathers design pattern (Bevy's own in-progress editor-widget crate, which has few public examples). After the library is reinforced and solidified, present a list of other UI widgets worth adding for games built on this framework (lazy loading, carousel, etc. — that follow-up list is out of scope for this plan and will be delivered separately once this hardening work lands).

Follow-up scoping conversation (via `AskUserQuestion`) narrowed "reinforce and solidify" to two tiers, both approved:
1. Efficiency + correctness fixes to the existing style-enforcement systems.
2. Keyboard/gamepad-ready focus support (`TabIndex`, visible focus outline, Enter/Space activation) across buttons/tabs/radio/sliders/dropdown toggles.
A third tier (full design-token theme resource, `bevy_ui_widgets` headless primitives, anchored popover combo box, disabled-state support) was explicitly declined for this pass — see Alternatives Considered.

## Feature Summary
`game/src/ui_widgets.rs` (2331 lines) is Last Beacon's hand-rolled widget library: buttons, tabs, radio rows, sliders, text/number inputs, combo boxes, and multiline-text scrollbars, all composed from reusable `.bsn` assets under `game/assets/ui/widgets/common/` and driven by raw `Interaction` polling plus a handful of string-keyed shared resources (`LastBeaconUiInputValues`, `LastBeaconUiDropdownStates`, `LastBeaconUiTabSelections`).

This feature closes two concrete gaps found by comparing that library against the real `bevy_feathers` 0.19.0 crate (already vendored in this workspace via `engine/crates/foundation-runtime-library`'s optional `dev-tools` feature, so its source is directly inspectable rather than guessed at):

1. **Efficiency/correctness:** `enforce_last_beacon_button_styles` recomputes and rewrites `BackgroundColor`/`BorderColor`/`TextColor` for every button and tab on every single frame, with no `Changed`/`Added` gating anywhere in its queries — unlike every equivalent Feathers system, and unlike Foundation's own generic button-interaction coloring, which already gates on `Changed<Interaction>`.
2. **Missing capability:** none of Last Beacon's interactive widgets are reachable by keyboard or gamepad today — there is no `TabIndex`, no visible focus indicator, and no keyboard activation anywhere in `game/src`. Feathers puts `TabIndex` + a focus outline on every interactive control. This matters more for a game than for an editor.

A third, larger tier (a central `UiTheme` design-token resource, adopting `bevy_ui_widgets` headless primitives in place of raw `Interaction` polling, an anchored popover combo box, and `InteractionDisabled` support) was explicitly scoped out of this pass by the user; see Alternatives Considered.

## Feature Area Classification
- Area: `game`
- Primary area: `game`
- Rationale: All affected files (`game/src/ui_widgets.rs`, `game/src/lib.rs`, `docs/ui-widgets.md`) are Last Beacon-owned. Nothing under `engine/` needs to change — `bevy_input_focus`'s tab-navigation machinery this feature relies on is already available through the `bevy` crate's own default features, and `bevy_feathers` is only used as a read-only architectural reference (via the locally vendored crate source), not as a new dependency.

## Codebase Research
- `game/src/ui_widgets.rs:1493-1536` (`enforce_last_beacon_button_styles`): registered unconditionally in `PostUpdate` (`game/src/lib.rs:240`), with no `Changed`/`Added` filter anywhere in `LastBeaconUiButtonStyleQuery`, `LastBeaconUiTabStyleQuery`, `MainMenuPrimaryButtonStyleQuery`, `BeaconPrimaryButtonStyleQuery`, or `BeaconTabButtonStyleQuery` (`ui_widgets.rs:413-476`). It rewrites color components for every matching entity every frame regardless of state.
- `engine/crates/foundation-runtime-library/src/menu.rs:754-765` (`FoundationMenuButtonInteractionQuery`): Foundation's own generic button coloring already gates on `Changed<Interaction>` — Last Beacon's override system is the outlier, not the norm, in this codebase.
- Grepped `game/src` for `TabIndex`, `bevy_ui_widgets`, `InteractionDisabled`, `AccessibilityNode`: zero matches. Only text inputs get `InputFocus` (`focus_last_beacon_ui_text_inputs`, `ui_widgets.rs:734`).
- `bevy::input_focus::{FocusCause, InputFocus}` is already imported and used (`ui_widgets.rs:18`), confirming the `bevy_input_focus` feature is already enabled by default in this workspace's `bevy = "0.19.0"` dependency (verified against `bevy-0.19.0/Cargo.toml`'s default feature list and `bevy_internal-0.19.0/src/lib.rs:58`, `pub use bevy_input_focus as input_focus;`). `InputFocusPlugin`/`InputDispatchPlugin` are therefore already active. `TabNavigationPlugin` (in `bevy::input_focus::tab_navigation`) is a separate plugin that is NOT currently registered anywhere in `game/src` (confirmed via grep), so Tab/Shift+Tab cycling does not work yet even though the underlying resources exist.
- `bevy_input_focus-0.19.0/src/tab_navigation.rs:55-56`: `TabIndex`'s own doc comment states an ancestor must carry `TabGroup` for tab cycling to have any effect at all. Last Beacon has no `TabGroup` anywhere today.
- `game/Cargo.toml`'s `default = ["dev-tools", "editor"]` enables `foundation-runtime-library/dev-tools`, which (`engine/crates/foundation-runtime-library/src/console/mod.rs:63-68`) conditionally installs `bevy_feathers::FeathersPlugins` — and `FeathersPlugins::build` (`bevy_feathers-0.19.0/src/lib.rs:119-125`) itself adds `TabNavigationPlugin`. The existing console code already guards its own `FeathersCorePlugin` add with `!app.is_plugin_added::<bevy_feathers::FeathersCorePlugin>()`; this plan's `TabNavigationPlugin` registration must use the same `is_plugin_added` guard, or a default (dev-tools-enabled) build will panic on a duplicate plugin.
- `bevy_ui-0.19.0/src/ui_node.rs:2367-2368` (`Outline` doc comment): explicitly recommends mutating `Outline.color` in place (`Color::NONE` to hide) instead of inserting/removing the component repeatedly, because repeated insert/remove causes archetype table moves. This shapes the focus-outline design in Phase 2.
- `bevy_ui-0.19.0/src/focus.rs:174-296` (`ui_focus_system`): only resets a non-hovered node's `Interaction` from `Hovered` back to `None`; it never resets a non-hovered `Pressed` back to `None` except via the `mouse_released` sweep at the top of the function. This means forcing `Interaction::Pressed` from a keyboard system (to "fake" a click) would get stuck indefinitely rather than clearing itself after one frame — ruled this approach out (see Alternatives Considered).
- `game/assets/ui/widgets/common/*.bsn` (24 files, e.g. `combo_box.bsn:21-22`, `radio_buttons.bsn:12-13`, `slider.bsn`): bake resting-state colors directly per instance. These are fully overwritten by the Rust style systems on the very first frame today (and will continue to be, since the Phase 1 fix still fires on `Added`/first `Changed<Interaction>`), so no `.bsn` files need to change for this plan.
- `docs/ui-widgets.md`: existing per-widget authoring/behavior reference; gets a short new section once keyboard/focus behavior lands.

## External Research
No web search/fetch was performed or needed. Instead, the actual pinned dependency source was read directly from the local Cargo registry cache (`bevy_feathers-0.19.0`, `bevy_input_focus-0.19.0`, `bevy_ui-0.19.0`, all under `C:\Users\jonla\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\`), which is more authoritative than a web search since it is the exact version resolved in `engine/Cargo.lock`/`game/Cargo.lock`. Files read in full or in relevant part: `bevy_feathers::{lib.rs, theme.rs, tokens.rs, focus.rs, controls::{mod.rs, button.rs, radio.rs, slider.rs, number_input.rs, text_input.rs, menu.rs, toggle_switch.rs, scrollbar.rs}}`, `bevy_input_focus::tab_navigation.rs`, `bevy_ui::{focus.rs, ui_node.rs}`.

## Affected Files And Systems
- `game/src/ui_widgets.rs`: gate the style-enforcement queries (Phase 1); add `LastBeaconUiFocusIndicator` component, `apply_last_beacon_ui_tab_groups_to_scene_roots`, `apply_last_beacon_ui_focusability`, `apply_last_beacon_ui_focus_outline`, and `activate_last_beacon_ui_focused_widget_on_keyboard_input` systems; extract `apply_value_button_activation`, `apply_dropdown_toggle_activation`, and `apply_tab_selection_activation` helper functions out of the existing mouse-driven systems so the keyboard path can reuse them (Phase 2).
- `game/src/lib.rs`: register `TabNavigationPlugin` (guarded against a double-add from the `dev-tools` console feature), register the new systems in the existing `Update` system tuples, and `register_type::<ui_widgets::LastBeaconUiFocusIndicator>()`.
- `docs/ui-widgets.md`: add a short "Keyboard & Focus" section documenting the new Tab/Enter/Space behavior.
- No `engine/` changes. No `.bsn` asset changes.

## Proposed Implementation Approach

### Phase 1 — Efficiency and correctness fix for style enforcement
1. Add `Changed<Interaction>` to the filter tuples of `LastBeaconUiButtonStyleQuery`, `MainMenuPrimaryButtonStyleQuery`, `BeaconPrimaryButtonStyleQuery`, and `BeaconTabButtonStyleQuery` (`ui_widgets.rs:413-476`), so these loops in `enforce_last_beacon_button_styles` only run for entities whose `Interaction` actually changed since the last time the system ran (this still fires on first spawn, since Bevy's change detection treats component insertion as a change).
2. Change `LastBeaconUiTabStyleQuery`'s `&'static Interaction` field to `Ref<'static, Interaction>` (no query-level `Changed` filter, because a tab's displayed style also depends on the shared `LastBeaconUiTabSelections` resource, not only its own `Interaction`). In the tab loop body, add a guard: skip recomputation unless `tab_interaction.is_changed() || tab_selections.is_changed()`.
3. Leave every other query/loop body in `enforce_last_beacon_button_styles` unchanged — the three "restore fixed accent color" loops (main menu primary, Beacon primary, Beacon tab) need no body changes once their queries are gated, since they always wrote the same fixed color regardless of state.
4. Add three tests to `ui_widgets.rs`'s existing `#[cfg(test)] mod tests`:
   - A button's `BackgroundColor` must NOT be rewritten on a frame where its `Interaction` did not change (seed a sentinel color after the first `app.update()`, run again, assert the sentinel survives).
   - A button's `BackgroundColor` MUST update the frame its `Interaction` changes (e.g. `None` -> `Hovered`).
   - Two tabs sharing a `LastBeaconUiTabSelections` group: selecting the second tab via the resource (without touching either tab's own `Interaction`) must restyle BOTH tabs (the newly-selected one and the newly-deselected one) on the next `app.update()`.

### Phase 2 — Keyboard/gamepad-ready focus support
1. In `game/src/lib.rs`, register `TabNavigationPlugin` guarded by `is_plugin_added`:
   ```rust
   if !app.is_plugin_added::<bevy::input_focus::tab_navigation::TabNavigationPlugin>() {
       app.add_plugins(bevy::input_focus::tab_navigation::TabNavigationPlugin);
   }
   ```
   This mirrors the existing guard pattern in `engine/crates/foundation-runtime-library/src/console/mod.rs:63-68` and avoids a duplicate-plugin panic when the default `dev-tools` feature's debug console has already added `FeathersPlugins` (which itself adds `TabNavigationPlugin`).
2. Add `apply_last_beacon_ui_tab_groups_to_scene_roots` in `ui_widgets.rs`: a reactive system over `Query<Entity, (Added<SceneOwner>, Without<ChildOf>)>` that inserts `TabGroup::modal()` onto every newly-spawned top-level scene root. Using `modal()` (rather than a plain non-modal group) keeps Tab cycling confined to whichever scene or overlay currently owns focus, so a pause overlay cannot leak Tab focus into the paused scene underneath it.
3. Add a new marker component `LastBeaconUiFocusIndicator` (mirrors Feathers' `FocusIndicator`) and a reactive system `apply_last_beacon_ui_focusability` that, on `Added<LastBeaconUiButton>` / `Added<LastBeaconUiTab>` / `Added<LastBeaconUiValueButton>` / `Added<LastBeaconUiDropdownToggle>` / `Added<LastBeaconUiSlider>`, inserts `(TabIndex(0), LastBeaconUiFocusIndicator, Outline::new(Val::Px(2.0), Val::Px(2.0), Color::NONE))`. Following `bevy_ui::Outline`'s own guidance, the outline is inserted once with a transparent color and only ever has its `.color` field mutated afterward — never removed/reinserted.
4. Add `apply_last_beacon_ui_focus_outline`: reads `InputFocus`/`InputFocusVisible` (early-return unless either `is_changed()`), and for every `Query<&mut Outline, With<LastBeaconUiFocusIndicator>>` entity sets `.color` to the shared amber accent (`Color::srgb(0.984, 0.749, 0.141)`, matching the existing tab/slider accent already used elsewhere in this file) when that entity is the focused entity and focus is visible, or `Color::NONE` otherwise — guarded by an equality check so it only writes (and only triggers downstream change detection) when the color actually needs to change.
5. Extract the body of `update_last_beacon_ui_value_buttons`, `toggle_last_beacon_ui_dropdowns`, and `update_last_beacon_ui_tab_selection` into three small helper functions (`apply_value_button_activation`, `apply_dropdown_toggle_activation`, `apply_tab_selection_activation`) that take the relevant component(s) plus `&mut` resource references and perform the state change — no `Interaction` involved. Each existing mouse-driven system becomes a thin loop that calls its helper when `*interaction == Interaction::Pressed`.
6. Add `activate_last_beacon_ui_focused_widget_on_keyboard_input`: reads `MessageReader<KeyboardInput>` + `Res<InputFocus>`; if Enter or Space (`Key::Enter | Key::Space`, matching the existing `keyboard_input_should_reveal_text_caret` convention already in this file) was just pressed while an entity is focused, looks that entity up in `Query<&LastBeaconUiValueButton>` / `Query<&LastBeaconUiDropdownToggle>` / `Query<&LastBeaconUiTab>` and calls the matching helper directly. This deliberately never touches the `Interaction` component itself (see Alternatives Considered for why).
7. Register the new systems in `game/src/lib.rs`'s existing `Update` system tuples (alongside `update_last_beacon_ui_value_buttons`, `toggle_last_beacon_ui_dropdowns`, `update_last_beacon_ui_tab_selection`) and `register_type::<ui_widgets::LastBeaconUiFocusIndicator>()`.
8. Add tests: scene root gains `TabGroup`; nested scene-owned entity does not; each marker type gains `TabIndex`/`Outline`/`LastBeaconUiFocusIndicator` on spawn; focus outline color follows `InputFocus`/`InputFocusVisible` and clears when focus moves elsewhere; keyboard Enter/Space on a focused value-button/dropdown-toggle/tab produces the same state change as a mouse `Pressed` interaction.
9. Add a short "Keyboard & Focus" section to `docs/ui-widgets.md` documenting: Tab/Shift+Tab cycles focus within a scene or overlay; focus shows an amber outline; Enter/Space activates the focused button, tab, radio option, or dropdown toggle; sliders are focusable but not yet keyboard-adjustable (documented as a known follow-up, not a bug).

### Phase 3 — Validation and PR readiness
1. Run `cargo fmt --manifest-path game/Cargo.toml -- --check`, `cargo clippy --manifest-path game/Cargo.toml --all-targets --all-features -- -D warnings`, `cargo test --manifest-path game/Cargo.toml --all-features`, `cargo doc --manifest-path game/Cargo.toml --all-features --no-deps`, and `scripts\validate.cmd`.
2. Manual QA pass (not automatable without a full render/input harness): launch the game, tab through the Main Menu and UI Playground scenes, confirm a visible focus outline, confirm Enter/Space activates the focused button/tab/dropdown toggle, confirm opening the pause overlay traps Tab focus inside it.
3. Push the branch and prepare a pull request into `dev` per the gitflow-workflow skill (no local merge).

## Submodule Plan
- Engine changes required: `no`
- Engine branch: `N/A`
- Engine commit expectation: `N/A`
- Bound engine commit hash: `N/A`
- Root pointer update required: `no`

## Alternatives Considered
- **Full architectural adoption of Feathers** (central `UiTheme` design-token resource replacing hardcoded `Color::srgb(...)` literals; migrating from raw `Interaction` polling to `bevy_ui_widgets` headless primitives such as `Hovered`/`Pressed`/`Checked`/`InteractionDisabled`; a true anchored `Popover`-based combo box with click-outside/Escape/arrow-key navigation; `InteractionDisabled` support) — presented to the user as a third tier and explicitly declined for this pass. Feathers' own doc comment (`bevy_feathers-0.19.0/src/lib.rs:8-12`) states it is "deliberately not intended for" game UI and recommends copying/adapting rather than depending on it; Last Beacon's string-keyed `.bsn`-driven data binding is also a deliberate, reasonable divergence (it lets non-Rust content author value bindings) rather than a defect to fix away. Left as a documented option for a future plan.
- **Forcing `Interaction::Pressed` from a keyboard system** to reuse the existing mouse-driven `Changed<Interaction>` systems unmodified — rejected after reading `bevy_ui::focus::ui_focus_system` (`bevy_ui-0.19.0/src/focus.rs:174-296`): it only resets a non-hovered node's `Interaction` from `Hovered` back to `None`, never from `Pressed`, except via a `mouse_released`-triggered sweep. A keyboard-forced `Pressed` on a non-hovered widget would therefore get stuck indefinitely. Extracting small activation helpers and calling them directly from both the mouse and keyboard paths avoids fighting `bevy_ui`'s ownership of `Interaction`.
- **Inserting/removing `Outline` per focus change** — rejected because `bevy_ui::Outline`'s own doc comment (`bevy_ui-0.19.0/src/ui_node.rs:2367-2368`) warns this causes archetype table moves; instead `Outline` is inserted once (color `Color::NONE`) alongside `TabIndex`, and the focus system only ever mutates `.color`.

## Risks, Constraints, And Assumptions
- True gamepad button-to-UI-activation mapping (D-pad moving focus, an "A" button pressing) is explicitly **not** included in this plan — there is no gamepad-to-keyboard-event remapping layer anywhere in this codebase today, and building one is a separate feature. This plan makes the widgets keyboard-focusable/activatable and gives visible focus feedback, which is the prerequisite groundwork a later gamepad-remap layer would need.
- Keyboard *stepping* of `LastBeaconUiSlider` values (e.g. arrow keys) is out of scope for this plan; sliders become focusable and show the outline, but are not yet keyboard-adjustable. Documented in `docs/ui-widgets.md` as a known follow-up, not silently dropped.
- Assumes `dev-tools`/`editor` default features remain enabled in typical dev builds (per `game/Cargo.toml:13`), which is why the `TabNavigationPlugin` registration must use the `is_plugin_added` guard described in Phase 2, Step 1.
- No `.bsn` asset files change in this plan; the Phase 1 fix is verified to still style widgets correctly on first spawn (change detection treats insertion as a change), so there is no expected visual regression.

## Open Questions
None blocking — scope was already narrowed via an explicit `AskUserQuestion` decision before this plan was written (see User Request).

## Documentation Expectations
- Every new public function/component gets a Rustdoc comment, matching the existing convention in `ui_widgets.rs` (every public item there already has one).
- `docs/ui-widgets.md` gets the new "Keyboard & Focus" section described in Phase 2, Step 9.
- `cargo doc --manifest-path game/Cargo.toml --all-features --no-deps` must succeed as part of Phase 3 validation.

## Implementation Handoff Notes
- Use Claude Sonnet 5 for implementation (fulfilling the `gpt-5.4` role per the user's standing instruction).
- Read `game/src/ui_widgets.rs` in full before editing — it is long (2331 lines) and has established conventions (query type aliases named `LastBeacon...Query`, reactive `Added<T>` systems, resource-`is_changed()`-gated `refresh_*` systems) that new code must match rather than introduce a second style.
- Apply `.pi/skills/rust-coding-standards/SKILL.md` throughout: descriptive names (no abbreviations like `ent`, even though the reference Feathers source uses them), named values before non-trivial function calls, and frequent "why" comments at points that cross schedule/ownership boundaries (e.g. the `Ref<Interaction>`-based tab gating, the `is_plugin_added` guard, the "why we don't touch `Interaction` from the keyboard path" note).
- Commit at the end of each phase (or more often per task), following the commit message format in `.pi/skills/gitflow-workflow/SKILL.md` (short title, capitalized, changed-file list, no author/co-author lines, no conventional-commit prefixes).

## Optional Review Focus Areas
- Use Claude Sonnet 5 for review (fulfilling the `gpt-5.5` role per the user's standing instruction).
- Confirm the `Ref<Interaction>`-based per-tab gating in `enforce_last_beacon_button_styles` doesn't regress the cross-entity tab-selection-changed case (covered by a dedicated test in Phase 1).
- Confirm the `TabNavigationPlugin` `is_plugin_added` guard actually prevents a duplicate-plugin panic when `dev-tools` is enabled (the default) — ideally exercised by running the game with default features, not only `cargo test`.
- Confirm no `.bsn` asset visually regresses on first paint now that `enforce_last_beacon_button_styles` is gated (should be a non-issue since insertion still counts as a change, but worth a manual look).

## Success Criteria
- `enforce_last_beacon_button_styles` no longer rewrites color components on a frame where nothing relevant changed (covered by the Phase 1 sentinel-color test).
- Tab/Shift+Tab visibly cycles focus among buttons, tabs, radio options, sliders, and dropdown toggles in every scene, with a visible focus outline; Enter/Space activates the focused discrete-action widget the same way a mouse click does; a pause overlay traps Tab focus inside itself.
- `cargo fmt --check`, `cargo clippy -D warnings`, `cargo test --all-features`, and `cargo doc --no-deps` all pass against `game/Cargo.toml`; `scripts\validate.cmd` passes.

## Testing Methodology
- Game validation: `scripts\validate.cmd`.
- Focused commands: `cargo fmt --manifest-path game/Cargo.toml -- --check`, `cargo clippy --manifest-path game/Cargo.toml --all-targets --all-features -- -D warnings`, `cargo test --manifest-path game/Cargo.toml --all-features`, `cargo doc --manifest-path game/Cargo.toml --all-features --no-deps`.
- New unit tests per task, using the existing `App::new() + MinimalPlugins` pattern already established in `ui_widgets.rs`'s `#[cfg(test)] mod tests`.
- Manual QA checklist (recorded in the tracker) for the keyboard-navigation behavior, which is not meaningfully automatable without a full render/input harness.
- Engine validation: `N/A` (no engine changes).
