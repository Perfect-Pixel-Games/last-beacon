# BSN Parser Panic Safety Tracker

## Metadata
- Feature slug: `bsn-parser-panic-safety`
- Feature area: `multi-area`
- Primary area: `engine`
- Root branch: `feature/async-scene-loading` (consolidated at the user's request; originally implemented on a dedicated `feature/bsn-parser-panic-safety` branch, then cherry-picked here so only one branch is active)
- Engine branch: `feature/async-scene-loading` (same consolidation)
- Root branch base verification: `Complete — originally created from root dev at 7cacf7cabfff058305c08d9988dc15bd935f49e4; cherry-picked onto feature/async-scene-loading`
- Engine branch base verification: `Complete — originally created from engine dev at 1bc59f9a0039dfe412b735c869a90f38a0d58582; cherry-picked onto feature/async-scene-loading`
- Engine submodule pointer: `004f7f1` (fix commit, cherry-picked from the original `3b32ed8`)
- Overall status: `Complete; ready for pull requests`
- Planning model: `gpt-5.5`
- Preferred implementation model: `gpt-5.4`
- Optional final review model: `gpt-5.5`
- Current handoff state: `Implementation complete; folded into feature/async-scene-loading; awaiting user decision on pull requests`
- Created: `2026-08-24`
- Last updated: `2026-08-24`

## Validation Rules
- Task complete only after required validation passes and documentation generation is recorded, unless a waiver is recorded.
- Phase complete only after required validation passes, documentation generation is recorded, required commits/pushes are complete, and required user confirmation is recorded.

## Repository State
- Root commit/push state: `Complete — cherry-picked as fb9d2cc (plan/tracker), submodule pointer bump pending final commit on feature/async-scene-loading`
- Engine commit/push state: `Complete — cherry-picked as 004f7f1 onto feature/async-scene-loading, pushed to origin`
- Root submodule pointer update: `Complete — bound to engine 004f7f1`

## Phase 1: Guard the four confirmed panic sites (engine)
**Status:** Complete
**Goal:** `dynamic_bsn.rs` returns `Err(DynamicBsnLoaderError::UnknownType(_))` instead of panicking for a typo'd/renamed enum variant name or an unregistered field type in authored `.bsn` content.

### Tasks
- [x] Add 4 regression tests (one per call site) to `dynamic_bsn.rs`, confirmed red against current code.
  - Status: Complete
  - Repository: `engine`
  - Notes: Each test panicked at exactly its target line (303, 396, 694, 854) before the fix, confirming the test reaches the intended call site. Built directly against `BsnAst`'s private conversion methods, bypassing the lexer/grammar, since no existing `.bsn` asset exercises an enum struct/tuple-variant-as-top-level-component today.
- [x] Fix `dynamic_bsn.rs:303` (`BsnPatch::Struct` variant lookup).
  - Status: Complete
  - Repository: `engine`
- [x] Fix `dynamic_bsn.rs:396` (`BsnPatch::NamedTuple` variant lookup).
  - Status: Complete
  - Repository: `engine`
- [x] Fix `dynamic_bsn.rs:694` (`BsnExpr::StringLit` type-registry lookup).
  - Status: Complete
  - Repository: `engine`
- [x] Fix `dynamic_bsn.rs:854` (`create_reflect_default` type-registry lookup).
  - Status: Complete
  - Repository: `engine`
- [x] Confirm all 4 new tests green, full existing suite unchanged.
  - Status: Complete
  - Repository: `engine`
  - Notes: Originally verified on the dedicated branch (96 tests: 92 baseline + 4 new). Re-verified after cherry-picking onto `feature/async-scene-loading`: 114 tests (110 baseline + 4 new), clippy `-D warnings` clean.

### Validation
- Engine validation: `Passed — cargo test (114 tests on the consolidated branch), cargo clippy --all-targets --all-features -D warnings (clean), cargo fmt --check (clean), cargo doc --no-deps (clean)`
- Documentation generation: `Passed — cargo doc --no-deps -p foundation-runtime-library`
- User confirmation: `Not required for this phase`

## Phase 2: Root submodule pointer update
**Status:** Complete
**Goal:** Last Beacon is bound to the fixed engine commit.

### Tasks
- [x] Update the `engine` submodule pointer to the Phase 1 engine commit and commit.
  - Status: Complete
  - Repository: `root`
  - Notes: Pointer moved to `004f7f1` on `feature/async-scene-loading`.
- [x] Run game validation to confirm Last Beacon still builds/passes against the new engine commit.
  - Status: Complete
  - Repository: `root`

### Validation
- Game validation: `Passed — cargo test, cargo clippy --all-targets -D warnings (clean)`
- Documentation generation: `N/A — no root doc changes`
- User confirmation: `Not required for this phase`

## Implementation / Review Handoff Notes
- Use `gpt-5.4` for implementation, `gpt-5.5` for optional final review.
- Read `.pi/skills/rust-workspace-dev/SKILL.md`, `.pi/skills/rust-coding-standards/SKILL.md`, and `.pi/skills/gitflow-workflow/SKILL.md` before implementation edits.
- Match the fix pattern already established at `dynamic_bsn.rs` lines 598–602, 670–674, and 752–758 exactly — see `plan.md`'s Proposed Implementation Approach for the literal replacement text for each of the 4 sites.
- Write each regression test before its corresponding fix and confirm it fails first (TDD red/green), per this project's established practice this session.

## Postponed Work
- Deleting the superseded `feature/scene-pop-in-investigation` branch (local + remote, both repos): identified during the codebase review as housekeeping. User asked to leave it alone ("ignore the old scene-pop branch").
- Opening pull requests for `feature/async-scene-loading` (now including this work) in both repos: not yet requested by the user.
- Any work on the "Not started" gameplay systems identified in the review (robot construction, cargo inventory, expeditions, Beacon upgrade persistence): explicitly out of scope — a robustness pass on existing code, not new feature design.

## Notes / Issues / Oversights
- This feature was originally implemented on its own dedicated `feature/bsn-parser-panic-safety` branch (root commits `99d7d87`, `a61b85a`; engine commit `3b32ed8`), per the standard one-feature-one-branch workflow. The user then asked to consolidate it onto `feature/async-scene-loading` so only one branch is active at a time, after noticing that testing on the separate branch showed the font pop-in regression returning — the separate branch legitimately lacked the async-scene-loading fixes, since it branched from `dev`. The engine fix (`3b32ed8`) cherry-picked cleanly onto `feature/async-scene-loading` as `004f7f1` with no conflicts, since it touches a file (`dynamic_bsn.rs`) untouched by the async-scene-loading work. The original `feature/bsn-parser-panic-safety` branches (root and engine, both pushed to `origin`) are left in place but superseded; the user has not yet said whether to delete them.

## Progress Log
- `2026-08-24`: Created plan and tracker following a full codebase review that identified these 4 sites via a targeted code-review pass (comparing each unguarded `.unwrap()` against its already-guarded structural sibling in the same file). Awaiting user review/approval before implementation.
- `2026-08-24`: User approved. Implemented Phase 1 on a dedicated `feature/bsn-parser-panic-safety` branch: 4 regression tests confirmed red at their exact target lines, then the 4 fixes, then full engine validation (96 tests, clippy, fmt, doc generation all clean). Engine commit `3b32ed8` pushed.
- `2026-08-24`: Implemented Phase 2 on the same dedicated branch: bumped the root `engine` submodule pointer to `3b32ed8`, game validation clean (11 tests, clippy clean). Root commit `a61b85a` pushed. Reported both phases complete.
- `2026-08-24`: User noticed the font pop-in fix appeared to regress, root-caused it to testing on the separate `feature/bsn-parser-panic-safety` branch (created from `dev`, missing all async-scene-loading fixes) rather than an actual regression. User asked to consolidate onto `feature/async-scene-loading` as the one active branch. Cherry-picked the engine fix (`3b32ed8` → `004f7f1`, clean, no conflicts) and the plan/tracker doc commit (`99d7d87` → `fb9d2cc`) onto `feature/async-scene-loading` in both repositories, re-verified full validation on the consolidated branch (114 engine tests, clean clippy/fmt/doc), and updated the submodule pointer accordingly. This tracker rewritten to reflect the consolidated branch state.
