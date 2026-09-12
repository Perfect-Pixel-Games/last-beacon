# Enhanced Input Adoption Tracker

## Metadata
- Feature slug: `enhanced-input-adoption`
- Feature area: `multi-area`
- Primary area: `engine`
- Root branch: `feature/enhanced-input-adoption`
- Engine branch: `feature/enhanced-input-adoption`
- Root branch base verification: `Verified` (created from `dev`, up to date with `origin/dev`)
- Engine branch base verification: `Verified` (created from `origin/dev` at `01f0cfaaebfe8e193096994642ac2da1848ded9a`)
- Engine submodule pointer: `58da948199f35c8662796a6609e0804a7f86bfb8`
- Overall status: `Implemented, awaiting user manual QA and merge decision`
- Planning model: `gpt-5.5` (role fulfilled by Claude Sonnet 5)
- Preferred implementation model: `gpt-5.4` (role fulfilled by Claude Sonnet 5)
- Optional final review model: `gpt-5.5` (role fulfilled by Claude Sonnet 5)
- Current handoff state: `Ready for gpt-5.5 sanity review, or user acceptance`
- Created: `2026-09-12`
- Last updated: `2026-09-12`

## Validation Rules
- Task complete only after required validation passes and documentation generation is recorded, unless a waiver is recorded.
- Phase complete only after required validation passes, documentation generation is recorded, required commits/pushes are complete, and required user confirmation is recorded.

## Repository State
- Root commit/push state: `Pending` (this commit)
- Engine commit/push state: `Committed and pushed` -- `58da948199f35c8662796a6609e0804a7f86bfb8` on `feature/enhanced-input-adoption`
- Root submodule pointer update: `Pending` (this commit)

## Phase 1: Dependency, plugin wiring, and context scaffolding (engine)
**Status:** Complete
**Goal:** `bevy_enhanced_input` compiles as a dependency of `foundation-runtime-library`, `EnhancedInputPlugin` is installed, and Foundation-owned context types exist.

### Tasks
- [x] Add `bevy_enhanced_input = "0.26.0"` to `engine/crates/foundation-runtime-library/Cargo.toml`; confirm it builds against the pinned `bevy = "0.19.0"`
  - Status: Complete
  - Repository: `engine`
  - Notes: Confirmed via `cargo check` that `bevy_enhanced_input` 0.26.0's `bevy ^0.19.0` requirement resolves cleanly.
- [x] Add `EnhancedInputPlugin` to Foundation's plugin bundle
  - Status: Complete
  - Repository: `engine`
  - Notes: Added a `pub(crate) add_enhanced_input_plugin_if_missing(app)` guard helper in `lib.rs` (mirrors the existing `is_plugin_added::<TabNavigationPlugin>()` pattern in `game/src/lib.rs`) so `FoundationPlugin` and each input-owning sub-plugin (menu, splash screen, console, free-fly camera) can all safely call it without double-adding the plugin.
- [x] Define context marker components and register each via `add_input_context::<C>()`
  - Status: Complete
  - Repository: `engine`
  - Notes: Named `FoundationMenuInput`, `FoundationSplashScreenInput`, `FoundationConsoleControlsInput` (see Phase 2 notes for the naming deviation), and `FoundationFreeFlyCameraInput` (Phase 3). All four defined and registered.

### Validation
- Engine validation: `Passed` -- `cargo check`/`cargo build -p foundation-runtime-library --all-features`
- Game validation: `N/A`
- Documentation generation: `N/A at this phase (covered in Phase 5)`
- User confirmation: `Not required for this phase`

## Phase 2: Migrate existing Foundation input (menu, splash screen, console)
**Status:** Complete
**Goal:** Menu Escape handling, splash-screen skip, and console toggle/navigation/scroll all route through the new input contexts with identical behavior to before.

### Tasks
- [x] Migrate `menu.rs`'s `open_pause_menus`/`close_on_escape` to `FoundationMenuInput`'s `FoundationMenuBack` action; rewrite affected tests
  - Status: Complete
  - Repository: `engine`
  - Notes: Both systems now take `Single<&ActionEvents, With<Action<FoundationMenuBack>>>` and check `.contains(ActionEvents::START)` (the crate deprecated `ActionEvents::STARTED` in favor of `START` mid-implementation; used the non-deprecated name). No test rewrites were needed beyond adding `app.finish(); app.cleanup();` (see cross-cutting discovery below) -- the existing `ButtonInput<KeyCode>::press(KeyCode::Escape)` simulation still works unchanged since `bevy_enhanced_input` reads the same underlying resource.
- [x] Migrate `splash_screen.rs`'s `splash_skip_requested` to `FoundationSplashScreenInput`'s `FoundationSplashScreenSkip` action; rewrite affected test
  - Status: Complete
  - Repository: `engine`
  - Notes: The old test called `splash_skip_requested` as a pure function on a bare `ButtonInput<KeyCode>`; rewritten as a small integration test that spawns a real `App` with the plugin, presses Escape, and asserts on the resulting `ActionEvents`. This pulled in `advance_splash_screens`'s full system (via `Update`), which needed `app.add_message::<SceneCommand>()` added manually since this test doesn't add the full `FoundationSceneStackPlugin`.
- [x] Migrate `console/mod.rs`'s toggle check, five control-key checks in `handle_console_keyboard_actions`, and mouse-wheel scroll read to input-context actions
  - Status: Complete
  - Repository: `engine`
  - Notes: **Naming deviation from the plan**: the context is `FoundationConsoleControlsInput`, not `FoundationConsoleInput` -- the latter name was already taken by a pre-existing marker component for the console's editable text-input entity. Six actions defined: `FoundationConsoleToggle` (Backquote), `FoundationConsoleClose` (Escape), `FoundationConsoleAutocomplete` (Tab), `FoundationConsoleHistoryPrevious`/`HistoryNext` (arrows), `FoundationConsoleSubmit` (Enter), `FoundationConsoleScroll` (Vec2, mouse wheel). Literal command-text typing (`Res<ButtonInput<Key>>` for control keys was migrated; the separate `KeyboardInput`-based character-typing mechanism elsewhere was correctly left untouched, confirming the plan's scope boundary). No existing test exercised these systems with simulated input, so no test rewrites were needed here.

### Validation
- Engine validation: `Passed` -- `cargo test -p foundation-runtime-library --lib` (156 passed, 0 failed), `cargo clippy -p foundation-runtime-library --all-targets --all-features -- -D warnings` (clean)
- Game validation: `N/A`
- Documentation generation: `Passed` -- `engine/scripts/doc-project.cmd`
- User confirmation: `Not required for this phase`

## Phase 3: Free-fly camera (new, engine)
**Status:** Complete
**Goal:** A reusable `FoundationFreeFlyCameraInput` context (Move/Vertical/Look actions) and camera movement system exist in `foundation-runtime-library`, ready for any Foundation game to use.

### Tasks
- [x] Implement Move (WASD, via the crate's `Cardinal::wasd_keys()` preset), Vertical (Space/Shift), and Look (mouse delta) actions and bindings
  - Status: Complete
  - Repository: `engine`
  - Notes: Split "up/down" into its own `FoundationFreeFlyCameraVertical` (f32) action rather than folding it into a Vec3 Move action, since Space/Shift map more naturally to a signed scalar than a third movement-vector axis.
- [x] Implement the camera movement system applying those actions to `Transform`
  - Status: Complete
  - Repository: `engine`
  - Notes: Yaw/pitch tracked in a new `FoundationFreeFlyCameraOrientation` component (quaternions can't be incrementally nudged directly). Default `move_speed: 10.0` units/sec, `look_sensitivity: 0.002` rad/pixel -- untuned defaults, easy to adjust later since no gameplay depends on them yet.
  - **Key implementation bug found and fixed during testing**: `Action<A>` components live on a *separate* entity related to the context entity via the `Actions<C>`/`ActionOf<C>` relationship (confirmed by reading the crate's own polling-API example) -- they are not co-located components on the context entity itself. The first implementation queried for `Action<A>` directly alongside `Transform`/`FoundationFreeFlyCameraInput` on one entity and silently matched zero entities (system body never ran, no panic, no test failure until assertions caught the resulting zero movement). Fixed by querying `&Actions<FoundationFreeFlyCameraInput>` on the context entity and using `.iter_many()` against separate `Query<&Action<A>>` for each action type, matching the crate's documented pattern.
  - Wrote two new tests (`pressing_w_moves_camera_forward`, `mouse_motion_rotates_camera_yaw`) that caught the above bug. The movement test asserts against the *actual* elapsed `Time::delta_secs()` read after `app.update()` rather than a hardcoded duration, since `TimePlugin` (part of `MinimalPlugins`) recomputes `Time` from the real wall clock every frame and overwrites any `Time::advance_by()` call made before `update()`.

### Validation
- Engine validation: `Passed` -- `cargo test -p foundation-runtime-library --lib free_fly_camera::` (2 passed), full crate suite (156 passed), clippy clean
- Game validation: `N/A`
- Documentation generation: `Passed` -- `engine/scripts/doc-project.cmd`
- User confirmation: `Not required for this phase`

## Phase 4: Migrate Last Beacon UI scroll/drag input (game)
**Status:** Complete
**Goal:** `game/src/ui_widgets.rs`'s scrollbar-drag, wheel-scroll, and slider-drag behavior is unchanged but routed through `LastBeaconUiScrollInput`'s actions instead of raw `ButtonInput`/`MouseWheel` reads.

### Tasks
- [x] Define `LastBeaconUiScrollInput` context (`LastBeaconUiPointerPrimaryHeld`, `LastBeaconUiScrollWheel`, `LastBeaconUiScrollModifier` actions)
  - Status: Complete
  - Repository: `root`
  - Notes: `bevy_enhanced_input = "0.26.0"` added directly to `game/Cargo.toml` (Last Beacon's context needs the crate's types directly, not just through `foundation-runtime-library`'s dependency). Context registration (`add_input_context` + `Startup` spawn system) wired into `LastBeaconPlugin::build` in `game/src/lib.rs`, matching where the consuming systems were already registered (not through the separate `LastBeaconUiLayoutWidgetsPlugin`, which doesn't own these systems).
- [x] Migrate `drag_last_beacon_ui_text_box_scrollbars` and `scroll_last_beacon_ui_text_inputs` to the new actions
  - Status: Complete
  - Repository: `root`
  - Notes: `TEXT_BOX_SCROLL_LINE_STEP` (16.0) is still applied on top of the crate's own already-unit-normalized wheel value, documented inline as a minor sensitivity difference for pixel-scrolling devices (trackpads) versus the old hand-rolled per-unit multiplier; line-unit mouse wheels (the common case) are unaffected. `KeyboardInput`-based text typing (`ui_widgets.rs:1494,1994`, confirmed by line-number search) was left untouched as planned.
  - **Scope addition beyond the original plan**: discovered a *third* raw-input call site while migrating -- `update_last_beacon_ui_sliders` (line ~1708) also read `Res<ButtonInput<MouseButton>>` for the same "is left mouse button held during a drag" concept used by the scrollbar system. Migrated it to reuse the same `LastBeaconUiPointerPrimaryHeld` action rather than leaving it as an inconsistent raw read, since it's the identical semantic concept and reuses an action already being built.
  - **Deref gotcha discovered**: `Single<&Action<A>>` requires three `*` to reach `A::Output` (`Single -> &Action<A> -> Action<A> -> A::Output`), one more than the two stars needed for a plain `&Action<A>` query-result reference (as used in `free_fly_camera.rs`/`console/mod.rs`'s scroll system). The compiler's own error messages made the exact count unambiguous.

### Validation
- Game validation: `Passed` -- `cargo build`/`cargo test`/`cargo clippy --all-targets --all-features -- -D warnings` all clean (51 lib tests + 4 integration tests passed)
- Engine validation: `N/A`
- Documentation generation: `Passed` -- `cargo doc --manifest-path game/Cargo.toml --all-features --no-deps` (one pre-existing, unrelated warning: `unresolved link to crate::LastBeaconPlugin::build` in `ui_theme.rs:19`, predates this feature)
- User confirmation: `Not required for this phase`

## Phase 5: Cross-repo integration and closeout
**Status:** In progress
**Goal:** Engine work is committed/pushed, root submodule pointer is updated, full validation passes, manual QA confirms no behavior regressions, and `world-landscape-testbed`'s tracker is updated to note its free-fly camera phase is unblocked.

### Tasks
- [x] Commit and push engine branch `feature/enhanced-input-adoption`; record exact engine commit hash
  - Status: Complete
  - Repository: `engine`
  - Notes: Commit `58da948199f35c8662796a6609e0804a7f86bfb8`, pushed to `origin/feature/enhanced-input-adoption`.
- [ ] Update root `engine` submodule pointer to the recorded engine commit hash; commit on root branch
  - Status: In progress (this commit)
  - Repository: `root`
  - Notes: None
- [ ] Manual QA: pause-menu Escape, splash-screen skip, console toggle/navigation/scroll, UI text-box scrollbar drag/wheel-scroll, slider drag all behave identically to before; free-fly camera moves/looks correctly when consumed by a game
  - Status: Pending -- requires interactive keyboard/mouse testing in a running window, which an AI agent cannot perform. Automated test coverage (156 engine tests + 55 game tests) exercises the underlying logic extensively, including two purpose-built regression tests for the new free-fly camera, but a human should confirm the actual in-game feel before merging.
  - Repository: `root`
  - Notes: Flagged to the user as an open item rather than claimed as done.
- [ ] Update `docs/plans/world-landscape-testbed/tracker.md` to note the free-fly camera phase is unblocked and should consume `FoundationFreeFlyCameraInput`
  - Status: Pending
  - Repository: `root`
  - Notes: None

### Validation
- Engine validation: `Passed` -- `engine/scripts/format-project.cmd`, `lint-project.cmd`, `test-project.cmd`, `compile-project.cmd`, `doc-project.cmd` all green
- Game validation: `Passed` -- `scripts/validate.cmd` (fmt, clippy, test, build, doc) all green
- Documentation generation: `Passed` for both repositories
- User confirmation: `Pending` -- interactive manual QA and merge decision are the user's call

## Implementation / Review Handoff Notes
- The `FoundationConsoleInput` name collision, the `Actions<C>` relationship query bug, the `Single<&Action<A>>` triple-deref requirement, and the `Plugin::finish()`/`cleanup()` requirement for bare `app.update()`-driven tests (see below) are all worth knowing before touching this code again.
- **Cross-cutting discovery**: any test that adds a plugin depending on `EnhancedInputPlugin` (menu, splash screen, console, free-fly camera, or the game's UI scroll context) and then calls `app.update()` must first call `app.finish(); app.cleanup();`. `bevy_enhanced_input` finishes per-context setup in `Plugin::finish`, which Bevy's `App::run()` calls automatically but bare `app.update()` loops do not. Four tests needed this fix during this feature (three in `menu.rs`, one in `splash_screen.rs`); any *new* test written against these systems will need it too.

## Optional Review Focus Areas
- Use `gpt-5.5` for review (role fulfilled by Claude Sonnet 5).
- Confirm every migrated call site behaves identically to its pre-migration behavior (same key, same modifier logic, same scroll direction/magnitude) -- especially the console mouse-wheel and UI text-box mouse-wheel unit-normalization differences noted above.
- Confirm the free-fly camera system is genuinely reusable (no Last Beacon-specific assumptions leaking into `foundation-runtime-library`).
- Confirm the `FoundationConsoleControlsInput` naming deviation from the plan is acceptable (it was necessary to avoid a real name collision, not a stylistic choice).

## Success Criteria
- [x] `bevy_enhanced_input` is a normal dependency of `foundation-runtime-library`, wired up via `EnhancedInputPlugin`.
- [x] Pause-menu Escape, splash-screen skip, console toggle/navigation/scroll, and UI text-box scrollbar drag/wheel-scroll (plus slider drag) all work exactly as before per automated tests, now routed through input contexts/actions instead of raw `ButtonInput`/`MouseWheel` reads.
- [x] A new, generically reusable `FoundationFreeFlyCameraInput` context and free-fly camera system exist in `foundation-runtime-library`, ready for `world-landscape-testbed` (or any other Foundation game) to consume.
- [x] All existing engine and game tests pass (rewritten as needed for the new input path); no regression found in menu/splash/console/UI-scroll behavior per automated tests.
- [ ] Interactive manual QA confirming actual in-game feel -- **pending user verification**.

## Testing Methodology
- Engine validation: `engine/scripts/format-project.cmd`, `lint-project.cmd`, `test-project.cmd`, `compile-project.cmd`, `doc-project.cmd` -- all passed.
- Game validation: `scripts/validate.cmd` -- passed.
- Manual QA: not performed by the implementing agent (requires interactive keyboard/mouse input in a running window); recommended before merging.

## Postponed Work
- Rebindable keybinding storage/UI (explicitly deferred to a future feature).
- Migrating `game/src/ui_widgets.rs`'s `KeyboardInput`-based text-typing handling (not applicable to action-based input; not planned as future work either, just noted as intentionally excluded).

## Progress Log
- `2026-09-12`: Plan and tracker created; root branch `feature/enhanced-input-adoption` created from `dev`; engine branch `feature/enhanced-input-adoption` created from `origin/dev` at `01f0cfaaebfe8e193096994642ac2da1848ded9a`.
- `2026-09-12`: User approved the plan ("continue"). Verified root branch is `feature/enhanced-input-adoption` and engine branch is `feature/enhanced-input-adoption` (both confirmed via `git branch --show-current`); engine working tree clean. Starting Phase 1 implementation.
- `2026-09-12`: Phases 1-4 implemented and validated (engine: format/lint/test/compile/doc all green; game: validate.cmd green). Engine work committed (`58da948199f35c8662796a6609e0804a7f86bfb8`) and pushed. Root submodule pointer update and root commit/push in progress. Manual interactive QA and `world-landscape-testbed` tracker cross-reference update still pending.
