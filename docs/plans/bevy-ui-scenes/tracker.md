# Bevy UI Scenes Tracker

## Metadata
- Feature slug: `bevy-ui-scenes`
- Feature area: `game`
- Primary area: `game`
- Root branch: `feature/bevy-ui-scenes`
- Engine branch: `N/A`
- Root branch base verification: `Rebased onto origin/dev at df9d52a7e2c94203904b8a7b72f96af57d1f6a80 on 2026-07-19`
- Engine branch base verification: `N/A`
- Engine submodule pointer: `1bc59f9a0039dfe412b735c869a90f38a0d58582`
- Overall status: `Shared silo UI styling tweaks awaiting visual review`
- Planning model: `gpt-5.5`
- Preferred implementation model: `gpt-5.4`
- Optional final review model: `gpt-5.5`
- Current handoff state: `Shared silo UI tweaks implemented with gpt-5.4; awaiting user visual review`
- Created: `2026-07-19`
- Last updated: `2026-07-19`

## Validation Rules
- Task complete only after required validation passes and documentation generation is recorded, unless a waiver is recorded.
- Phase complete only after required validation passes, documentation generation is recorded, required commits/pushes are complete, and required user confirmation is recorded.

## Repository State
- Root commit/push state: `Shared silo UI styling commit 7012ce1 pending push; tracker state commit pending; previous Main Menu primary styling commit 9f712c3 pushed to origin/feature/bevy-ui-scenes.`
- Engine commit/push state: `N/A`
- Root submodule pointer update: `N/A`
- Prototype reference state: `Prototype is now included through origin/dev at df9d52a7e2c94203904b8a7b72f96af57d1f6a80, which merged f4d2abb Add UI prototype.`
- Working tree note: `Untracked prototype build artifacts may remain locally under prototypes/ from the prior prototype branch; do not include them in this feature unless explicitly requested.`
- Current tweak state: `Shared Beacon/silo UI chrome and styling pass approved by user for commit.`

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
- [x] User approval to begin implementation.
  - Status: Complete
  - Repository: `root`
  - Notes: User approved the plan and asked that no pull request be created at the end so they can review the branch first.

### Validation
- Game validation: `N/A for planning-only changes`
- Engine validation: `N/A`
- Documentation generation: `Pending for implementation; planning docs created manually.`
- User confirmation: `Received on 2026-07-19`

## Phase 2: Scene Catalog And Navigation
**Status:** Complete  
**Goal:** Register all new scene keys and establish the intended menu/Beacon/gameplay navigation flow.

### Tasks
- [x] Update `game/src/scenes/mod.rs` with new scene constants and registry entries.
  - Status: Complete
  - Repository: `root`
  - Notes: Added Dashboard, Hangar, Garage, Mission Control, Fabrication, and Silo Upgrades scene keys and BSN registrations.
- [x] Update Rust tests for required scene keys and representative registry mappings.
  - Status: Complete
  - Repository: `root`
  - Notes: Preserved splash, credits, gameplay, main menu, settings/options, and pause assertions and added representative new scene mappings.
- [x] Decide final Main Menu button routing.
  - Status: Complete
  - Repository: `root`
  - Notes: Continue/New Game open Dashboard, Quick Run opens the current gameplay level, Settings opens as an overlay, and Credits opens the existing credits scene.

### Validation
- Game validation: `Passed via scripts/validate.cmd on 2026-07-19`
- Engine validation: `N/A`
- Documentation generation: `Passed via cargo doc --manifest-path game/Cargo.toml --all-features --no-deps on 2026-07-19`
- User confirmation: `Implementation approval received; no additional routing confirmation required`

## Phase 3: Prototype-Matched Static UI Scenes
**Status:** Complete  
**Goal:** Replace old UI BSN assets and add new static UI BSN assets that closely match the prototype layout with mock data.

### Tasks
- [x] Replace `game/assets/scenes/main_menu.bsn` with prototype-style Main Menu.
  - Status: Complete
  - Repository: `root`
  - Notes: Added left menu rail, current-save mock panel, credits/settings/gameplay/dashboard flow, and simple 3D background component.
- [x] Replace `game/assets/scenes/options_menu.bsn` with prototype-style Settings Menu.
  - Status: Complete
  - Repository: `root`
  - Notes: Added static mock settings groups and tabs/sections; no real settings persistence.
- [x] Add Dashboard scene BSN.
  - Status: Complete
  - Repository: `root`
  - Notes: Added resources, colony needs, equipped robot panels, and simple 3D background component.
- [x] Add Hangar scene BSN.
  - Status: Complete
  - Repository: `root`
  - Notes: Added deployment display placeholder and Launch Expedition button to the current gameplay level.
- [x] Add Garage scene BSN.
  - Status: Complete
  - Repository: `root`
  - Notes: Added robot roster cards and selected robot mock stats.
- [x] Add Mission Control scene BSN.
  - Status: Complete
  - Repository: `root`
  - Notes: Added static main/side/passive mission lists and selected mission detail panel.
- [x] Add Fabrication scene BSN.
  - Status: Complete
  - Repository: `root`
  - Notes: Added static module browser, robot stat deltas, and feature list.
- [x] Add Silo Upgrades scene BSN.
  - Status: Complete
  - Repository: `root`
  - Notes: Added static upgrade tree and selected-node detail panel.
- [x] Replace `game/assets/scenes/pause_menu.bsn` with prototype-style Pause Menu.
  - Status: Complete
  - Repository: `root`
  - Notes: Added Resume, Abandon Run, Settings, Save and Quit to Menu, Save and Quit Game, and current expedition mock stats. The pause scene does not spawn its own 3D background so the current gameplay level remains visible underneath.
- [x] Preserve splash screens, credits scene, and current gameplay level.
  - Status: Complete
  - Repository: `root`
  - Notes: `gameplay_level.bsn`, splash BSN files, and credits BSN file were not changed; gameplay still opens `last-beacon/pause_menu`.

### Validation
- Game validation: `Passed via scripts/validate.cmd on 2026-07-19`
- Engine validation: `N/A`
- Documentation generation: `Passed via cargo doc --manifest-path game/Cargo.toml --all-features --no-deps on 2026-07-19`
- User confirmation: `Runtime smoke checks launched each new scene with --scene and found no BSN parse/load errors before timeout termination.`

## Phase 4: Validation, Commit, And Push
**Status:** Awaiting final tracker commit  
**Goal:** Prove the feature builds, document validation, and prepare the branch for pull request review.

### Tasks
- [x] Run focused Rust checks.
  - Status: Complete
  - Repository: `root`
  - Notes: Passed `cargo fmt --manifest-path game/Cargo.toml -- --check`, `cargo clippy --manifest-path game/Cargo.toml --all-targets --all-features -- -D warnings`, `cargo test --manifest-path game/Cargo.toml --all-features`, `cargo build --manifest-path game/Cargo.toml --all-features`, and `cargo doc --manifest-path game/Cargo.toml --all-features --no-deps`.
- [x] Run root game validation.
  - Status: Complete
  - Repository: `root`
  - Notes: Passed `scripts/validate.cmd`.
- [x] Manually verify scene flow when practical.
  - Status: Complete
  - Repository: `root`
  - Notes: Smoke-launched Main Menu, Settings Menu, Dashboard, Hangar, Garage, Mission Control, Fabrication, Silo Upgrades, and Gameplay Level with `cargo run --manifest-path game/Cargo.toml -- --log-inline --scene <scene>`. Runs were terminated by timeout after startup and showed no BSN parse/load errors.
- [x] Commit completed work with required commit message format.
  - Status: Complete
  - Repository: `root`
  - Notes: Implementation commit `daaf8f6 Add Bevy UI scenes` includes the required changed file list.
- [x] Push `feature/bevy-ui-scenes` to origin.
  - Status: Complete
  - Repository: `root`
  - Notes: Pushed implementation commit `daaf8f6` to `origin/feature/bevy-ui-scenes`. No pull request was created per user request.

### Validation
- Game validation: `Passed scripts/validate.cmd on 2026-07-19`
- Engine validation: `N/A`
- Documentation generation: `Passed cargo doc --manifest-path game/Cargo.toml --all-features --no-deps on 2026-07-19`
- User confirmation: `Pending final implementation review/acceptance; no pull request will be created before user review.`

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
- The feature branch was originally created from older local `dev` at c3aa296820dc54dc69e38e88dc065b84b878e208, then rebased onto latest `origin/dev` at df9d52a7e2c94203904b8a7b72f96af57d1f6a80 after the prototype branch was merged.
- The old scene name `options_menu` may remain as the internal key for Settings Menu to minimize engine/menu integration churn, even though the user-facing label should be Settings Menu.
- Main Menu styling now starts a reusable BSN widget library under `game/assets/ui/widgets/main_menu/`. The first implementation adds game-owned `LastBeaconBsnWidget` slots so scenes can compose widget BSN assets without Foundation Engine changes.
- Dedicated widget assets currently cover Main Menu brand, menu buttons, current save panel, and footer. Other scenes still use the earlier static layout and should be migrated as follow-up tweaks.
- Shared silo UI pass is now targeting the Svelte website's Beacon layout: absolute 16:9 frame feel, centered top tab navigation, resource chip group, 2px slate panel borders, slate-900 panel fills, cyan active tab underline, compact uppercase labels, and floating/edge panels over the existing simple 3D stand-in.
- Added game-owned Beacon button style markers so generic Foundation button interaction colors do not overwrite cyan primary actions or transparent Beacon top-nav tabs.
- Updated BSN asset flow tests to register the new marker types, wait long enough for all BSN assets to load, and serialize the asset-flow tests because the Bevy asset pipeline is shared enough for these app-level tests to race when run concurrently.
- Main Menu widgets were revised to match the Svelte prototype more closely: Tailwind slate palette values, `#fbbf24` menu accent, `rounded-sm`-style 2px radius, button border/padding proportions, rail width/padding/gaps, and NotoSans font application.
- Continue is explicitly treated as the primary menu button and now has a game-owned style enforcement marker so generic Foundation button interaction styling cannot override its yellow background. The Main Menu left rail and viewport placeholder no longer author visible borders.
- The Main Menu smoke run was terminated by timeout after startup and showed no BSN parse/load/apply errors; this confirms startup loading but is not a human visual review.

## Progress Log
- `2026-07-19`: User reviewed the first UI pass and requested reusable BSN widgets in a dedicated assets directory, starting with Main Menu styling.
- `2026-07-19`: Added game-owned BSN widget composition support, moved Main Menu pieces into `game/assets/ui/widgets/main_menu/`, rewrote `main_menu.bsn` to compose those widgets, and validated the focused checks plus root validation.
- `2026-07-19`: Committed and pushed Main Menu widget refinement as `5735044 Refine main menu widgets`; no pull request created.
- `2026-07-19`: Fetched latest origin and rebased `feature/bevy-ui-scenes` onto `origin/dev` at df9d52a7e2c94203904b8a7b72f96af57d1f6a80 so the prototype merge is beneath the feature changes.
- `2026-07-19`: Refined Main Menu widget colors, button shape, font handling, padding, margins, and panel sizing to better match the Svelte prototype; validation passed and commit `9fd1d5e Match main menu prototype style` was pushed.
- `2026-07-19`: Made Continue persist as a yellow primary button with black text and removed visible borders from the Main Menu rail and viewport placeholder; focused validation passed and commit `9f712c3 Refine main menu primary styling` was pushed.
- `2026-07-19`: User clarified that the shared silo UI should be fixed to match the website more closely; implementation resumed for Dashboard, Hangar, Garage, Mission Control, Fabrication, and Silo Upgrades shared chrome/styling.
- `2026-07-19`: Reworked shared Beacon/silo scene chrome for Dashboard, Hangar, Garage, Mission Control, Fabrication, and Silo Upgrades to better match the Svelte website; passed `cargo fmt --manifest-path game/Cargo.toml -- --check`, `cargo clippy --manifest-path game/Cargo.toml --all-targets --all-features -- -D warnings`, `cargo test --manifest-path game/Cargo.toml --all-features`, and `cargo doc --manifest-path game/Cargo.toml --all-features --no-deps`.
- `2026-07-19`: User approved the shared Beacon/silo UI styling pass for commit.
- `2026-07-19`: Committed shared Beacon/silo UI styling pass as `7012ce1 Match shared silo UI prototype style`; tracker state commit and push pending.
- `2026-07-19`: Created `feature/bevy-ui-scenes` from `dev`.
- `2026-07-19`: Confirmed user scope, including preserving current gameplay level and replacing only the pause menu used by gameplay.
- `2026-07-19`: Created plan and tracker for user review.
- `2026-07-19`: User approved implementation and requested that no pull request be created before their review.
- `2026-07-19`: Implemented registered Bevy/BSN UI scenes, replaced the old Main Menu/Settings/Pause assets, preserved splash/credits/current gameplay level, and validated focused checks plus root validation.
- `2026-07-19`: Committed and pushed implementation as `daaf8f6 Add Bevy UI scenes`; no pull request created.
