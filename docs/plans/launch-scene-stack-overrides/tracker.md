# Launch Scene Stack Overrides Tracker

## Metadata
- Feature slug: `launch-scene-stack-overrides`
- Feature area: `multi-area`
- Primary area: `engine`
- Root branch: `feature/launch-scene-stack-overrides`
- Engine branch: `feature/launch-scene-stack-overrides` planned; pending creation in `engine/`
- Root branch base verification: `Verified: created from root dev; origin/dev is an ancestor of HEAD on 2026-07-18`
- Engine branch base verification: `Pending: engine is detached at dev commit 3688245e962d752a8cc4645a3d4b3fd10834dda2`
- Engine submodule pointer: `3688245e962d752a8cc4645a3d4b3fd10834dda2`
- Overall status: `Implementation approved; preparing engine branch`
- Planning model: `gpt-5.5`
- Preferred implementation model: `gpt-5.4`
- Optional final review model: `gpt-5.5`
- Current handoff state: `Implementation starting with gpt-5.4; engine branch creation pending`
- Created: `2026-07-18`
- Last updated: `2026-07-18`

## Validation Rules
- Task complete only after required validation passes and documentation generation is recorded, unless a waiver is recorded.
- Phase complete only after required validation passes, documentation generation is recorded, required commits/pushes are complete, and required user confirmation is recorded.
- Engine work must be committed inside `engine/` before the root repository commits the updated `engine` submodule pointer.
- Shipping behavior must be validated or explicitly waived before completion.

## Repository State
- Root commit/push state: `Planning docs pending commit before implementation`
- Engine commit/push state: `Pending; no engine edits yet`
- Root submodule pointer update: `Pending; required after engine implementation commit`
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
**Status:** Starting  
**Goal:** Add reusable non-shipping scene stack override parsing and command emission in Foundation Runtime Library.

### Tasks
- [ ] Create/switch engine branch `feature/launch-scene-stack-overrides` from engine `dev` before edits.
  - Status: Planned
  - Repository: `engine`
  - Notes: Required because `engine/` is currently detached.
- [ ] Add `--scene <value>` startup override parsing for either one scene or a bracketed ordered list.
  - Status: Planned
  - Repository: `engine`
  - Notes: Proposed syntax: `--scene last-beacon/main_menu` or `--scene [last-beacon/gameplay_level, scenes/testing_mode.bsn]`; bracketed values may need shell quoting. Parser must trim whitespace around commas and reject empty entries clearly.
- [ ] Add public startup helper that emits override scene commands or caller-provided default commands.
  - Status: Planned
  - Repository: `engine`
  - Notes: Must preserve default behavior when no scenes are supplied.
- [ ] Gate override behavior out of shipping/no-dev-tools builds.
  - Status: Planned
  - Repository: `engine`
  - Notes: Preferred fallback is to ignore overrides and emit default startup commands when `dev-tools` is disabled.
- [ ] Add runtime tests.
  - Status: Planned
  - Repository: `engine`
  - Notes: Cover no override, single registered/direct string, multiple scenes, and shipping/no-dev-tools fallback where practical.

### Validation
- Engine validation: `Pending`
- Documentation generation: `Pending`
- User confirmation: `Not required until phase completion unless scope changes`

## Phase 3: Foundation Launcher And Documentation
**Status:** Planned  
**Goal:** Forward startup scene override arguments through Foundation launcher paths and document usage.

### Tasks
- [ ] Update loose Foundation launcher parsing/forwarding for `--scene <value>`.
  - Status: Planned
  - Repository: `engine`
  - Notes: Preserve existing known flag behavior and unknown argument errors; reject duplicate `--scene` unless implementation finds a strong reason to merge them.
- [ ] Verify Foundation build tool runtime argument forwarding still works after `--`.
  - Status: Planned
  - Repository: `engine`
  - Notes: Update build-tool tests only if existing forwarding behavior is insufficient.
- [ ] Update engine docs.
  - Status: Planned
  - Repository: `engine`
  - Notes: Update `foundation-engine.md`, `scene-system.md`, and `build-packaging.md`.
- [ ] Add launcher tests.
  - Status: Planned
  - Repository: `engine`
  - Notes: Include single value and bracketed-list `--scene` parsing/forwarding expectations.

### Validation
- Engine validation: `Pending`
- Documentation generation: `Pending`
- User confirmation: `Not required until phase completion unless CLI shape changes`

## Phase 4: Last Beacon Integration
**Status:** Planned  
**Goal:** Let Last Beacon use startup override scenes while preserving the current default splash flow.

### Tasks
- [ ] Update `game/src/scenes/mod.rs` startup system to use the Foundation helper.
  - Status: Planned
  - Repository: `root`
  - Notes: Default remains `last-beacon/splash_pixel_perfect` with stack clear and fullscreen startup key.
- [ ] Add or update Last Beacon tests.
  - Status: Planned
  - Repository: `root`
  - Notes: Confirm registered scene catalog and default startup behavior stay stable.
- [ ] Record engine commit hash and update root `engine` submodule pointer.
  - Status: Planned
  - Repository: `both`
  - Notes: Engine commit must happen first.

### Validation
- Game validation: `Pending`
- Engine validation: `Pending from prior engine phases`
- Documentation generation: `Pending`
- User confirmation: `Not required until phase completion unless behavior changes`

## Phase 5: Final Validation, Commits, And Handoff
**Status:** Planned  
**Goal:** Prove the feature works, commit/push all required changes, and prepare PR-ready handoff.

### Tasks
- [ ] Run focused engine validation and full engine validation as required.
  - Status: Planned
  - Repository: `engine`
  - Notes: Include docs and shipping/no-dev-tools evidence.
- [ ] Run focused game validation and root validation.
  - Status: Planned
  - Repository: `root`
  - Notes: Include `scripts/validate.cmd` and focused Cargo tests when useful.
- [ ] Smoke-test no override, single-scene override, direct asset path, and multi-scene startup where practical.
  - Status: Planned
  - Repository: `root`
  - Notes: Interactive runs may need timeout/manual observation.
- [ ] Commit and push engine changes.
  - Status: Planned
  - Repository: `engine`
  - Notes: Record exact engine commit hash.
- [ ] Commit and push root changes, including the submodule pointer, plan, and tracker.
  - Status: Planned
  - Repository: `root`
  - Notes: Root commit must include `engine` pointer update after engine commit.

### Validation
- Game validation: `Pending`
- Engine validation: `Pending`
- Documentation generation: `Pending`
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
- `engine/` is currently detached at `3688245e962d752a8cc4645a3d4b3fd10834dda2`; implementation must create/switch the planned engine branch before edits.
- Multiple fullscreen scenes will stack but upper scenes can hide/block lower scenes by existing presentation rules. If the intended test-mode scene must overlay gameplay without blocking visibility/update, the CLI may need presentation options in this or a follow-up feature.

## Progress Log
- `2026-07-18`: User requested a non-shipping launch feature for direct `.bsn` scene stack startup by registered key or asset path, with fallback to the game's default startup scene when no scenes are passed.
- `2026-07-18`: User confirmed the summarized request.
- `2026-07-18`: Created root branch `feature/launch-scene-stack-overrides` from root `dev` and verified `origin/dev` is an ancestor of `HEAD`.
- `2026-07-18`: Plan and tracker created under `docs/plans/launch-scene-stack-overrides/`.
- `2026-07-18`: User revised the desired CLI shape away from repeated `--scene` flags. Plan/tracker now propose one `--scene` value that accepts either a single key/path or a bracketed ordered list.
- `2026-07-18`: User clarified the bracketed-list parser must safely handle spaces before and/or after commas.
- `2026-07-18`: User approved the plan/tracker and asked to commit them before starting implementation.
