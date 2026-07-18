# Launch Scene Stack Overrides Plan

## Metadata
- Feature slug: `launch-scene-stack-overrides`
- Feature area: `multi-area`
- Primary area: `engine`
- Root branch: `feature/launch-scene-stack-overrides`
- Engine branch: `feature/launch-scene-stack-overrides` planned; not created yet because `engine/` is currently detached at the dev commit
- Engine submodule pointer: `3688245e962d752a8cc4645a3d4b3fd10834dda2`
- Status: `Planned`
- Planning model: `gpt-5.5`
- Implementation model: `gpt-5.4`
- Review model: `gpt-5.5`
- Created: `2026-07-18`
- Last updated: `2026-07-18`

## User Request
Add a non-shipping Foundation feature that lets a developer launch directly into one or more `.bsn` scenes instead of the game's normal default startup scene. Each launch scene should accept either a registered scene key such as `last-beacon/main_menu` or a direct asset path such as `scenes/main_menu.bsn`. If multiple scenes are supplied, Foundation should open them in order as a stack, such as gameplay first and a testing/game-mode scene above it. If no scenes are supplied, the game should load its normal default scene. This must work in any build other than shipping.

## Feature Summary
Foundation should provide a reusable startup-scene override path for development, test, and editor workflows. Games keep ownership of their default startup flow, but can delegate startup selection to a Foundation helper that chooses command-line scene stack overrides when present and falls back to the game's default scene when absent.

## Feature Area Classification
- Area: `multi-area`
- Primary area: `engine`
- Rationale: The reusable argument parsing, scene-stack startup helper, shipping exclusion, documentation, and tests belong in Foundation Engine. Last Beacon needs a small integration change so its current default splash flow is bypassed only when startup override scenes are supplied.

## Codebase Research
- `engine/crates/foundation/src/launch.rs` currently parses `--game`, `--editor`, `--log`, and `--log-inline`, then forwards known runtime flags to the selected game process in loose Cargo-package mode.
- `scripts/foundation-game.cmd` already supports forwarding arbitrary runtime arguments after `--` to `engine/scripts/foundation-build.cmd` for external project runs.
- `engine/docs/build-packaging.md` documents that runtime arguments can be passed after `--`; this is the preferred local game run workflow.
- `engine/crates/foundation-runtime-library/src/scene_stack.rs` already has `SceneCommand`, `SceneSource::bsn_scene`, `OpenSceneOptions`, `ScenePresentation`, and ordered stack mutation support.
- `engine/crates/foundation-runtime-library/src/bsn_assets.rs` resolves registered keys through `FoundationBsnSceneRegistry` and treats unregistered keys as direct asset paths, matching the requested key-or-path behavior.
- `game/src/scenes/mod.rs` currently registers Last Beacon scene keys and always opens `last-beacon/splash_pixel_perfect` in `open_initial_scene`.
- `game/Cargo.toml` enables `dev-tools` and `editor` by default. Shipping builds use `--no-default-features`, so dev-only runtime support can be gated behind `dev-tools` while shipping keeps the fallback default scene behavior.
- `engine/Cargo.toml` defines `foundation-shipping` and the workspace dependency graph. Foundation runtime dev tooling is already feature-gated in `FoundationPlugin`.

## External Research
No external online research was performed because this feature is specific to the existing Foundation scene-stack, launch, and build-tool architecture.

## Affected Files And Systems
- `engine/crates/foundation-runtime-library/src/lib.rs`: expose any new startup override APIs in the prelude when appropriate.
- `engine/crates/foundation-runtime-library/src/scene_stack.rs` or a new focused runtime module: parse non-shipping startup scene override arguments and emit startup `SceneCommand`s.
- `engine/crates/foundation/src/launch.rs`: teach the loose Foundation launcher to accept and forward the new scene override arguments instead of rejecting them.
- `engine/docs/foundation-engine.md`: document launcher support for startup scene overrides.
- `engine/docs/scene-system.md`: document direct launch into registered scene keys or asset paths and stacked startup behavior.
- `engine/docs/build-packaging.md`: document passing startup scene overrides through the build tool after `--`.
- `game/src/scenes/mod.rs`: replace the hard-coded startup open with a Foundation helper or equivalent branch: use override stack if present, otherwise open the existing default splash scene.
- `game/src/lib.rs`: likely unchanged except for imports; verify during implementation.
- `docs/plans/launch-scene-stack-overrides/*`: track the cross-repository plan and implementation status.

## Proposed Implementation Approach
1. Add a reusable non-shipping startup scene override API in Foundation Runtime Library.
   - Proposed CLI shape: one `--scene <value>` argument that accepts either a single scene key/path or a bracketed ordered list.
   - Single scene example: `--scene last-beacon/main_menu`.
   - Stacked scenes example: `--scene [last-beacon/gameplay_level,scenes/testing_mode.bsn]`.
   - The list parser must tolerate spaces around commas, e.g. `[last-beacon/gameplay_level, scenes/testing_mode.bsn]` and `[last-beacon/gameplay_level , scenes/testing_mode.bsn]`.
   - Order: list entries are opened bottom-to-top in the supplied order.
   - If no `--scene` value is present, the helper emits the caller-provided default startup command(s).
   - The initial scene command should clear the stack before opening the first override scene; subsequent override scenes should open normally so they stack.
2. Provide a small public helper for game startup systems, for example `open_startup_scene_stack_or_default(...)`, that can be used from a Startup system with `MessageWriter<SceneCommand>`.
3. Gate runtime override parsing so shipping builds ignore or compile out the feature.
   - Preferred behavior: when `foundation-runtime-library/dev-tools` is disabled, the helper always emits the default startup command and does not honor override args.
   - Non-shipping `debug`, `test`, and `game-editor` builds keep the override behavior.
4. Update Last Beacon startup to use the helper.
   - Default fallback remains the current clear/open of `last-beacon/splash_pixel_perfect` with key `startup-splash` and fullscreen presentation.
   - Override examples should include `--scene last-beacon/main_menu` and `--scene scenes/main_menu.bsn`.
5. Update the loose Foundation launcher to parse and forward the new `--scene` arguments to the selected game process.
   - Keep unknown argument rejection for truly unsupported launcher args.
   - Preserve existing `--editor`, `--log`, `--log-inline`, and `--help` behavior.
6. Verify the Foundation build tool path supports runtime override arguments after `--`; update docs and tests if any forwarding behavior is missing.
7. Add tests.
   - Runtime parser tests for no scenes, one scene, multiple scenes, and shipping/no-dev-tools fallback where practical.
   - Scene command generation tests proving the first override clears the stack and later overrides stack in order.
   - Foundation launcher parse/forward tests for one or more `--scene` arguments.
   - Last Beacon test confirming the default startup key is unchanged when no override args are present.
8. Update docs with concise usage examples.

## Submodule Plan
- Engine changes required: `yes`
- Engine branch: `feature/launch-scene-stack-overrides` must be created inside `engine/` from engine `dev` before implementation edits.
- Engine commit expectation: commit Foundation runtime, launcher, tests, and docs inside `engine/`, then push when `origin` is available.
- Bound engine commit hash: pending implementation.
- Root pointer update required: `yes`, after the engine commit is made. The root feature must commit the updated `engine` submodule pointer plus Last Beacon integration and tracker updates.

## Alternatives Considered
- Only add Last Beacon-specific argument parsing: rejected because the user requested an engine feature and other Foundation games should be able to use it.
- Treat unregistered scene names as errors: rejected because the current BSN registry intentionally supports direct asset paths when a key is not registered.
- Repeat `--scene` multiple times: rejected after user feedback because the desired command shape is one argument that accepts either a single value or a bracketed ordered list.
- Support presentation policy per scene in the first pass: deferred unless the user requests it. The first version should prove stack loading by opening supplied scenes in order with existing default scene presentation. A later enhancement can add per-scene presentation syntax if needed for overlay-specific workflows.

## Risks, Constraints, And Assumptions
- Shipping must not honor scene overrides. The implementation must rely on compile-time feature gating, build configuration, or both, not only a runtime convention.
- `--scene` is the proposed CLI name, with a value parser that accepts either one scene key/path or a bracketed list. Bracketed lists may need shell quoting in some terminals, e.g. `--scene "[last-beacon/gameplay_level, scenes/testing_mode.bsn]"`. The parser must trim whitespace around each comma-separated entry and reject empty entries with a clear message.
- Opening multiple scenes with default `ScenePresentation::FULLSCREEN` creates stack entries but upper fullscreen scenes hide/block lower scenes. This matches existing default stack semantics, but overlay-like test scenes may later need per-scene presentation options.
- The Foundation loose launcher and Foundation build tool use different argument paths. Both must be documented so developers know when to pass args directly and when to put them after `--`.
- Engine work cannot start while `engine/` is detached. The implementation phase must create or switch to the planned engine feature branch first.

## Open Questions
- Confirm exact list syntax details: proposed `--scene <key-or-path>` for one scene or `--scene [<key-or-path>, <key-or-path>]` for an ordered stack. Spaces around commas must be accepted.
- Should the first version include per-scene presentation options, or is plain ordered stack loading enough for this feature?

## Documentation Expectations
- Public APIs added by this feature must have Rustdoc comments.
- Update `engine/docs/foundation-engine.md`, `engine/docs/scene-system.md`, and `engine/docs/build-packaging.md` with examples.
- Generated documentation must be produced with the engine doc validation command before engine work is complete.
- Last Beacon root docs are not required unless the implementation changes the user-facing root wrapper behavior beyond normal runtime argument forwarding.

## Implementation Handoff Notes
- Use `gpt-5.4` for implementation.
- Never use Anthropic models.
- Read `.pi/skills/feature-tracker-update/SKILL.md`, `.pi/skills/feature-plan-docs/SKILL.md`, `.pi/skills/rust-workspace-dev/SKILL.md`, `.pi/skills/rust-coding-standards/SKILL.md`, `.pi/skills/foundation-architecture/SKILL.md`, and `.pi/skills/gitflow-workflow/SKILL.md` before implementation edits.
- Verify root branch is `feature/launch-scene-stack-overrides` and root `origin/dev` is an ancestor of `HEAD` before implementation.
- Create/switch `engine/` to `feature/launch-scene-stack-overrides` from engine `dev` before any engine edits; currently `engine/` is detached at `3688245e962d752a8cc4645a3d4b3fd10834dda2`.
- Keep reusable startup override parsing in Foundation Runtime Library and only minimal game-specific fallback wiring in Last Beacon.
- Do not add Jackdaw dependencies.

## Optional Review Focus Areas
- Use `gpt-5.5` for review.
- Confirm shipping builds ignore or exclude startup scene overrides.
- Confirm registered scene keys and direct asset paths both work through existing `FoundationBsnSceneRegistry` behavior.
- Confirm bracketed scene lists preserve the requested order.
- Confirm the engine submodule commit hash recorded in the root tracker matches the root submodule pointer.

## Success Criteria
- Running a non-shipping Last Beacon build with no scene override still starts with the existing default splash flow.
- Running with one override can open a registered scene key directly, e.g. `--scene last-beacon/main_menu`.
- Running with one override can open a direct asset path, e.g. `--scene scenes/main_menu.bsn`.
- Running with a bracketed list opens those scenes in order as a Foundation scene stack, e.g. `--scene [last-beacon/gameplay_level, scenes/testing_mode.bsn]`, with spaces around commas accepted.
- Shipping builds do not honor scene override arguments.
- Engine and root tests/docs/validation pass or any waiver is explicitly recorded and approved.

## Testing Methodology
- Game validation: `scripts/validate.cmd` and focused `cargo test --manifest-path game/Cargo.toml --all-features` for Last Beacon integration.
- Game smoke run: non-shipping `scripts/run.cmd -- --scene last-beacon/main_menu` and a bracketed multi-scene variant such as `scripts/run.cmd -- --scene "[last-beacon/gameplay_level,scenes/testing_mode.bsn]"` when practical; use timeout/manual observation if an interactive window opens.
- Engine focused validation: `cargo test --manifest-path engine/Cargo.toml -p foundation-runtime-library --all-features`, `cargo test --manifest-path engine/Cargo.toml -p foundation --all-features`, and doc generation.
- Engine full validation before completion: `engine/scripts/validate-project.cmd` unless a focused-validation waiver is approved.
- Shipping check: include a no-default-features focused check or package/run validation that proves override handling is absent or ignored in shipping-compatible builds.
