# Vehicle Fixes Plan

## Metadata
- Feature slug: `vehicle-fixes`
- Feature area: `multi-area`
- Primary area: `engine`
- Root branch: `feature/vehicle-fixes`
- Engine branch: `feature/vehicle-fixes`
- Engine submodule pointer: `aed8ad2a66da95746339912d9346cc96c1c359bc` (engine commit bound as of this update)
- Status: `In Progress`
- Planning model: Claude Sonnet 5 (standing in for `gpt-5.5` per durable user direction; see repo AGENTS.md model-policy note)
- Implementation model: Claude Sonnet 5 (standing in for `gpt-5.4`)
- Review model: Claude Sonnet 5 (standing in for `gpt-5.5`)
- Created: `2026-09-15`
- Last updated: `2026-09-15`

## User Request
General bug-fixing and cleanup branch for the vehicle module system. Immediate driver: an intermittent (~50% of the time) crash when the vehicle wagon collides with the ground. A second phase of "small tidy-ups" to the vehicle definitions was requested but not yet scoped by the user as of this writing -- it will be appended to this plan/tracker once specified, rather than blocking the crash fix.

## Feature Area Classification
- Area: `multi-area`
- Primary area: `engine`
- Rationale: The root cause and fix live in the `avian3d` physics dependency, which both `engine/Cargo.toml` (workspace dependency, consumed by `foundation-runtime-library`'s `FoundationPhysicsPlugin`) and `game/Cargo.toml` (direct dependency, used by `game/src/shared/vehicle/*`) declare. Both manifests must move together since Cargo unifies the dependency graph through `game/Cargo.lock`.

## Codebase Research
- The vehicle module fusion architecture (`game/src/shared/vehicle/connection.rs`) fuses weld connections into compound rigid bodies and keeps wheel hinges as real Avian3D `RevoluteJoint`s -- see that file's module doc comment.
- `game/tests/vehicle_module_terrain_stability.rs` is a regression test (sweeping 8 terrain seeds, real ground contact) that was `#[ignore]`d because Avian3D 0.7.0 panicked internally the moment the vehicle touched any ground.
- Reproduced the panic locally: `avian3d-0.7.0/src/dynamics/solver/islands/mod.rs:547`, `Option::unwrap()` on `None`, inside `PhysicsIslands::add_contact`. Root cause: when a contact stops touching, Avian3D doesn't always unlink it from its island's linked list before the `ContactId` gets recycled by a later contact; a later `add_contact` then dereferences a `head_contact_id` no longer present in the contact graph. This is timing/contact-history-dependent, matching the user's "crashes about half the time" report.

## External Research
- Upstream issue confirming the same bug class, still open: [avianphysics/avian#1025](https://github.com/avianphysics/avian/issues/1025) ("Head contact has no island" / `Option::unwrap()` panic in island bookkeeping).
- A community fix was attempted but abandoned unmerged: [avianphysics/avian#1052](https://github.com/avianphysics/avian/pull/1052), closed by its own author in Sep 2026 as stale, never merged.
- No new stable release exists since `v0.7.0`; avian's `main` branch is at version `0.8.0-dev` (unreleased) as of this investigation.
- Verified directly: pointing `avian3d` at avian's `main` branch (commit `47d5948b2a8ee829848fbc3d4ce5e44ad0fd863f`) fixes the panic. The previously-`#[ignore]`d terrain stability test passes across all 8 seeds with real ground contact, and the full existing suite (114 tests across game + engine) passes with zero regressions. Only one source change was required: `avian3d::dynamics::solver::joint_graph::JointGraph` moved to `avian3d::dynamics::joints::joint_graph::JointGraph` (module reorg on avian's main, not an intentional behavior change).

## Affected Files And Systems
- `engine/Cargo.toml`: `avian3d` workspace dependency pinned to the fixed git commit instead of the crates.io `"0.7"` requirement.
- `game/Cargo.toml`: same pin, mirrored (must match for Cargo to unify the dependency graph).
- `game/src/shared/vehicle/connection.rs`: one test-helper import path update for avian's `JointGraph` module move.
- `game/tests/vehicle_module_terrain_stability.rs`: un-ignored; doc comment updated to describe the fix instead of the open blocker.

## Proposed Implementation Approach
1. Pin `avian3d` to the verified-fixed commit in both `engine/Cargo.toml` and `game/Cargo.toml`, with an inline comment explaining why an unreleased commit is used instead of the published version.
2. Fix the one resulting compile break (`JointGraph` import path).
3. Un-ignore `vehicle_module_testbed_stays_bounded_on_rough_terrain_across_seeds` and update its doc comment to reflect the fix.
4. Run full validation (`scripts/validate.cmd`: fmt, clippy, test, build, doc) to confirm no regressions.
5. Commit inside `engine/` first, push, record the exact engine commit hash; then commit the root repo (including the updated `engine` submodule pointer) and push.

## Submodule Plan
- Engine changes required: `yes`
- Engine branch: `feature/vehicle-fixes`
- Engine commit expectation: pin `avian3d` in `engine/Cargo.toml`'s `[workspace.dependencies]` to the fixed commit.
- Bound engine commit hash: pending (recorded in tracker once committed)
- Root pointer update required: `yes`

## Alternatives Considered
- **Patch avian3d 0.7.0 locally** (vendor a rebase of the abandoned PR #1052's fix onto the stable release): more API stability, but means maintaining a physics-engine patch ourselves indefinitely, and the abandoned PR itself may not directly apply after avian's later "Contiguous Solver Bodies" (#1037) refactor. Rejected in favor of pinning main, which the maintainer's own commits already carry forward.
- **Wait for an official avian 0.8.0 release**: cleanest long-term outcome, but no release timeline exists and the crash is a current, confirmed blocker on core gameplay (vehicle physics). Rejected as a blocking dependency; tracked as follow-up work instead.

## Risks, Constraints, And Assumptions
- We now depend on an unreleased Avian3D commit (`0.8.0-dev`) with no semver guarantee. A future upstream push to `main` cannot silently affect us since the dependency is pinned by exact `rev`, not `branch`, but *we* must deliberately re-verify before ever moving that pin forward.
- Assumption: avian's eventual 0.8.0 stable release will include this fix (it's already on `main`), making this a temporary state rather than a permanent fork.
- Follow-up work: revisit this pin when avian publishes a stable release at or after this commit, and un-pin back to a normal crates.io version requirement.

## Open Questions
- What specific vehicle-definition tidy-ups does the user want in phase 2 of this branch? Not yet scoped; to be added to this plan/tracker once known.

## Documentation Expectations
- No new public APIs are introduced by this fix; the Cargo.toml comments explain the pin's rationale for future maintainers.
- `cargo doc` is part of `scripts/validate.cmd` and will be run as part of validation.

## Implementation Handoff Notes
- The `avian3d` pin comment in both `Cargo.toml` files must stay next to the dependency line if either file is reformatted later -- it documents a non-obvious constraint, not a description of what the line does.

## Optional Review Focus Areas
- Confirm the `rev` pin is an exact commit hash (not a branch) in both manifests.
- Confirm `game/Cargo.lock` reflects the same resolved commit as `engine`'s resolution (single unified graph via `game/Cargo.lock`).

## Success Criteria
- `vehicle_module_testbed_stays_bounded_on_rough_terrain_across_seeds` passes (no longer ignored).
- Full `scripts/validate.cmd` passes with zero regressions.
- No remaining references to the old `avian3d::dynamics::solver::joint_graph` path.

## Testing Methodology
- Game validation: `scripts/validate.cmd` (fmt, clippy `-D warnings`, full test suite, build, doc generation).
- Engine validation: not independently re-run for this change (the engine crate itself has no vehicle-specific tests; its dependency pin is exercised transitively through the game's test suite via the path dependency). Noted as a lighter-than-default validation choice because the change is a dependency version bump with no engine-crate source edits.
