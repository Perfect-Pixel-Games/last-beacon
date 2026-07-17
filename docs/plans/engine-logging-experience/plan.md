# Engine Logging Experience Plan

## Metadata
- Feature slug: `engine-logging-experience`
- Feature area: `engine`
- Primary area: `engine`
- Root branch: `feature/engine-logging-experience`
- Root branch status: `Required before implementation; current root branch is dev with pre-existing modified game/Cargo.lock`
- Engine branch: `feature/engine-logging-experience`
- Engine branch status: `Required before implementation; engine submodule is currently detached at b75c0f95d865a674b8cc45282f4ab293715e3687`
- Engine submodule pointer: `Bound to engine feature commit efd4d76041a90523eba8091203adede8f9e6c306`
- Status: `Planned`
- Planning model: `gpt-5.5`
- Implementation model: `gpt-5.4`
- Review model: `gpt-5.5`
- Created: `2026-07-17`
- Last updated: `2026-07-17`

## User Request
Improve the engine logging experience for all Foundation games. By default, a launched game should not show a log window. A log window should be shown only when the game is launched with `--log`. In shipping builds, the log window must never be open, even when `--log` is passed. For any build other than shipping, the engine should save a log file under `/<exe root>/saved/logs/`, overriding the previous run's normal log each run. If the game crashes, save a timestamped crash log that is never overridden by later runs. This belongs in the engine and should apply to all games.

## Feature Summary
Add a reusable Foundation logging subsystem that centralizes runtime log-window policy, per-run file logging, and panic/crash log preservation. The feature should be engine-owned so every Foundation game receives consistent behavior through the Foundation runtime/build conventions rather than duplicating logging setup in each game.

## Feature Area Classification
- Area: `engine`
- Primary area: `engine`
- Rationale: The behavior is shared by all Foundation games and belongs in Foundation runtime/build infrastructure. Last Beacon only needs the root tracker and a submodule pointer update after the engine feature is committed.

## Codebase Research
- Root `game/Cargo.toml` depends on `foundation-runtime-library` through `../engine/crates/foundation-runtime-library` with default features disabled, and enables `foundation-runtime-library/dev-tools` through its own `dev-tools` feature.
- Root `game/src/lib.rs` currently builds Bevy `DefaultPlugins`, disables `GilrsPlugin`, customizes `AssetPlugin` and `RenderPlugin`, then adds `FoundationPlugin`. Because `DefaultPlugins` includes Bevy `LogPlugin`, the current game-owned plugin construction is part of the logging path and must be moved behind an engine-owned reusable helper or policy.
- `engine/Cargo.toml` defines `foundation-test` and `foundation-shipping` profiles. `foundation-shipping` sets `panic = "abort"`, which affects how much crash handling can run in shipping and reinforces the requirement that shipping should not show a log window.
- `engine/crates/foundation-runtime-library/Cargo.toml` has `default = ["dev-tools"]`; shipping builds disable default features through the build tool. Dev-only runtime behavior should continue to be guarded by configuration/features where appropriate.
- `engine/crates/foundation-runtime-library/src/lib.rs` installs shared runtime plugins through `FoundationPlugin`, and currently installs debug console tooling only when `dev-tools` is enabled. Logging should follow this central Foundation pattern while recognizing that Bevy's `LogPlugin` is normally configured before runtime plugins if games add `DefaultPlugins` themselves.
- `engine/crates/foundation/src/launch.rs` only accepts `--game`, `--editor`, and help. It currently rejects `--log`, so the loose Foundation launcher must be updated if `cargo run -p foundation -- --game last-beacon --log` should work.
- `engine/crates/foundation-build/src/lib.rs` already models `Debug`, `Test`, and `Shipping`; non-shipping configurations enable dev tools and shipping disables them. It forwards runtime arguments after `--` for `run`, which is the correct path for `--log` in packaged-style runs.
- `engine/docs/plans/foundation-build-packaging/plan.md` already names log windows as a dev/test tool and says shipping should exclude log windows/dev tools. This feature should implement that deferred logging portion consistently.

## External Research
- Bevy `LogPlugin` is part of `DefaultPlugins`, writes to stdout by default, and supports `custom_layer` and `fmt_layer` hooks for tracing subscriber customization. It should not be added multiple times in one process.
- Bevy documentation and examples show disabling `LogPlugin` from `DefaultPlugins` when installing a custom tracing subscriber, and adding custom tracing layers for file output.
- Bevy community discussion around file logging recommends using tracing/tracing-subscriber layers, often through `LogPlugin.custom_layer`, and keeping any non-blocking writer guard alive for the life of the app.

## Affected Files And Systems
- `engine/crates/foundation-runtime-library/src/lib.rs`: likely export and install a reusable logging configuration API/plugin/resource, or expose a helper used before `DefaultPlugins` are added.
- `engine/crates/foundation-runtime-library/src/logging.rs` or `src/logging/mod.rs`: likely new module for argument parsing, log path resolution, Bevy `LogPlugin` configuration helpers, panic hook/crash-log handling, and tests.
- `engine/crates/foundation-runtime-library/Cargo.toml`: may need tracing/file-writer dependencies if Bevy re-exports are not sufficient; dependency additions should be minimal and justified.
- `engine/Cargo.toml`: may need workspace dependency entries for any logging helper crates.
- `engine/crates/foundation/src/launch.rs`: accept and forward `--log` to the selected game process in loose launcher mode while preserving existing `--editor` behavior.
- `engine/crates/foundation/src/main.rs`: update usage text to include `--log` if the launcher accepts it.
- `engine/crates/foundation-build/src/lib.rs`: add tests/documentation around forwarded `--log` and ensure shipping run/package semantics ignore log-window display even if `--log` is passed.
- `engine/docs/`: add/update user-facing logging/build documentation, likely in `docs/foundation-engine.md` or a new `docs/logging.md` linked from existing engine docs.
- `game/src/lib.rs`: integration change may be needed so Last Beacon uses the new Foundation default plugin/logging helper instead of directly constructing Bevy `DefaultPlugins` in a way that bypasses engine-owned logging policy.
- `game/Cargo.toml` / `game/Cargo.lock`: may change only if the public engine helper requires a new dependency or if Last Beacon integration code changes dependency resolution.

## Proposed Implementation Approach
1. Define the Foundation logging contract:
   - `--log` requests visible console/log-window output only in non-shipping builds.
   - Shipping builds ignore `--log` for window visibility.
   - Non-shipping builds always write the normal log to `<exe-dir>/saved/logs/latest.log` or an equivalent stable filename that is overwritten at process start.
   - Panic/crash preservation writes `<exe-dir>/saved/logs/crash-<timestamp>.log` and never reuses that filename.
2. Add a small engine-owned logging settings/API surface in `foundation-runtime-library`, such as `FoundationLoggingSettings` plus a helper to build/configure Bevy `DefaultPlugins` with Foundation's `LogPlugin` policy.
3. Ensure games can adopt the helper before Bevy `LogPlugin` is installed. Because Bevy `LogPlugin` is part of `DefaultPlugins`, the Foundation helper should either return a configured `PluginGroupBuilder` or document a required pattern for games to call before adding `FoundationPlugin`.
4. Implement non-shipping file logging through Bevy/tracing subscriber customization:
   - Add a file layer that writes plain text without ANSI color codes.
   - Create the `saved/logs` directory relative to `std::env::current_exe().parent()`.
   - Open the normal log path with truncation at startup so each run overwrites the previous normal log.
5. Implement visible log-window policy:
   - On Windows, treat the native console attached to stdout/stderr as the log window. Hide/avoid it by default where possible, and allocate/show/attach only when `--log` is present and the build is non-shipping.
   - On non-Windows platforms, treat `--log` as enabling normal stdout/stderr log formatting where applicable because there is no equivalent Foundation-managed native console window.
   - Keep this code isolated behind platform-specific functions so unsupported platforms degrade predictably.
6. Implement crash-log preservation for Rust panics in non-shipping builds:
   - Install a panic hook after log setup.
   - Append panic information/backtrace availability notes to the active log or crash copy.
   - Copy or flush the active normal log to a timestamped crash log before delegating to the previous hook.
   - Document that hard aborts/OS-level crashes may not run Rust panic hooks.
7. Update launch/build argument forwarding:
   - `crates/foundation` should parse `--log` and forward it to the selected game process.
   - `foundation-build run ... -- --log` already has a runtime-argument path; add regression tests so it remains preserved.
   - Shipping code paths should not show a log window even if `--log` arrives in runtime arguments.
8. Update Last Beacon integration only as a consumer of the engine-owned helper, not as game-specific logging logic.
9. Add tests for argument parsing, path derivation, normal-log truncation intent, crash-log filename uniqueness/timestamp format, and build-configuration policy. Add manual smoke tests for visible/no-visible log behavior.

## Submodule Plan
- Engine changes required: `yes`
- Engine branch: `feature/engine-logging-experience`
- Engine commit expectation: Commit Foundation logging subsystem, launcher/build updates, engine docs, tests, and validation results inside `engine/` first.
- Bound engine commit hash: `efd4d76041a90523eba8091203adede8f9e6c306`
- Root pointer update required: `yes, after the engine commit is created and pushed`

## Alternatives Considered
- Implement logging directly in Last Beacon: rejected because the user explicitly wants an engine feature available to all games.
- Keep Bevy's default stdout logging and only add a file layer: insufficient because it would not satisfy the default no-log-window requirement.
- Use only build-script or Cargo profile settings to control console windows: insufficient because `--log` requires runtime opt-in for non-shipping builds.
- Rely on rotating log appenders for normal logs: rejected for normal runs because the user wants the previous normal log overridden per run; timestamped names are reserved for crash logs.

## Risks, Constraints, And Assumptions
- `log window` is interpreted as the native console/stdout log window, especially on Windows packaged games. If the intended UI is different, clarify before implementation.
- Bevy logging is globally initialized once per process. The implementation must not add a second subscriber after Bevy `LogPlugin` has already initialized logging.
- Rust panic hooks can preserve crash logs for panics, but cannot guarantee preservation for hard process aborts, access violations, GPU driver crashes, or `panic = "abort"` configurations.
- Shipping currently disables dev tools via feature selection in `foundation-build`; direct `cargo run` with default features is a development path, not a shipping path.
- If Windows console control requires a platform crate, add the smallest suitable dependency and keep it Windows-only where possible.
- The root repository currently has a pre-existing `game/Cargo.lock` modification. Do not overwrite or claim ownership of that change unless implementation later verifies it is part of this feature.

## Open Questions
- Confirm whether the normal overwritten log filename should be `latest.log`, `<game-name>.log`, or another exact name. Plan recommendation: `latest.log` under `<exe-dir>/saved/logs/` unless the user prefers a game-named file.
- Confirm whether crash-log preservation is required only for Rust panics or whether out-of-process crash reporting/minidumps are desired later. Plan recommendation: start with panic-preserved crash logs and document hard-crash limitations.
- Confirm whether `--log` should also force a visible console when running from `cargo run`, where a terminal may already exist.

## Documentation Expectations
- Public APIs added or changed by this feature must have Rustdoc comments, especially any `FoundationLoggingSettings`, default-plugin helper, or path-policy types exposed to game crates.
- Add user-facing engine documentation that explains `--log`, shipping behavior, log paths, normal-log overwrite behavior, crash-log naming, and hard-crash limitations.
- Update launcher/build usage text where `--log` is accepted or forwarded.
- Generated documentation must be produced before the feature is considered complete.

## Implementation Handoff Notes
- Use `gpt-5.4` for implementation.
- Never use Anthropic models.
- Before implementation edits, read `.pi/skills/feature-tracker-update/SKILL.md`, `.pi/skills/feature-plan-docs/SKILL.md`, `.pi/skills/rust-workspace-dev/SKILL.md`, `.pi/skills/rust-coding-standards/SKILL.md`, `.pi/skills/gitflow-workflow/SKILL.md`, and `.pi/skills/foundation-architecture/SKILL.md`.
- Create/switch root to `feature/engine-logging-experience` from root `dev` before root edits; protect the pre-existing modified `game/Cargo.lock`.
- Create/switch engine to `feature/engine-logging-experience` from engine `dev` before engine edits; do not edit engine while detached or on `main`/`dev`.
- Keep reusable Foundation implementation inside `engine/`. Root Last Beacon changes should be limited to adopting engine APIs and updating the submodule pointer/tracker.
- Keep shipping behavior compile-time safe where possible, not only runtime gated.
- Prefer tests around pure policy/path/argument code and manual smoke checks for native window behavior.

## Optional Review Focus Areas
- Use `gpt-5.5` for review.
- Verify games cannot accidentally bypass the engine logging policy by using old `DefaultPlugins` patterns without a clear compile/documentation signal.
- Verify shipping builds ignore `--log` for window visibility.
- Verify normal logs are overwritten every non-shipping run and crash logs are timestamped/preserved.
- Verify Foundation launcher/build docs and tests cover `--log`.

## Success Criteria
- Launching a non-shipping Foundation game without `--log` does not open a visible log window by default.
- Launching a non-shipping Foundation game with `--log` opens/shows the log window or equivalent stdout/stderr log output.
- Launching a shipping build with `--log` does not open/show a log window.
- Every non-shipping run writes a normal log under `<exe-dir>/saved/logs/` and overwrites that normal log on the next run.
- A Rust panic/crash in a non-shipping run preserves a timestamped crash log under `<exe-dir>/saved/logs/` without overwriting previous crash logs.
- The behavior is implemented in Foundation engine code and is reusable by all games.
- Last Beacon uses the engine-owned logging path rather than custom game-specific logging behavior.
- Tests and documentation cover the new logging policy.

## Testing Methodology
- Engine validation:
  - `engine/scripts/format-project.cmd`
  - `engine/scripts/lint-project.cmd`
  - `engine/scripts/test-project.cmd`
  - `engine/scripts/compile-project.cmd`
  - `engine/scripts/doc-project.cmd`
  - `engine/scripts/validate-project.cmd`
- Focused engine commands as useful:
  - `cargo test --manifest-path engine/Cargo.toml -p foundation-runtime-library`
  - `cargo test --manifest-path engine/Cargo.toml -p foundation`
  - `cargo test --manifest-path engine/Cargo.toml -p foundation-build`
- Root validation after adopting the engine pointer/API:
  - `scripts/validate.cmd`
  - `scripts/build.cmd --platform windows-x64 --configuration test --target game`
  - `scripts/package.cmd --platform windows-x64 --configuration shipping --target game`
- Manual smoke tests:
  - Non-shipping run without `--log`: confirm no visible log window and normal log file exists.
  - Non-shipping run with `--log`: confirm visible log window/log output and normal log file exists.
  - Re-run non-shipping: confirm normal log is overwritten.
  - Trigger a controlled panic in a non-shipping test path or temporary smoke harness: confirm timestamped crash log is preserved.
  - Shipping run/package with `--log`: confirm no visible log window.
