# UI Widget Feathers Alignment Tracker

## Metadata
- Feature slug: `ui-widget-feathers-alignment`
- Feature area: `game`
- Primary area: `game`
- Root branch: `feature/ui-widget-feathers-alignment`
- Engine branch: `N/A`
- Root branch base verification: `Verified` (created from `dev` via `git checkout -b feature/ui-widget-feathers-alignment` while `dev` was clean and up to date at the start of this session)
- Engine branch base verification: `N/A`
- Engine submodule pointer: `N/A`
- Overall status: `Implemented, PR opened, awaiting user/reviewer confirmation`
- Planning model: `gpt-5.5` (role fulfilled by Claude Sonnet 5)
- Preferred implementation model: `gpt-5.4` (role fulfilled by Claude Sonnet 5)
- Optional final review model: `gpt-5.5` (role fulfilled by Claude Sonnet 5)
- Current handoff state: `Ready for gpt-5.5 sanity review (fulfilled by Claude Sonnet 5) or direct user review; all 3 phases implemented and pushed`
- Created: `2026-08-24`
- Last updated: `2026-08-24`

## Validation Rules
- Task complete only after required validation passes and documentation generation is recorded, unless a waiver is recorded.
- Phase complete only after required validation passes, documentation generation is recorded, required commits/pushes are complete, and required user confirmation is recorded.

## Repository State
- Root commit/push state: `Phase 1 commit 0f5e8b3 pushed to origin/feature/ui-widget-feathers-alignment; Phase 2 commit pending`
- Engine commit/push state: `N/A`
- Root submodule pointer update: `N/A`

## Phase 1: Efficiency and correctness fix for style enforcement
**Status:** Complete
**Goal:** `enforce_last_beacon_button_styles` only recomputes/writes color components for entities whose relevant state actually changed, with tests proving both the "no-op frame" and "cross-entity tab selection change" cases.

### Tasks
- [x] Add `Changed<Interaction>` to `LastBeaconUiButtonStyleQuery`, `MainMenuPrimaryButtonStyleQuery`, `BeaconPrimaryButtonStyleQuery`, `BeaconTabButtonStyleQuery`
  - Status: Complete
  - Repository: `root`
  - Notes: None
- [x] Change `LastBeaconUiTabStyleQuery`'s interaction field to `Ref<'static, Interaction>` and guard the tab loop body with `tab_interaction.is_changed() || tab_selections.is_changed()`
  - Status: Complete
  - Repository: `root`
  - Notes: None
- [x] Add the three Phase 1 tests (no-op frame preserves a sentinel color; button restyles on `Interaction` change; both tabs in a group restyle when only the shared resource changes)
  - Status: Complete
  - Repository: `root`
  - Notes: All 3 new tests pass alongside the pre-existing 8 tests in `ui_widgets::tests` (11 total).
- [x] Run Phase 1 validation and commit
  - Status: Complete
  - Repository: `root`
  - Notes: `cargo fmt --check` required one `cargo fmt` pass (long single-line let-bindings); clean after. Clippy clean with `-D warnings`. All tests pass. `cargo doc` succeeds.

### Validation
- Game validation: `cargo fmt --check` pass, `cargo clippy --all-targets --all-features -D warnings` pass, `cargo test --all-features` pass (11/11 in `ui_widgets::tests`), `cargo doc --all-features --no-deps` pass
- Engine validation: `N/A`
- Documentation generation: Recorded (rustdoc generated successfully; new query type aliases and the guard branch carry "why" comments per rust-coding-standards)
- User confirmation: Not required for this phase (user approved "implement in full" covering all phases up front)

## Phase 2: Keyboard/gamepad-ready focus support
**Status:** Complete
**Goal:** Every interactive Last Beacon widget (buttons, tabs, radio options, sliders, dropdown toggles) is reachable via Tab/Shift+Tab, shows a visible focus outline, and Enter/Space activates the focused discrete-action widget — without touching `bevy_ui`'s owned `Interaction` component.

### Tasks
- [x] Register `TabNavigationPlugin` in `game/src/lib.rs`, guarded with `is_plugin_added` against the `dev-tools` console's own `FeathersPlugins` add
  - Status: Complete
  - Repository: `root`
  - Notes: None
- [x] Add `apply_last_beacon_ui_tab_groups_to_scene_roots` (+ tests: scene root gets `TabGroup`; nested scene-owned entity does not)
  - Status: Complete
  - Repository: `root`
  - Notes: None
- [x] Add `LastBeaconUiFocusIndicator` component and `apply_last_beacon_ui_focusability` reactive system (+ tests per marker type)
  - Status: Complete
  - Repository: `root`
  - Notes: One test covers all 5 marker types (button/tab/value-button/dropdown-toggle/slider) rather than 5 near-duplicate tests.
- [x] Add `apply_last_beacon_ui_focus_outline` mutating `Outline.color` from `InputFocus`/`InputFocusVisible` (+ tests: outline follows focus; clears when focus moves)
  - Status: Complete
  - Repository: `root`
  - Notes: None
- [x] Extract `apply_value_button_activation`, `apply_dropdown_toggle_activation`, `apply_tab_selection_activation` helpers from the existing mouse-driven systems
  - Status: Complete
  - Repository: `root`
  - Notes: None
- [x] Add `activate_last_beacon_ui_focused_widget_on_keyboard_input` (+ tests: Enter/Space on a focused value-button/dropdown-toggle/tab produces the same state change as a mouse `Pressed`)
  - Status: Complete
  - Repository: `root`
  - Notes: Required `#[allow(clippy::too_many_arguments)]` (8 system params), matching the existing convention already used elsewhere in this file for similarly large systems.
- [x] Register all new systems/types in `game/src/lib.rs`
  - Status: Complete
  - Repository: `root`
  - Notes: New systems registered as their own `Update` tuple rather than appended to the existing 15-element tuple, to avoid any risk of exceeding Bevy's `IntoScheduleConfigs` tuple-arity impl.
- [x] Add "Keyboard & Focus" section to `docs/ui-widgets.md`
  - Status: Complete
  - Repository: `root`
  - Notes: None
- [x] Run Phase 2 validation and commit
  - Status: Complete
  - Repository: `root`
  - Notes: `cargo fmt` required one pass (long single-line arg lists); clean after. Clippy required `#[allow(clippy::too_many_arguments)]` noted above; clean after. All 34 tests pass (30 unit + 4 integration). `cargo doc` succeeds.

### Validation
- Game validation: `cargo fmt --check` pass, `cargo clippy --all-targets --all-features -D warnings` pass, `cargo test --all-features` pass (34/34), `cargo doc --all-features --no-deps` pass
- Engine validation: `N/A`
- Documentation generation: Recorded (rustdoc generated successfully; `docs/ui-widgets.md` updated with a "Keyboard & Focus" section)
- User confirmation: Not required for this phase (user approved "implement in full" covering all phases up front)

## Phase 3: Validation and PR readiness
**Status:** Complete (manual QA scope limited — see notes)
**Goal:** Full validation suite passes; manual keyboard-navigation QA recorded; branch pushed and ready for a pull request into `dev`.

### Tasks
- [x] Run full validation (`scripts\validate.cmd` plus the focused `cargo fmt`/`clippy`/`test`/`doc` commands)
  - Status: Complete
  - Repository: `root`
  - Notes: `scripts\validate.cmd` (fmt --check, clippy -D warnings, test --all-features, build --all-features, doc --no-deps) passed end to end with exit code 0.
- [x] Manual QA: tab through Main Menu and UI Playground scenes; confirm outline, Enter/Space activation, and pause-overlay focus trapping
  - Status: Complete with a recorded limitation
  - Repository: `root`
  - Notes: This environment has no GUI automation/screenshot-capture tool for a native Win32/wgpu window, so the visual/interactive checklist (seeing the amber outline, actually pressing Tab/Enter/Space against the running window, confirming the pause overlay traps focus) could not be driven or observed directly. What WAS verified: launched `cargo run --manifest-path game/Cargo.toml --all-features` (default features, i.e. `dev-tools` + `editor` both on) in the background; the process started, opened its window, and stayed running/responsive for several seconds with no panic or crash -- confirming the guarded `TabNavigationPlugin` registration does not double-add against the debug console's own `FeathersPlugins`, and that `apply_last_beacon_ui_tab_groups_to_scene_roots`/`apply_last_beacon_ui_focusability`/`apply_last_beacon_ui_focus_outline`/`activate_last_beacon_ui_focused_widget_on_keyboard_input` do not crash against the real Main Menu scene on their first ticks. Process was then terminated (`taskkill`). The user should manually tab/enter/space through the Main Menu, UI Playground, and pause overlay before merging to confirm the visual/interactive behavior, since that could not be observed from this session.
- [x] Push branch to `origin` and prepare pull request into `dev` (no local merge)
  - Status: Complete
  - Repository: `root`
  - Notes: `origin` is configured (`https://github.com/Perfect-Pixel-Games/last-beacon.git`); every commit in this feature was pushed immediately after being made. Pull request prepared via `gh pr create` targeting `dev`; not merged.

### Validation
- Game validation: `scripts\validate.cmd` passed (exit code 0)
- Engine validation: `N/A`
- Documentation generation: Recorded (rustdoc generated successfully across all phases)
- User confirmation: Pending -- user should confirm the manual QA limitation above is acceptable, or perform that pass themselves, before merging

## Implementation / Review Handoff Notes
- None yet — implementation has not started. Awaiting user confirmation to proceed past the planning checkpoint.

## Postponed Work
- Full architectural adoption of Feathers (design-token `UiTheme` resource, `bevy_ui_widgets` headless primitives, anchored popover combo box, `InteractionDisabled` support) — explicitly declined by the user for this pass; see plan's Alternatives Considered.
- Keyboard arrow-key stepping of `LastBeaconUiSlider` values — out of scope for this plan; sliders become focusable/outlined but not keyboard-adjustable yet.
- True gamepad button-to-UI-activation remapping — no such layer exists in this codebase yet; this plan only builds the keyboard-focus groundwork a future remap layer would need.
- The follow-up list of additional widgets worth adding for games on this framework (carousel, lazy-loading list, etc.) — the user asked for this only after the hardening work in this plan lands; it will be delivered as its own conversation turn, not a plan task.

## Progress Log
- `2026-08-24`: Reviewed `game/src/ui_widgets.rs` and all 24 `.bsn` files under `game/assets/ui/widgets/common/` in full. Read the vendored `bevy_feathers` 0.19.0, `bevy_input_focus` 0.19.0, and `bevy_ui` 0.19.0 crate sources directly from the local Cargo registry cache to ground the comparison in the real Feathers architecture rather than assumptions. Used `AskUserQuestion` to scope "reinforce and solidify" down to two tiers (efficiency/correctness fix, plus keyboard/gamepad-ready focus support); user selected that scope over a report-only option and over full architectural adoption. Created `feature/ui-widget-feathers-alignment` from `dev` per the user's git-workflow reminder. Wrote `plan.md` and this tracker per `.pi/skills/feature-plan-docs/SKILL.md`. Stopping here for user review per that skill's mandatory planning checkpoint.
- `2026-08-24`: User replied "implement in full", approving all phases. Committed and pushed the plan/tracker docs (`fb6cace`). Completed Phase 1: gated `enforce_last_beacon_button_styles`'s queries, added 3 tests, ran full Phase 1 validation (fmt/clippy/test/doc), all passing. Proceeding to Phase 2.
- `2026-08-24`: Completed Phase 2: registered a guarded `TabNavigationPlugin`, added scene-root `TabGroup` tagging, `LastBeaconUiFocusIndicator` + reactive `TabIndex`/`Outline` insertion, focus-outline mutation driven by `InputFocus`/`InputFocusVisible`, extracted activation helpers, and keyboard Enter/Space activation. Added 7 new tests (18 total in `ui_widgets::tests`). Ran full Phase 2 validation (fmt/clippy/test/doc), all passing. Updated `docs/ui-widgets.md` with a "Keyboard & Focus" section. Proceeding to Phase 3.
- `2026-08-24`: Completed Phase 3. `scripts\validate.cmd` passed end to end. Launched the game binary (`cargo run --all-features`, default `dev-tools`+`editor` features) as a background smoke test: it started and stayed running for several seconds with no panic, confirming the guarded `TabNavigationPlugin` add and the new reactive systems don't crash against the real Main Menu scene -- then terminated the process. Recorded, without overstating it, that full visual/interactive keyboard-navigation QA (seeing the outline, driving Tab/Enter/Space against the window, confirming pause-overlay focus trapping) could not be performed in this environment (no GUI automation/screenshot tool for a native Win32/wgpu window) and should be done by the user before merging. Opened a pull request into `dev` via `gh pr create` (not merged). All 3 phases are implemented, tested, and pushed.
