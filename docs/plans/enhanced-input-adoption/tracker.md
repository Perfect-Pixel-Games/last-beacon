# Enhanced Input Adoption Tracker

## Metadata
- Feature slug: `enhanced-input-adoption`
- Feature area: `multi-area`
- Primary area: `engine`
- Root branch: `feature/enhanced-input-adoption`
- Engine branch: `feature/enhanced-input-adoption`
- Root branch base verification: `Verified` (created from `dev`, up to date with `origin/dev`)
- Engine branch base verification: `Verified` (created from `origin/dev` at `01f0cfaaebfe8e193096994642ac2da1848ded9a`)
- Engine submodule pointer: `01f0cfaaebfe8e193096994642ac2da1848ded9a` (base; pending update once engine work lands)
- Overall status: `Planned`
- Planning model: `gpt-5.5` (role fulfilled by Claude Sonnet 5)
- Preferred implementation model: `gpt-5.4` (role fulfilled by Claude Sonnet 5)
- Optional final review model: `gpt-5.5` (role fulfilled by Claude Sonnet 5)
- Current handoff state: `Ready for implementation`
- Created: `2026-09-12`
- Last updated: `2026-09-12`

## Validation Rules
- Task complete only after required validation passes and documentation generation is recorded, unless a waiver is recorded.
- Phase complete only after required validation passes, documentation generation is recorded, required commits/pushes are complete, and required user confirmation is recorded.

## Repository State
- Root commit/push state: `Pending`
- Engine commit/push state: `Pending`
- Root submodule pointer update: `Pending`

## Phase 1: Dependency, plugin wiring, and context scaffolding (engine)
**Status:** Planned
**Goal:** `bevy_enhanced_input` compiles as a dependency of `foundation-runtime-library`, `EnhancedInputPlugin` is installed, and the four Foundation-owned context types exist (empty/unused actions are fine at this stage).

### Tasks
- [ ] Add `bevy_enhanced_input = "0.26.0"` to `engine/crates/foundation-runtime-library/Cargo.toml`; confirm it builds against the pinned `bevy = "0.19.0"`
  - Status: Planned
  - Repository: `engine`
  - Notes: None
- [ ] Add `EnhancedInputPlugin` to Foundation's plugin bundle
  - Status: Planned
  - Repository: `engine`
  - Notes: None
- [ ] Define `FoundationMenuInput`, `FoundationSplashScreenInput`, `FoundationConsoleInput`, `FoundationFreeFlyCameraInput` context marker components and register each via `add_input_context::<C>()`
  - Status: Planned
  - Repository: `engine`
  - Notes: None

### Validation
- Engine validation: `Pending`
- Game validation: `N/A`
- Documentation generation: Pending
- User confirmation: Not required yet

## Phase 2: Migrate existing Foundation input (menu, splash screen, console)
**Status:** Planned
**Goal:** Menu Escape handling, splash-screen skip, and console toggle/navigation/scroll all route through the new input contexts with identical behavior to before.

### Tasks
- [ ] Migrate `menu.rs`'s `open_pause_menus`/`close_on_escape` to `FoundationMenuInput`'s back/pause action; rewrite affected tests
  - Status: Planned
  - Repository: `engine`
  - Notes: None
- [ ] Migrate `splash_screen.rs`'s `splash_skip_requested` to `FoundationSplashScreenInput`'s skip action; rewrite affected test
  - Status: Planned
  - Repository: `engine`
  - Notes: None
- [ ] Migrate `console/mod.rs`'s toggle check, five control-key checks in `handle_console_keyboard_actions`, and mouse-wheel scroll read to `FoundationConsoleInput`'s actions
  - Status: Planned
  - Repository: `engine`
  - Notes: Leave literal command-text typing (`KeyboardInput`-based) untouched.

### Validation
- Engine validation: `Pending`
- Game validation: `N/A`
- Documentation generation: Pending
- User confirmation: Not required yet

## Phase 3: Free-fly camera (new, engine)
**Status:** Planned
**Goal:** A reusable `FoundationFreeFlyCameraInput` context (Move/Look actions) and camera movement system exist in `foundation-runtime-library`, ready for any Foundation game to use.

### Tasks
- [ ] Implement Move (WASD + up/down) and Look (mouse delta) actions and bindings
  - Status: Planned
  - Repository: `engine`
  - Notes: None
- [ ] Implement the camera movement system applying those actions to a `Transform`
  - Status: Planned
  - Repository: `engine`
  - Notes: Movement speed/sensitivity defaults are implementation-time choices; easy to tune later.

### Validation
- Engine validation: `Pending`
- Game validation: `N/A`
- Documentation generation: Pending
- User confirmation: Not required yet

## Phase 4: Migrate Last Beacon UI scroll input (game)
**Status:** Planned
**Goal:** `game/src/ui_widgets.rs`'s scrollbar-drag and wheel-scroll behavior is unchanged but routed through `LastBeaconUiScrollInput`'s actions instead of raw `ButtonInput`/`MouseWheel` reads.

### Tasks
- [ ] Define `LastBeaconUiScrollInput` context (`PointerPrimaryHeld`, `ScrollWheel`, `ScrollModifier` actions)
  - Status: Planned
  - Repository: `root`
  - Notes: Confirm whether `game/Cargo.toml` needs a direct `bevy_enhanced_input` dependency or can go through `foundation-runtime-library`'s re-exports.
- [ ] Migrate `drag_last_beacon_ui_text_box_scrollbars` and `scroll_last_beacon_ui_text_inputs` to the new actions
  - Status: Planned
  - Repository: `root`
  - Notes: Preserve exact current drag/scroll/shift-modifier behavior. Do not touch `KeyboardInput`-based text typing (`ui_widgets.rs:1494,1994`).

### Validation
- Game validation: `Pending`
- Engine validation: `N/A`
- Documentation generation: Pending
- User confirmation: Not required yet

## Phase 5: Cross-repo integration and closeout
**Status:** Planned
**Goal:** Engine work is committed/pushed, root submodule pointer is updated, full validation passes, manual QA confirms no behavior regressions, and `world-landscape-testbed`'s tracker is updated to note its free-fly camera phase is unblocked.

### Tasks
- [ ] Commit and push engine branch `feature/enhanced-input-adoption`; record exact engine commit hash
  - Status: Planned
  - Repository: `engine`
  - Notes: None
- [ ] Update root `engine` submodule pointer to the recorded engine commit hash; commit on root branch
  - Status: Planned
  - Repository: `root`
  - Notes: None
- [ ] Manual QA: pause-menu Escape, splash-screen skip, console toggle/navigation/scroll, UI text-box scrollbar drag/wheel-scroll all behave identically to before
  - Status: Planned
  - Repository: `root`
  - Notes: None
- [ ] Update `docs/plans/world-landscape-testbed/tracker.md` to note the free-fly camera phase is unblocked and should consume `FoundationFreeFlyCameraInput`
  - Status: Planned
  - Repository: `root`
  - Notes: None

### Validation
- Engine validation: `Pending`
- Game validation: `Pending`
- Documentation generation: Pending
- User confirmation: Pending

## Implementation / Review Handoff Notes
- None

## Postponed Work
- Rebindable keybinding storage/UI (explicitly deferred to a future feature).
- Migrating `game/src/ui_widgets.rs`'s `KeyboardInput`-based text-typing handling (not applicable to action-based input; not planned as future work either, just noted as intentionally excluded).

## Progress Log
- `2026-09-12`: Plan and tracker created; root branch `feature/enhanced-input-adoption` created from `dev`; engine branch `feature/enhanced-input-adoption` created from `origin/dev` at `01f0cfaaebfe8e193096994642ac2da1848ded9a`.
