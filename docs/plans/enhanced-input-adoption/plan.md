# Enhanced Input Adoption Plan

## Metadata
- Feature slug: `enhanced-input-adoption`
- Feature area: `multi-area`
- Primary area: `engine`
- Root branch: `feature/enhanced-input-adoption`
- Engine branch: `feature/enhanced-input-adoption`
- Engine submodule pointer: `01f0cfaaebfe8e193096994642ac2da1848ded9a` (base; will change once engine work is committed)
- Status: `Planned`
- Planning model: `gpt-5.5` (role fulfilled by Claude Sonnet 5, per the user's standing instruction that Claude/subagents replace GPT in this workflow)
- Implementation model: `gpt-5.4` (role fulfilled by Claude Sonnet 5)
- Review model: `gpt-5.5` (role fulfilled by Claude Sonnet 5)
- Created: `2026-09-12`
- Last updated: `2026-09-12`

## User Request
While reviewing the (separate, already-planned) `world-landscape-testbed` feature, the user asked what input system the project currently uses, then decided the project should adopt [`bevy_enhanced_input` 0.26.0](https://crates.io/crates/bevy_enhanced_input/0.26.0) as the default input system -- not just for the landscape testbed's free-fly camera, but as Foundation Engine's default input system, so any game built on Foundation gets it for free.

This is a distinct, engine-primary feature from `world-landscape-testbed` (which explicitly declared no engine changes) and is planned/tracked separately per the decomposition guidance: input handling is reusable Foundation runtime behavior (`.pi/skills/foundation-architecture/SKILL.md`), not Last Beacon gameplay content. `world-landscape-testbed`'s free-fly camera work is paused until this feature lands, then consumes the `FoundationFreeFlyCameraInput` context this feature produces.

Clarifying decisions made during brainstorming:
- **Migration scope: full migration now.** Every existing raw `ButtonInput<KeyCode>`/`ButtonInput<MouseButton>`/`MouseWheel` read for discrete key/button/scroll actions is migrated to `bevy_enhanced_input`, not just new code.
- **Free-fly debug camera ownership: Foundation-owned, reusable.** The camera's input context *and* its movement system live in `foundation-runtime-library`, since a debug free-fly camera is generically useful to any Foundation-based game, not Last Beacon-specific. Foundation's own existing input (menu, splash screen, console) also migrates here as the first consumers.
- **Rebinding: out of scope.** Fixed, hardcoded default bindings only (matching today's behavior). Persisted, user-rebindable keybinding storage/UI is a separate, larger future feature.
- **Text entry is explicitly not part of this migration.** Confirmed by reading the code: `game/src/ui_widgets.rs:1494,1994` already handle literal character typing via raw `MessageReader<KeyboardInput>` (Bevy's text-bearing keyboard event), which is a fundamentally different concern from action/binding-based input. This migration only touches discrete control-key/button/scroll-wheel reads, never character-level text entry.

## Feature Summary
Adds `bevy_enhanced_input` as a core (non-optional) dependency of `engine/crates/foundation-runtime-library` and migrates every existing raw discrete-input read in the workspace to it, organized into a handful of input contexts:
- `FoundationMenuInput` (Foundation, `menu.rs`): consolidates the pause-open and close-on-escape `KeyCode::Escape` checks into one shared back/pause action.
- `FoundationSplashScreenInput` (Foundation, `splash_screen.rs`): skip-splash action (currently `KeyCode::Escape`).
- `FoundationConsoleInput` (Foundation, `console/mod.rs`): toggle-console (backquote) action, plus the console's own Escape/Tab/ArrowUp/ArrowDown/Enter control actions and its mouse-wheel output-scroll action. Literal command-line text typing is untouched.
- `FoundationFreeFlyCameraInput` (Foundation, new): Move (WASD + up/down) and Look (mouse delta) actions, plus a new reusable free-fly camera movement system. This is a genuinely new capability -- no free-fly camera exists in the codebase today -- built as Foundation runtime content so any Foundation game can use it, with `world-landscape-testbed`'s World scene as its first consumer.
- `LastBeaconUiScrollInput` (Last Beacon, `game/src/ui_widgets.rs`): `PointerPrimaryHeld` (bool, left mouse button), `ScrollWheel` (Vec2), `ScrollModifier` (bool, shift) actions, replacing the raw reads in `drag_last_beacon_ui_text_box_scrollbars` and `scroll_last_beacon_ui_text_inputs`. Bevy UI's own `Interaction` component (hover/pressed state Bevy already computes for buttons and scroll tracks) is untouched -- it is not a raw input read.

## Feature Area Classification
- Area: `multi-area` (engine + game)
- Primary area: `engine`
- Rationale: The core dependency addition, plugin wiring, and the majority of migrated call sites (menu, splash screen, console, plus the new free-fly camera) live in `engine/crates/foundation-runtime-library`, per `foundation-architecture`'s rule that reusable input handling is Foundation runtime behavior. `game/src/ui_widgets.rs`'s migration is Last Beacon-specific and secondary.

## Codebase Research
- `engine/crates/foundation-runtime-library/src/menu.rs:767-781` (`open_pause_menus`) and `:1124-1134` (`close_on_escape`): both read `Res<ButtonInput<KeyCode>>` and check `just_pressed(KeyCode::Escape)` independently. Tests at `:1271,1298-1299,1316,1379` construct `ButtonInput<KeyCode>` directly and call `.press(KeyCode::Escape)` -- these tests need rewriting to go through the new input context instead of (or in addition to) raw `ButtonInput`.
- `engine/crates/foundation-runtime-library/src/splash_screen.rs:408-455` (`advance_splash_screens`/`splash_skip_requested`): reads `Res<ButtonInput<KeyCode>>`, checks `just_pressed(KeyCode::Escape)`. Test at `:587-591` presses `KeyCode::Escape` directly on a `ButtonInput<KeyCode>` resource.
- `engine/crates/foundation-runtime-library/src/console/mod.rs:271-276` (`toggle_console_scene`): reads `Res<ButtonInput<KeyCode>>`, checks `just_pressed(KeyCode::Backquote)`. `:365-433` (`handle_console_keyboard_actions`): reads `Res<ButtonInput<Key>>` (Bevy's *logical* key enum, not `KeyCode`) and checks `Key::Escape`/`Key::Tab`/`Key::ArrowUp`/`Key::ArrowDown`/`Key::Enter` for console-specific navigation -- these are control-key actions, not text entry, and are in scope. `:669-670` (`scroll_console_output`): reads `MessageReader<MouseWheel>` for output scrolling.
- No free-fly/debug-camera code exists anywhere in the workspace today (confirmed by an earlier search across `game/` and `engine/` for `fly|FreeCam|DebugCamera|camera_controller|PanOrbit`); this is new Foundation runtime content, not a migration of existing behavior.
- `game/src/ui_widgets.rs:1308-1417` (`drag_last_beacon_ui_text_box_scrollbars`): reads `Res<ButtonInput<MouseButton>>` (`.pressed(MouseButton::Left)`) to continue a scrollbar drag gesture once started via Bevy UI's `Interaction::Pressed`. `:1419-1451+` (`scroll_last_beacon_ui_text_inputs`): reads `MessageReader<MouseWheel>` and `Res<ButtonInput<KeyCode>>` (`KeyCode::ShiftLeft`/`ShiftRight`) to scroll multiline text boxes, swapping horizontal/vertical based on the shift modifier.
- `game/src/ui_widgets.rs:1494,1994`: `MessageReader<KeyboardInput>` (Bevy's text-bearing keyboard event, carrying `Key`/`text`) already handles literal character typing into text boxes -- confirms text entry is a separate mechanism from the `ButtonInput<KeyCode>`/`ButtonInput<MouseButton>` reads being migrated, and is correctly excluded from this feature.
- `game/Cargo.toml:22-27` and `engine/Cargo.toml` workspace deps: `bevy = "0.19.0"`/`0.19.0` respectively, no existing input-mapping crate. `engine/docs/plans/remove-jackdaw-editor/plan.md:46` shows `bevy_enhanced_input` was previously a transitive dependency pulled in by the now-removed Jackdaw editor stack (`jackdaw`, `jackdaw_api`, `jackdaw_runtime`, `bevy_enhanced_input`, `rfd`, `ctrlc` were all removed together) -- it was collateral cleanup, not removed due to any problem with the crate itself.
- `engine/crates/foundation-editor-library/Cargo.toml`: currently depends only on `bevy.workspace = true`; no input-mapping crate remains anywhere in the workspace today.

## External Research
- Confirmed via the crates.io API (`https://crates.io/api/v1/crates/bevy_enhanced_input/0.26.0/dependencies`) that `bevy_enhanced_input` 0.26.0 depends on `bevy ^0.19.0` with `mouse, keyboard, gamepad, touch` features -- an exact match for this workspace's pinned `bevy = "0.19.0"` dependency, so no version-compatibility risk.
- Confirmed via `docs.rs/bevy_enhanced_input/0.26.0` the core API shape: define an action type via `#[derive(InputAction)] #[action_output(bool|f32|Vec2|Vec3)]`; register a context type via `app.add_input_context::<C>()`; spawn a context marker component on an entity together with `actions!(C[(Action::<A>::new(), bindings![...])])`; read results either push-style via observers on `Start<A>`/`Fire<A>`/`Complete<A>` or pull-style via the `Action<A>` component.

## Affected Files And Systems
- `engine/Cargo.toml` / `engine/crates/foundation-runtime-library/Cargo.toml`: add `bevy_enhanced_input = "0.26.0"` as a normal dependency.
- `engine/crates/foundation-runtime-library/src/lib.rs` (or wherever Foundation's plugin bundle is assembled): add `bevy_enhanced_input::EnhancedInputPlugin` and register the new input context types (`add_input_context::<C>()` for each).
- `engine/crates/foundation-runtime-library/src/menu.rs`: replace the two independent `KeyCode::Escape` checks with `FoundationMenuInput`'s back/pause action; update tests.
- `engine/crates/foundation-runtime-library/src/splash_screen.rs`: replace `splash_skip_requested`'s raw check with `FoundationSplashScreenInput`'s skip action; update tests.
- `engine/crates/foundation-runtime-library/src/console/mod.rs`: replace `toggle_console_scene`'s raw check, `handle_console_keyboard_actions`'s five control-key checks, and `scroll_console_output`'s wheel read with `FoundationConsoleInput`'s actions; leave character-typing handling untouched.
- `engine/crates/foundation-runtime-library/src/` (new module, e.g. `free_fly_camera.rs`): new `FoundationFreeFlyCameraInput` context, Move/Look actions, and the camera movement system -- new reusable content, not a migration.
- `game/src/ui_widgets.rs`: replace the raw reads in `drag_last_beacon_ui_text_box_scrollbars` and `scroll_last_beacon_ui_text_inputs` with `LastBeaconUiScrollInput`'s actions.
- `game/Cargo.toml`: no new dependency expected (Last Beacon consumes Foundation's re-exported/plugin-provided input types via `foundation-runtime-library`'s existing dependency path); confirm at implementation time whether `bevy_enhanced_input` types need to be imported directly in `game/`, which would require adding it there too.
- `docs/plans/enhanced-input-adoption/plan.md`, `docs/plans/enhanced-input-adoption/tracker.md`: this plan and its tracker.
- `docs/plans/world-landscape-testbed/plan.md`: needs a follow-up update once this feature lands, so its free-fly camera phase consumes `FoundationFreeFlyCameraInput` instead of raw input (tracked as a note in that plan's tracker, not re-litigated here).

## Proposed Implementation Approach
1. Add `bevy_enhanced_input = "0.26.0"` to `engine/crates/foundation-runtime-library`'s dependencies; add `EnhancedInputPlugin` to Foundation's plugin bundle.
2. Define `FoundationMenuInput`, `FoundationSplashScreenInput`, `FoundationConsoleInput`, and `FoundationFreeFlyCameraInput` context marker components and their actions; register each via `add_input_context::<C>()`.
3. Migrate `menu.rs`'s two Escape checks to `FoundationMenuInput`'s action; update/rewrite the affected tests.
4. Migrate `splash_screen.rs`'s skip check to `FoundationSplashScreenInput`'s action; update the affected test.
5. Migrate `console/mod.rs`'s toggle check, the five control-key checks in `handle_console_keyboard_actions`, and the mouse-wheel scroll read to `FoundationConsoleInput`'s actions; leave literal command-text typing untouched.
6. Build the new `FoundationFreeFlyCameraInput` context (Move: WASD + up/down as a `Vec3`-or-composite action; Look: mouse delta as a `Vec2` action) and a movement system that applies them to a camera's `Transform`, as new reusable Foundation runtime content.
7. Migrate `game/src/ui_widgets.rs`'s `drag_last_beacon_ui_text_box_scrollbars` and `scroll_last_beacon_ui_text_inputs` to `LastBeaconUiScrollInput`'s actions, preserving current drag/scroll/shift-modifier behavior exactly.
8. Run full engine and game validation (see Testing Methodology). Manually verify: pause-menu Escape, splash-screen skip, console toggle/navigation/scroll, and UI text-box scrollbar drag/wheel-scroll all behave identically to before.
9. Update `docs/plans/world-landscape-testbed/tracker.md` to note that its free-fly camera phase is now unblocked and should consume `FoundationFreeFlyCameraInput`.

## Submodule Plan
- Engine changes required: `yes`
- Engine branch: `feature/enhanced-input-adoption` (created from `origin/dev` at `01f0cfaaebfe8e193096994642ac2da1848ded9a`)
- Engine commit expectation: `bevy_enhanced_input` dependency addition, plugin wiring, new input contexts, and all Foundation-side migrations (menu, splash screen, console, free-fly camera).
- Bound engine commit hash: `Pending` -- to be recorded once engine work is committed and pushed.
- Root pointer update required: `yes`, after engine work is committed and pushed.

## Alternatives Considered
- **New code only, existing raw-input call sites left alone**: rejected per explicit user decision -- the user asked for full migration now, accepting the larger surface area in exchange for one consistent input pattern across the codebase.
- **Free-fly camera owned by Last Beacon (`game/src/world/`)**: rejected per explicit user decision -- a debug free-fly camera is generically useful to any Foundation-based game, so it belongs in `foundation-runtime-library` alongside Foundation's other reusable runtime systems.
- **Rebindable keybinding storage/UI as part of this feature**: rejected per explicit user decision -- fixed default bindings only for now; rebinding is a separate, larger feature with its own settings/UI design work.
- **Migrating `game/src/ui_widgets.rs`'s text-typing (`KeyboardInput`) handling too**: not applicable -- text entry is not raw discrete-action input and bevy_enhanced_input is not designed for it; conflating the two would be a scope and design error, not a simplification.

## Risks, Constraints, And Assumptions
- Assumes `bevy_enhanced_input` 0.26.0's `bevy ^0.19.0` dependency resolves cleanly against this workspace's exact `bevy = "0.19.0"` pin; must be verified with `cargo build` at implementation start before any further work.
- Existing tests that construct `ButtonInput<KeyCode>` directly and call `.press(...)` (menu.rs, splash_screen.rs) will need to be rewritten to also register the relevant input context/plugin so the action-based code path is actually exercised, not just the underlying raw resource. This is expected rework, not an incidental side effect.
- The free-fly camera's exact feel (movement speed, mouse sensitivity, acceleration) is not specified numerically here; reasonable defaults should be chosen during implementation and are easy to tune later since there's no dependent gameplay yet.
- `game/Cargo.toml` may or may not need a direct `bevy_enhanced_input` dependency depending on whether Last Beacon's `LastBeaconUiScrollInput` context needs to reference the crate's types directly or can go through re-exports from `foundation-runtime-library`; this is an implementation-time decision, not a planning blocker.

## Open Questions
- None outstanding; all decisions were resolved during brainstorming (see User Request section).

## Documentation Expectations
- Public APIs added by this feature (all new context/action types, the free-fly camera system, and any newly-`pub` items in `foundation-runtime-library`) must have Rustdoc comments, following the existing documentation style in `menu.rs`/`console/mod.rs`.
- No new `docs/` architecture documentation is expected beyond this plan; the input-context pattern established here should be discoverable from the code and Rustdoc alone.
- Generated documentation (`cargo doc`) must be produced for both `engine/` and `game/` before the feature is considered complete.

## Implementation Handoff Notes
- Use `gpt-5.4` for implementation (role fulfilled by Claude Sonnet 5 per this project's standing instruction).
- Never use Anthropic models unless already directed otherwise by the user's standing instruction recorded above.
- Do the engine-side work first (dependency, plugin wiring, contexts, migrations, free-fly camera) and commit/push it inside `engine/` before updating the root submodule pointer, per `gitflow-workflow`.
- Preserve exact current behavior for every migrated call site (same keys, same modifiers, same scroll behavior) -- this is a mechanism swap, not a UX change.
- Do not touch `game/src/ui_widgets.rs:1494,1994`'s `KeyboardInput` text-entry handling; it is explicitly out of scope.
- After this feature's engine work is committed and the root submodule pointer is updated, update `docs/plans/world-landscape-testbed/tracker.md` to note the free-fly camera phase is unblocked.

## Optional Review Focus Areas
- Use `gpt-5.5` for review (role fulfilled by Claude Sonnet 5).
- Confirm every migrated call site behaves identically to its pre-migration behavior (same key, same modifier logic, same scroll direction/magnitude).
- Confirm the free-fly camera system is genuinely reusable (no Last Beacon-specific assumptions leaking into `foundation-runtime-library`).
- Confirm rewritten menu/splash-screen tests actually exercise the new input-context path, not just the underlying raw `ButtonInput` resource.

## Success Criteria
- `bevy_enhanced_input` is a normal dependency of `foundation-runtime-library`, wired up via `EnhancedInputPlugin`.
- Pause-menu Escape, splash-screen skip, console toggle/navigation/scroll, and UI text-box scrollbar drag/wheel-scroll all work exactly as before, now routed through input contexts/actions instead of raw `ButtonInput`/`MouseWheel` reads.
- A new, generically reusable `FoundationFreeFlyCameraInput` context and free-fly camera system exist in `foundation-runtime-library`, ready for `world-landscape-testbed` (or any other Foundation game) to consume.
- All existing engine and game tests pass (rewritten as needed for the new input path); no regression in menu/splash/console/UI-scroll behavior.

## Testing Methodology
- Engine validation: `engine/scripts/format-project.cmd`, `engine/scripts/lint-project.cmd`, `engine/scripts/test-project.cmd`, `engine/scripts/compile-project.cmd`, `engine/scripts/doc-project.cmd`, `engine/scripts/validate-project.cmd`.
- Game validation: `scripts/validate.cmd`; focused `cargo fmt`/`cargo clippy`/`cargo test`/`cargo doc` with `--manifest-path game/Cargo.toml`.
- Manual QA (required, no automated visual test): run the game, confirm pause-menu Escape, splash-screen skip, console toggle (backquote) plus Tab/ArrowUp/ArrowDown/Enter navigation and mouse-wheel output scroll, and UI text-box scrollbar drag/wheel-scroll all behave identically to before.
