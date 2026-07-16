# Last Beacon Pi Project Guide

This repository contains the **Last Beacon** game and uses the **Foundation Engine** as the `engine/` Git submodule.

Last Beacon work happens in the root repository. Foundation Engine work may happen too, but only inside the `engine/` submodule checkout and with the same Gitflow discipline used by the root repository.

## Model policy
- Use `gpt-5.5` for planning.
- Use `gpt-5.4` for implementation.
- Use `gpt-5.5` for review.
- Never use Anthropic models.

## Repository layout
- `README.md` is the project vision and setup source of truth.
- `game/` contains the Last Beacon Cargo package, game source, game manifest, and game-owned assets.
- `game/assets/` contains Last Beacon assets.
- `engine/` is the Foundation Engine Git submodule, checkout, junction, or symlink used by the game-facing scripts.
- `scripts/` contains Last Beacon build, run, package, and validation wrappers.
- `.pi/skills/` contains the project workflow skills that must be followed when their task descriptions apply.

Foundation-owned source, reusable engine systems, engine assets, and engine validation belong under `engine/`. Do not duplicate or patch Foundation Engine code in the root repository outside the submodule.

## Required Pi project resources
This repository expects Pi project resources to be available after cloning and trusting the project.

### Project-local skills
The following skills are mandatory when their matching task descriptions apply:
- `.pi/skills/feature-plan-docs/SKILL.md`
- `.pi/skills/feature-review-handoff/SKILL.md`
- `.pi/skills/feature-tracker-update/SKILL.md`
- `.pi/skills/foundation-architecture/SKILL.md`
- `.pi/skills/gitflow-workflow/SKILL.md`
- `.pi/skills/rust-coding-standards/SKILL.md`
- `.pi/skills/rust-workspace-dev/SKILL.md`

If a cloned checkout is missing one of these files, stop and report that the repository is incomplete.

### Expected non-project Pi extensions
The project expects these Pi extensions/packages when their tools or skills are needed:
- `@tintinweb/pi-subagents:src`
- `pi-hermes-memory:src`
- `pi-web-access`

If an extension-provided tool, command, or resource is unavailable when needed, stop and tell the user to install or reload the required Pi extensions/packages rather than silently substituting another workflow.

The `librarian` skill may be provided by the global `pi-web-access` installation. Use it when the task asks for evidence-backed open-source library research with GitHub permalinks. If it is missing, report that limitation instead of pretending it was used.

## Standard Rust workflow
When the user asks for Rust workspace, crate, module, test, build, lint, dependency, or validation work:
1. Read `.pi/skills/rust-workspace-dev/SKILL.md` first.
2. Read `.pi/skills/rust-coding-standards/SKILL.md` before writing, refactoring, generating, or reviewing Rust code.
3. Inspect the relevant manifest before proposing architecture or editing code:
   - `game/Cargo.toml` for Last Beacon game work.
   - `engine/Cargo.toml` for Foundation Engine work.
4. Inspect relevant source, asset, config, script, and test files before editing.
5. Prefer idiomatic Rust, minimal dependencies, and the project's self-documenting code standards.
6. Use game-facing validation from the root repository for Last Beacon work:
   - `scripts/validate.cmd`
   - focused `cargo` commands with `--manifest-path game/Cargo.toml` when appropriate.
7. Use Foundation Engine validation from inside the submodule for engine work:
   - `engine/scripts/format-project.cmd`
   - `engine/scripts/lint-project.cmd`
   - `engine/scripts/test-project.cmd`
   - `engine/scripts/compile-project.cmd`
   - `engine/scripts/doc-project.cmd`
   - `engine/scripts/validate-project.cmd` when a full engine validation is needed.

## Foundation Engine submodule rule
Foundation Engine changes are allowed, but they must be made inside `engine/`.

Rules:
- Treat `engine/` as an independent Git repository.
- Before editing engine files, inspect `engine/README.md`, `engine/AGENTS.md`, and relevant engine docs/source.
- Use `git -C engine ...` for submodule Git status, branch, commit, and push operations.
- Do not edit Foundation-owned files from the root repository path if they are not inside `engine/`.
- When engine changes are committed in the submodule, update and commit the root repository's `engine` submodule pointer as part of the Last Beacon feature work.
- If `FOUNDATION_ENGINE_PATH` is set to an alternate checkout, inspect that checkout before use, but do not modify it unless the user explicitly asks to work in that checkout and its Git state is safe.

## Foundation architecture workflow
When the user asks about Foundation runtime systems, Foundation engine launch behavior, Foundation editor tooling, game settings, scene-stack behavior, reusable game components, BSN scenes, Last Beacon integration with Foundation crates, or TemplateGame integration:
1. Read `.pi/skills/foundation-architecture/SKILL.md` first.
2. Keep reusable Foundation Engine code inside `engine/`.
3. Keep Last Beacon source, game manifests, game assets, and game-specific plugin glue inside `game/`.
4. Do not vendor Foundation Engine crates into the root repository.
5. Do not add game-specific behavior to generic Foundation Engine crates unless the plan clearly identifies it as reusable engine functionality.
6. Do not add Jackdaw dependencies unless the user explicitly approves a major architecture change; the current Foundation direction is Bevy-only.

## Git workflow
When the user asks about Git workflow, branch strategy, pull requests, commits, pushes, or submodule history:
1. Read `.pi/skills/gitflow-workflow/SKILL.md` first.
2. Treat that skill as the source of truth for both the root repository and the `engine/` submodule.
3. Root repository work must happen on `feature/*` or `hotfix/*` branches from the correct base.
4. Engine submodule work must also happen on `feature/*` or `hotfix/*` branches inside `engine/`, from the engine repository's correct base.
5. Never work on engine code while `engine/` is on `main` or `dev`; switch or create a valid `feature/*` or `hotfix/*` branch first.
6. Do not commit directly to `main` or `dev` in either repository.
7. Do not merge directly into `main` or `dev` in either repository; all integration must go through pull requests for both game and engine.
8. Push commits to `origin` when available so pull requests can be opened; record `N/A (local-only repository)` when no origin exists.
9. Maintain the exact Foundation Engine commit hash that the game is bound to whenever the root `engine` submodule pointer changes.

## Feature planning workflow
When the user asks to plan a new feature:
1. Read `.pi/skills/feature-plan-docs/SKILL.md` first.
2. Use `gpt-5.5` for planning. Never use Anthropic models.
3. Do not start implementation until both planning documents exist under `docs/plans/<new-feature>/` and the user has approved proceeding.
4. Ensure the root feature is associated with a dedicated `feature/*` branch from root `dev`, following `.pi/skills/gitflow-workflow/SKILL.md`.
5. If the feature includes Foundation Engine changes, also plan a dedicated engine submodule `feature/*` branch from engine `dev` and record it separately.
6. Record root branch, engine branch when applicable, and submodule pointer expectations in both plan and tracker.
7. Planning must stop after the plan and tracker are created or updated. Ask the user to review and confirm whether to proceed.
8. Treat natural approval phrasing such as `continue`, `carry on`, `go ahead`, `implement`, `proceed`, or equivalent affirmative review feedback as approval to begin implementation.
9. Treat negative or revision-seeking feedback such as `no`, `not yet`, `needs more work`, `revise this`, or equivalent responses as planning iteration requests rather than implementation approval.

## Feature implementation workflow
When the user asks to begin, continue, or update feature implementation:
1. If approved planning documents do not exist yet, stop and follow `.pi/skills/feature-plan-docs/SKILL.md` first.
2. If approved planning documents exist, read `.pi/skills/feature-tracker-update/SKILL.md` first, then read `.pi/skills/feature-plan-docs/SKILL.md`, `.pi/skills/rust-workspace-dev/SKILL.md`, `.pi/skills/rust-coding-standards/SKILL.md`, and `.pi/skills/gitflow-workflow/SKILL.md` as required context.
3. Use `gpt-5.4` for implementation. Never use Anthropic models.
4. Before implementation edits, read the relevant `plan.md` and `tracker.md`.
5. Confirm the root branch matches the planned `feature/*` or `hotfix/*` branch and verify its base when possible.
6. If engine work is planned, confirm the `engine/` submodule branch matches the planned engine `feature/*` or `hotfix/*` branch and verify its base when possible.
7. Update the tracker before edits to record that implementation is starting or resuming.
8. Keep the tracker updated with progress, validation state, issues found, postponements, model handoff state, root commit/push state, engine commit/push state, and submodule pointer state.
9. Commit each completed task and phase, including tracker updates. For engine changes, commit inside `engine/` first, record the exact engine commit hash that the game is bound to, then commit the root submodule pointer update.
10. Push each commit checkpoint to `origin` when available so pull requests can be opened. Do not merge branches directly; integration goes through pull requests.

## Final review workflow
When the user asks for a final sanity review of implemented feature work:
1. Read `.pi/skills/feature-review-handoff/SKILL.md` first.
2. Read `.pi/skills/rust-coding-standards/SKILL.md` before reviewing Rust code.
3. Use `gpt-5.5` for review. Never use Anthropic models.
4. Review both root and engine submodule changes when the feature touched both repositories.
5. Any review findings must be written to the tracker and presented to the user.
6. The user must choose whether to accept the implementation as-is, defer findings, or send findings back for `gpt-5.4` fixes.

## Enforcement rules
- For any feature planning request, the `feature-plan-docs` skill is mandatory.
- For any feature implementation request, the `feature-plan-docs` and `feature-tracker-update` skills are mandatory.
- For any optional final review request, the `feature-review-handoff` and `feature-tracker-update` skills are mandatory.
- For any Foundation Engine or Foundation integration request, the `foundation-architecture` skill is mandatory.
- For any branch, commit, pull request, push, or submodule pointer update, the `gitflow-workflow` skill is mandatory.
- If planning documents do not exist yet for feature implementation, stop and create them before implementation.

## Important notes
- Treat `README.md` as the source of truth for Last Beacon vision, setup, and high-level layout.
- Treat `game/Cargo.toml` as the source of truth for the Last Beacon game crate.
- Treat `engine/Cargo.toml` as the source of truth for Foundation Engine workspace structure.
- Use `scripts/build.cmd`, `scripts/run.cmd`, `scripts/package.cmd`, and `scripts/validate.cmd` for game-facing workflows.
- Root plans and trackers live under `docs/plans/<feature>/`.
- Engine plans and trackers, if created by engine-local work, live under `engine/docs/plans/<feature>/` unless the root feature plan intentionally tracks both repositories from root.
- Put generated root outputs under `artifacts/` and logs under `logs/` if persistent generated files are needed.
- Do not edit machine-local environment files unless the user explicitly asks.
