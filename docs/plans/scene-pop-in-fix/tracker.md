# Scene Pop-In Fix Tracker

## Metadata
- Feature slug: `scene-pop-in-fix`
- Feature area: `multi-area`
- Primary area: `engine`
- Root branch: `feature/scene-pop-in-investigation`
- Engine branch: `feature/scene-pop-in-investigation`
- Root branch base verification: `Verified: dev (7cacf7cabfff058305c08d9988dc15bd935f49e4) is an ancestor of this branch; only the investigation doc commit sits on top`
- Engine branch base verification: `Pending; engine currently detached at 1bc59f9a0039dfe412b735c869a90f38a0d58582 (= engine origin/dev tip); branch not yet created`
- Engine submodule pointer: `1bc59f9a0039dfe412b735c869a90f38a0d58582 (unchanged so far)`
- Overall status: `Planned`
- Planning model: `gpt-5.5`
- Preferred implementation model: `gpt-5.4`
- Optional final review model: `gpt-5.5`
- Current handoff state: `Ready for gpt-5.4 implementation, pending user approval to proceed`
- Created: `2026-07-20`
- Last updated: `2026-07-20`

## Validation Rules
- Task complete only after required validation passes and documentation
  generation is recorded, unless a waiver is recorded.
- Phase complete only after required validation passes, documentation
  generation is recorded, required commits/pushes are complete, and
  required user confirmation is recorded.
- Engine work must be committed inside `engine/` before the root repository
  commits the updated `engine` submodule pointer.
- The exact engine commit hash bound to Last Beacon must be recorded before
  root completion.

## Repository State
- Root commit/push state: `Investigation doc commit 8f224e4 pushed to origin/feature/scene-pop-in-investigation; no fix commits yet`
- Engine commit/push state: `N/A yet; engine branch not yet created`
- Root submodule pointer update: `Pending`
- Root pull request state: `Pending`
- Engine pull request state: `Pending`

## Phase 1: Planning
**Status:** Complete
**Goal:** Capture an approved plan/tracker for fixing the scene pop-in issue documented in `docs/scene-pop-in-investigation.md`.

### Tasks
- [x] Investigate and document root cause.
  - Status: Complete
  - Repository: `both` (read-only research)
  - Notes: `docs/scene-pop-in-investigation.md`, committed as `8f224e4` on `feature/scene-pop-in-investigation`.
- [x] Answer open questions from investigation via user observation.
  - Status: Complete
  - Repository: `root`
  - Notes: User confirmed pop-in is both structural and font-related, happens every open (not just first), and takes under ~100ms per scene. Recorded in `docs/scene-pop-in-investigation.md`.
- [x] Create fix plan and tracker.
  - Status: Complete
  - Repository: `root`
  - Notes: Created `docs/plans/scene-pop-in-fix/plan.md` and this tracker. Continuing on `feature/scene-pop-in-investigation` rather than a new branch (see plan's Branch Note).
- [ ] User review and approval to begin implementation.
  - Status: Pending
  - Repository: `root`
  - Notes: Awaiting user confirmation to proceed past this planning checkpoint.

### Validation
- Game validation: `N/A for planning-only docs`
- Engine validation: `N/A for planning-only docs`
- Documentation generation: `N/A for planning-only docs`
- User confirmation: `Pending`

## Phase 2: Engine Readiness Gating
**Status:** Planned
**Goal:** Foundation scene-owned roots only become visible once their BSN content has actually applied, not merely because the scene stack marked them visible.

### Tasks
- [ ] Create engine submodule branch `feature/scene-pop-in-investigation` from engine `dev`.
  - Status: Planned
  - Repository: `engine`
- [ ] Spawn BSN roots with an explicit not-ready `Visibility` state in `bsn_assets.rs`.
  - Status: Planned
  - Repository: `engine`
- [ ] Gate `sync_scene_entity_visibility` (or an added system) in `scene_stack.rs` on readiness AND stack visibility.
  - Status: Planned
  - Repository: `engine`
- [ ] Reconcile `splash_screen.rs`'s authored-marker wait with the new generic gate.
  - Status: Planned
  - Repository: `engine`
- [ ] Add/extend engine tests for readiness-gated visibility, including interaction with `covers_previous`.
  - Status: Planned
  - Repository: `engine`

### Validation
- Engine validation: `Pending`
- Documentation generation: `Pending`
- User confirmation: `Not required until phase handoff unless implementation discovers scope changes`

## Phase 3: Game-Side Readiness Participation And Font Preload
**Status:** Planned
**Goal:** Last Beacon's nested widget loading and font reassignment participate in the same readiness signal instead of popping independently.

### Tasks
- [ ] Expose `LastBeaconBsnWidgetPending`/`LastBeaconBsnWidgetFailed` state so parent-scene readiness can account for it.
  - Status: Planned
  - Repository: `root`
- [ ] Wire Last Beacon widget readiness into the engine gate from Phase 2.
  - Status: Planned
  - Repository: `root`
- [ ] Add a `Startup` system preloading `fonts/NotoSans-Regular.ttf` and `fonts/NotoSansSymbols2-Regular.ttf` before `scenes::open_initial_scene`.
  - Status: Planned
  - Repository: `root`
- [ ] Add/extend game tests for widget-pending-aware readiness and font preload.
  - Status: Planned
  - Repository: `root`

### Validation
- Game validation: `Pending`
- Documentation generation: `Pending`
- User confirmation: `Not required until phase handoff unless implementation discovers scope changes`

## Phase 4: Documentation, Validation, Commits, And Root Pointer Update
**Status:** Planned
**Goal:** Document the new readiness mechanism, validate engine/root behavior, commit/push both repositories, and record exact pointer state.

### Tasks
- [ ] Update `engine/docs/scene-system.md` with readiness-gating behavior.
  - Status: Planned
  - Repository: `engine`
- [ ] Update `docs/ui-widgets.md` if `LastBeaconBsnWidget` gains new pending/ready-query behavior.
  - Status: Planned
  - Repository: `root`
- [ ] Run full engine validation (`engine/scripts/validate-project.cmd`).
  - Status: Planned
  - Repository: `engine`
- [ ] Commit and push engine changes; record exact engine commit hash.
  - Status: Planned
  - Repository: `engine`
- [ ] Update root `engine` submodule pointer to the new engine commit.
  - Status: Planned
  - Repository: `root`
- [ ] Run full root validation (`scripts/validate.cmd`).
  - Status: Planned
  - Repository: `root`
- [ ] Manual smoke test: open/revisit main menu, options, a Beacon page, ui_playground, and both splash scenes, confirming no visible pop.
  - Status: Planned
  - Repository: `root`
- [ ] Commit and push root changes.
  - Status: Planned
  - Repository: `root`

### Validation
- Engine validation: `Pending`
- Game validation: `Pending`
- Documentation generation: `Pending`
- User confirmation: `Pending`

## Implementation / Review Handoff Notes
- Use `gpt-5.4` for implementation, `gpt-5.5` for optional final review.
- Read `.pi/skills/feature-tracker-update/SKILL.md`, `.pi/skills/rust-workspace-dev/SKILL.md`,
  `.pi/skills/rust-coding-standards/SKILL.md`, `.pi/skills/gitflow-workflow/SKILL.md`, and
  `.pi/skills/foundation-architecture/SKILL.md` before implementation edits.
- Do not edit engine files until `engine/` is on `feature/scene-pop-in-investigation` from engine `dev`.
- See `docs/plans/scene-pop-in-fix/plan.md` for full proposed approach, risks, and alternatives considered.

## Postponed Work
- None yet.

## Notes / Issues / Oversights
- None yet.

## Progress Log
- `2026-07-20`: Investigated scene/widget pop-in; wrote and committed `docs/scene-pop-in-investigation.md` (`8f224e4`) on `feature/scene-pop-in-investigation`; pushed to `origin/feature/scene-pop-in-investigation`.
- `2026-07-20`: User answered the investigation's three open questions via direct observation; recorded answers and their implications in `docs/scene-pop-in-investigation.md`.
- `2026-07-20`: Created `docs/plans/scene-pop-in-fix/plan.md` and this tracker, continuing on `feature/scene-pop-in-investigation`. Awaiting user review and approval to begin implementation.
