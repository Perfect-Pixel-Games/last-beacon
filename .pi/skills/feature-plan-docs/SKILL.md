---
name: feature-plan-docs
description: Create a feature plan and tracker under docs/plans/<new-feature>/ before implementation starts, including root and Foundation Engine submodule branch tracking, research, validation expectations, and model handoff details.
---

# Feature Plan And Tracker Creation

## Purpose
Use this skill before any implementation starts for a new feature.

This skill creates or updates:
- `docs/plans/<new-feature>/plan.md`
- `docs/plans/<new-feature>/tracker.md`

Do not start implementation until both documents exist and the user has approved moving from planning to implementation.

## Model policy
- Planning model: `gpt-5.5`
- Implementation model recorded for handoff: `gpt-5.4`
- Optional final review model: `gpt-5.5`
- Never use Anthropic models for planning, implementation, or review.

The plan and tracker are the persistence layer between models. Record enough detail that implementation or review can continue later without relying on chat-only context.

## Required pre-work
1. Read `.pi/skills/rust-workspace-dev/SKILL.md` for Rust workflow.
2. Read `.pi/skills/gitflow-workflow/SKILL.md` for branch, commit, push, and submodule pointer rules.
3. If the feature touches Foundation Engine, Foundation integration, engine launch behavior, editor-time support, settings, scenes, or assets, read `.pi/skills/foundation-architecture/SKILL.md`.
4. Ask or confirm whether the feature is `game`, `engine`, or `multi-area` before finalizing the plan. If it spans multiple areas, record all applicable areas and the primary area. Workflow/tooling-only changes should be recorded as `multi-area` with a primary area rationale unless the taxonomy is expanded later.
5. Inspect relevant manifests and files before writing the plan:
   - `README.md` and `game/Cargo.toml` for Last Beacon game work.
   - `engine/README.md`, `engine/AGENTS.md`, and `engine/Cargo.toml` for Foundation Engine work.
6. If online research tools are available and useful, perform external research relevant to the feature.
7. If online research tools are not available or not needed, explicitly note that in the plan.

## Branch requirement
Every feature must have a dedicated root branch from root `dev` unless the user explicitly states that the current valid feature branch must be used.

Rules:
- Use a `feature/<work-being-done>` branch name following the gitflow skill.
- Record the root branch name in both `plan.md` and `tracker.md`.
- If the feature includes engine changes, record a separate engine submodule branch name in both documents.
- Engine branches must also be `feature/*` or `hotfix/*` branches from the engine repository's correct base.
- If a required branch does not exist yet, propose the branch name and clearly mark branch creation as required before implementation starts.
- Do not plan implementation work directly on `main` or `dev` in either repository.

## Directory and file creation
For feature slug `<new-feature>`, create:
- `docs/plans/<new-feature>/plan.md`
- `docs/plans/<new-feature>/tracker.md`

If template files exist under `docs/plans/_templates/`, use them as a starting point. If they do not exist, create the plan and tracker directly from the requirements and suggested templates in this skill.

The feature slug should usually match the root feature branch suffix. Use a `feature/*` branch name for feature planning documents, and create or verify that branch from `dev` before implementation when possible.

## Plan document requirements
`plan.md` must include:
1. Feature name and user request.
2. Feature area classification: `game`, `engine`, or a clearly marked multi-area combination with one primary area.
3. Root branch name and current status.
4. Engine submodule branch name and current status when engine work is involved, otherwise `N/A`.
5. Current engine submodule pointer and expected pointer handling when engine work is involved.
6. Codebase research findings.
7. External research findings or a note that none was performed.
8. Affected Rust crates/modules/APIs/configuration/assets/scripts.
9. Proposed implementation approach.
10. Documentation expectations, including public API documentation and generated documentation requirements.
11. Alternatives considered when relevant.
12. Risks, constraints, assumptions, and open questions.
13. Success criteria.
14. Testing and validation methodology for each affected repository.
15. Planning model used: `gpt-5.5`.
16. Handoff notes for `gpt-5.4` implementation.
17. Optional review focus areas for `gpt-5.5`.

## Tracker document requirements
`tracker.md` must include:
1. Feature name and slug.
2. Feature area classification: `game`, `engine`, or a clearly marked multi-area combination with one primary area.
3. Root branch name.
4. Engine submodule branch name when applicable.
5. Root and engine branch-base verification state.
6. Root and engine commit/push state sections when applicable.
7. Engine submodule pointer state when applicable.
8. Overall status.
9. Ordered phases and tasks.
10. Validation state for each task and phase.
11. Notes/issues/oversights discovered during work.
12. Postponed work and reasons.
13. Progress log entries.
14. Planning model: `gpt-5.5`.
15. Preferred implementation model: `gpt-5.4`.
16. Optional final review model: `gpt-5.5`.
17. Current handoff state.

## Validation rules
Default game validation, unless the plan states a justified alternative:
- `scripts/validate.cmd`
- focused format/lint/test/build/doc commands using `--manifest-path game/Cargo.toml` when useful
- `scripts/build.cmd --platform windows-x64 --configuration test --target game` when game launch/package integration is affected

Default engine validation, unless the plan states a justified alternative:
- `engine/scripts/format-project.cmd`
- `engine/scripts/lint-project.cmd`
- `engine/scripts/test-project.cmd`
- `engine/scripts/compile-project.cmd`
- `engine/scripts/doc-project.cmd`
- `engine/scripts/validate-project.cmd` when full engine validation is required

A task may only be marked complete when required validation for that task has passed and documentation generation has been recorded, unless a user-approved waiver is recorded.

A phase may only be marked complete when:
- required validation has passed or a waiver is recorded,
- documentation generation has been recorded or waived,
- engine commits and root submodule pointer commits are complete when engine work is involved, and
- the user has confirmed the phase is suitable when user confirmation is required.

## Commit and pull request expectations
Use the gitflow skill as the source of truth.

Requirements:
- All root work must happen on a dedicated `feature/*` or `hotfix/*` branch from the correct base.
- All engine work must happen inside `engine/` on a dedicated `feature/*` or `hotfix/*` branch from the engine repository's correct base.
- Never edit or commit engine code while `engine/` is on `main` or `dev`.
- Every completed task must be committed.
- Every completed phase must be committed, including the final phase.
- Engine changes must be committed and pushed in `engine/` before the root repository commits the updated `engine` submodule pointer.
- Record the exact engine commit hash that the game is bound to whenever the root `engine` pointer is updated.
- When remote `origin` exists, every commit must be pushed to `origin` so pull requests can be opened.
- Do not merge directly into `main` or `dev`; all root and engine integration must go through pull requests.
- If `origin` is not configured, record push and pull request status as `N/A (local-only repository)` for that repository.
- Regular feature commits must include current `docs/plans/<feature>/plan.md` and `docs/plans/<feature>/tracker.md` changes.

## Mandatory stop after planning
After `plan.md` and `tracker.md` are created or updated:
1. Stop planning work.
2. Summarize the plan and tracker for the user.
3. Ask the user to review and confirm whether implementation should begin.
4. Do not begin implementation in the same turn unless the user explicitly asks to proceed after that review checkpoint.

Treat clear affirmative responses such as `continue`, `carry on`, `go ahead`, `implement`, or `proceed` as approval to begin implementation. Treat negative or revision-seeking responses as planning iteration requests.

## Suggested `plan.md` template
```md
# <Feature Name> Plan

## Metadata
- Feature slug: `<new-feature>`
- Feature area: `<game | engine | multi-area>`
- Primary area: `<game | engine>`
- Root branch: `feature/<work-being-done>`
- Engine branch: `<feature/<work-being-done> | N/A>`
- Engine submodule pointer: `<current hash | N/A>`
- Status: `Planned`
- Planning model: `gpt-5.5`
- Implementation model: `gpt-5.4`
- Review model: `gpt-5.5`
- Created: `<YYYY-MM-DD>`
- Last updated: `<YYYY-MM-DD>`

## User Request
<What the user asked for>

## Feature Summary
<What the feature is and why it exists>

## Feature Area Classification
- Area: `<game | engine | multi-area>`
- Primary area: `<game | engine>`
- Rationale: <why this area owns the feature>

## Codebase Research
- <Relevant game or engine finding>

## External Research
- <Finding and source>
- Or: `No external online research was performed because it was not needed or tooling was unavailable.`

## Affected Files And Systems
- `<path or subsystem>`: <why it matters>

## Proposed Implementation Approach
1. <Step>

## Submodule Plan
- Engine changes required: `<yes | no>`
- Engine branch: `<branch | N/A>`
- Engine commit expectation: `<description | N/A>`
- Bound engine commit hash: `<hash once known | N/A>`
- Root pointer update required: `<yes | no>`

## Alternatives Considered
- <Alternative>: <why rejected or deferred>

## Risks, Constraints, And Assumptions
- <Risk / constraint / assumption>

## Open Questions
- <Question, if any>

## Documentation Expectations
- Public APIs added or changed by this feature must have Rustdoc comments, or the plan must explicitly justify why they are internal/undocumented.
- Feature-level architecture or usage documentation should be added under `docs/` or `engine/docs/` when Rustdoc alone is insufficient.
- Generated documentation must be produced before the feature is considered complete.

## Implementation Handoff Notes
- Use `gpt-5.4` for implementation.
- Never use Anthropic models.
- <Specific guidance another model must follow>

## Optional Review Focus Areas
- Use `gpt-5.5` for review.
- <Area to sanity-check later>

## Success Criteria
- <Observable outcome>

## Testing Methodology
- Game validation: `scripts/validate.cmd` when game work is affected.
- Engine validation: `engine/scripts/validate-project.cmd` or focused engine checks when engine work is affected.
```

## Suggested `tracker.md` template
```md
# <Feature Name> Tracker

## Metadata
- Feature slug: `<new-feature>`
- Feature area: `<game | engine | multi-area>`
- Primary area: `<game | engine>`
- Root branch: `feature/<work-being-done>`
- Engine branch: `<feature/<work-being-done> | N/A>`
- Root branch base verification: `Pending`
- Engine branch base verification: `<Pending | N/A>`
- Engine submodule pointer: `<hash | N/A>`
- Overall status: `Planned`
- Planning model: `gpt-5.5`
- Preferred implementation model: `gpt-5.4`
- Optional final review model: `gpt-5.5`
- Current handoff state: `Ready for gpt-5.4 implementation`
- Created: `<YYYY-MM-DD>`
- Last updated: `<YYYY-MM-DD>`

## Validation Rules
- Task complete only after required validation passes and documentation generation is recorded, unless a waiver is recorded.
- Phase complete only after required validation passes, documentation generation is recorded, required commits/pushes are complete, and required user confirmation is recorded.

## Repository State
- Root commit/push state: `Pending`
- Engine commit/push state: `<Pending | N/A>`
- Root submodule pointer update: `<Pending | N/A>`

## Phase 1: <Phase name>
**Status:** Planned  
**Goal:** <testable deliverable>

### Tasks
- [ ] <Task 1>
  - Status: Planned
  - Repository: `<root | engine | both>`
  - Notes: None

### Validation
- Game validation: `<Pending | N/A>`
- Engine validation: `<Pending | N/A>`
- Documentation generation: Pending
- User confirmation: Pending / Not required yet

## Implementation / Review Handoff Notes
- None

## Postponed Work
- None

## Progress Log
- `<YYYY-MM-DD>`: Plan and tracker created.
```
