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
- Overall status: `Planned`
- Planning model: `gpt-5.5` (role fulfilled by Claude Sonnet 5)
- Preferred implementation model: `gpt-5.4` (role fulfilled by Claude Sonnet 5)
- Optional final review model: `gpt-5.5` (role fulfilled by Claude Sonnet 5)
- Current handoff state: `Ready for implementation pending user approval to proceed`
- Created: `2026-08-24`
- Last updated: `2026-08-24`

## Validation Rules
- Task complete only after required validation passes and documentation generation is recorded, unless a waiver is recorded.
- Phase complete only after required validation passes, documentation generation is recorded, required commits/pushes are complete, and required user confirmation is recorded.

## Repository State
- Root branch: `feature/ui-grid-and-aspect-ratio-widgets`
- Root branch base verification: `Verified from dev on 2026-08-24`
- Root commit/push state: `Pending` (no commits made yet — plan/tracker creation itself is the first commit due)
- Engine commit/push state: `N/A`
- Root submodule pointer update: `N/A`
- Note: `feature/ui-widget-feathers-alignment` (PR #16) is still open/unmerged as of this branch's creation; this feature intentionally branches from `dev` directly rather than stacking on it (see plan's Risks section).

## Phase 1: Uniform Grid (docs + preset asset, no Rust)
**Status:** Planned
**Goal:** A documented, reusable grid-container `.bsn` preset exists, using Bevy's native `Display::Grid` support with zero new Rust code.

### Tasks
- [ ] Add `game/assets/ui/widgets/common/uniform_grid.bsn` (three equal-width `RepeatedGridTrack::flex` columns as a starting default, documented as author-adjustable)
  - Status: Planned
  - Repository: `root`
  - Notes: None
- [ ] Add "Uniform Grid" section to `docs/ui-widgets.md`
  - Status: Planned
  - Repository: `root`
  - Notes: None
- [ ] Run Phase 1 validation and commit
  - Status: Planned
  - Repository: `root`
  - Notes: No Rust changes in this phase, so validation is `cargo fmt`/`clippy`/`test`/`doc` as a no-op regression check plus a manual read of the new `.bsn` asset for correctness.

### Validation
- Game validation: `Pending`
- Engine validation: `N/A`
- Documentation generation: Pending
- User confirmation: Not required yet

## Phase 2: Span Grid
**Status:** Planned
**Goal:** `LastBeaconUiGridItem { column_span, row_span }` lets a grid cell span multiple columns/rows via Bevy's native `GridPlacement`, without being able to panic on an authored `0`.

### Tasks
- [ ] Add `LastBeaconUiGridItem` component
  - Status: Planned
  - Repository: `root`
  - Notes: None
- [ ] Add `apply_last_beacon_ui_grid_item_span` reactive system (mutates the entity's existing `Node.grid_column`/`Node.grid_row`, clamping span to a minimum of `1`)
  - Status: Planned
  - Repository: `root`
  - Notes: None
- [ ] Add tests: normal 2-column/3-row span produces matching `GridPlacement::span`; an authored `column_span: 0` still yields `GridPlacement::span(1)` instead of panicking
  - Status: Planned
  - Repository: `root`
  - Notes: None
- [ ] Add "Span Grid" section to `docs/ui-widgets.md`
  - Status: Planned
  - Repository: `root`
  - Notes: None
- [ ] Add combined "GRID" gallery card to `game/assets/scenes/ui_playground.bsn` (uniform grid + one spanning cell, demonstrating both Phase 1 and Phase 2 together)
  - Status: Planned
  - Repository: `root`
  - Notes: Matches the existing `#XWidgetGallery` card pattern already used for every other showcased widget.
- [ ] Run Phase 2 validation and commit
  - Status: Planned
  - Repository: `root`
  - Notes: None

### Validation
- Game validation: `Pending`
- Engine validation: `N/A`
- Documentation generation: Pending
- User confirmation: Not required yet

## Phase 3: Aspect Ratio Container
**Status:** Planned
**Goal:** `LastBeaconUiAspectRatioBounds { min_aspect_ratio, max_aspect_ratio }` keeps a widget's own size within a clamped aspect-ratio band derived from its parent's available space, as a single widget covering both the fixed-ratio case (`min == max`) and the flexible-band case.

### Tasks
- [ ] Add `LastBeaconUiAspectRatioBounds` component
  - Status: Planned
  - Repository: `root`
  - Notes: None
- [ ] Add `apply_last_beacon_ui_aspect_ratio_bounds` system (reads parent `ComputedNode::content_box()` post-layout via `ChildOf`, swaps a backwards min/max defensively, clamps the ratio, computes a contain-fit size, writes `Node.width`/`height` guarded by an equality check)
  - Status: Planned
  - Repository: `root`
  - Notes: None
- [ ] Add tests: fixed ratio (`min == max`) fits-and-centers correctly; a band that already contains the parent's ratio passes the parent size through unchanged; a parent ratio below the band clamps to `min`; a parent ratio above the band clamps to `max`; an authored-backwards `min > max` still produces the correctly-clamped result; the inert `Default` passes any parent size through unchanged
  - Status: Planned
  - Repository: `root`
  - Notes: None
- [ ] Add "Aspect Ratio Container" section to `docs/ui-widgets.md` (including the "parent must center this and must not size itself from it" authoring requirement)
  - Status: Planned
  - Repository: `root`
  - Notes: None
- [ ] Add "ASPECT RATIO" gallery card to `game/assets/scenes/ui_playground.bsn` (fixed-size panel containing a `LastBeaconUiAspectRatioBounds` child with a visible fill)
  - Status: Planned
  - Repository: `root`
  - Notes: None
- [ ] Run Phase 3 validation and commit
  - Status: Planned
  - Repository: `root`
  - Notes: None

### Validation
- Game validation: `Pending`
- Engine validation: `N/A`
- Documentation generation: Pending
- User confirmation: Not required yet

## Phase 4: Feathers-inspired plugin structure
**Status:** Planned
**Goal:** `LastBeaconUiGridItem` and `LastBeaconUiAspectRatioBounds` are registered through one dedicated `LastBeaconUiLayoutWidgetsPlugin` (mirroring Feathers' one-plugin-per-widget-family composition), rather than inline calls in the monolithic `LastBeaconPlugin` builder chain.

### Tasks
- [ ] Add `LastBeaconUiLayoutWidgetsPlugin` to `game/src/ui_widgets.rs`, registering both new components' types and both new systems (grid-item span in `Update`; aspect-ratio bounds in `PostUpdate` `.after(bevy::ui::UiSystems::PostLayout)`)
  - Status: Planned
  - Repository: `root`
  - Notes: None
- [ ] Add `.add_plugins(ui_widgets::LastBeaconUiLayoutWidgetsPlugin)` to `LastBeaconPlugin::build` in `game/src/lib.rs`
  - Status: Planned
  - Repository: `root`
  - Notes: Does not change how any existing widget is registered.
- [ ] Run Phase 4 validation and commit
  - Status: Planned
  - Repository: `root`
  - Notes: None

### Validation
- Game validation: `Pending`
- Engine validation: `N/A`
- Documentation generation: Pending
- User confirmation: Not required yet

## Phase 5: Validation and PR readiness
**Status:** Planned
**Goal:** Full validation suite passes; showcase scene smoke-tested; branch pushed and ready for a pull request into `dev`, noted as independent of the still-open PR #16.

### Tasks
- [ ] Run full validation (`scripts\validate.cmd` plus the focused `cargo fmt`/`clippy`/`test`/`doc` commands)
  - Status: Planned
  - Repository: `root`
  - Notes: None
- [ ] Background-smoke-launch the game (`cargo run --all-features`) to confirm the UI Playground scene (now including the two new gallery cards) opens without panicking
  - Status: Planned
  - Repository: `root`
  - Notes: Same recorded limitation as PR #16: no GUI automation tool available in this environment, so this confirms "doesn't crash," not full visual review. The user should look at the two new gallery cards before merging.
- [ ] Push branch to `origin` and prepare pull request into `dev` (no local merge)
  - Status: Planned
  - Repository: `root`
  - Notes: `origin` is configured, so push/PR is not `N/A`. PR description should note independence from PR #16.

### Validation
- Game validation: `Pending`
- Engine validation: `N/A`
- Documentation generation: Pending
- User confirmation: Pending

## Implementation / Review Handoff Notes
- None yet — implementation has not started. Awaiting user confirmation to proceed past the planning checkpoint.

## Postponed Work
- The rest of the earlier widget-ideas conversation (modal/confirm dialog, toast queue, progress/meter bar, tooltip, accordion, drag-and-drop cargo grid, virtualized list, carousel, badge/chip, breadcrumb, search/filter bar, minimap, tree view, keybind row, virtual joystick) — not part of this plan; a separate future conversation per the user.
- Context menu — explicitly ruled out by the user as unnecessary for a game.
- A responsive `GridTrackRepetition::AutoFill`-based uniform grid variant — not requested; noted as a possible future enhancement to the same `.bsn` preset in the plan's Alternatives Considered.
- A thin `LastBeaconUiUniformGrid` Rust marker component — Uniform Grid intentionally ships as docs + a `.bsn` preset only; add a marker component later only if a concrete need for it (tooling/reflection) shows up.

## Progress Log
- `2026-08-24`: User approved moving the Uniform Grid / Span Grid / Aspect Ratio Container follow-up into a plan ("ready"). Confirmed `feature/ui-widget-feathers-alignment` (PR #16) is still open/unmerged; created this new feature branch from current `origin/dev` (`018b317`) rather than stacking on the unmerged branch, since these widgets have no functional dependency on it. Read the vendored `bevy_ui` 0.19.0 grid/grid-placement/computed-node source directly to confirm Uniform Grid and Span Grid are almost entirely native-Bevy-supported, and that `ComputedNode` is directly constructible in tests without a real layout pass. Wrote `plan.md` and this tracker per `.pi/skills/feature-plan-docs/SKILL.md`. Stopping here for user review per that skill's mandatory planning checkpoint.
- `2026-08-24`: Before approval, user added two requirements: (1) showcase examples in `game/assets/scenes/ui_playground.bsn`, (2) these widgets should be "designed in a similar way" to Bevy Feathers. Read `ui_playground.bsn` in full to confirm its existing "gallery card" showcase pattern and added two new gallery cards (combined grid+span demo, aspect-ratio demo) to Phases 2/3. For the Feathers request, adopted a stated interpretation (flagged as an open question for the user to correct, not treated as blocking): group the two new components' registration into one dedicated `LastBeaconUiLayoutWidgetsPlugin`, mirroring Feathers' one-plugin-per-widget-family composition, plus Feathers-style docs and an inert `Default` for the aspect-ratio bounds -- explicitly not adopting Feathers' `SceneComponent`/`bsn!`-macro construction or `bevy_ui_widgets` primitives, consistent with that already being scoped out of PR #16. Added this as new Phase 4, renumbering the former Phase 4 (validation) to Phase 5. Updated both plan.md and this tracker; still awaiting the user's approval to begin implementation.
- `2026-08-24`: User confirmed the open question: these widgets should be authored via `.bsn` "as much as possible, the same as the other components we have made" -- no Rust-side `Props`/builder spawn-function API. This matches the plan as already written (no changes needed to the widget design itself); closed the Open Questions entry. Still awaiting explicit approval to begin implementation.
