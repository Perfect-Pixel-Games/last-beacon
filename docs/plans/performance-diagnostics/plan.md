# Performance Diagnostics Tooling Plan

## Metadata
- Feature slug: `performance-diagnostics`
- Feature area: `multi-area`
- Primary area: `engine`
- Branch: `feature/performance-diagnostics` (root and `engine/` submodule, both created from `dev`)
- Status: `Planned`
- Planning model: `gpt-5.5` (Claude Sonnet 5, per durable user override recorded 2026-07-20 — see repo AGENTS.md model-policy note)
- Implementation model: `gpt-5.4` (Claude Sonnet 5, same override)
- Review model: `gpt-5.5` (Claude Sonnet 5, same override)
- Created: `2026-09-09`
- Last updated: `2026-09-09`

## User Request
The game is hitting roughly 40fps and the user wants diagnostic tooling added to help monitor performance and figure out where the perf issues are coming from. The user is open to adding CPU/GPU capture tooling.

## Feature Summary
Three-phase effort:
1. Fix a dev-profile Cargo misconfiguration that unoptimizes all dependencies (including Bevy/glam/wgpu) for the exact workflow (`cargo run --manifest-path game/Cargo.toml`) the user uses day to day — a well-known Bevy footgun that can by itself explain most or all of the reported 40fps.
2. Wire in Tracy profiler support (Bevy's built-in `trace_tracy` feature) so individual system-level CPU cost is visible as a flamegraph, not just aggregate frame time.
3. Extend the existing `stat.perf` overlay (`engine/crates/foundation-runtime-library/src/perf_overlay.rs`) with a frame-time history graph (to catch stutter/spikes the running average hides) and a one-shot log-dump console command.

## Feature Area Classification
- Area: `multi-area`
- Primary area: `engine`
- Rationale: The overlay/graph/console-command work is reusable Foundation Engine functionality and lives entirely in `engine/crates/foundation-runtime-library`. The Tracy support is a single Cargo feature flag added to `game/Cargo.toml` (and mirrored in `engine/Cargo.toml` for engine-only dev workflows) with no app code required, since `bevy_log` wires the Tracy tracing layer automatically when the feature is compiled in.

## Codebase Research
- `game/Cargo.toml` and `engine/Cargo.toml` both had `[profile.dev] opt-level = 0` and `[profile.dev.package."*"] opt-level = 0` — i.e. dependencies are fully unoptimized even in the default `cargo run` dev build. Bevy's own setup guidance recommends `opt-level = 1` for the workspace's own crates and `opt-level = 3` for dependencies in dev builds specifically because unoptimized ECS/math/render dependency code can cost several times the frame budget.
- `game/Cargo.toml` builds as its own standalone workspace (`[workspace]` with no members, i.e. workspace root = itself); engine crates are pulled in as path dependencies. This means `engine/Cargo.toml`'s `[profile.*]` tables only govern builds run from inside `engine/` (e.g. `engine/scripts/*`, the `foundation` binary, `test-project`), not `cargo run --manifest-path game/Cargo.toml`. Both manifests were still fixed for consistency, since engine-local dev workflows hit the identical problem.
- `engine/crates/foundation-runtime-library/src/perf_overlay.rs` already implements a `FoundationPerfOverlayPlugin` (added in engine commit `5e66108`, "Adding perf overlay") toggled by the `stat.perf` console command. It already reads Bevy's `FrameTimeDiagnosticsPlugin`, `EntityCountDiagnosticsPlugin`, `SystemInformationDiagnosticsPlugin`, and `RenderDiagnosticsPlugin`, plus a custom `foundation/cpu_process_frame_time` diagnostic, and renders a 7-row grid (FPS, Frame Time, CPU Process, CPU Render, GPU Render, Memory, Entities) with "Now" and "Avg 1s" columns. It runs a tumbling 1-second running-average accumulator (`FoundationPerfOverlayRunningAverages`) independent of visibility, and is careful to only write `Text`/`Node` components when their rendered value actually changes (to avoid self-inflicted layout-invalidation cost — the overlay explicitly documents this as "the exact performance bug this overlay exists to help catch").
- `engine/crates/foundation-console-macros/src/lib.rs` implements the `#[console_command(name = "...")]` attribute macro used to register console commands; `stat.perf` is itself a dotted command name already, confirming dotted names are supported by the existing convention (no special dot-namespacing logic — the name is just a string).
- `game/Cargo.toml`'s `[features]` table already has a `dev-tools` feature (`dep:linkme`, `foundation-runtime-library/dev-tools`) and an `editor` feature, giving an established pattern for adding an opt-in `profiling` feature alongside them.
- Verified locally against the vendored crate source (`~/.cargo/registry/src/.../bevy-0.19.1/Cargo.toml`) that bevy 0.19 defines:
  ```
  trace = ["bevy_internal/trace"]
  trace_tracy = ["trace", "bevy_internal/trace_tracy", "debug"]
  ```
  So enabling `bevy/trace_tracy` is sufficient — no additional explicit dependency (e.g. `tracing-tracy`) needs to be added to either manifest.

## External Research
No external online research tooling was used. Findings on Bevy's `trace_tracy` feature and dev-profile optimization guidance are from established Bevy ecosystem knowledge, cross-checked directly against the vendored `bevy-0.19.1` crate source in the local Cargo registry cache (see Codebase Research above) rather than assumed from memory alone.

## Affected Files And Systems
- `game/Cargo.toml`: dev profile fix (done as an exploratory build already, pending confirmation); new `profiling` feature.
- `engine/Cargo.toml`: dev profile fix (done as an exploratory build already, pending confirmation).
- `engine/crates/foundation-runtime-library/src/perf_overlay.rs`: add frame-time history ring buffer + graph rendering; add `stat.perf.dump` console command.
- New doc: `docs/performance-profiling.md` (root-level, since it documents the game-facing workflow of building with `--features profiling` and using Tracy).

## Proposed Implementation Approach
1. **Dev profile fix** (already applied as an exploratory local build on this branch, pending fps confirmation from the user before being treated as done):
   - `[profile.dev] opt-level = 1`
   - `[profile.dev.package."*"] opt-level = 3`
   - Applied identically in both `game/Cargo.toml` and `engine/Cargo.toml`.
2. **Tracy integration**:
   - Add to `game/Cargo.toml`:
     ```toml
     [features]
     profiling = ["bevy/trace_tracy"]
     ```
   - No plugin/app code changes needed; `bevy_log`'s `LogPlugin` (part of `DefaultPlugins`) auto-installs the `tracing-tracy` layer when the feature is compiled in.
   - Add `docs/performance-profiling.md` covering: building/running with `cargo run --manifest-path game/Cargo.toml --features profiling`, obtaining and launching the (external, not vendored) Tracy profiler desktop app from its GitHub releases, connecting to a running capture, and reading the flamegraph alongside the `stat.perf` overlay's own rows.
3. **Extend `stat.perf` overlay**:
   - Add a `FoundationPerfOverlayFrameTimeHistory` resource holding a fixed-size ring buffer (target: last 5 seconds at 60fps ≈ 300 samples, capped so memory/update cost stays bounded regardless of actual frame rate) of recent frame-time-ms samples, fed from the same `FrameTimeDiagnosticsPlugin` data the "Frame Time" row already reads.
   - Render it as a row of thin `Node`-based bars (consistent with the existing pure-Bevy-UI approach used by the rest of the overlay — no custom rendering pipeline) beneath the existing stat grid, height-scaled against a fixed ms ceiling so spikes are visually obvious. Bar heights update only when the underlying sample actually changes, following the overlay's existing "don't write unchanged values" discipline.
   - Add `#[crate::console_command(name = "stat.perf.dump")]` that reads the current `DiagnosticsStore` + running averages the same way `refresh_perf_overlay_text` does, and logs one line per stat at `info` level, independent of whether the overlay is currently visible.
4. Update the root `engine` submodule pointer commit in the root repository once engine-side work (`perf_overlay.rs` changes) is committed in the `engine/` submodule.

## Alternatives Considered
- **GPU frame capture (RenderDoc/PIX) support**: considered but explicitly deferred by the user for this round — Tracy + the extended overlay were judged sufficient for now, and GPU capture tooling is mostly an external-tool documentation exercise that can be added later without touching this feature's code.
- **Custom render-pass-level profiling plugin**: Bevy's existing `RenderDiagnosticsPlugin` (already consumed by the overlay) already sums per-pass CPU/GPU timings; building a bespoke replacement was rejected as unnecessary duplication.

## Risks, Constraints, And Assumptions
- Assumption: the reported 40fps was measured under the unoptimized `dev` profile via plain `cargo run`; this plan treats confirming/refuting that as the first checkpoint before treating Tracy/overlay work as the primary fix.
- Risk: raising `opt-level` for the `dev` profile increases first-build (and dependency-change) compile time. This is the standard, accepted Bevy tradeoff (incremental rebuilds of the workspace's own crates stay fast since only they use `opt-level = 1`; only dependency compiles, which are cached, pay the `opt-level = 3` cost).
- Risk: the `trace_tracy` feature has real runtime overhead even when no Tracy client is attached; mitigated by keeping it strictly opt-in via the `profiling` Cargo feature rather than default-on.
- Constraint: the Tracy desktop profiler application itself is an external tool (not a Rust crate) and will not be vendored into this repository; the doc will link build/download instructions rather than bundling a binary.

## Open Questions
- None currently blocking. Whether GPU capture tooling (RenderDoc/PIX) gets added later depends on whether Tracy's CPU-side data is sufficient to explain the remaining perf picture after this feature lands.

## Documentation Expectations
- `docs/performance-profiling.md` is new user-facing/developer-facing documentation covering the Tracy workflow end to end.
- New public items in `perf_overlay.rs` (the history resource, any newly-public helpers) get Rustdoc comments consistent with the file's existing documentation density.
- No generated-API-doc-only items are expected to need special treatment beyond the standard `cargo doc` pass.

## Implementation Handoff Notes
- Implementer: Claude Sonnet 5 (standing in for `gpt-5.4` per the durable model-policy override).
- Follow `engine/crates/foundation-runtime-library/src/perf_overlay.rs`'s existing conventions exactly: change-gated writes (never mark `Node`/`Text` changed when the value is identical), the tumbling real-time averaging pattern, and its existing test style (plain `App`-based unit tests, no integration harness).
- Confirm the fps delta from the profile fix with the user (they will run the game and report back via the existing `stat.perf` overlay) before considering Phase 1 validated.
- Reuse the existing `sum_render_pass_diagnostics`/`current_value` helpers for the new dump command rather than re-deriving stat values.

## Optional Review Focus Areas
- Reviewer: Claude Sonnet 5 (standing in for `gpt-5.5` per the durable model-policy override).
- Confirm the frame-time history buffer has a bounded, fixed memory footprint regardless of session length or frame rate.
- Confirm the `profiling` feature does not leak into `foundation-shipping` builds (shipping passes `--no-default-features`, but double check nothing wires `profiling` in transitively by default).

## Success Criteria
- `cargo run --manifest-path game/Cargo.toml` (the user's normal workflow) shows a measurably higher, user-confirmed fps after the profile fix.
- `cargo run --manifest-path game/Cargo.toml --features profiling` builds and connects to a running Tracy capture session, showing per-system spans.
- The `stat.perf` overlay displays a frame-time history graph that visibly reflects real spikes/stutter.
- `stat.perf.dump` writes a readable one-line-per-stat snapshot to the log.

## Testing Methodology
- `scripts/validate.cmd` (root, game-facing)
- `engine\scripts\format-project.cmd`
- `engine\scripts\lint-project.cmd`
- `engine\scripts\test-project.cmd`
- `engine\scripts\compile-project.cmd`
- `engine\scripts\validate-project.cmd`
- `cargo build --manifest-path game/Cargo.toml --features profiling` as an explicit extra check that the new feature compiles.
