# Launch Scene Stack Overrides Tracker

## Metadata
- Feature slug: `launch-scene-stack-overrides`
- Feature area: `multi-area`
- Primary area: `engine`
- Root branch: `feature/launch-scene-stack-overrides`
- Engine branch: `feature/launch-scene-stack-overrides`
- Root branch base verification: `Verified: created from root dev; origin/dev is an ancestor of HEAD on 2026-07-18`
- Engine branch base verification: `Verified: created from engine dev; origin/dev is an ancestor of HEAD on 2026-07-18`
- Engine submodule pointer: `d3c5b89587c2dc54f9edd06f31d528dda854f79d` bound for implementation; root pointer update pending commit
- Overall status: `Implementation complete; root and engine branches pushed`
- Planning model: `gpt-5.5`
- Preferred implementation model: `gpt-5.4`
- Optional final review model: `gpt-5.5`
- Current handoff state: `Implementation in progress with gpt-5.4`
- Created: `2026-07-18`
- Last updated: `2026-07-18`

## Validation Rules
- Task complete only after required validation passes and documentation generation is recorded, unless a waiver is recorded.
- Phase complete only after required validation passes, documentation generation is recorded, required commits/pushes are complete, and required user confirmation is recorded.
- Engine work must be committed inside `engine/` before the root repository commits the updated `engine` submodule pointer.
- Shipping behavior must be validated or explicitly waived before completion.

## Repository State
- Root commit/push state: `Planning commit 7006fda pushed; integration commit 9f52687 pushed; final tracker push status update pending commit`
- Engine commit/push state: `Committed d3c5b89587c2dc54f9edd06f31d528dda854f79d and pushed to origin/feature/launch-scene-stack-overrides`
- Root submodule pointer update: `Committed and pushed in root integration commit 9f52687; root points at engine d3c5b89587c2dc54f9edd06f31d528dda854f79d`
- Root pull request state: `Pending`
- Engine pull request state: `Pending`

## Phase 1: Planning
**Status:** Complete  
**Goal:** Persist an approved feature plan and tracker before implementation.

### Tasks
- [x] Capture user request and feature scope.
  - Status: Complete
  - Repository: `root`
  - Notes: User confirmed summary on 2026-07-18.
- [x] Read mandatory planning, Gitflow, Rust workspace, and Foundation architecture guidance.
  - Status: Complete
  - Repository: `root`
  - Notes: Read required project skills before creating docs.
- [x] Inspect relevant manifests, launcher/runtime source, docs, and current scene integration.
  - Status: Complete
  - Repository: `both`
  - Notes: Inspected root README, `game/Cargo.toml`, `engine/Cargo.toml`, engine README/AGENTS, Foundation launcher, scene stack, BSN bridge, build docs, and Last Beacon scene startup.
- [x] Create root feature branch.
  - Status: Complete
  - Repository: `root`
  - Notes: Created `feature/launch-scene-stack-overrides` from root `dev`; verified `origin/dev` is an ancestor of `HEAD`.
- [x] Create plan and tracker documents.
  - Status: Complete
  - Repository: `root`
  - Notes: `docs/plans/launch-scene-stack-overrides/plan.md` and this tracker created.

### Validation
- Game validation: `N/A for planning-only docs`
- Engine validation: `N/A for planning-only docs`
- Documentation generation: `Pending for implementation; not required for planning-only docs`
- User confirmation: `Approved on 2026-07-18; user asked to commit plan/tracker, then start implementation`

## Phase 2: Foundation Runtime Startup Override API
**Status:** Complete  
**Goal:** Add reusable non-shipping scene stack override parsing and command emission in Foundation Runtime Library.

### Tasks
- [x] Create/switch engine branch `feature/launch-scene-stack-overrides` from engine `dev` before edits.
  - Status: Complete
  - Repository: `engine`
  - Notes: Created engine branch and verified `origin/dev` is an ancestor of `HEAD`.
- [x] Add `--scene <value>` startup override parsing for either one scene or a bracketed ordered list.
  - Status: Complete
  - Repository: `engine`
  - Notes: Added `startup_scene` parser. Syntax supports `--scene last-beacon/main_menu` and bracketed lists; parser trims whitespace around commas and rejects empty entries clearly.
- [x] Add public startup helper that emits override scene commands or caller-provided default commands.
  - Status: Complete
  - Repository: `engine`
  - Notes: Added `startup_scene_commands_or_default`; it clears the stack for the first override scene and opens later scenes above it.
- [x] Gate override behavior out of shipping/no-dev-tools builds.
  - Status: Complete
  - Repository: `engine`
  - Notes: When `dev-tools` is disabled, the helper returns caller-provided default commands and does not parse override args.
- [x] Add runtime tests.
  - Status: Complete
  - Repository: `engine`
  - Notes: Added parser/default/ordering/error tests and no-default-features fallback coverage.

### Validation
- Engine validation: `Passed focused tests and clippy: runtime all-features startup_scene tests; runtime no-default-features startup_scene tests; runtime clippy all-targets/all-features -D warnings`
- Documentation generation: `Passed cargo doc for foundation-runtime-library and foundation --all-features --no-deps`
- User confirmation: `Not required until phase completion unless scope changes`

## Phase 3: Foundation Launcher And Documentation
**Status:** Complete  
**Goal:** Forward startup scene override arguments through Foundation launcher paths and document usage.

### Tasks
- [x] Update loose Foundation launcher parsing/forwarding for `--scene <value>`.
  - Status: Complete
  - Repository: `engine`
  - Notes: Launcher accepts one `--scene` value, rejects duplicates/missing values, and forwards the value to the selected game.
- [x] Verify Foundation build tool runtime argument forwarding still works after `--`.
  - Status: Complete
  - Repository: `engine`
  - Notes: Existing build-tool runtime delimiter remains the documented path for external game runtime arguments; no code changes required.
- [x] Update engine docs.
  - Status: Complete
  - Repository: `engine`
  - Notes: Updated `foundation-engine.md`, `scene-system.md`, and `build-packaging.md` with startup override examples and shipping notes.
- [x] Add launcher tests.
  - Status: Complete
  - Repository: `engine`
  - Notes: Added launcher parsing tests for bracketed-list value, duplicate scene argument, and missing scene value.

### Validation
- Engine validation: `Passed focused foundation tests and clippy all-targets/all-features -D warnings`
- Documentation generation: `Passed cargo doc for foundation-runtime-library and foundation --all-features --no-deps`
- User confirmation: `Not required until phase completion unless CLI shape changes`

## Phase 4: Last Beacon Integration
**Status:** Complete  
**Goal:** Let Last Beacon use startup override scenes while preserving the current default splash flow.

### Tasks
- [x] Update `game/src/scenes/mod.rs` startup system to use the Foundation helper.
  - Status: Complete
  - Repository: `root`
  - Notes: Default remains `last-beacon/splash_pixel_perfect` with stack clear and fullscreen startup key; malformed overrides log an error and fall back to default startup.
- [x] Add or update Last Beacon tests.
  - Status: Complete
  - Repository: `root`
  - Notes: Added test coverage that default startup commands still open the pixel-perfect splash.
- [x] Record engine commit hash and update root `engine` submodule pointer.
  - Status: Awaiting root commit
  - Repository: `both`
  - Notes: Engine commit `d3c5b89587c2dc54f9edd06f31d528dda854f79d` is committed and pushed; root working tree now points at that engine commit.

### Validation
- Game validation: `Passed scripts/validate.cmd; focused default startup test, no-default-features cargo check, game clippy all-targets/all-features -D warnings, game cargo doc, and scripts/build.cmd test game also passed`
- Engine validation: `Passed engine/scripts/validate-project.cmd plus focused engine tests/clippy/docs`
- Documentation generation: `Passed focused engine/game cargo doc and full wrapper doc validation through validation scripts`
- User confirmation: `Not required until phase completion unless behavior changes`

## Phase 5: Final Validation, Commits, And Handoff
**Status:** Complete  
**Goal:** Prove the feature works, commit/push all required changes, and prepare PR-ready handoff.

### Tasks
- [x] Run focused engine validation and full engine validation as required.
  - Status: Complete
  - Repository: `engine`
  - Notes: Passed focused tests/clippy/docs, no-default-features runtime startup_scene tests, and `engine/scripts/validate-project.cmd`.
- [x] Run focused game validation and root validation.
  - Status: Complete
  - Repository: `root`
  - Notes: Passed focused game tests/checks/clippy/docs, `scripts/validate.cmd`, and `scripts/build.cmd --platform windows-x64 --configuration test --target game`.
- [x] Smoke-test no override, single-scene override, direct asset path, and multi-scene startup where practical.
  - Status: Complete
  - Repository: `root`
  - Notes: `scripts/run.cmd -- --scene last-beacon/main_menu` built and launched until timeout termination. Direct executable smoke runs with `--scene scenes/main_menu.bsn` and `--scene '[last-beacon/gameplay_level, scenes/main_menu.bsn]'` also stayed alive until timeout with `FOUNDATION_ASSET_ROOT` set. Initial script smoke attempts timed out during first build before the separate build completed.
- [x] Commit and push engine changes.
  - Status: Complete
  - Repository: `engine`
  - Notes: Engine commit `d3c5b89587c2dc54f9edd06f31d528dda854f79d` pushed to `origin/feature/launch-scene-stack-overrides`.
- [x] Commit and push root changes, including the submodule pointer, plan, and tracker.
  - Status: Complete
  - Repository: `root`
  - Notes: Root integration commit `9f52687` includes the `engine` pointer update, Last Beacon integration, and tracker updates, and was pushed to origin.

### Validation
- Game validation: `Passed scripts/validate.cmd and focused checks`
- Engine validation: `Passed engine/scripts/validate-project.cmd and focused checks`
- Documentation generation: `Passed focused docs and validation-wrapper docs`
- User confirmation: `Pending final implementation review or optional sanity review request`

## Implementation / Review Handoff Notes
- Use `gpt-5.4` for implementation and `gpt-5.5` for optional final review.
- Never use Anthropic models.
- Before implementation edits, read the plan, this tracker, mandatory skills, and relevant source again.
- Implement the revised CLI shape as one `--scene <value>` argument accepting either a single key/path or a bracketed ordered list. Be safe around commas: trim spaces before/after commas and reject empty list items with a clear error.
- Keep reusable Foundation behavior in `engine/`; keep Last Beacon-specific startup fallback in `game/`.
- Do not mark complete until the tracker records validation, engine commit hash, root submodule pointer state, and push/PR readiness.

## Postponed Work
- Per-scene presentation CLI options are postponed unless user confirms they are required for the first version. Existing `ScenePresentation` remains available for normal code/menu-driven opens.
- A separate editor UI for choosing startup scenes is not part of this feature.

## Notes / Issues / Oversights
- Multiple fullscreen scenes will stack but upper scenes can hide/block lower scenes by existing presentation rules. If the intended test-mode scene must overlay gameplay without blocking visibility/update, the CLI may need presentation options in this or a follow-up feature.

## Progress Log
- `2026-07-18`: User requested a non-shipping launch feature for direct `.bsn` scene stack startup by registered key or asset path, with fallback to the game's default startup scene when no scenes are passed.
- `2026-07-18`: User confirmed the summarized request.
- `2026-07-18`: Created root branch `feature/launch-scene-stack-overrides` from root `dev` and verified `origin/dev` is an ancestor of `HEAD`.
- `2026-07-18`: Plan and tracker created under `docs/plans/launch-scene-stack-overrides/`.
- `2026-07-18`: User revised the desired CLI shape away from repeated `--scene` flags. Plan/tracker now propose one `--scene` value that accepts either a single key/path or a bracketed ordered list.
- `2026-07-18`: User clarified the bracketed-list parser must safely handle spaces before and/or after commas.
- `2026-07-18`: User approved the plan/tracker and asked to commit them before starting implementation.
- `2026-07-18`: Committed and pushed planning docs in root commit `7006fda`.
- `2026-07-18`: Created engine branch `feature/launch-scene-stack-overrides` from engine dev and verified `origin/dev` ancestry.
- `2026-07-18`: Implemented Foundation startup scene override parsing/helper, launcher forwarding, docs, and Last Beacon integration.
- `2026-07-18`: Focused validation passed: engine runtime startup_scene tests with all features and no default features, foundation launcher tests, Last Beacon default startup test, no-default-features game check, clippy for changed engine/game packages, and cargo doc for changed engine/game packages.
- `2026-07-18`: Committed and pushed engine commit `d3c5b89587c2dc54f9edd06f31d528dda854f79d`; root submodule pointer commit remains pending.
- `2026-07-18`: Full validation passed: `scripts/validate.cmd`, `engine/scripts/validate-project.cmd`, `scripts/build.cmd --platform windows-x64 --configuration test --target game`, plus focused no-default-features and smoke checks.
- `2026-07-18`: Smoke runs confirmed registered-key, direct-path, and bracketed-list startup override launches stay alive until timeout termination.
- `2026-07-18`: Created and pushed root integration commit `9f52687` with Last Beacon integration, engine submodule pointer update, and tracker updates.
