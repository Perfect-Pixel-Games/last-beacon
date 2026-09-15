# Vehicle Fixes Tracker

## Metadata
- Feature slug: `vehicle-fixes`
- Feature area: `multi-area`
- Primary area: `engine`
- Root branch: `feature/vehicle-fixes`
- Engine branch: `feature/vehicle-fixes`
- Root branch base verification: `Verified (branched from dev at origin/dev, clean working tree)`
- Engine branch base verification: `Verified (branched from engine dev at 3f33dafa64262de52940b2407f02d1bab963ca45, matching origin/dev)`
- Engine submodule pointer: `aed8ad2a66da95746339912d9346cc96c1c359bc` (engine commit bound as of this update)
- Overall status: `In Progress`
- Planning model: Claude Sonnet 5 (standing in for `gpt-5.5`)
- Preferred implementation model: Claude Sonnet 5 (standing in for `gpt-5.4`)
- Optional final review model: Claude Sonnet 5 (standing in for `gpt-5.5`)
- Current handoff state: `In progress within a single continuous session`
- Created: `2026-09-15`
- Last updated: `2026-09-15`

## Validation Rules
- Task complete only after required validation passes and documentation generation is recorded, unless a waiver is recorded.
- Phase complete only after required validation passes, documentation generation is recorded, required commits/pushes are complete, and required user confirmation is recorded.

## Repository State
- Root commit/push state: `Committed 18dc384, pushed to origin/feature/vehicle-fixes`
- Engine commit/push state: `Committed aed8ad2a66da95746339912d9346cc96c1c359bc, pushed to origin/feature/vehicle-fixes`
- Root submodule pointer update: `Done (root now points at aed8ad2, committed and pushed)`

## Phase 1: Fix the vehicle ground-contact crash
**Status:** Done
**Goal:** The vehicle no longer panics on real ground contact; regression test proves it.

### Tasks
- [x] Root-cause the crash (Avian3D 0.7.0 island/contact-bookkeeping bug; see plan.md External Research)
  - Status: Done
  - Repository: `N/A (investigation)`
  - Notes: Confirmed via upstream issue avianphysics/avian#1025 and reproducing locally at `islands/mod.rs:547`.
- [x] Verify avian's main branch (commit `47d5948b2a8ee829848fbc3d4ce5e44ad0fd863f`) fixes it
  - Status: Done
  - Repository: `N/A (investigation)`
  - Notes: Previously-ignored terrain stability test passed across all 8 seeds; full 114-test suite passed with zero regressions.
- [x] Create `feature/vehicle-fixes` branch in root repo from `dev`
  - Status: Done
  - Repository: `root`
- [x] Create `feature/vehicle-fixes` branch in `engine/` from engine `dev`
  - Status: Done
  - Repository: `engine`
- [x] Pin `avian3d` to the fixed commit in `engine/Cargo.toml` and `game/Cargo.toml`
  - Status: Done
  - Repository: `both`
  - Notes: Used exact `rev`, not `branch`, for reproducibility. Inline comment added explaining the pin.
- [x] Fix `JointGraph` import path in `connection.rs` test helper
  - Status: Done
  - Repository: `root`
- [x] Un-ignore `vehicle_module_testbed_stays_bounded_on_rough_terrain_across_seeds`
  - Status: Done
  - Repository: `root`
- [x] Run full `scripts/validate.cmd` and confirm clean pass
  - Status: Done
  - Repository: `root`
  - Notes: fmt, clippy -D warnings, test (114 passed incl. terrain stability across 8 seeds), build, and doc all passed. Pre-existing rustdoc warnings (unrelated broken intra-doc links) and Windows SymInitialize noise observed, neither introduced by this change.

### Validation
- Game validation: `Passed (scripts/validate.cmd: fmt, clippy, 114 tests, build, doc)`
- Engine validation: `N/A (dependency-only change, exercised transitively through game validation)`
- Documentation generation: `Done (cargo doc succeeded with 8 pre-existing warnings, none new)`
- User confirmation: Not required yet (approved approach in conversation prior to implementation)

## Phase 2: Vehicle definition tidy-ups (not yet scoped)
**Status:** Planned
**Goal:** TBD -- user has not yet specified which vehicle-definition items to tidy.

### Tasks
- [ ] Get scope from user
  - Status: Planned
  - Repository: `TBD`
  - Notes: Deferred by user during Phase 1 ("we'll get to that in a bit").

## Implementation / Review Handoff Notes
- None yet.

## Postponed Work
- Revisit the `avian3d` git-commit pin once avian ships an official stable release at or after commit `47d5948b2a8ee829848fbc3d4ce5e44ad0fd863f`, and move back to a normal crates.io version requirement.

## Progress Log
- `2026-09-15`: Plan and tracker created. Root-cause investigation, upstream research, and fix verification completed before this doc existed (see plan.md for findings). Branches created, avian3d pinned in both manifests, `JointGraph` import fixed, terrain stability test un-ignored. Full validation passed. Engine committed (`aed8ad2`) and pushed; root committed (`18dc384`) with updated submodule pointer and pushed. Phase 1 complete.
