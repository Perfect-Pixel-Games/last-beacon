# Bevy UI Scenes Tracker

## Metadata
- Feature slug: `bevy-ui-scenes`
- Feature area: `game`
- Primary area: `game`
- Root branch: `feature/bevy-ui-scenes`
- Engine branch: `N/A`
- Root branch base verification: `Verified against dev at c3aa296820dc54dc69e38e88dc065b84b878e208 on 2026-07-19`
- Engine branch base verification: `N/A`
- Engine submodule pointer: `1bc59f9a0039dfe412b735c869a90f38a0d58582`
- Overall status: `Planned`
- Planning model: `gpt-5.5`
- Preferred implementation model: `gpt-5.4`
- Optional final review model: `gpt-5.5`
- Current handoff state: `Ready for user review before gpt-5.4 implementation`
- Created: `2026-07-19`
- Last updated: `2026-07-19`

## Validation Rules
- Task complete only after required validation passes and documentation generation is recorded, unless a waiver is recorded.
- Phase complete only after required validation passes, documentation generation is recorded, required commits/pushes are complete, and required user confirmation is recorded.

## Repository State
- Root commit/push state: `Planning docs uncommitted; implementation not started.`
- Engine commit/push state: `N/A`
- Root submodule pointer update: `N/A`
- Prototype reference state: `Prototype is available on local branch feature/ui-prototype at f4d2abb Add UI prototype, but is not part of this feature branch from dev.`
- Working tree note: `Untracked prototype build artifacts may remain locally under prototypes/ from the prior prototype branch; do not include them in this feature unless explicitly requested.`

## Phase 1: Planning
**Status:** In progress  
**Goal:** Capture approved scope, branch state, affected files, and validation plan before implementation.

### Tasks
- [x] Confirm user-requested scope.
  - Status: Complete
  - Repository: `root`
  - Notes: User confirmed summary and added that the current gameplay level must remain while using the new pause menu.
- [x] Read required project skills.
  - Status: Complete
  - Repository: `root`
  - Notes: Read feature planning, gitflow, Foundation architecture, Rust workspace, and Rust coding standards guidance.
- [x] Inspect relevant manifests and scene files.
  - Status: Complete
  - Repository: `root`
  - Notes: Inspected `README.md`, `game/Cargo.toml`, `game/src/scenes/mod.rs`, current menu/gameplay BSN assets, and prototype references from `feature/ui-prototype`.
- [x] Create feature plan and tracker.
  - Status: Complete
  - Repository: `root`
  - Notes: Created `docs/plans/bevy-ui-scenes/plan.md` and this tracker.
- [ ] User approval to begin implementation.
  - Status: Pending
  - Repository: `root`
  - Notes: Required before code or BSN implementation edits.

### Validation
- Game validation: `N/A for planning-only changes`
- Engine validation: `N/A`
- Documentation generation: `Pending for implementation; planning docs created manually.`
- User confirmation: `Pending`

## Phase 2: Scene Catalog And Navigation
**Status:** Planned  
**Goal:** Register all new scene keys and establish the intended menu/Beacon/gameplay navigation flow.

### Tasks
- [ ] Update `game/src/scenes/mod.rs` with new scene constants and registry entries.
  - Status: Planned
  - Repository: `root`
  - Notes: Add Dashboard, Hangar, Garage, Mission Control, Fabrication, and Silo Upgrades scene keys.
- [ ] Update Rust tests for required scene keys and representative registry mappings.
  - Status: Planned
  - Repository: `root`
  - Notes: Preserve assertions for splash, credits, gameplay, main menu, settings/options, and pause.
- [ ] Decide final Main Menu button routing.
  - Status: Planned
  - Repository: `root`
  - Notes: Proposed default is Continue/New Game to Dashboard and Quick Run to current gameplay level.

### Validation
- Game validation: `Pending`
- Engine validation: `N/A`
- Documentation generation: `Pending`
- User confirmation: `Not required after implementation approval unless routing question remains unresolved`

## Phase 3: Prototype-Matched Static UI Scenes
**Status:** Planned  
**Goal:** Replace old UI BSN assets and add new static UI BSN assets that closely match the prototype layout with mock data.

### Tasks
- [ ] Replace `game/assets/scenes/main_menu.bsn` with prototype-style Main Menu.
  - Status: Planned
  - Repository: `root`
  - Notes: Left menu rail, current-save mock panel, credits/settings/gameplay/dashboard flow.
- [ ] Replace `game/assets/scenes/options_menu.bsn` with prototype-style Settings Menu.
  - Status: Planned
  - Repository: `root`
  - Notes: Static mock settings groups and tabs/sections; no real settings persistence.
- [ ] Add Dashboard scene BSN.
  - Status: Planned
  - Repository: `root`
  - Notes: Resource, colony needs, and equipped robot panels over gameplay/simple 3D background.
- [ ] Add Hangar scene BSN.
  - Status: Planned
  - Repository: `root`
  - Notes: Deployment/map-area placeholder should be a UI placeholder only; Launch Expedition opens current gameplay level.
- [ ] Add Garage scene BSN.
  - Status: Planned
  - Repository: `root`
  - Notes: Robot roster cards and selected robot mock stats.
- [ ] Add Mission Control scene BSN.
  - Status: Planned
  - Repository: `root`
  - Notes: Static main/side/passive mission lists and selected mission detail panel.
- [ ] Add Fabrication scene BSN.
  - Status: Planned
  - Repository: `root`
  - Notes: Static module browser, robot stat deltas, and feature list.
- [ ] Add Silo Upgrades scene BSN.
  - Status: Planned
  - Repository: `root`
  - Notes: Static upgrade tree/cards and selected-node detail panel.
- [ ] Replace `game/assets/scenes/pause_menu.bsn` with prototype-style Pause Menu.
  - Status: Planned
  - Repository: `root`
  - Notes: Resume, abandon/save-to-menu/settings buttons and current expedition mock stats.
- [ ] Preserve splash screens, credits scene, and current gameplay level.
  - Status: Planned
  - Repository: `root`
  - Notes: `gameplay_level.bsn` should still use current gameplay level and open the new pause menu.

### Validation
- Game validation: `Pending`
- Engine validation: `N/A`
- Documentation generation: `Pending`
- User confirmation: `Manual scene-flow confirmation required when practical`

## Phase 4: Validation, Commit, And Push
**Status:** Planned  
**Goal:** Prove the feature builds, document validation, and prepare the branch for pull request review.

### Tasks
- [ ] Run focused Rust checks.
  - Status: Planned
  - Repository: `root`
  - Notes: `cargo fmt`, `cargo clippy`, `cargo test`, `cargo build`, and `cargo doc` with `--manifest-path game/Cargo.toml`.
- [ ] Run root game validation.
  - Status: Planned
  - Repository: `root`
  - Notes: `scripts/validate.cmd` unless a user-approved waiver is recorded.
- [ ] Manually verify scene flow when practical.
  - Status: Planned
  - Repository: `root`
  - Notes: Launch Main Menu and Gameplay Level with `--scene` overrides; verify Pause Menu from gameplay.
- [ ] Commit completed work with required commit message format.
  - Status: Planned
  - Repository: `root`
  - Notes: Include changed file list in commit body.
- [ ] Push `feature/bevy-ui-scenes` to origin.
  - Status: Planned
  - Repository: `root`
  - Notes: Required because origin is configured.

### Validation
- Game validation: `Pending`
- Engine validation: `N/A`
- Documentation generation: `Pending`
- User confirmation: `Pending final implementation review/acceptance`

## Implementation / Review Handoff Notes
- Use `gpt-5.4` for implementation and `gpt-5.5` for optional final review.
- Never use Anthropic models.
- Do not implement until the user confirms this plan.
- Do not edit `engine/` unless a revised plan/tracker explicitly adds engine work and a valid engine feature branch is created.
- Use `git show feature/ui-prototype:<path>` to inspect prototype source if it is not merged into the implementation branch.
- Preserve `game/assets/scenes/credits.bsn`, `pixel_perfect_splash.bsn`, `bevy_splash.bsn`, and current gameplay behavior.
- Keep all UI data static/mock for this feature.

## Postponed Work
- Hooking UI to real save, settings, colony, mission, robot, fabrication, or upgrade data is postponed because the user requested mock data only.
- Recreating the prototype's placeholder map or background art is postponed/rejected because the user requested a 3D/gameplay-style background instead.
- Dynamic tab, selection, mission toggle, module selection, and upgrade selection behavior may be postponed unless achievable with existing Foundation menu buttons without new runtime systems.

## Notes / Issues / Oversights
- The feature branch was created from `dev` after the prototype branch was committed and pushed. Because the prototype work is not on `dev`, implementation must reference `feature/ui-prototype` explicitly or wait for the prototype branch to be merged.
- The old scene name `options_menu` may remain as the internal key for Settings Menu to minimize engine/menu integration churn, even though the user-facing label should be Settings Menu.

## Progress Log
- `2026-07-19`: Created `feature/bevy-ui-scenes` from `dev`.
- `2026-07-19`: Confirmed user scope, including preserving current gameplay level and replacing only the pause menu used by gameplay.
- `2026-07-19`: Created plan and tracker for user review.
