# Gameplay Space Separation Tracker

## Metadata
- Feature slug: `gameplay-space-separation`
- Feature area: `game`
- Primary area: `game`
- Root branch: `feature/gameplay-space-separation`
- Engine branch: `N/A`
- Root branch base verification: `Verified` (created from `dev` at a clean working tree, 2026-09-12)
- Engine branch base verification: `N/A`
- Engine submodule pointer: `N/A`
- Overall status: `Implemented`
- Planning model: `gpt-5.5` (role fulfilled by Claude Sonnet 5)
- Preferred implementation model: `gpt-5.4` (role fulfilled by Claude Sonnet 5)
- Optional final review model: `gpt-5.5` (role fulfilled by Claude Sonnet 5)
- Current handoff state: `Ready for gpt-5.5 sanity review (optional; not yet requested by user)`
- Created: `2026-09-12`
- Last updated: `2026-09-12`

## Validation Rules
- Task complete only after required validation passes and documentation generation is recorded, unless a waiver is recorded.
- Phase complete only after required validation passes, documentation generation is recorded, required commits/pushes are complete, and required user confirmation is recorded.

## Repository State
- Root commit/push state: `Pending` (implementation not yet committed)
- Engine commit/push state: `N/A`
- Root submodule pointer update: `N/A`

## Phase 1: Shared communication layer and space detection
**Status:** Complete
**Goal:** `game/src/shared/mod.rs` exists with `LastBeaconSharedGameplayState`, `LastBeaconGameplayEvent`, `LastBeaconGameplaySpace`, its resolution system, and the two run-conditions, all wired via `LastBeaconSharedGameplayPlugin` into `LastBeaconPlugin`, with a passing resolution test.

### Tasks
- [x] Add `game/src/shared/mod.rs` with `LastBeaconSharedGameplayState` resource and `LastBeaconGameplayEvent` message enum
  - Status: Complete
  - Repository: `root`
  - Notes: Both intentionally empty (no fields/variants) per the "pure plumbing" scope decision.
- [x] Add `LastBeaconGameplaySpace` resource + resolution system reading `SceneStack`
  - Status: Complete
  - Repository: `root`
  - Notes: **Deviation from the plan's phrasing, found during implementation:** resolution does not need to enumerate every Beacon page key (`DASHBOARD_SCENE`, `HANGAR_SCENE`, etc.). Confirmed via `game/assets/scenes/main_menu.bsn` (`stack_key: "beacon-shell"`) and `game/src/scenes/mod.rs`'s `open_beacon_page` that `BEACON_SCENE` is opened once as a persistent shell under stack key `beacon-shell`, and Beacon pages only ever replace the separate `beacon-page` stack key -- so `BEACON_SCENE` itself stays on the stack for the entire time any Beacon page is showing. Checking for `BEACON_SCENE`'s presence alone is therefore sufficient and simpler than enumerating page keys, and was used instead. Also confirmed via `game/assets/scenes/hangar.bsn`'s `LAUNCH EXPEDITION` button (`action: "clear_and_open_scene"`) that Hub and World are mutually exclusive on the stack by construction (launching clears the whole stack before opening `GAMEPLAY_LEVEL_SCENE`), though the resolution system does not rely on that exclusivity -- it independently checks for either key's presence.
- [x] Add `last_beacon_is_in_hub`/`last_beacon_is_in_world` run-conditions
  - Status: Complete
  - Repository: `root`
  - Notes: Mirrors `foundation_is_not_paused`/`foundation_is_paused` shape.
- [x] Bundle everything into `LastBeaconSharedGameplayPlugin` and wire into `game/src/lib.rs`
  - Status: Complete
  - Repository: `root`
  - Notes: Resolution system registered in `Last` (not `PostUpdate`) to match the existing `hide_last_beacon_menu_ui_behind_settings` precedent, since Foundation's scene-stack command processing (`process_scene_commands`) is a private system in `foundation-runtime-library` and cannot be ordered against directly from `game/`; running in `Last` guarantees it observes each frame's fully processed stack regardless.
- [x] Add `LastBeaconGameplaySpace` resolution unit test
  - Status: Complete
  - Repository: `root`
  - Notes: Four tests added, covering Hub scene, World scene, neither, and overlay-does-not-change-space (pause overlay opened over the Beacon scene). All pass.

### Validation
- Game validation: `Passed` -- `cargo check`, `cargo fmt -- --check` (one nit auto-fixed), `cargo clippy --all-targets --all-features -- -D warnings` (clean), `cargo test --all-features` (55/55 passed, including the 4 new tests), `cargo doc --all-features --no-deps` (no new warnings; one pre-existing unrelated warning in `ui_theme.rs` remains), and `scripts\validate.cmd` (full pass) all run successfully on 2026-09-12.
- Engine validation: `N/A`
- Documentation generation: Recorded -- `cargo doc` run above; all new public items carry Rustdoc comments.
- User confirmation: Pending (phase presented to user together with Phase 2 below)

## Phase 2: Hub and World module scaffolding
**Status:** Complete
**Goal:** `game/src/hub/mod.rs` and `game/src/world/mod.rs` exist with empty `LastBeaconHubGameplayPlugin`/`LastBeaconWorldGameplayPlugin`, wired into `LastBeaconPlugin`, each depending only on `shared`.

### Tasks
- [x] Add `game/src/hub/mod.rs` with `LastBeaconHubGameplayPlugin` and a module doc comment stating the one-way dependency rule
  - Status: Complete
  - Repository: `root`
  - Notes: None
- [x] Add `game/src/world/mod.rs` with `LastBeaconWorldGameplayPlugin` and a matching module doc comment
  - Status: Complete
  - Repository: `root`
  - Notes: None
- [x] Wire both plugins into `game/src/lib.rs` (after `LastBeaconSharedGameplayPlugin`)
  - Status: Complete
  - Repository: `root`
  - Notes: Added `pub mod hub;`/`pub mod shared;`/`pub mod world;` in alphabetical order alongside the existing `pub mod scenes;`/`pub mod ui_theme;`/`pub mod ui_widgets;` declarations.

### Validation
- Game validation: `Passed` -- covered by the same validation run recorded under Phase 1 (both phases were implemented and validated together in one pass).
- Engine validation: `N/A`
- Documentation generation: Recorded -- covered by the same `cargo doc` run recorded under Phase 1.
- User confirmation: Pending

## Implementation / Review Handoff Notes
- No gameplay behavior changed: `LastBeaconHubGameplayPlugin`/`LastBeaconWorldGameplayPlugin` add no systems yet, so this feature has no observable in-game effect beyond the new `LastBeaconGameplaySpace` resource existing and resolving correctly.
- Manual verification beyond automated tests was not performed (no UI/visual surface changed by this feature); automated test coverage plus `scripts\validate.cmd` is considered sufficient evidence for this plumbing-only change.
- Optional `gpt-5.5` sanity review (role fulfilled by Claude Sonnet 5) has not been requested by the user yet.

## Postponed Work
- All concrete gameplay systems (hub mechanics, world/expedition mechanics, real shared-state fields, real gameplay message variants) are explicitly out of scope for this feature and postponed to follow-up features, per the plan's "pure plumbing" scope decision.

## Progress Log
- `2026-09-12`: Brainstormed and approved design with the user (space naming: Hub/World; comms mechanism: shared resource + messages; scope: pure plumbing). Created `feature/gameplay-space-separation` from `dev`. Plan and tracker created.
- `2026-09-12`: Implemented Phase 1 (`game/src/shared/mod.rs`) and Phase 2 (`game/src/hub/mod.rs`, `game/src/world/mod.rs`) together, wired into `game/src/lib.rs`. Ran full validation (`cargo check`/`fmt`/`clippy`/`test`/`doc` plus `scripts\validate.cmd`); all passed. Tracker updated to `Implemented`, pending user review and commit.
