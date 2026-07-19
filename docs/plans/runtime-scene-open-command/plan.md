# Runtime Scene Open Command Plan

## Metadata
- Feature slug: `runtime-scene-open-command`
- Feature area: `engine`
- Primary area: `engine`
- Root branch: `feature/runtime-scene-open-command`
- Engine branch: `feature/runtime-scene-open-command` required before engine implementation edits; not created yet because `engine/` is currently detached at the dev commit
- Engine submodule pointer: `7fe0e2b68a6c7f6c4a9b2abf07325582b57264a0`
- Status: `Planned`
- Planning model: `gpt-5.5`
- Implementation model: `gpt-5.4`
- Review model: `gpt-5.5`
- Created: `2026-07-19`
- Last updated: `2026-07-19`

## User Request
Add an in-game console command for when the game is already open that behaves like the existing non-shipping `--scene` startup override. The user should type `open ####` for one scene, or `open #### #### ####` for multiple scenes. Each scene argument is opened in order as a scene stack. Opening through this command must completely clear the current world entities and start fresh.

The console autocomplete/predictive lookup should also suggest BSN scene names for the `open` command. For example, if a scene is registered as `last-beacon/my_map`, typing `open las` should predict the full registered key. The debug console should expand from showing only one predicted command to showing a floating suggestion box above the history with all matching command/scene predictions in a list.

## Feature Summary
Foundation should provide a reusable non-shipping debug-console command named `open` that reopens the running game into one or more BSN scenes. It should accept registered scene keys such as `last-beacon/main_menu` or direct `.bsn` paths relative to the active assets directory, such as `scenes/main_menu.bsn`, matching the existing startup override resolver behavior. The command should clear the existing scene stack/world-owned scene entities before opening the first requested scene, then stack later scenes in the typed order. The console should also expose a richer autocomplete UI that lists all matching predictions and includes registered BSN scene keys when completing `open` arguments.

## Feature Area Classification
- Area: `engine`
- Primary area: `engine`
- Rationale: Runtime scene switching, console command parsing, scene-stack reset semantics, reusable tests, and engine docs belong entirely in Foundation Runtime Library. The root Last Beacon repository only owns the planning/tracking documents and will update the `engine` submodule pointer after the engine feature is committed.

## Codebase Research
- `engine/crates/foundation-runtime-library/src/startup_scene.rs` already parses `--scene` into scene commands. It clears the stack for the first override scene and opens later scenes normally.
- `engine/crates/foundation-runtime-library/src/console/mod.rs` owns the dev-only debug console, distributed command registry, command-line parsing, command execution, autocomplete, the single-line suggestion text, and built-in command examples.
- Console command parsing currently splits on whitespace and supports `name=value` parameters only after the command name. A command shaped as `open scene_a scene_b` does not fit the current named-parameter parser and will require command-specific raw argument support or a special parsing path.
- `engine/crates/foundation-runtime-library/src/scene_stack.rs` owns `SceneCommand`, `OpenSceneOptions::clear_stack`, `SceneCommand::Clear`, scene-owned cleanup, and `SceneLoadRequested`. Clearing the scene stack is the existing mechanism for removing current scene-owned entities.
- `engine/crates/foundation-runtime-library/src/bsn_assets.rs` owns `FoundationBsnSceneRegistry`, which currently maps registered scene keys to asset paths and resolves unregistered keys as direct paths. It will likely need a public sorted/listing API so console autocomplete can suggest registered scene keys.
- `game/src/scenes/mod.rs` registers Last Beacon BSN scene keys and already uses `startup_scene_commands_or_default` for launch-time overrides.
- `game/Cargo.toml` enables `dev-tools` by default and disables them for shipping-compatible builds, so the console command can remain non-shipping/dev-only.
- The root repository is now on `feature/runtime-scene-open-command` from `origin/dev`. The engine submodule is detached at `7fe0e2b68a6c7f6c4a9b2abf07325582b57264a0` and must be moved to a valid engine feature branch before implementation edits.

## External Research
No external online research was performed because this feature is specific to the existing Foundation console, scene-stack, and startup override architecture.

## Affected Files And Systems
- `engine/crates/foundation-runtime-library/src/console/mod.rs`: add support for the raw-token `open` command or a raw-argument console command path, execute it by writing scene-stack reset/open commands, expand autocomplete beyond one suggestion, render a floating suggestion list, and add tests.
- `engine/crates/foundation-runtime-library/src/startup_scene.rs`: consider extracting shared scene-list-to-command generation so startup overrides and runtime console opens share ordering/clear-stack behavior.
- `engine/crates/foundation-runtime-library/src/scene_stack.rs`: likely unchanged, but verify `SceneCommand::Clear` cleanup semantics fully satisfy “clear current entities in world”.
- `engine/crates/foundation-runtime-library/src/bsn_assets.rs`: expose registered BSN scene keys in a deterministic order for console suggestions.
- `engine/docs/debug-console.md` and/or `engine/docs/scene-system.md`: document `open <scene> [scene ...]` usage, examples, and dev/shipping constraints if these docs exist.
- `game/src/scenes/mod.rs` / `game/src/lib.rs`: no planned changes; Last Beacon scene registration is only useful for manual smoke examples after the engine submodule is updated.
- `docs/plans/runtime-scene-open-command/*`: root-owned planning/tracking only; not part of the runtime feature implementation.

## Proposed Implementation Approach
1. Create/switch the engine submodule to `feature/runtime-scene-open-command` from engine `dev` before any engine edits.
2. Add reusable scene-open command building that accepts a non-empty ordered list of scene keys/paths and returns scene commands where:
   - the first scene clears the existing stack/world scene content before opening;
   - later scenes are opened in order above the first scene;
   - registered keys and direct asset paths continue to be represented as `SceneSource::bsn_scene`.
3. Extend the dev-only console command parser to support raw positional arguments for at least the `open` command, preserving existing named-parameter command behavior for macro-registered commands.
4. Register/handle the built-in `open` command in Foundation Runtime Library, not Last Beacon. It should reject `open` with no scenes and report a clear console error.
5. Add console prediction support for `open` scene arguments:
   - typing `open las` should list registered keys such as `last-beacon/my_map`;
   - predictions should come only from the active `FoundationBsnSceneRegistry` so games contribute their own scene keys without engine-specific hard-coding;
   - direct `.bsn` asset paths relative to the active assets directory remain valid user input, but they are not predicted unless they are also registered scene keys.
6. Expand the console suggestion UI from one inline/single prediction to a floating suggestion list above the history area that can show all matching command names or scene keys. Keep Tab completion deterministic, preferably selecting the first sorted suggestion unless keyboard navigation is added in scope.
7. Ensure the command and scene predictions are absent or unreachable when `foundation-runtime-library/dev-tools` is disabled.
8. Add tests for:
   - parsing/executing `open last-beacon/main_menu`;
   - parsing/executing `open last-beacon/gameplay_level last-beacon/pause_menu` with preserved order;
   - rejecting `open` with no scenes;
   - predicting registered BSN scene keys for `open las`;
   - listing multiple predictions deterministically;
   - preserving existing named-parameter commands and command-name autocomplete.
9. Update relevant engine docs with usage examples and autocomplete behavior.
10. If engine code changes are made, commit/push inside `engine/`, record the exact engine commit hash, then commit the root `engine` submodule pointer plus tracker updates.

## Submodule Plan
- Engine changes required: `yes`
- Engine branch: `feature/runtime-scene-open-command` must be created inside `engine/` from engine `dev` before implementation edits.
- Engine commit expectation: commit Foundation runtime console/scene helper/tests/docs inside `engine/`, then push when `origin` is available.
- Bound engine commit hash: pending implementation.
- Root pointer update required: `yes`, after the engine commit is made, but no Last Beacon source changes are planned.

## Alternatives Considered
- Add a Last Beacon-only console command: rejected because the command is reusable Foundation runtime behavior and should work for any Foundation game with registered scenes.
- Reuse the `--scene [a, b]` bracketed syntax inside the console: rejected for this feature because the requested runtime command shape is `open #### #### ####` with spaces separating scenes.
- Clear the entire Bevy world directly: risky because Foundation/Bevy resources, plugins, windows, assets, and console UI must survive. The planned approach clears the scene stack so scene-owned entities are removed while app infrastructure remains intact.

## Risks, Constraints, And Assumptions
- “Completely clear current entities in world” is interpreted as clearing current scene-owned gameplay/UI entities through Foundation scene-stack cleanup, not deleting core Bevy/Foundation infrastructure entities required to keep the app alive.
- The debug console scene itself is a scene-stack runtime scene. If `open` is executed while the console is open, clearing the stack should also close/remove the console overlay as part of starting fresh.
- Existing console parsing is whitespace-based. Scene keys/paths therefore cannot contain spaces in this first pass unless a broader console quoting feature is added.
- The current console UI shows only one prediction. Showing all predictions in a floating box may require changing `FoundationConsoleSuggestion` state/rendering and ensuring the box layers cleanly above the history without obscuring the input.
- Multiple fullscreen scenes will stack in order, but upper fullscreen scenes can hide lower scenes by existing scene-presentation semantics.
- Engine work cannot start while `engine/` is detached; implementation must create/switch the engine branch first.

## Open Questions
- Should `open` be the only raw positional command, or should the console command system gain a general raw-argument descriptor mode for future commands?
- Is scene-stack clearing sufficient for “clear current entities in world,” or should implementation also identify non-scene-owned transient entities that must be cleaned up?
- Should the floating suggestion list support Up/Down selection now, or should Tab complete the first sorted suggestion for this first pass?

## Documentation Expectations
- Public APIs added by this feature must have Rustdoc comments.
- Engine debug console and scene-system documentation should include `open last-beacon/main_menu` and `open last-beacon/gameplay_level last-beacon/pause_menu` examples.
- Debug console documentation should explain that `open` autocomplete suggests registered BSN scene keys and that the prediction UI displays multiple matches.
- Generated documentation must be produced before completion.

## Implementation Handoff Notes
- Use `gpt-5.4` for implementation.
- Never use Anthropic models.
- Read `.pi/skills/feature-tracker-update/SKILL.md`, `.pi/skills/feature-plan-docs/SKILL.md`, `.pi/skills/rust-workspace-dev/SKILL.md`, `.pi/skills/rust-coding-standards/SKILL.md`, `.pi/skills/foundation-architecture/SKILL.md`, and `.pi/skills/gitflow-workflow/SKILL.md` before implementation edits.
- Verify root branch is `feature/runtime-scene-open-command` and `origin/dev` is an ancestor of `HEAD`.
- Create/switch `engine/` to `feature/runtime-scene-open-command` from engine `dev` before any engine edits.
- Keep all runtime implementation in `engine/`; do not change Last Beacon source unless the user explicitly expands scope.

## Optional Review Focus Areas
- Use `gpt-5.5` for review.
- Confirm `open` is dev-only and unavailable in shipping/no-dev-tools builds.
- Confirm executing `open` while the console is open removes the old scene stack and starts fresh.
- Confirm ordered multi-scene opens match startup override behavior.
- Confirm the root submodule pointer and recorded engine commit hash match.

## Success Criteria
- In a non-shipping Foundation game build such as Last Beacon, typing `open last-beacon/main_menu` in the debug console clears the current scene/world content and opens the main menu fresh.
- Typing `open last-beacon/gameplay_level last-beacon/pause_menu` clears current content, opens gameplay, then opens pause menu above it.
- Direct `.bsn` asset paths relative to the active assets directory work the same way as registered scene keys when typed explicitly.
- Typing `open las` predicts registered scene keys such as `last-beacon/main_menu` in a floating multi-result suggestion list; unregistered direct asset-relative `.bsn` paths are allowed but not predicted.
- The debug console can show multiple command-name or scene-key predictions rather than only a single predicted command.
- `open` with no scene arguments returns a clear console error.
- Existing console commands continue to parse and execute as before.
- Shipping/no-dev-tools builds do not include the runtime scene-open console command.

## Testing Methodology
- Engine focused tests: `cargo test --manifest-path engine/Cargo.toml -p foundation-runtime-library --all-features` and a no-default-features/dev-tools-disabled focused check where practical.
- Engine focused lint/docs: `cargo clippy --manifest-path engine/Cargo.toml -p foundation-runtime-library --all-targets --all-features -- -D warnings` and `cargo doc --manifest-path engine/Cargo.toml -p foundation-runtime-library --all-features --no-deps`.
- Engine full validation: `engine/scripts/validate-project.cmd` before completion unless a waiver is approved.
- Game validation: `scripts/validate.cmd` after root submodule pointer update, to verify Last Beacon still builds against the updated engine.
- Manual smoke: run a non-shipping Foundation game build, open the console with backtick, execute single-scene and multi-scene `open` commands, and observe that old scene content is cleared and requested scenes load.
