# BSN Parser Panic Safety Tracker

## Metadata
- Feature slug: `bsn-parser-panic-safety`
- Feature area: `multi-area`
- Primary area: `engine`
- Root branch: `feature/bsn-parser-panic-safety`
- Engine branch: `feature/bsn-parser-panic-safety`
- Root branch base verification: `Pending — create from root dev at 7cacf7cabfff058305c08d9988dc15bd935f49e4`
- Engine branch base verification: `Pending — create from engine dev at 1bc59f9a0039dfe412b735c869a90f38a0d58582`
- Engine submodule pointer: `6faaf445edbe8fa1ba1548cec72a1d0c5663a669` (current; will update once the engine fix commits)
- Overall status: `Planned`
- Planning model: `gpt-5.5`
- Preferred implementation model: `gpt-5.4`
- Optional final review model: `gpt-5.5`
- Current handoff state: `Awaiting user approval to begin implementation`
- Created: `2026-08-24`
- Last updated: `2026-08-24`

## Validation Rules
- Task complete only after required validation passes and documentation generation is recorded, unless a waiver is recorded.
- Phase complete only after required validation passes, documentation generation is recorded, required commits/pushes are complete, and required user confirmation is recorded.

## Repository State
- Root commit/push state: `Pending`
- Engine commit/push state: `Pending`
- Root submodule pointer update: `Pending`

## Phase 1: Guard the four confirmed panic sites (engine)
**Status:** Planned
**Goal:** `dynamic_bsn.rs` returns `Err(DynamicBsnLoaderError::UnknownType(_))` instead of panicking for a typo'd/renamed enum variant name or an unregistered field type in authored `.bsn` content.

### Tasks
- [ ] Create `feature/bsn-parser-panic-safety` branch inside `engine/` from `dev`.
  - Status: Planned
  - Repository: `engine`
- [ ] Add 4 regression tests (one per call site) to `dynamic_bsn.rs`, confirmed red against current code.
  - Status: Planned
  - Repository: `engine`
- [ ] Fix `dynamic_bsn.rs:303` (`BsnPatch::Struct` variant lookup).
  - Status: Planned
  - Repository: `engine`
- [ ] Fix `dynamic_bsn.rs:396` (`BsnPatch::NamedTuple` variant lookup).
  - Status: Planned
  - Repository: `engine`
- [ ] Fix `dynamic_bsn.rs:694` (`BsnExpr::StringLit` type-registry lookup).
  - Status: Planned
  - Repository: `engine`
- [ ] Fix `dynamic_bsn.rs:854` (`create_reflect_default` type-registry lookup).
  - Status: Planned
  - Repository: `engine`
- [ ] Confirm all 4 new tests green, full existing suite unchanged.
  - Status: Planned
  - Repository: `engine`

### Validation
- Engine validation: `Pending — engine/scripts/validate-project.cmd`
- Documentation generation: `Pending — engine/scripts/doc-project.cmd`
- User confirmation: `Not required for this phase`

## Phase 2: Root submodule pointer update
**Status:** Planned
**Goal:** Last Beacon is bound to the fixed engine commit.

### Tasks
- [ ] Create `feature/bsn-parser-panic-safety` branch in the root repository from `dev`.
  - Status: Planned
  - Repository: `root`
- [ ] Update the `engine` submodule pointer to the Phase 1 engine commit and commit.
  - Status: Planned
  - Repository: `root`
- [ ] Run game validation to confirm Last Beacon still builds/passes against the new engine commit.
  - Status: Planned
  - Repository: `root`

### Validation
- Game validation: `Pending — scripts/validate.cmd`
- Documentation generation: `N/A — no root doc changes`
- User confirmation: `Not required for this phase`

## Implementation / Review Handoff Notes
- Use `gpt-5.4` for implementation, `gpt-5.5` for optional final review.
- Read `.pi/skills/rust-workspace-dev/SKILL.md`, `.pi/skills/rust-coding-standards/SKILL.md`, and `.pi/skills/gitflow-workflow/SKILL.md` before implementation edits.
- Match the fix pattern already established at `dynamic_bsn.rs` lines 598–602, 670–674, and 752–758 exactly — see `plan.md`'s Proposed Implementation Approach for the literal replacement text for each of the 4 sites.
- Write each regression test before its corresponding fix and confirm it fails first (TDD red/green), per this project's established practice this session.

## Postponed Work
- Deleting the superseded `feature/scene-pop-in-investigation` branch (local + remote, both repos) and opening the pull request for `feature/async-scene-loading`: identified during the codebase review as housekeeping, but deliberately kept out of this feature's scope since neither is a code change. Left for the user to request explicitly.
- Any work on the "Not started" gameplay systems identified in the review (robot construction, cargo inventory, expeditions, Beacon upgrade persistence): explicitly out of scope — a robustness pass on existing code, not new feature design.

## Notes / Issues / Oversights
- None yet.

## Progress Log
- `2026-08-24`: Created plan and tracker following a full codebase review that identified these 4 sites via a targeted code-review pass (comparing each unguarded `.unwrap()` against its already-guarded structural sibling in the same file). Awaiting user review/approval before implementation.
