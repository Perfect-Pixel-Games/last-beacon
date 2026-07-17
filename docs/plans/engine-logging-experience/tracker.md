# Engine Logging Experience Tracker

## Metadata
- Feature slug: `engine-logging-experience`
- Feature area: `engine`
- Primary area: `engine`
- Root branch: `feature/engine-logging-experience`
- Engine branch: `feature/engine-logging-experience`
- Root branch base verification: `Verified from root dev on 2026-07-17`
- Engine branch base verification: `Verified from engine origin/dev on 2026-07-17`
- Engine submodule pointer: `Bound to engine feature commit efd4d76041a90523eba8091203adede8f9e6c306`
- Overall status: `In progress`
- Planning model: `gpt-5.5`
- Preferred implementation model: `gpt-5.4`
- Optional final review model: `gpt-5.5`
- Current handoff state: `Implementation in progress with gpt-5.4`
- Created: `2026-07-17`
- Last updated: `2026-07-17`

## Validation Rules
- Task complete only after required validation passes and documentation generation is recorded, unless a waiver is recorded.
- Phase complete only after required validation passes, documentation generation is recorded, required commits/pushes are complete, and required user confirmation is recorded.
- Never use Anthropic models.

## Repository State
- Root commit/push state: `Committed 14cf03ff8adeeaf83809cb1fe00402a525e136b7 and pushed to origin/feature/engine-logging-experience on 2026-07-17`
- Engine commit/push state: `Committed efd4d76041a90523eba8091203adede8f9e6c306 and pushed to origin/feature/engine-logging-experience on 2026-07-17`
- Root submodule pointer update: `Committed in root commit 14cf03ff8adeeaf83809cb1fe00402a525e136b7; root binds engine commit efd4d76041a90523eba8091203adede8f9e6c306`
- Pre-existing local changes: `Root game/Cargo.lock is modified before this feature starts; protect it until ownership is verified`
- Current engine checkout: `feature/engine-logging-experience at 7bfb91f7a7cfdc11f083352f5ce6b2a681a587e4`

## Phase 1: Branch And Architecture Setup
**Status:** Complete  
**Goal:** Move both repositories onto valid feature branches and establish the engine-owned logging API shape before code edits.

### Tasks
- [x] Create or switch root to `feature/engine-logging-experience` from root `dev`.
  - Status: Complete
  - Repository: `root`
  - Notes: Created root branch from `dev` on 2026-07-17. Preserved pre-existing `game/Cargo.lock` modification.
- [x] Create or switch engine submodule to `feature/engine-logging-experience` from engine `dev`.
  - Status: Complete
  - Repository: `engine`
  - Notes: Created engine branch from `origin/dev` on 2026-07-17 after fetching `origin/dev`.
- [x] Review current Bevy `DefaultPlugins` / `LogPlugin` setup in Last Beacon and Foundation runtime to decide the public integration helper shape.
  - Status: Complete
  - Repository: `both`
  - Notes: Added engine-owned `foundation_log_plugin()` so games configure Foundation logging at the same point they add Bevy `DefaultPlugins`, before global logging initializes.
- [x] Update tracker with branch/base verification and implementation start.
  - Status: Complete
  - Repository: `root`
  - Notes: Tracker updated before implementation edits with branch/base verification and `gpt-5.4` implementation start.

### Validation
- Game validation: `Passed as part of later root validation via scripts/validate.cmd on 2026-07-17`
- Engine validation: `Passed as part of later engine focused tests and wrappers on 2026-07-17`
- Documentation generation: `Passed via engine and game cargo doc on 2026-07-17`
- User confirmation: `User approved implementation start on 2026-07-17`

## Phase 2: Foundation Logging Runtime Policy
**Status:** Complete  
**Goal:** Add reusable engine-owned logging behavior for non-shipping file logs, `--log` visibility policy, and crash-log preservation.

### Tasks
- [x] Add Foundation logging settings/API surface in `foundation-runtime-library`.
  - Status: Complete
  - Repository: `engine`
  - Notes: Added `logging` module, `foundation_log_plugin()`, path/policy helpers, constants, `FoundationLoggingPaths`, Rustdoc, and prelude exports. Committed in engine `efd4d76041a90523eba8091203adede8f9e6c306`.
- [x] Add non-shipping normal log file output under `<exe-dir>/saved/logs/` with overwrite-per-run behavior.
  - Status: Complete
  - Repository: `engine`
  - Notes: Added non-shipping file layer that truncates `<exe-dir>/saved/logs/latest.log` at startup. Manual smoke confirmed marker text is overwritten on relaunch.
- [x] Add panic/crash preservation that writes timestamped crash logs without overwriting earlier crash logs.
  - Status: Complete
  - Repository: `engine`
  - Notes: Added panic hook that flushes/copies `latest.log` to `crash-<timestamp>.log`, with suffix fallback to avoid overwrites. Hard-abort limitations are documented.
- [x] Add platform-specific visible log-window handling for `--log`, with shipping suppression.
  - Status: Complete
  - Repository: `engine`
  - Notes: Non-shipping `--log` uses visible stderr formatting and allocates a Windows console when available; default/shipping use a sink formatter. Shipping smoke with `--log` produced zero visible log lines.
- [x] Add unit tests for path policy, argument/policy decisions, normal/crash filename behavior, and shipping suppression where practical.
  - Status: Complete
  - Repository: `engine`
  - Notes: Added Foundation runtime logging unit tests and focused launcher/build-tool regression coverage.

### Validation
- Game validation: `Passed cargo check/clippy/doc and scripts/validate.cmd after Last Beacon adoption on 2026-07-17`
- Engine validation: `Passed cargo test -p foundation-runtime-library, no-default-features check, focused clippy, and engine format/lint/test/doc/compile wrappers on 2026-07-17`
- Documentation generation: `Passed via cargo doc and engine doc-project wrapper on 2026-07-17`
- User confirmation: `Not required yet`

## Phase 3: Launcher, Build Tool, And Game Adoption
**Status:** Complete  
**Goal:** Ensure `--log` flows through Foundation launch/build paths and Last Beacon consumes the engine-owned logging helper.

### Tasks
- [x] Update `crates/foundation` launcher parsing, forwarding, and usage text for `--log`.
  - Status: Complete
  - Repository: `engine`
  - Notes: Launcher now parses and forwards `--log`; usage text includes `[--log]`.
- [x] Add/adjust `foundation-build` tests so runtime `--log` remains forwarded and shipping policy is represented.
  - Status: Complete
  - Repository: `engine`
  - Notes: Updated runtime-argument preservation test to include `--log`; shipping feature policy remains covered by `shipping_game_disables_default_features`.
- [x] Update Last Beacon to use the Foundation logging/default-plugin helper instead of local logging policy.
  - Status: Complete
  - Repository: `root`
  - Notes: Last Beacon `DefaultPlugins` now sets `foundation_log_plugin()`, and the Windows binary entry point uses the Windows subsystem so a console is not created by default. Committed in root `14cf03ff8adeeaf83809cb1fe00402a525e136b7`.
- [x] Update docs for launch/build usage, `--log`, log paths, crash logs, and shipping behavior.
  - Status: Complete
  - Repository: `engine`
  - Notes: Added `docs/logging.md` and linked logging behavior from `docs/foundation-engine.md`.

### Validation
- Game validation: `Passed cargo check, cargo clippy, cargo doc, shipping check/build, scripts/validate.cmd, and manual smoke checks on 2026-07-17`
- Engine validation: `Passed foundation/foundation-build/foundation-runtime-library focused tests and wrappers on 2026-07-17`
- Documentation generation: `Passed via cargo doc and engine doc-project wrapper on 2026-07-17`
- User confirmation: `Not required yet`

## Phase 4: Validation, Commits, And Pointer Update
**Status:** Awaiting user acceptance  
**Goal:** Validate the engine and root integration, commit/push engine first, then commit/push the root submodule pointer and tracker updates.

### Tasks
- [x] Run focused engine tests and full engine validation wrappers.
  - Status: Complete
  - Repository: `engine`
  - Notes: Passed focused tests/checks plus engine format, lint, test, doc, and compile wrappers on 2026-07-17.
- [x] Run root validation and relevant build/package smoke checks.
  - Status: Complete
  - Repository: `root`
  - Notes: Passed root check, clippy, docs, `scripts/validate.cmd`, `scripts/build.cmd --platform windows-x64 --configuration test --target game`, shipping check/build, and `scripts/package.cmd --platform windows-x64 --configuration shipping --target game` on 2026-07-17.
- [x] Perform manual smoke checks for no-log-window default, `--log` opt-in, normal log overwrite, crash log preservation, and shipping suppression.
  - Status: Complete
  - Repository: `both`
  - Notes: Runtime smoke checks covered default hidden logs, visible `--log`, overwrite, and shipping suppression. Crash-log preservation is covered by unit tests for timestamped non-overwriting paths and panic-hook implementation; no destructive manual panic harness was kept.
- [x] Commit and push engine changes on `feature/engine-logging-experience`.
  - Status: Complete
  - Repository: `engine`
  - Notes: Committed and pushed `efd4d76041a90523eba8091203adede8f9e6c306` to `origin/feature/engine-logging-experience`.
- [x] Update root `engine` submodule pointer, plan/tracker validation state, and commit/push root changes.
  - Status: Complete
  - Repository: `root`
  - Notes: Root commit `14cf03ff8adeeaf83809cb1fe00402a525e136b7` includes `engine` pointer, Last Beacon adoption, Cargo.lock update, and plan/tracker updates; pushed to origin.

### Validation
- Game validation: `Passed on 2026-07-17`
- Engine validation: `Passed on 2026-07-17`
- Documentation generation: `Passed on 2026-07-17`
- User confirmation: `Required before marking feature accepted`

## Implementation / Review Handoff Notes
- Use `gpt-5.4` for implementation.
- Never use Anthropic models.
- Re-read the plan before edits.
- Re-read `.pi/skills/feature-tracker-update/SKILL.md`, `.pi/skills/rust-workspace-dev/SKILL.md`, `.pi/skills/rust-coding-standards/SKILL.md`, `.pi/skills/gitflow-workflow/SKILL.md`, and `.pi/skills/foundation-architecture/SKILL.md` before implementation.
- Do not edit engine while it is detached or on `main`/`dev`.
- Do not commit root work directly to `dev`.
- Keep reusable logging code in `engine/`; Last Beacon changes should only adopt the engine API and update the root submodule pointer.
- Treat OS-level crash capture beyond Rust panic hooks as out of scope unless the user expands the requirement.

## Postponed Work
- None yet.

## Notes / Issues / Oversights
- The exact normal log filename is not yet confirmed. Plan recommendation is `<exe-dir>/saved/logs/latest.log`.
- The user said "crashes"; the implementation plan covers Rust panics first and documents hard-abort limitations.
- The visible "log window" is interpreted as native console/stdout log visibility, primarily on Windows.

## Progress Log
- `2026-07-17`: User requested an engine-wide logging feature: no log window by default, `--log` opt-in outside shipping, shipping always suppresses log window, non-shipping per-run file log under `<exe-dir>/saved/logs/`, and timestamped preserved crash logs.
- `2026-07-17`: Read required planning, Foundation architecture, Gitflow, and Rust workspace skills.
- `2026-07-17`: Inspected root README, `game/Cargo.toml`, Last Beacon entry points, `engine/README.md`, `engine/AGENTS.md`, `engine/Cargo.toml`, Foundation launcher, Foundation runtime library, Foundation build tool, and prior Foundation build/packaging plan.
- `2026-07-17`: Recorded current repository state: root branch `dev` with modified `game/Cargo.lock`; engine submodule detached at `b75c0f95d865a674b8cc45282f4ab293715e3687`.
- `2026-07-17`: Created plan and tracker under `docs/plans/engine-logging-experience/` and stopped before implementation pending user review.
- `2026-07-17`: User approved the plan. Created root and engine feature branches, verified branch bases, and started implementation with `gpt-5.4`.
- `2026-07-17`: Implemented Foundation logging runtime policy, launcher `--log` forwarding, build-tool regression coverage, docs, and Last Beacon adoption.
- `2026-07-17`: Validation passed: `cargo test --manifest-path engine/Cargo.toml -p foundation-runtime-library`, `cargo test --manifest-path engine/Cargo.toml -p foundation`, `cargo test --manifest-path engine/Cargo.toml -p foundation-build`, `cargo check --manifest-path engine/Cargo.toml -p foundation-runtime-library --no-default-features`, focused engine clippy, engine format/lint/test/doc/compile wrappers, `cargo check --manifest-path game/Cargo.toml`, `cargo check --manifest-path game/Cargo.toml --no-default-features --profile foundation-shipping`, root clippy, root docs, shipping build, `scripts/validate.cmd`, `scripts/build.cmd --platform windows-x64 --configuration test --target game`, and `scripts/package.cmd --platform windows-x64 --configuration shipping --target game`.
- `2026-07-17`: Manual smoke checks passed: non-shipping launch without `--log` produced zero visible log lines and created `saved/logs/latest.log`; non-shipping `--log` produced visible Bevy log output and created `latest.log`; relaunch overwrote a marker in `latest.log`; shipping `--log` produced zero visible log lines and no non-shipping latest log.
- `2026-07-17`: Committed and pushed engine changes as `efd4d76041a90523eba8091203adede8f9e6c306` on `origin/feature/engine-logging-experience`.
- `2026-07-17`: Committed and pushed root changes as `14cf03ff8adeeaf83809cb1fe00402a525e136b7` on `origin/feature/engine-logging-experience`, binding Last Beacon to engine commit `efd4d76041a90523eba8091203adede8f9e6c306`.
- `2026-07-17`: User found `scripts/run.cmd ... -- --log` passed `--project` after the runtime delimiter, causing `foundation-build` to report `Expected --game <name> or --project <path>`. Fixed `scripts/foundation-game.cmd` to insert `--project` before runtime arguments, and smoke-checked that `scripts/run.cmd --platform windows-x64 --configuration test --target game -- --log` reaches the game and emits visible logs.
