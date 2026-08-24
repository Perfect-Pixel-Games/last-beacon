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
- Overall status: `Planned`
- Planning model: `gpt-5.5` (role fulfilled by Claude Sonnet 5)
- Preferred implementation model: `gpt-5.4` (role fulfilled by Claude Sonnet 5)
- Optional final review model: `gpt-5.5` (role fulfilled by Claude Sonnet 5)
- Current handoff state: `Implementation in progress (Claude Sonnet 5 fulfilling gpt-5.4)`
- Created: `2026-08-24`
- Last updated: `2026-08-24`

## Validation Rules
- Task complete only after required validation passes and documentation generation is recorded, unless a waiver is recorded.
- Phase complete only after required validation passes, documentation generation is recorded, required commits/pushes are complete, and required user confirmation is recorded.

## Repository State
- Root commit/push state: `Plan/tracker commit fb6cace pushed to origin/feature/ui-widget-feathers-alignment`
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
**Status:** Planned
**Goal:** Every interactive Last Beacon widget (buttons, tabs, radio options, sliders, dropdown toggles) is reachable via Tab/Shift+Tab, shows a visible focus outline, and Enter/Space activates the focused discrete-action widget — without touching `bevy_ui`'s owned `Interaction` component.

### Tasks
- [ ] Register `TabNavigationPlugin` in `game/src/lib.rs`, guarded with `is_plugin_added` against the `dev-tools` console's own `FeathersPlugins` add
  - Status: Planned
  - Repository: `root`
  - Notes: None
- [ ] Add `apply_last_beacon_ui_tab_groups_to_scene_roots` (+ tests: scene root gets `TabGroup`; nested scene-owned entity does not)
  - Status: Planned
  - Repository: `root`
  - Notes: None
- [ ] Add `LastBeaconUiFocusIndicator` component and `apply_last_beacon_ui_focusability` reactive system (+ tests per marker type)
  - Status: Planned
  - Repository: `root`
  - Notes: None
- [ ] Add `apply_last_beacon_ui_focus_outline` mutating `Outline.color` from `InputFocus`/`InputFocusVisible` (+ tests: outline follows focus; clears when focus moves)
  - Status: Planned
  - Repository: `root`
  - Notes: None
- [ ] Extract `apply_value_button_activation`, `apply_dropdown_toggle_activation`, `apply_tab_selection_activation` helpers from the existing mouse-driven systems
  - Status: Planned
  - Repository: `root`
  - Notes: None
- [ ] Add `activate_last_beacon_ui_focused_widget_on_keyboard_input` (+ tests: Enter/Space on a focused value-button/dropdown-toggle/tab produces the same state change as a mouse `Pressed`)
  - Status: Planned
  - Repository: `root`
  - Notes: None
- [ ] Register all new systems/types in `game/src/lib.rs`
  - Status: Planned
  - Repository: `root`
  - Notes: None
- [ ] Add "Keyboard & Focus" section to `docs/ui-widgets.md`
  - Status: Planned
  - Repository: `root`
  - Notes: None
- [ ] Run Phase 2 validation and commit
  - Status: Planned
  - Repository: `root`
  - Notes: None

### Validation
- Game validation: `Pending`
- Engine validation: `N/A`
- Documentation generation: Pending
- User confirmation: Not required yet

## Phase 3: Validation and PR readiness
**Status:** Planned
**Goal:** Full validation suite passes; manual keyboard-navigation QA recorded; branch pushed and ready for a pull request into `dev`.

### Tasks
- [ ] Run full validation (`scripts\validate.cmd` plus the focused `cargo fmt`/`clippy`/`test`/`doc` commands)
  - Status: Planned
  - Repository: `root`
  - Notes: None
- [ ] Manual QA: tab through Main Menu and UI Playground scenes; confirm outline, Enter/Space activation, and pause-overlay focus trapping
  - Status: Planned
  - Repository: `root`
  - Notes: None
- [ ] Push branch to `origin` and prepare pull request into `dev` (no local merge)
  - Status: Planned
  - Repository: `root`
  - Notes: `origin` is configured (`https://github.com/Perfect-Pixel-Games/last-beacon.git`), so push/PR is not `N/A`.

### Validation
- Game validation: `Pending`
- Engine validation: `N/A`
- Documentation generation: Pending
- User confirmation: Pending

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
