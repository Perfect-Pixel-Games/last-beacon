# Runtime Scene Open Command Tracker

## Metadata
- Feature slug: `runtime-scene-open-command`
- Feature area: `engine`
- Primary area: `engine`
- Root branch: `feature/runtime-scene-open-command`
- Engine branch: `feature/runtime-scene-open-command`
- Root branch base verification: `Verified: created from root dev; origin/dev is an ancestor of HEAD on 2026-07-19`
- Engine branch base verification: `Verified: created from engine dev; origin/dev is an ancestor of HEAD on 2026-07-19`
- Engine submodule pointer: `9348e08d6f4af507643343a4e534862b88f5575c` bound for history-click/blank-preview fix after engine commit
- Overall status: `Implementation complete; interactive smoke verification pending`
- Planning model: `gpt-5.5`
- Preferred implementation model: `gpt-5.4`
- Optional final review model: `gpt-5.5`
- Current handoff state: `Implementation in progress with gpt-5.4`
- Created: `2026-07-19`
- Last updated: `2026-07-19`

## Validation Rules
- Task complete only after required validation passes and documentation generation is recorded, unless a waiver is recorded.
- Phase complete only after required validation passes, documentation generation is recorded, required commits/pushes are complete, and required user confirmation is recorded.
- Engine work must be committed inside `engine/` before the root repository commits the updated `engine` submodule pointer.
- Shipping/no-dev-tools behavior must be validated or explicitly waived before completion.

## Repository State
- Root commit/push state: `Planning commit 78da598 pushed; root pointer/tracker commit 0f388dd pushed; tracker status commit 5408898 pushed; follow-up pointer/tracker commit 1e29599 pushed; click-to-reuse pointer/tracker commit 3cf75f2 pushed; history-click gating pointer/tracker commit b5d5955 pushed; history-click fix pointer/tracker commit pending`
- Engine commit/push state: `Committed engine work through history-click/blank-preview fix 9348e08d6f4af507643343a4e534862b88f5575c; all pushed to origin/feature/runtime-scene-open-command`
- Root submodule pointer update: `Pending history-click/blank-preview fix root commit after validation; working tree points at engine 9348e08d6f4af507643343a4e534862b88f5575c`
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
**Status:** Complete  
**Goal:** Add a dev-only `open` console command that clears current scene content and opens one or more BSN scenes in order.

### Tasks
- [x] Create/switch engine branch `feature/runtime-scene-open-command` from engine `dev` before edits.
  - Status: Complete
  - Repository: `engine`
  - Notes: Created engine branch from detached dev commit and verified `origin/dev` is an ancestor of `HEAD`.
- [x] Add shared scene-open command construction for ordered scene keys/paths.
  - Status: Complete
  - Repository: `engine`
  - Notes: Added `runtime_open_scene_commands`; first scene uses `OpenSceneOptions::clear_stack()`, later scenes open normally above it.
- [x] Extend console parsing/execution to support `open <scene> [scene ...]` positional arguments.
  - Status: Complete
  - Repository: `engine`
  - Notes: `FoundationConsoleRegistry::execute_command_line` special-cases the built-in `open` command while preserving existing macro/named-parameter command execution.
- [x] Return clear errors for missing scene arguments.
  - Status: Complete
  - Repository: `engine`
  - Notes: `open` with no scenes returns `ConsoleCommandError::MissingOpenSceneArgument`.
- [x] Keep command dev-only/no-dev-tools disabled.
  - Status: Complete
  - Repository: `engine`
  - Notes: The command lives in the `console` module, which is compiled only with `dev-tools`; no-default-features tests passed.
- [x] Expose registered BSN scene keys for prediction.
  - Status: Complete
  - Repository: `engine`
  - Notes: `FoundationBsnSceneRegistry` now exposes deterministic registered-key listing and prefix matching. Users may still type direct `.bsn` paths relative to the active assets directory, but predictions only list registered scene keys.

### Validation
- Engine validation: `Passed focused runtime console tests, full runtime all-features tests, no-default-features runtime tests, clippy, cargo doc, and engine/scripts/validate-project.cmd`
- Documentation generation: `Passed cargo doc --manifest-path engine/Cargo.toml -p foundation-runtime-library --all-features --no-deps and engine validation doc generation`
- User confirmation: `Not required until phase completion unless scope changes`

## Phase 3: Console Prediction UI, Tests, And Documentation
**Status:** Complete  
**Goal:** Prove command behavior, expand predictive lookup, and document usage.

### Tasks
- [x] Add Foundation runtime tests for single-scene open, multi-scene open, no-argument error, BSN scene-key predictions, multi-result prediction ordering, and existing command compatibility.
  - Status: Complete
  - Repository: `engine`
  - Notes: Added focused console tests for ordered open commands, executing against scene stack, missing arguments, registered-key predictions, and command suggestions.
- [x] Add no-dev-tools or shipping-compatible validation coverage where practical.
  - Status: Complete
  - Repository: `engine`
  - Notes: `cargo test --manifest-path engine/Cargo.toml -p foundation-runtime-library --no-default-features` passed, proving the dev-only console module is absent in no-dev-tools builds.
- [x] Expand debug console suggestion UI from one prediction to a floating multi-result list.
  - Status: Complete
  - Repository: `engine`
  - Notes: Reworked `FoundationConsoleSuggestion` as a hidden-when-empty floating suggestions box above the input/history area, showing all matching command names or registered BSN scene keys. Follow-up chained console input/suggestion systems so previews update as the user types, changed command matching from prefix to contains, and starts scene-key previews while `open` is still partially typed.
- [x] Update engine debug console and/or scene docs.
  - Status: Complete
  - Repository: `engine`
  - Notes: Updated `engine/docs/debug-console.md` and `engine/docs/scene-system.md` with `open last-beacon/main_menu`, `open last-beacon/gameplay_level last-beacon/pause_menu`, direct asset-relative path, and autocomplete examples.

### Validation
- Engine validation: `Passed focused runtime console tests, full runtime all-features tests, no-default-features runtime tests, clippy, cargo doc, and engine/scripts/validate-project.cmd`
- Documentation generation: `Passed cargo doc --manifest-path engine/Cargo.toml -p foundation-runtime-library --all-features --no-deps and engine validation doc generation`
- User confirmation: `Not required until phase completion unless docs reveal behavior changes`

## Phase 4: Integration, Validation, Commits, And Handoff
**Status:** Planned  
**Goal:** Validate against Last Beacon, commit/push engine and root changes, and prepare PR-ready handoff.

### Tasks
- [x] Run focused engine validation.
  - Status: Complete
  - Repository: `engine`
  - Notes: Passed format check, focused console tests, full runtime all-features tests, no-default-features runtime tests, clippy, and cargo doc.
- [x] Run full engine validation as required.
  - Status: Complete
  - Repository: `engine`
  - Notes: Passed `engine/scripts/validate-project.cmd`.
- [ ] Update root submodule pointer after engine commit.
  - Status: Awaiting history-click/blank-preview fix root commit
  - Repository: `both`
  - Notes: Engine commits through history-click/blank-preview fix `9348e08d6f4af507643343a4e534862b88f5575c` are committed and pushed; root working tree now points at the fix engine commit.
- [x] Run root game validation.
  - Status: Complete
  - Repository: `root`
  - Notes: Passed `scripts/validate.cmd` after initial pointer update, prediction follow-up pointer update, click-to-reuse pointer update, history-click gating pointer update, and history-click/blank-preview fix pointer update; no Last Beacon runtime source changes were made.
- [ ] Smoke-test the runtime command in Last Beacon where practical.
  - Status: Pending manual verification
  - Repository: `root`
  - Notes: Automated engine tests covered command behavior and Last Beacon validation passed. Interactive console smoke remains pending because it requires opening the game, toggling the console, and typing commands.
- [x] Commit and push engine changes.
  - Status: Complete
  - Repository: `engine`
  - Notes: Engine commits through `9348e08d6f4af507643343a4e534862b88f5575c` pushed to `origin/feature/runtime-scene-open-command`.
- [x] Commit and push root changes, including submodule pointer and tracker updates.
  - Status: Complete
  - Repository: `root`
  - Notes: Root commits `0f388dd`, follow-up `1e29599`, click-to-reuse `3cf75f2`, and history-click gating `b5d5955` with submodule pointer/tracker updates pushed to `origin/feature/runtime-scene-open-command`.

### Validation
- Game validation: `Passed scripts/validate.cmd after initial, prediction follow-up, click-to-reuse, history-click gating, and history-click/blank-preview fix pointer updates`
- Engine validation: `Passed focused checks and engine/scripts/validate-project.cmd; follow-up engine/scripts/validate-project.cmd also passed`
- Documentation generation: `Passed focused cargo doc and engine validation doc generation for initial and follow-up commits`},{
- User confirmation: `Pending user/manual smoke verification or optional sanity review request`

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
- Interactive Last Beacon smoke verification is pending manual/user verification because automated tests cover the command behavior without opening a game window, while using the console requires runtime UI input.

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
- `2026-07-19`: User approved the plan/tracker, requested committing them, then starting implementation.
- `2026-07-19`: Committed and pushed root planning commit `78da598`.
- `2026-07-19`: Created engine branch `feature/runtime-scene-open-command` and verified `origin/dev` ancestry; implementation started with gpt-5.4.
- `2026-07-19`: Implemented engine-only runtime `open` console command, registered BSN scene-key prediction, and floating multi-result console suggestions.
- `2026-07-19`: Focused validation passed: format check, console tests, runtime all-features tests, runtime no-default-features tests, clippy, and cargo doc.
- `2026-07-19`: Full engine validation passed with `engine/scripts/validate-project.cmd`.
- `2026-07-19`: Committed and pushed engine commit `de6265a543d91d0561761df5437544b2373dd2b5`.
- `2026-07-19`: Root validation passed with `scripts/validate.cmd` against engine commit `de6265a543d91d0561761df5437544b2373dd2b5`; interactive console smoke remains pending manual verification.
- `2026-07-19`: Committed and pushed root submodule pointer/tracker commit `0f388dd`.
- `2026-07-19`: User reported follow-up console prediction issues: previews should update while typing, command predictions should match text contained anywhere (for example `op`), and scene-key predictions for `open ...` should appear as soon as the user starts typing the `open` command rather than only after `open` is fully written.
- `2026-07-19`: Implemented follow-up prediction behavior: chained console UI update systems, command contains matching, scene-key contains matching, full-command displays such as `open last-beacon/mapmap`, and partial `open` previews while typing `op`.
- `2026-07-19`: Follow-up focused validation passed: console tests, format check, clippy, cargo doc, and full `engine/scripts/validate-project.cmd`.
- `2026-07-19`: Committed and pushed engine follow-up commit `d23603785bb39f8a75ef151bc5d111ef45f4e945`.
- `2026-07-19`: Root validation passed with `scripts/validate.cmd` against follow-up engine commit `d23603785bb39f8a75ef151bc5d111ef45f4e945`.
- `2026-07-19`: Committed and pushed root follow-up submodule pointer/tracker commit `1e29599`.
- `2026-07-19`: User confirmed prediction behavior works perfectly and requested one small addition: users should be able to click preview options or history/log items to reuse them instead of only using Up/Down arrows.
- `2026-07-19`: Implemented click-to-reuse for console suggestion options and command history items, keeping the input focused after a click.
- `2026-07-19`: Click-to-reuse validation passed with focused console tests and full `engine/scripts/validate-project.cmd`.
- `2026-07-19`: Committed and pushed engine click-to-reuse follow-up commit `ef8a5810c3d341c070bb052efcf9005a01b43a4e`.
- `2026-07-19`: Root validation passed with `scripts/validate.cmd` against click-to-reuse engine commit `ef8a5810c3d341c070bb052efcf9005a01b43a4e`.
- `2026-07-19`: Committed and pushed root click-to-reuse submodule pointer/tracker commit `3cf75f2`.
- `2026-07-19`: User confirmed click-to-reuse looks good but clarified that console history must also be clickable like preview items, and history clicking should be disabled while the preview/suggestion list is open.
- `2026-07-19`: Implemented history click gating so history entries can refill the input only when the preview is hidden; preview item clicks still work while preview is open.
- `2026-07-19`: History-click gating validation passed with focused console tests, clippy, cargo doc, and full `engine/scripts/validate-project.cmd`.
- `2026-07-19`: Committed and pushed engine history-click gating follow-up commit `64825f052ec00f5f2c887f781389343dd5584498`.
- `2026-07-19`: Root validation passed with `scripts/validate.cmd` against history-click gating engine commit `64825f052ec00f5f2c887f781389343dd5584498`.
- `2026-07-19`: Committed and pushed root history-click gating submodule pointer/tracker commit `b5d5955`.
- `2026-07-19`: User reported preview item clicks work but history item clicks do not, and requested previews be hidden when input is blank or whitespace-only. Implementation resumed for a follow-up fix.
- `2026-07-19`: Implemented fix by building clickable history items immediately when the console opens and hiding autocomplete suggestions for blank/whitespace-only input.
- `2026-07-19`: Fix validation passed with focused console tests, clippy, cargo doc, and full `engine/scripts/validate-project.cmd`.
- `2026-07-19`: Committed and pushed engine fix commit `9348e08d6f4af507643343a4e534862b88f5575c`.
- `2026-07-19`: Root validation passed with `scripts/validate.cmd` against history-click/blank-preview fix engine commit `9348e08d6f4af507643343a4e534862b88f5575c`.
