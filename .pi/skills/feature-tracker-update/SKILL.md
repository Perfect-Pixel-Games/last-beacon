---
name: feature-tracker-update
description: Update an existing docs/plans/<new-feature>/tracker.md during implementation, including root and Foundation Engine submodule progress, validation, commits, pushes, submodule pointer state, handoff notes, postponements, review findings, and completion state.
---

# Feature Tracker Update

## Purpose
Use this skill while implementing a feature to keep `docs/plans/<new-feature>/tracker.md` accurate.

The tracker is the continuity layer between `gpt-5.5` planning, `gpt-5.4` implementation, and optional `gpt-5.5` review.

## Model policy
- Use `gpt-5.4` for implementation.
- Use `gpt-5.5` for planning and review.
- Never use Anthropic models.

## Required pre-work
1. Read `docs/plans/<new-feature>/plan.md`.
2. Read `docs/plans/<new-feature>/tracker.md`.
3. Read `.pi/skills/rust-workspace-dev/SKILL.md`.
4. Read `.pi/skills/gitflow-workflow/SKILL.md` if branch, commit, push, pull request, or submodule pointer updates are involved.
5. Read `.pi/skills/foundation-architecture/SKILL.md` if Foundation Engine or Foundation integration work is involved.
6. Confirm the active root branch matches the branch recorded in the docs.
7. Verify the root branch was created from current `dev` when possible; if it cannot be proven, record that limitation in the tracker before implementation edits.
8. If engine work is involved, confirm the active `engine/` branch matches the engine branch recorded in the docs.
9. If engine work is involved, stop before editing if `engine/` is on `main` or `dev`; switch or create a valid `feature/*` or `hotfix/*` branch first.
10. If engine work is involved, verify the engine branch was created from engine `dev` when possible; if it cannot be proven, record that limitation in the tracker before implementation edits.
11. Before the first implementation edit in a session, update the tracker to show implementation is starting or resuming with `gpt-5.4`.

## What to update
Update the tracker as work progresses, including:
- current task and phase status
- active implementation or review model
- root branch and branch-base verification state
- engine branch and branch-base verification state when applicable
- root commit and push state
- engine commit, push, and pull request state when applicable
- exact engine commit hash that the game is bound to when applicable
- root `engine` submodule pointer state when applicable
- game validation state
- engine validation state
- documentation generation state
- notes about issues, oversights, blockers, or discoveries
- postponed work and reasons
- progress log entries
- optional final review state and findings

## Required tracking rules
### Notes and discoveries
Record useful implementation notes, blockers, and decisions under the relevant task or phase.

### Postponements
A task or phase may be postponed only when there is a clear reason. Record the original location, new location, reason, and impact.

### Task completion
A task may only be marked complete when the required validation for that task has passed and documentation generation has been recorded, unless a user-approved waiver is recorded.

Default game validation:
- `scripts/validate.cmd`
- focused format/lint/test/build/doc commands with `--manifest-path game/Cargo.toml` when useful

Default engine validation:
- `engine/scripts/format-project.cmd`
- `engine/scripts/lint-project.cmd`
- `engine/scripts/test-project.cmd`
- `engine/scripts/compile-project.cmd`
- `engine/scripts/doc-project.cmd`
- `engine/scripts/validate-project.cmd` when full engine validation is required

If implementation is done but validation has not passed yet, use a non-final status such as `In progress`, `Awaiting validation`, `Blocked`, or `Postponed`.

### Engine work completion
For any task that changes `engine/`:
- The engine branch must be recorded and must not be `main` or `dev`.
- Engine validation or a waiver must be recorded.
- Engine changes must be committed inside `engine/` before the task is complete.
- The exact engine commit hash that the game is bound to must be recorded.
- Engine push and pull request status must be recorded when `origin` exists.
- The root `engine` submodule pointer update must be recorded when root work depends on the engine commit.
- `git -C engine status --short` must not contain uncommitted source changes for completed engine work.

### Phase transition
Do not start the next phase until required validation for the current phase is recorded, unless a documented feature-level waiver exists.

### Phase completion
A phase may only be marked complete when required validation has passed or a waiver is recorded, generated documentation has been recorded or waived, required commits and pushes are recorded, required submodule pointer updates are complete, and required user confirmation has been recorded.

### Review handling
An optional final sanity review uses `gpt-5.5`.

If review findings exist:
- record them in the tracker
- present them to the user
- record whether the user accepted the current code, deferred the feedback, or sent the feedback back for `gpt-5.4` fixes

Do not silently apply a review loop without telling the user.

## Branch, commit, and pull request expectations
Use the gitflow skill as the source of truth.

Requirements:
- The root feature must remain on a dedicated `feature/*` or `hotfix/*` branch created from the correct base; record branch-base verification or the reason it cannot be proven.
- Engine work must remain on a dedicated `feature/*` or `hotfix/*` branch inside `engine/`; record branch-base verification or the reason it cannot be proven.
- Never edit or commit engine code while `engine/` is on `main` or `dev`.
- Every completed task must be committed on the relevant feature branch.
- Every completed phase must be committed on the relevant feature branch, including the final phase.
- Engine commits must happen inside `engine/` before root commits that update the `engine` submodule pointer.
- The exact engine commit hash that the game is bound to must be recorded for any engine pointer update.
- When remote `origin` exists, every commit must be pushed to `origin` so pull requests can be opened.
- Do not merge directly into `main` or `dev`; all root and engine integration must go through pull requests.
- If `origin` is not configured, record push and pull request state as `N/A (local-only repository)` for that repository.
- Regular feature commits must include current `docs/plans/<feature>/plan.md` and `docs/plans/<feature>/tracker.md` updates.

## Recommended update workflow
1. Read the current plan and tracker.
2. Verify root and engine repository status as applicable.
3. Update the tracker before implementation resumes.
4. Make focused Rust or documentation changes using `gpt-5.4`.
5. Update task statuses and notes.
6. Run and record validation results, including generated documentation.
7. For engine work, commit and push inside `engine/` first, then record the exact engine commit hash that the game is bound to.
8. For root work or submodule pointer updates, commit and push in the root repository second.
9. Record pull request readiness or pull request links when available; do not perform direct merges.
10. Update phase status based on validation, documentation generation, commits, pushes, pull request readiness, pointer updates, and user confirmation.
11. Append a progress log entry.

## Suggested status vocabulary
- `Planned`
- `In progress`
- `Awaiting format`
- `Awaiting lint`
- `Awaiting tests`
- `Awaiting build`
- `Awaiting documentation generation`
- `Awaiting validation`
- `Awaiting engine commit`
- `Awaiting root pointer commit`
- `Awaiting push`
- `Awaiting pull request`
- `Awaiting review`
- `Review feedback pending`
- `Blocked`
- `Postponed`
- `Complete`

## Suggested handoff vocabulary
- `Planning in progress with gpt-5.5`
- `Ready for gpt-5.4 implementation`
- `Implementation in progress with gpt-5.4`
- `Ready for gpt-5.5 sanity review`
- `Review feedback pending`
- `Accepted without further review changes`

## Example repository state entry
```md
## Repository State
- Root branch: `feature/example`
- Root branch base verification: `Verified from dev on 2026-07-16`
- Root commit/push state: `Pending`
- Engine branch: `feature/example`
- Engine branch base verification: `Pending`
- Bound engine commit hash: `Pending`
- Engine commit/push/pull request state: `Pending`
- Root submodule pointer update: `Pending`
```

## Example review entry
```md
## Review Findings
- `2026-07-16` - gpt-5.5 sanity review
  - Summary: <high-level outcome>
  - Findings:
    - <Issue or concern>
  - User decision: `Send to gpt-5.4 for fixes` / `Accept as-is` / `Defer`
```

## Output expectations
- Preserve existing tracker history.
- Make incremental updates instead of rewriting intent.
- Keep notes attached to the relevant task or phase.
- Do not mark tasks or phases complete until validation, commit, push, and pointer rules are satisfied.
- Treat the tracker as the durable handoff log between planning, implementation, and review models.
