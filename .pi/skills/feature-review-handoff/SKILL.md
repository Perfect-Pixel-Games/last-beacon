---
name: feature-review-handoff
description: Perform an optional final sanity review of an implemented feature across the root repository and Foundation Engine submodule, record findings in the tracker, and hand review feedback back to implementation only if the user chooses.
---

# Feature Review Handoff

## Purpose
Use this skill after implementation when the user wants an additional sanity pass over code that was already written.

## Model policy
- Use `gpt-5.5` for review.
- Send accepted fix work back to `gpt-5.4` implementation.
- Never use Anthropic models.

## Required pre-work
1. Read `docs/plans/<new-feature>/plan.md`.
2. Read `docs/plans/<new-feature>/tracker.md`.
3. Read `.pi/skills/rust-workspace-dev/SKILL.md`.
4. Read `.pi/skills/gitflow-workflow/SKILL.md`.
5. Read `.pi/skills/foundation-architecture/SKILL.md` if the feature touched Foundation Engine or Foundation integration.
6. Inspect changed root implementation files.
7. Inspect changed `engine/` implementation files when the feature touched the submodule.
8. Review game and engine validation results if they exist.
9. Identify plan requirements, success criteria, documentation expectations, risks, or review focus areas that should shape the review.

## Review scope
Review against:
- the user request
- the recorded plan
- the tracker state
- changed root files and implementation details
- changed engine submodule files and implementation details when applicable
- root and engine commit, push, and pull request state
- exact engine commit hash that the game is bound to when applicable
- root `engine` submodule pointer state when applicable
- validation results already performed, including documentation generation
- risks, assumptions, and deferred work recorded during planning or implementation

Focus on practical issues such as:
- correctness concerns
- Rust ownership/lifetime/error-handling problems
- API compatibility and feature flag concerns
- architectural mismatches with the plan
- incorrect root-versus-engine ownership
- missing or uncommitted engine submodule work
- root submodule pointer not matching the intended engine commit
- missing tracker record of the exact engine commit hash bound to the game
- direct merge risk instead of pull request integration
- missing edge-case handling
- maintainability concerns
- missing validation coverage
- missing public API documentation or missing generated documentation evidence

## Output requirements
When performing the review:
1. Summarize the overall result clearly.
2. Separate must-fix concerns from optional improvements when possible.
3. Record findings in `tracker.md` under a review or handoff section.
4. Present the findings to the user.
5. Ask whether to accept the implementation as-is, send the review feedback back to `gpt-5.4` for fixes, or defer the findings.

Do not automatically start a fix pass unless the user chooses it.

## Submodule-specific checks
When the feature touched `engine/`:
- Verify `git -C engine status --short` is clean or that remaining dirt is explicitly recorded and not considered complete.
- Verify the engine branch is the planned `feature/*` or `hotfix/*` branch, never `main` or `dev`.
- Verify engine commits are recorded in the tracker.
- Verify the exact engine commit hash bound to the game is recorded in the tracker.
- Verify root `git status --short` does not show an unexpected dirty submodule.
- Verify the root repository committed the intended `engine` submodule pointer update when required.
- Verify engine validation evidence is recorded or waived.
- Verify pull request readiness or pull request links are recorded instead of direct merge completion.

## Suggested tracker addition
```md
## Review Findings
- `2026-07-16` - gpt-5.5 sanity review
  - Overall result: <pass / pass with concerns / needs follow-up>
  - Root repository state: <summary>
  - Engine submodule state: <summary or N/A>
  - Must-fix:
    - <Issue>
  - Optional improvements:
    - <Improvement>
  - User decision: `Accept as-is` / `Send to gpt-5.4 for fixes` / `Defer`
```
