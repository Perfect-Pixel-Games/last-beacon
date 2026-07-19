# Runtime Scene Open Command Tracker

## Metadata
- Feature slug: `runtime-scene-open-command`
- Feature area: `engine`
- Primary area: `engine`
- Root branch: `feature/runtime-scene-open-command`
- Engine branch: `feature/runtime-scene-open-command`
- Root branch base verification: `Verified: created from root dev; origin/dev is an ancestor of HEAD on 2026-07-19`
- Engine branch base verification: `Pending; engine/ is currently detached at dev commit 7fe0e2b68a6c7f6c4a9b2abf07325582b57264a0`
- Engine submodule pointer: `7fe0e2b68a6c7f6c4a9b2abf07325582b57264a0`
- Overall status: `Planned`
- Planning model: `gpt-5.5`
- Preferred implementation model: `gpt-5.4`
- Optional final review model: `gpt-5.5`
- Current handoff state: `Ready for gpt-5.4 implementation after user approval`
- Created: `2026-07-19`
- Last updated: `2026-07-19`

## Validation Rules
- Task complete only after required validation passes and documentation generation is recorded, unless a waiver is recorded.
- Phase complete only after required validation passes, documentation generation is recorded, required commits/pushes are complete, and required user confirmation is recorded.
- Engine work must be committed inside `engine/` before the root repository commits the updated `engine` submodule pointer.
- Shipping/no-dev-tools behavior must be validated or explicitly waived before completion.

## Repository State
- Root commit/push state: `Planning docs uncommitted`
- Engine commit/push state: `Pending`
- Root submodule pointer update: `Pending engine implementation`
- Root pull request state: `Pending`
- Engine pull request state: `Pending`

## Phase 1: Planning
**Status:** Complete  
**Goal:** Persist an approved feature plan and tracker before implementation.

### Tasks
- [x] Capture user request and feature scope.
  - Status: Complete
  - Repository: `root`
  - Notes: Runtime console command `open #### ####` should clear current world scene content and open the supplied scene stack in order. User clarified this should be entirely an engine feature; root only owns planning/tracking and the eventual submodule pointer update. User also added that predictive lookup should suggest registered BSN scene names for `open` arguments and the debug console should show all predictions in a floating list instead of one predicted command.
- [x] Read mandatory planning, Gitflow, Rust workspace, and Foundation architecture guidance.
  - Status: Complete
  - Repository: `root`
  - Notes: Read required project skills before creating docs.
- [x] Inspect relevant manifests, runtime source, console source, scene source, and previous startup override plan.
  - Status: Complete
  - Repository: `both`
  - Notes: Inspected root README, `game/Cargo.toml`, `engine/Cargo.toml`, engine README/AGENTS, `startup_scene.rs`, `console/mod.rs`, `scene_stack.rs`, and Last Beacon scene registration/startup.
- [x] Create root feature branch.
  - Status: Complete
  - Repository: `root`
  - Notes: Created `feature/runtime-scene-open-command` from root `dev`; verified `origin/dev` is an ancestor of `HEAD`.
- [x] Create plan and tracker documents.
  - Status: Complete
  - Repository: `root`
  - Notes: `docs/plans/runtime-scene-open-command/plan.md` and this tracker created.

### Validation
- Game validation: `N/A for planning-only docs`
- Engine validation: `N/A for planning-only docs`
- Documentation generation: `Pending for implementation; not required for planning-only docs`
- User confirmation: `Pending approval to begin implementation`

## Phase 2: Foundation Runtime Console Command
**Status:** Planned  
**Goal:** Add a dev-only `open` console command that clears current scene content and opens one or more BSN scenes in order.

### Tasks
- [ ] Create/switch engine branch `feature/runtime-scene-open-command` from engine `dev` before edits.
  - Status: Planned
  - Repository: `engine`
  - Notes: Engine is currently detached; no engine edits should occur until this is done.
- [ ] Add shared scene-open command construction for ordered scene keys/paths.
  - Status: Planned
  - Repository: `engine`
  - Notes: First scene must clear stack/world scene content; later scenes open normally above it.
- [ ] Extend console parsing/execution to support `open <scene> [scene ...]` positional arguments.
  - Status: Planned
  - Repository: `engine`
  - Notes: Preserve existing named-parameter command behavior.
- [ ] Return clear errors for missing scene arguments.
  - Status: Planned
  - Repository: `engine`
  - Notes: `open` with no scenes must not silently do nothing.
- [ ] Keep command dev-only/no-dev-tools disabled.
  - Status: Planned
  - Repository: `engine`
  - Notes: Shipping-compatible builds must not include this dev command.
- [ ] Expose registered BSN scene keys for prediction.
  - Status: Planned
  - Repository: `engine`
  - Notes: `FoundationBsnSceneRegistry` should provide deterministic registered-key suggestions for `open` arguments such as `open las` -> `last-beacon/...`. Users may still type direct `.bsn` paths relative to the active assets directory, but predictions should only list registered scene keys.

### Validation
- Engine validation: `Pending`
- Documentation generation: `Pending`
- User confirmation: `Not required until phase completion unless scope changes`

## Phase 3: Console Prediction UI, Tests, And Documentation
**Status:** Planned  
**Goal:** Prove command behavior, expand predictive lookup, and document usage.

### Tasks
- [ ] Add Foundation runtime tests for single-scene open, multi-scene open, no-argument error, BSN scene-key predictions, multi-result prediction ordering, and existing command compatibility.
  - Status: Planned
  - Repository: `engine`
  - Notes: Prefer non-window tests using `World`/`App` and scene command messages/resources.
- [ ] Add no-dev-tools or shipping-compatible validation coverage where practical.
  - Status: Planned
  - Repository: `engine`
  - Notes: Prove command is absent or unavailable without `dev-tools`.
- [ ] Expand debug console suggestion UI from one prediction to a floating multi-result list.
  - Status: Planned
  - Repository: `engine`
  - Notes: The floating list should sit above the history area and show matching command names or registered BSN scene keys.
- [ ] Update engine debug console and/or scene docs.
  - Status: Planned
  - Repository: `engine`
  - Notes: Include `open last-beacon/main_menu`, `open last-beacon/gameplay_level last-beacon/pause_menu`, and `open las` autocomplete examples.

### Validation
- Engine validation: `Pending`
- Documentation generation: `Pending`
- User confirmation: `Not required until phase completion unless docs reveal behavior changes`

## Phase 4: Integration, Validation, Commits, And Handoff
**Status:** Planned  
**Goal:** Validate against Last Beacon, commit/push engine and root changes, and prepare PR-ready handoff.

### Tasks
- [ ] Run focused engine validation.
  - Status: Planned
  - Repository: `engine`
  - Notes: Runtime tests, clippy, docs.
- [ ] Run full engine validation as required.
  - Status: Planned
  - Repository: `engine`
  - Notes: Use `engine/scripts/validate-project.cmd` unless a waiver is approved.
- [ ] Update root submodule pointer after engine commit.
  - Status: Planned
  - Repository: `both`
  - Notes: Record exact engine commit hash first.
- [ ] Run root game validation.
  - Status: Planned
  - Repository: `root`
  - Notes: Use `scripts/validate.cmd` after pointer update only to verify Last Beacon still builds against the updated engine; no Last Beacon runtime source changes are planned.
- [ ] Smoke-test the runtime command in Last Beacon where practical.
  - Status: Planned
  - Repository: `root`
  - Notes: Open console with backtick and execute single- and multi-scene examples.
- [ ] Commit and push engine changes.
  - Status: Planned
  - Repository: `engine`
  - Notes: Push to origin if available.
- [ ] Commit and push root changes, including submodule pointer and tracker updates.
  - Status: Planned
  - Repository: `root`
  - Notes: Push to origin if available.

### Validation
- Game validation: `Pending`
- Engine validation: `Pending`
- Documentation generation: `Pending`
- User confirmation: `Pending final implementation review or optional sanity review request`

## Implementation / Review Handoff Notes
- Use `gpt-5.4` for implementation and `gpt-5.5` for optional final review.
- Never use Anthropic models.
- Before implementation edits, read the plan, this tracker, mandatory skills, and relevant source again.
- Confirm root branch remains `feature/runtime-scene-open-command` and create/switch engine branch from engine `dev` before edits.
- Keep all runtime implementation in `engine/`; do not change Last Beacon source unless the user explicitly expands scope.
- Do not mark complete until validation, docs generation, engine commit hash, root submodule pointer state, and push/PR readiness are recorded.

## Postponed Work
- Quoted scene paths containing spaces are postponed unless the console parser is generalized for shell-like quoting in a separate feature.
- Per-scene presentation options are postponed; scenes use existing default presentation unless opened later by authored menu/runtime systems.

## Notes / Issues / Oversights
- Existing console command parsing expects named `name=value` arguments. The `open scene scene` command requires either a special parsing path or a reusable raw-argument command descriptor mode.
- Clearing all Bevy world entities literally would remove required infrastructure. The plan interprets the request as clearing Foundation scene-stack-owned entities and stack state.
- Current console prediction UI only shows one predicted command. Scope now includes a floating multi-result suggestion list and BSN scene-key predictions for `open` arguments.

## Progress Log
- `2026-07-19`: User requested a runtime console command analogous to `--scene`, using `open #### #### ####`, clearing current entities/world scene content and opening supplied scenes in order.
- `2026-07-19`: Read mandatory planning, Gitflow, Rust workspace, and Foundation architecture skills; inspected relevant manifests and source.
- `2026-07-19`: Created root branch `feature/runtime-scene-open-command` from root `dev` and verified `origin/dev` ancestry.
- `2026-07-19`: Created plan and tracker under `docs/plans/runtime-scene-open-command/`.
- `2026-07-19`: User clarified the feature should be entirely within the engine. Updated plan/tracker classification to `engine` and kept root scope limited to planning/tracking plus eventual submodule pointer validation.
- `2026-07-19`: User added predictive lookup requirements: `open las` should suggest registered BSN scene keys such as `last-beacon/my_map`, and the debug console should show all matching predictions in a floating list instead of only one predicted command.
- `2026-07-19`: User clarified that direct `.bsn` paths are valid explicit `open` inputs, and those direct paths mean paths relative to the active assets directory, but autocomplete predictions should only come from registered scene keys.
