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
- Overall status: `Planned`
- Planning model: `gpt-5.5` (role fulfilled by Claude Sonnet 5)
- Preferred implementation model: `gpt-5.4` (role fulfilled by Claude Sonnet 5)
- Optional final review model: `gpt-5.5` (role fulfilled by Claude Sonnet 5)
- Current handoff state: `Ready for implementation (pending user go-ahead)`
- Created: `2026-09-12`
- Last updated: `2026-09-12`

## Validation Rules
- Task complete only after required validation passes and documentation generation is recorded, unless a waiver is recorded.
- Phase complete only after required validation passes, documentation generation is recorded, required commits/pushes are complete, and required user confirmation is recorded.

## Repository State
- Root commit/push state: `Pending`
- Engine commit/push state: `N/A`
- Root submodule pointer update: `N/A`

## Phase 1: Shared communication layer and space detection
**Status:** Planned
**Goal:** `game/src/shared/mod.rs` exists with `LastBeaconSharedGameplayState`, `LastBeaconGameplayEvent`, `LastBeaconGameplaySpace`, its resolution system, and the two run-conditions, all wired via `LastBeaconSharedGameplayPlugin` into `LastBeaconPlugin`, with a passing resolution test.

### Tasks
- [ ] Add `game/src/shared/mod.rs` with `LastBeaconSharedGameplayState` resource and `LastBeaconGameplayEvent` message enum
  - Status: Planned
  - Repository: `root`
  - Notes: Both intentionally empty (no fields/variants) per the "pure plumbing" scope decision.
- [ ] Add `LastBeaconGameplaySpace` resource + resolution system reading `SceneStack`
  - Status: Planned
  - Repository: `root`
  - Notes: Must correctly ignore pause/options overlay entries and resolve all current Beacon page scene keys, not just `BEACON_SCENE`.
- [ ] Add `last_beacon_is_in_hub`/`last_beacon_is_in_world` run-conditions
  - Status: Planned
  - Repository: `root`
  - Notes: Mirrors `foundation_is_not_paused`/`foundation_is_paused` shape.
- [ ] Bundle everything into `LastBeaconSharedGameplayPlugin` and wire into `game/src/lib.rs`
  - Status: Planned
  - Repository: `root`
  - Notes: None
- [ ] Add `LastBeaconGameplaySpace` resolution unit test
  - Status: Planned
  - Repository: `root`
  - Notes: Covers Hub scene, World scene, neither, and overlay-does-not-change-space cases.

### Validation
- Game validation: `Pending`
- Engine validation: `N/A`
- Documentation generation: Pending
- User confirmation: Pending

## Phase 2: Hub and World module scaffolding
**Status:** Planned
**Goal:** `game/src/hub/mod.rs` and `game/src/world/mod.rs` exist with empty `LastBeaconHubGameplayPlugin`/`LastBeaconWorldGameplayPlugin`, wired into `LastBeaconPlugin`, each depending only on `shared`.

### Tasks
- [ ] Add `game/src/hub/mod.rs` with `LastBeaconHubGameplayPlugin` and a module doc comment stating the one-way dependency rule
  - Status: Planned
  - Repository: `root`
  - Notes: None
- [ ] Add `game/src/world/mod.rs` with `LastBeaconWorldGameplayPlugin` and a matching module doc comment
  - Status: Planned
  - Repository: `root`
  - Notes: None
- [ ] Wire both plugins into `game/src/lib.rs` (after `LastBeaconSharedGameplayPlugin`)
  - Status: Planned
  - Repository: `root`
  - Notes: None

### Validation
- Game validation: `Pending`
- Engine validation: `N/A`
- Documentation generation: Pending
- User confirmation: Pending

## Implementation / Review Handoff Notes
- None yet.

## Postponed Work
- All concrete gameplay systems (hub mechanics, world/expedition mechanics, real shared-state fields, real gameplay message variants) are explicitly out of scope for this feature and postponed to follow-up features, per the plan's "pure plumbing" scope decision.

## Progress Log
- `2026-09-12`: Brainstormed and approved design with the user (space naming: Hub/World; comms mechanism: shared resource + messages; scope: pure plumbing). Created `feature/gameplay-space-separation` from `dev`. Plan and tracker created.
