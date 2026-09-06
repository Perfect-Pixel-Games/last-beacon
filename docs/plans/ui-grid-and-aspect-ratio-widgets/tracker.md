# UI Grid And Aspect Ratio Widgets Tracker

## Metadata
- Feature slug: `ui-grid-and-aspect-ratio-widgets`
- Feature area: `game`
- Primary area: `game`
- Root branch: `feature/ui-grid-and-aspect-ratio-widgets`
- Engine branch: `N/A`
- Root branch base verification: `Verified` (created via `git checkout dev` then `git checkout -b feature/ui-grid-and-aspect-ratio-widgets`; local `dev` matched `origin/dev` at `018b317` at branch-creation time)
- Engine branch base verification: `N/A`
- Engine submodule pointer: `N/A`
- Overall status: `Implemented, PR opened, awaiting user/reviewer confirmation`
- Planning model: `gpt-5.5` (role fulfilled by Claude Sonnet 5)
- Preferred implementation model: `gpt-5.4` (role fulfilled by Claude Sonnet 5)
- Optional final review model: `gpt-5.5` (role fulfilled by Claude Sonnet 5)
- Current handoff state: `Ready for gpt-5.5 sanity review (fulfilled by Claude Sonnet 5) or direct user review; all 5 phases implemented and pushed`
- Created: `2026-08-24`
- Last updated: `2026-08-24`

## Validation Rules
- Task complete only after required validation passes and documentation generation is recorded, unless a waiver is recorded.
- Phase complete only after required validation passes, documentation generation is recorded, required commits/pushes are complete, and required user confirmation is recorded.

## Repository State
- Root branch: `feature/ui-grid-and-aspect-ratio-widgets`
- Root branch base verification: `Verified from dev on 2026-08-24`
- Root commit/push state: `Plan/tracker commit 1bf0c00 pushed; implementation commits follow this update`
- Engine commit/push state: `N/A`
- Root submodule pointer update: `N/A`
- Note: `feature/ui-widget-feathers-alignment` (PR #16) is still open/unmerged as of this branch's creation; this feature intentionally branches from `dev` directly rather than stacking on it (see plan's Risks section).

## Phase 1: Uniform Grid
**Status:** Complete
**Goal:** `LastBeaconUiUniformGrid { column_count }` turns a `.bsn`-authored entity into a uniform-column CSS grid container using Bevy's native `Display::Grid` support.

### Tasks
- [x] Add `LastBeaconUiUniformGrid` component
  - Status: Complete
  - Repository: `root`
  - Notes: Revised during implementation from the original "zero Rust" plan -- Last Beacon's `.bsn` grammar cannot call `RepeatedGridTrack::flex(...)` directly (reflection-only construction, no function calls); see plan's Codebase Research correction.
- [x] Add `apply_last_beacon_ui_uniform_grid` reactive system (sets `Node.display`, `Node.grid_auto_flow`, `Node.grid_template_columns` from `column_count`, clamped to a minimum of `1`)
  - Status: Complete
  - Repository: `root`
  - Notes: None
- [x] Add tests: `column_count: 3` produces a matching `RepeatedGridTrack::flex(3, 1.0)` grid; `column_count: 0` still produces a 1-column grid instead of a degenerate empty one
  - Status: Complete
  - Repository: `root`
  - Notes: Required an explicit `::<Vec<RepeatedGridTrack>>` turbofish in the test assertions (the generic `flex<T: From<Self>>` constructor can't infer `T` from an `assert_eq!` comparison the way it can from a field assignment).
- [x] Add `game/assets/ui/widgets/common/uniform_grid.bsn` authoring `LastBeaconUiUniformGrid { column_count: 3 }`
  - Status: Complete
  - Repository: `root`
  - Notes: Contains its own 3 sample cells (matching every other widget asset's "ships with real example content" convention); the middle cell also carries `LastBeaconUiGridItem { column_span: 2 }`, so this one file demonstrates both Uniform Grid and Span Grid together.
- [x] Add "Uniform Grid" section to `docs/ui-widgets.md`
  - Status: Complete
  - Repository: `root`
  - Notes: None
- [x] Run Phase 1 validation and commit
  - Status: Complete
  - Repository: `root`
  - Notes: Folded into the combined validation pass recorded in Phase 5 (all three widgets' Rust landed together); see Phase 5 for the actual command output.

### Validation
- Game validation: See Phase 5 (combined run covers all phases)
- Engine validation: `N/A`
- Documentation generation: Recorded
- User confirmation: Not required for this phase (user approved "implement in full" covering all phases up front)

## Phase 2: Span Grid
**Status:** Complete
**Goal:** `LastBeaconUiGridItem { column_span, row_span }` lets a grid cell span multiple columns/rows via Bevy's native `GridPlacement`, without being able to panic on an authored `0`.

### Tasks
- [x] Add `LastBeaconUiGridItem` component
  - Status: Complete
  - Repository: `root`
  - Notes: None
- [x] Add `apply_last_beacon_ui_grid_item_span` reactive system (mutates the entity's existing `Node.grid_column`/`Node.grid_row`, clamping span to a minimum of `1`)
  - Status: Complete
  - Repository: `root`
  - Notes: None
- [x] Add tests: normal 2-column/3-row span produces matching `GridPlacement::span`; an authored `column_span: 0` still yields `GridPlacement::span(1)` instead of panicking
  - Status: Complete
  - Repository: `root`
  - Notes: None
- [x] Add "Span Grid" section to `docs/ui-widgets.md`
  - Status: Complete
  - Repository: `root`
  - Notes: None
- [x] Add combined "GRID" gallery card to `game/assets/scenes/ui_playground.bsn`
  - Status: Complete
  - Repository: `root`
  - Notes: The gallery card itself is just a `LastBeaconBsnWidget { asset_path: "ui/widgets/common/uniform_grid.bsn" }` reference, matching how every other widget is showcased -- the grid+span demo content lives inside `uniform_grid.bsn` itself (see Phase 1 notes), not composed inline in the playground scene.
- [x] Run Phase 2 validation and commit
  - Status: Complete
  - Repository: `root`
  - Notes: See Phase 5.

### Validation
- Game validation: See Phase 5
- Engine validation: `N/A`
- Documentation generation: Recorded
- User confirmation: Not required for this phase (user approved "implement in full" covering all phases up front)

## Phase 3: Aspect Ratio Container
**Status:** Complete
**Goal:** `LastBeaconUiAspectRatioBounds { min_aspect_ratio, max_aspect_ratio }` keeps a widget's own size within a clamped aspect-ratio band derived from its parent's available space, as a single widget covering both the fixed-ratio case (`min == max`) and the flexible-band case.

### Tasks
- [x] Add `LastBeaconUiAspectRatioBounds` component
  - Status: Complete
  - Repository: `root`
  - Notes: None
- [x] Add `apply_last_beacon_ui_aspect_ratio_bounds` system (reads parent `ComputedNode::content_box()` post-layout via `ChildOf`, swaps a backwards min/max defensively, clamps the ratio, computes a contain-fit size, writes `Node.width`/`height` guarded by an equality check)
  - Status: Complete
  - Repository: `root`
  - Notes: None
- [x] Add tests: fixed ratio (`min == max`) fits-and-centers correctly; a band that already contains the parent's ratio passes the parent size through unchanged; a parent ratio below the band clamps to `min`; a parent ratio above the band clamps to `max`; an authored-backwards `min > max` still produces the correctly-clamped result; the inert `Default` passes any parent size through unchanged
  - Status: Complete
  - Repository: `root`
  - Notes: The backwards-min/max test originally used `min == max` (which trivially passes regardless of ordering and doesn't actually exercise the swap); corrected to a genuinely-backwards case (`min: 2.0, max: 1.0` against a parent ratio that only fits inside the *correctly-ordered* band) before it would have been a false confidence test. Also confirmed `f32::clamp` panics unconditionally (not just in debug builds) if `min > max` is passed through unswapped, making the swap load-bearing, not just a nicety.
- [x] Add "Aspect Ratio Container" section to `docs/ui-widgets.md`
  - Status: Complete
  - Repository: `root`
  - Notes: None
- [x] Add "ASPECT RATIO" gallery card to `game/assets/scenes/ui_playground.bsn`
  - Status: Complete
  - Repository: `root`
  - Notes: Unlike every other gallery card (width-only), this card also gets an explicit `height: Val::Px(260.0)` plus a `flex_grow: 1.0` wrapper around the widget reference, so the aspect-ratio widget has a stable, non-content-derived 2D area to fit within -- satisfying the "parent must not size itself from this widget" requirement documented for the widget itself.
- [x] Run Phase 3 validation and commit
  - Status: Complete
  - Repository: `root`
  - Notes: See Phase 5.

### Validation
- Game validation: See Phase 5
- Engine validation: `N/A`
- Documentation generation: Recorded
- User confirmation: Not required for this phase (user approved "implement in full" covering all phases up front)

## Phase 4: Feathers-inspired plugin structure
**Status:** Complete
**Goal:** `LastBeaconUiUniformGrid`, `LastBeaconUiGridItem`, and `LastBeaconUiAspectRatioBounds` are registered through one dedicated `LastBeaconUiLayoutWidgetsPlugin` (mirroring Feathers' one-plugin-per-widget-family composition), rather than inline calls in the monolithic `LastBeaconPlugin` builder chain.

### Tasks
- [x] Add `LastBeaconUiLayoutWidgetsPlugin` to `game/src/ui_widgets.rs`, registering all three new components' types and all three new systems (uniform-grid + grid-item-span in `Update`; aspect-ratio bounds in `PostUpdate` `.after(bevy::ui::UiSystems::PostLayout)`)
  - Status: Complete
  - Repository: `root`
  - Notes: None
- [x] Add `.add_plugins(ui_widgets::LastBeaconUiLayoutWidgetsPlugin)` to `LastBeaconPlugin::build` in `game/src/lib.rs`
  - Status: Complete
  - Repository: `root`
  - Notes: Does not change how any existing widget is registered.
- [x] Run Phase 4 validation and commit
  - Status: Complete
  - Repository: `root`
  - Notes: See Phase 5.

### Validation
- Game validation: See Phase 5
- Engine validation: `N/A`
- Documentation generation: Recorded
- User confirmation: Not required for this phase (user approved "implement in full" covering all phases up front)

## Phase 5: Validation and PR readiness
**Status:** Complete (manual visual QA scope limited -- see notes, same limitation pattern as PR #16)
**Goal:** Full validation suite passes; showcase scene smoke-tested; branch pushed and ready for a pull request into `dev`, noted as independent of the still-open PR #16.

### Tasks
- [x] Run full validation (`scripts\validate.cmd` plus the focused `cargo fmt`/`clippy`/`test`/`doc` commands)
  - Status: Complete
  - Repository: `root`
  - Notes: `scripts\validate.cmd` (fmt --check, clippy -D warnings, test --all-features, build --all-features, doc --no-deps) passed end to end with exit code 0. 33 tests pass (29 unit + 4 integration), including 9 new unit tests and the existing `bsn_asset_flow` integration test extended to load both new `.bsn` files as `ScenePatch` assets directly (both parsed successfully, confirming the hand-authored BSN text is valid, not just the Rust side).
- [x] Background-smoke-launch the game (`cargo run --all-features`) to confirm it starts without panicking with the new plugin/components registered
  - Status: Complete with a recorded limitation
  - Repository: `root`
  - Notes: Same limitation as PR #16: no GUI automation/screenshot tool available in this environment for a native Win32/wgpu window, so the visual review of the two new gallery cards (do they render correctly, does the aspect-ratio panel visibly hold its band, does the grid visibly show the spanning cell) could not be performed here. What WAS verified: the game launched with default features, stayed running/responsive for several seconds with no panic (confirming `LastBeaconUiLayoutWidgetsPlugin` registers cleanly and the new reactive systems don't crash on startup), then was terminated. The `bsn_asset_flow` integration test (see above) is the strongest automated signal that the new assets are well-formed; the user should still look at the "GRID" and "ASPECT RATIO" gallery cards in the running UI Playground scene before merging.
- [x] Push branch to `origin` and prepare pull request into `dev` (no local merge)
  - Status: Complete
  - Repository: `root`
  - Notes: `origin` is configured; every commit was pushed immediately after being made. Pull request prepared via `gh pr create` targeting `dev`, noted as independent of PR #16; not merged.

### Validation
- Game validation: `scripts\validate.cmd` passed (exit code 0)
- Engine validation: `N/A`
- Documentation generation: Recorded (rustdoc generated successfully; `docs/ui-widgets.md` has three new sections)
- User confirmation: Pending -- user should look at the two new gallery cards in the running game before merging

## Implementation / Review Handoff Notes
- The single most important discovery from this implementation: Last Beacon's `.bsn` files are parsed by a project-owned reflection-based grammar (`dynamic_bsn_grammar.lalrpop`/`dynamic_bsn.rs`), not Bevy's `bsn!` macro, and that grammar cannot call arbitrary associated functions (only tuple-struct/enum-variant construction via `TypeRegistry` reflection). Any future widget idea that seems to need a raw Bevy constructor function (not a plain enum variant or public-field struct) authored directly in `.bsn` text will hit the same wall Uniform Grid did -- plan for a small Rust component + system from the start rather than assuming a "zero Rust, pure asset" design will work.

## Postponed Work
- The rest of the earlier widget-ideas conversation (modal/confirm dialog, toast queue, progress/meter bar, tooltip, accordion, drag-and-drop cargo grid, virtualized list, carousel, badge/chip, breadcrumb, search/filter bar, minimap, tree view, keybind row, virtual joystick) -- not part of this plan; a separate future conversation per the user.
- Context menu -- explicitly ruled out by the user as unnecessary for a game.
- A responsive `GridTrackRepetition::AutoFill`-based uniform grid variant -- not requested; noted as a possible future enhancement to the same `.bsn` preset in the plan's Alternatives Considered.

## Progress Log
- `2026-08-24`: User approved moving the Uniform Grid / Span Grid / Aspect Ratio Container follow-up into a plan ("ready"). Confirmed `feature/ui-widget-feathers-alignment` (PR #16) is still open/unmerged; created this new feature branch from current `origin/dev` (`018b317`) rather than stacking on the unmerged branch, since these widgets have no functional dependency on it. Read the vendored `bevy_ui` 0.19.0 grid/grid-placement/computed-node source directly to confirm Uniform Grid and Span Grid are almost entirely native-Bevy-supported, and that `ComputedNode` is directly constructible in tests without a real layout pass. Wrote `plan.md` and this tracker per `.pi/skills/feature-plan-docs/SKILL.md`. Stopping here for user review per that skill's mandatory planning checkpoint.
- `2026-08-24`: Before approval, user added two requirements: (1) showcase examples in `game/assets/scenes/ui_playground.bsn`, (2) these widgets should be "designed in a similar way" to Bevy Feathers. Read `ui_playground.bsn` in full to confirm its existing "gallery card" showcase pattern and added two new gallery cards (combined grid+span demo, aspect-ratio demo) to Phases 2/3. For the Feathers request, adopted a stated interpretation (flagged as an open question for the user to correct, not treated as blocking): group the two new components' registration into one dedicated `LastBeaconUiLayoutWidgetsPlugin`, mirroring Feathers' one-plugin-per-widget-family composition, plus Feathers-style docs and an inert `Default` for the aspect-ratio bounds -- explicitly not adopting Feathers' `SceneComponent`/`bsn!`-macro construction or `bevy_ui_widgets` primitives, consistent with that already being scoped out of PR #16. Added this as new Phase 4, renumbering the former Phase 4 (validation) to Phase 5. Updated both plan.md and this tracker; still awaiting the user's approval to begin implementation.
- `2026-08-24`: User confirmed the open question: these widgets should be authored via `.bsn` "as much as possible, the same as the other components we have made" -- no Rust-side `Props`/builder spawn-function API. This matches the plan as already written (no changes needed to the widget design itself); closed the Open Questions entry. Still awaiting explicit approval to begin implementation.
- `2026-08-24`: User replied "implement in full". Committed and pushed plan/tracker docs (`1bf0c00`). Before writing `uniform_grid.bsn`, read this project's actual `.bsn` parser (`dynamic_bsn_grammar.lalrpop`/`dynamic_bsn.rs`) to confirm the exact expression grammar, and discovered `RepeatedGridTrack::flex(...)` (an associated function) cannot be authored directly in `.bsn` text -- only reflection-based tuple-struct/enum-variant construction is supported. Revised Uniform Grid to use a small `LastBeaconUiUniformGrid { column_count }` component + Rust system instead, updated plan.md and this tracker accordingly, and proceeded with implementation.
- `2026-08-24`: Implemented all three widgets, the `LastBeaconUiLayoutWidgetsPlugin`, 9 new unit tests, the two new `.bsn` assets (each containing their own sample content, matching every other widget's convention), the two new `ui_playground.bsn` gallery cards, three new `docs/ui-widgets.md` sections, and extended the existing `bsn_asset_flow` integration test to load the two new `.bsn` files directly (confirming they parse). Fixed a weak test along the way (the backwards-min/max test originally couldn't have failed even with a broken swap). Ran full `scripts\validate.cmd` (pass) and a background smoke launch of the game (starts and stays running, no panic). Opened a pull request into `dev` via `gh pr create` (not merged), noted as independent of PR #16.
- `2026-08-24`: User feedback: the standalone "ASPECT RATIO" gallery card (a small arbitrary-sized demo box) didn't demonstrate the widget's actual intended use well. Restructured `ui_playground.bsn` so the whole scene's content (header + every gallery, including Grid) is now wrapped in a new `#UiPlaygroundAspectRatioFrame` node carrying `LastBeaconUiAspectRatioBounds { min_aspect_ratio: 1.7777778, max_aspect_ratio: 1.7777778 }` (fixed 16:9), centered in the actual window-pinned root -- so the entire UI Playground now demonstrates the widget for real (letterboxed to 16:9 on any window shape) rather than a token box. Removed the now-redundant standalone gallery card; `aspect_ratio_container.bsn` remains available as a smaller standalone example. Re-verified: `bsn_asset_flow` integration test still passes (confirms the restructured scene still parses), full `scripts\validate.cmd` passes, background smoke launch still starts cleanly. Pushed as a follow-up commit to the same PR #17 (not yet visually confirmed by the user -- same outstanding limitation as before).
- `2026-08-24`: Two more pieces of user feedback: (1) the 16:9 frame had no visible border/fill, so its edges weren't obvious against the rest of the scene; (2) the grid showcase only ever demonstrated Span Grid (every cell-demo included a spanning cell) with no plain Uniform Grid example to compare against. Fixed both: gave `#UiPlaygroundAspectRatioFrame` an amber border and a slightly lighter background than the window behind it, so the 16:9 boundary reads clearly. Split the grid demo into two separate `.bsn` assets and two separate gallery cards -- `uniform_grid.bsn` is now a plain 6-cell grid with no spanning, and a new `grid_item_span.bsn` (copied from the original combined demo) keeps the spanning-cell example. Added `grid_item_span.bsn` to the `bsn_asset_flow` integration test's coverage list and to `docs/ui-widgets.md`'s Span Grid section. Re-ran the full validation suite (pass) and a background smoke launch (starts cleanly, no panic). Pushed as another follow-up commit to PR #17.
- `2026-08-31`: User reported a manual-QA bug on this branch: the Number Field widget's center value didn't display until the `-`/`+` buttons were clicked. Root-caused via `superpowers:systematic-debugging`: six `refresh_last_beacon_ui_*` systems (`refresh_last_beacon_ui_value_text`, `refresh_last_beacon_ui_slider_fills`, `refresh_last_beacon_ui_radio_icons`, `refresh_last_beacon_ui_dropdown_icons`, `refresh_last_beacon_ui_dropdown_panels`, `refresh_last_beacon_ui_tab_panels`) all gated their entire sync loop behind a shared resource's `Res::is_changed()`, which only reflects mutations made in the exact current frame -- not widget entities that spawn on a *later* frame (async nested-BSN-widget loading, or a scene reopened after the value was already set elsewhere in the session). Such entities kept showing their `.bsn`-authored placeholder until literally any other interaction happened to flip the resource's changed-flag, at which point the next refresh caught every widget up at once -- exactly matching "only after clicking the buttons does the value display." Confirmed via Bevy 0.19 source (`bevy_text`/`bevy_ui`'s `EditableText`/change-detection internals), not guesswork. Fixed all six by also syncing whenever a newly-spawned instance of the relevant marker component is present (`Added<T>` query, widened early-return guard), matching the existing code's small-system style rather than introducing a new abstraction. Verified with a targeted regression test: temporarily reverted just the two `Number Field`/`Slider` guards and confirmed the two new tests (`value_text_spawned_after_the_stored_value_already_exists_still_shows_it`, `slider_fill_spawned_after_the_stored_value_already_exists_still_shows_it`) fail without the fix and pass with it. Full `scripts\validate.cmd` passes (fmt, clippy `-D warnings`, all 45 tests, build, doc); one font-loading test flaked once under concurrent build load and passed cleanly on rerun in isolation and in a second full validate run, unrelated to this change. Not yet committed -- awaiting user go-ahead.
- `2026-08-31`: User reported two more manual-QA bugs from the same session: the `-`/`+` Number Field buttons sometimes don't react to a click, and clicking into a text box (editable field) sometimes makes its content disappear with no visible caret or editability afterward. Investigated extensively but could not reach a confirmed root cause. Ruled out (with evidence, not just plausibility): (1) `EditableText` fully replacing `Text`-based rendering once present -- confirmed via `bevy_text` 0.19 source, but the refresh systems already handle both; (2) a genuine duplicate-authoring bug found in `number_field.bsn` -- `#NumberValueText` carried its own redundant `LastBeaconUiTextInput`, which `text_field.bsn`/`text_box.bsn` don't do, causing `initialize_last_beacon_ui_text_inputs` to run twice and double-insert `EditableText` on the same entity at spawn -- fixed (removed the redundant component) since it's real and safe, though it does not explain the reported symptoms (the double-insert is idempotent at spawn); (3) a `FocusPolicy::Block`-by-default occlusion theory (decorative label `Text` children silently swallowing clicks meant for their parent `Button`, since `Text` requires `Node`) -- built and ran an actual regression test for this and it was **disproven**: `bevy_ui`'s own `Node` component already requires `FocusPolicy` defaulting to `Pass` (only `Button` itself overrides to `Block`), so plain label text was never blocking anything; the disproving test caught this before the fix shipped, so it was fully reverted (no trace left in the diff). Confirmed as a real, still-unresolved architectural concern (not proven to cause these symptoms, but a legitimate suspect worth flagging): Last Beacon's own hand-rolled `focus_last_beacon_ui_text_inputs`/`queue_text_input_click_placement`/`initialize_last_beacon_ui_text_inputs` (`Interaction`-polling based) run simultaneously alongside `bevy_ui_widgets`' native `EditableTextInputPlugin` (`on_pointer_press`/`on_focused_keyboard_input` observers, pulled in transitively via `FeathersPlugins` from `FoundationConsolePlugin`, which is always active regardless of whether the debug console is open) -- two independent, differently-triggered (poll-vs-observer) systems both reacting to the same clicks on the same `EditableText` widgets. No GUI automation is available in this environment for this native Win32/wgpu window (attempted a screenshot via PowerShell + Win32 API; it captured the IDE/terminal instead of the game's actual render window, confirming the same limitation noted for PR #16/#17's earlier manual-QA steps), so this could not be visually reproduced or confirmed. Left open pending more specific repro details from the user or explicit direction to attempt a bigger architectural change (e.g. consolidating onto one click-handling path) without full certainty it addresses the report.
