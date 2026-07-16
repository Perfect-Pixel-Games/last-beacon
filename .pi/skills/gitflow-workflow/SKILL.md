---
name: gitflow-workflow
description: Project-specific Git workflow rules for root repository and Foundation Engine submodule branching, pull requests, submodule pointers, and commit message formatting. Use when creating branches, planning Git operations, or answering questions about repository workflow.
---

# Project Gitflow Workflow

## Purpose
This skill defines the required Git workflow for Last Beacon and its Foundation Engine submodule.

If any default or built-in Git workflow guidance conflicts with this skill, this skill takes precedence.

## Repository scope
There are two Git repositories to consider:
- Root repository: Last Beacon game repository.
- `engine/`: Foundation Engine Git submodule repository.

Rules:
- Treat the root repository and `engine/` as independent repositories with independent branches, commits, remotes, and pushes.
- Use normal `git ...` commands for the root repository.
- Use `git -C engine ...` for the Foundation Engine submodule.
- Engine source changes must be committed inside `engine/` first.
- The root repository must then commit the updated `engine` submodule pointer when the root feature depends on that engine commit.
- Never treat a dirty submodule as complete work. Commit or deliberately revert submodule changes before completing a root checkpoint.

## Branch model
Each repository must use these long-lived branches:
- `main`
- `dev`

Rules for both repositories:
- Never commit directly to `main`.
- Never commit directly to `dev`.
- `main` and `dev` must only receive changes through reviewed pull request merges.

## Allowed working branches
All work must be done on a short-lived branch created from the appropriate base branch. Feature branches must be created from `dev`.

Allowed branch types:
- `feature/`
- `hotfix/`

Rules:
- Every non-pull-request-integration code change must be made on either a `feature/*` branch or a `hotfix/*` branch.
- Create `feature/*` branches from `dev` and verify or record the branch base before implementation starts.
- Do not create work directly on `main` or `dev`.
- Do not use version numbers, ticket numbers, iteration numbers, or commit counters in branch names unless the user explicitly requires them for a separate process.
- When a root feature needs engine changes, create or use a corresponding `feature/*` or `hotfix/*` branch inside `engine/`; prefer the same branch suffix unless there is a clear reason not to.

## Branch naming
Branch names must follow this format:

- `feature/<work-being-done>`
- `hotfix/<work-being-done>`

Examples:
- `feature/robot-assembly`
- `hotfix/fix-startup-crash`

Naming rules:
- Use lowercase words.
- Separate words with hyphens.
- Keep the name descriptive but concise.
- Do not append version numbers.
- Do not append commit iteration numbers.

## Pull request expectations
All integration must go through pull requests.

Rules:
- Do not merge branches locally for the user; all integration must go through pull requests.
- Do not directly merge completed root feature work into root `dev`; open or prepare a pull request into root `dev`.
- Do not directly merge completed engine feature work into engine `dev`; open or prepare a pull request into engine `dev`.
- Root `dev` should receive the resulting submodule pointer update through a root pull request, not a direct commit or local merge.
- Completed hotfix work must go through the appropriate pull request release process, never by committing or merging directly to `main` or `dev` in either repository.
- Preserve the rule that `main` and `dev` receive changes only through reviewed pull requests.
- Push feature and hotfix branches to `origin` so pull requests can be opened.
- If `origin` is not configured, treat that repository as local-only and record pull request and push status as `N/A (local-only repository)`.
- If a required push fails, do not treat the pull request checkpoint as complete; record the failure and remediate before proceeding.

## Remote backup expectations
- After every commit on `feature/*` or `hotfix/*`, push that branch to `origin` immediately when `origin` is configured.
- If the branch has no upstream yet, set it on first push, for example `git push -u origin <branch>` or `git -C engine push -u origin <branch>`.
- Track root and engine push status separately when both repositories are involved.
- If `origin` is not configured, treat that repository as local-only and record push status as `N/A (local-only repository)`.
- If a required push fails, do not treat the commit checkpoint as complete; record the failure and remediate before continuing phase progression or pull request steps.

## Submodule pointer expectations
When a feature changes Foundation Engine code:
1. Confirm `engine/` is on a valid `feature/*` or `hotfix/*` branch before editing.
2. Commit and push the engine branch inside `engine/` first.
3. Record the exact engine commit hash that Last Beacon is bound to.
4. Return to the root repository and verify `git status --short` shows the `engine` pointer change.
5. Commit the root pointer update together with any root integration changes and tracker updates.
6. Record both commits in the feature tracker:
   - engine branch name
   - exact engine commit hash that the game is bound to
   - engine push and pull request state
   - root commit hash and push state
   - root pull request state
   - root `engine` submodule pointer state
7. Do not mark the root task complete if engine changes exist only as uncommitted submodule work or if the tracker does not state the exact engine commit hash used by the game.

## Commit message rules
Commit messages must:
- be short
- briefly describe the work being done
- use correct capitalization in the title line
- start the title line with a capitalized first word, for example `Add plugin compile prompt`
- preserve the source's original capitalization when naming a file, class, property, function, symbol, or code snippet
- not use prefixes like `feat(...)`, `fix(...)`, or similar conventional commit prefixes unless the user explicitly asks for them
- not include author or co-author lines
- include a list of the files changed

### Commit message format
Use this structure:

```text
Short description of work

Files changed:
- path/to/file1
- path/to/file2
```

For engine commits, paths should be relative to `engine/` because the commit is made inside the submodule. For root commits that update the submodule pointer, include `engine` in the changed file list.

Example root commit:

```text
Update project workflow guidance

Files changed:
- AGENTS.md
- .pi/skills/gitflow-workflow/SKILL.md
```

Example engine commit:

```text
Add runtime scene helper

Files changed:
- crates/foundation-runtime-library/src/scenes.rs
```

## How to apply this skill
When asked to help with Git workflow in this project:
1. Determine whether the work affects the root repository, the `engine/` submodule, or both.
2. Ensure each affected repository is on a `feature/*` or `hotfix/*` branch before editing.
3. Refuse or redirect any request that would commit directly to `main` or `dev`.
4. Refuse or redirect any request to merge directly into `main` or `dev`; prepare pull requests instead.
5. For engine work, perform submodule commits from inside `engine/` before root pointer commits.
6. After each commit, require a push to `origin` when `origin` exists so a pull request can be opened.
7. If no `origin` exists, explicitly record push and pull request state as `N/A (local-only repository)` for that repository.
8. If push fails, record the failure and keep the checkpoint in a non-complete state until push remediation succeeds.
9. Record the exact engine commit hash that the game is bound to whenever engine work is involved.
10. Generate commit messages using the required short format, proper title capitalization, and changed file list.
11. Preserve exact capitalization for any source names referenced in the commit title or body.
12. Do not add commit prefixes, author lines, or co-author lines.

## Summary
Required policy:
- Long-lived branches in root and engine: `main`, `dev`
- No direct commits to `main` or `dev`
- No direct local merges into `main` or `dev`; integration goes through pull requests
- Work only on `feature/*` or `hotfix/*`
- Branch naming: `<branch-type>/<work-being-done>`
- Engine changes must be committed inside `engine/`
- Root repository commits the resulting `engine` submodule pointer when needed
- Track the exact engine commit hash that the game is bound to
- Push to `origin` after every commit when `origin` exists so pull requests can be opened
- Local-only repos must record push and pull request state as `N/A (local-only repository)`
- Push failures must be recorded and remediated before checkpoints are considered complete
- Short commit messages with capitalized titles and changed file lists
- No author or co-author lines
