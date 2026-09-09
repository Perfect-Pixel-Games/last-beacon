# Performance Diagnostics Tooling Tracker

## Metadata
- Feature slug: `performance-diagnostics`
- Feature area: `multi-area`
- Primary area: `engine`
- Branch: `feature/performance-diagnostics` (root and `engine/` submodule)
- Overall status: `Implemented, awaiting user fps confirmation`
- Planning model: `gpt-5.5` (Claude Sonnet 5, per durable override)
- Preferred implementation model: `gpt-5.4` (Claude Sonnet 5, per durable override)
- Optional final review model: `gpt-5.5` (Claude Sonnet 5, per durable override)
- Current handoff state: `Ready for user review / PR`
- Created: `2026-09-09`
- Last updated: `2026-09-09`

## Validation Rules
- Task complete only after required Rust validation passes and documentation generation is recorded, unless a waiver is recorded.
- Phase complete only after required validation passes, documentation generation is recorded, and required user confirmation is recorded.

## Phase 1: Dev profile fix
**Status:** Done (validation), pending user fps confirmation
**Goal:** `cargo run --manifest-path game/Cargo.toml` uses `opt-level = 1` for workspace crates and `opt-level = 3` for dependencies, and the user confirms an fps improvement.

### Tasks
- [x] Set `[profile.dev] opt-level = 1` and `[profile.dev.package."*"] opt-level = 3` in `game/Cargo.toml`
  - Status: Done
- [x] Set the same profile fix in `engine/Cargo.toml`
  - Status: Done
  - Notes: `game/Cargo.toml` is its own workspace root (path deps don't inherit `engine/Cargo.toml` profiles), so this only affects engine-local dev workflows (`engine/scripts/*`), not the game's own `cargo run` — fixed anyway for consistency.
- [x] Confirm background build succeeded
  - Status: Done — `cargo build --manifest-path game/Cargo.toml` finished clean in 11m05s (one-time dependency recompile cost).
- [ ] User runs the game and reports fps via the `stat.perf` overlay to confirm the improvement
  - Status: Pending — asked the user; awaiting response.

### Validation
- Format: Pass
- Lint: Pass
- Tests: Pass
- Build: Pass (`cargo build --manifest-path game/Cargo.toml`, 11m05s clean)
- Documentation generation: N/A (no public API changed)
- Full validation wrapper: Pass
- User confirmation: Pending — required (fps must be confirmed improved)

## Phase 2: Tracy integration
**Status:** Done
**Goal:** `cargo run --manifest-path game/Cargo.toml --features profiling` connects to a running Tracy capture and shows per-system spans.

### Tasks
- [x] Add `profiling = ["bevy/trace_tracy"]` to `game/Cargo.toml` `[features]`
  - Status: Done
- [x] Write `docs/performance-profiling.md`
  - Status: Done

### Validation
- Format: Pass
- Lint: Pass
- Tests: N/A (Cargo feature flag + doc only, no new code paths)
- Build: Pass — `cargo build --manifest-path game/Cargo.toml --features profiling` finished clean in 11m03s (separate feature set forced another one-time dependency recompile)
- Documentation generation: Pass (`docs/performance-profiling.md` added)
- Full validation wrapper: Pass
- User confirmation: Not required (no behavior change to default build)

## Phase 3: Extend `stat.perf` overlay
**Status:** Done
**Goal:** Overlay shows a frame-time history graph; `stat.perf.dump` logs a one-shot snapshot.

### Tasks
- [x] Add frame-time history ring-buffer resource and bar-graph rendering to `perf_overlay.rs`
  - Status: Done — `FoundationPerfOverlayFrameTimeHistory` (120-sample ring buffer of raw/unsmoothed frame times) + a 120-bar UI strip (`FoundationPerfOverlayHistoryBar`/`FoundationPerfOverlayHistoryBarEntities`) rendered beneath the existing stat grid, clamped at a 50ms (20fps) ceiling.
- [x] Add `stat.perf.dump` console command
  - Status: Done — logs current+avg1s for all 7 stats at `info` level, independent of overlay visibility.
- [x] Add/extend unit tests following the file's existing test style
  - Status: Done — 6 new tests (bar-height clamping/scaling, history recording+capping, graph refresh visible/hidden/partially-filled).
- [x] Commit engine submodule work, record exact engine commit hash, update root submodule pointer
  - Status: Done

### Validation
- Format: Pass (`cargo fmt --all` applied; some new code needed reflow, now clean)
- Lint: Pass (`engine\scripts\lint-project.cmd`, `-D warnings`) — one fix needed: `stat_perf_dump` had to be non-`pub` because its `Res<FoundationPerfOverlayRunningAverages>` parameter type is private (`private_interfaces` lint), matching the file's other internal systems' visibility.
- Tests: Pass — `engine\scripts\test-project.cmd`: 154 passed, 0 failed (includes the 6 new tests).
- Build: Pass — `engine\scripts\compile-project.cmd` finished clean in 5m59s.
- Documentation generation: Doc comments added on all new public/private items consistent with file's existing density; no separate generated-doc step needed (no new public API surface beyond the console command).
- Full validation wrapper: Pass — `engine\scripts\validate-project.cmd` folded into the above per-step runs.
- User confirmation: Not yet requested — overlay/graph/dump command have not been manually exercised in a running game by the user yet (build succeeded; behavior not yet eyeballed in-game).

## Cross-Phase Validation Notes
- Root `scripts\validate.cmd` failed once with 2 `ui_widgets::tests::*` font-loading test failures (`reveal_text_once_fonts_load_marks_text_font_changed`, `text_finishing_its_font_load_clears_the_loading_marker`), in code this feature never touched. Root-caused to environmental contention: several other heavy cargo builds (engine test-project.cmd, engine compile-project.cmd, the `--features profiling` build) were running concurrently in the background at the time, and those two tests do a fixed 600-iteration `app.update()` spin-wait racing real on-disk async font loading with no timeout/sleep fallback — under contention the spin loop can exhaust its budget before the IO task pool finishes the read.
  - Verified: ran the `ui_widgets::` test module 5x back-to-back with no other background builds running — 0 failures across all 5 runs (116–124s each).
  - Verified: re-ran `scripts\validate.cmd` solo (no concurrent builds) — passed clean, including both previously-failing tests.
  - Conclusion: pre-existing test fragility (unrelated to this feature's changes), not a regression. Not fixed as part of this feature (out of scope — the plan didn't touch `ui_widgets.rs` and the user didn't ask for test-harness hardening); flagged here for visibility in case it recurs in CI, where similar resource contention is plausible.

## Implementation / Review Handoff Notes
- All three phases implemented, formatted, linted, tested, and built clean. Root and engine branches pushed to `origin`.
- Outstanding before this can be considered fully validated: user needs to (1) run the game and report the fps delta from the profile fix via `stat.perf`, and (2) ideally eyeball the new history graph / try `stat.perf.dump` once in a live session.
- The pre-existing `ui_widgets.rs` font-load test fragility noted above was not touched; flagging it to the user separately from this feature's own results.

## Postponed Work
- GPU frame capture (RenderDoc/PIX) tooling: explicitly deferred by the user for this round (see plan's Alternatives Considered).
- Hardening the two flaky `ui_widgets.rs` font-load tests against resource contention (see Cross-Phase Validation Notes): not requested, not part of this feature's scope.

## Progress Log
- `2026-09-09`: Brainstormed with user; confirmed 40fps was observed via plain `cargo run` (unoptimized dev profile). User approved fixing the dev profile first, then scoped diagnostic tooling to Tracy integration + extending the `stat.perf` overlay (declined GPU capture tooling for now).
- `2026-09-09`: Created `feature/performance-diagnostics` branch in both root and `engine/` submodule from their respective `dev` branches.
- `2026-09-09`: Applied dev-profile fix to `game/Cargo.toml` and `engine/Cargo.toml`; committed and pushed both branches (engine commit `144d821`, root commit `0adf9b3`).
- `2026-09-09`: Plan and tracker created; user approved proceeding with full implementation ("lets do it").
- `2026-09-09`: Implemented `profiling` Cargo feature + `docs/performance-profiling.md` (Phase 2).
- `2026-09-09`: Implemented frame-time history graph + `stat.perf.dump` command + tests in `perf_overlay.rs` (Phase 3).
- `2026-09-09`: Ran full engine validation (format, lint, test, compile) — all pass after one lint fix (command visibility). Investigated and resolved a transient root-validation flake unrelated to this feature (see Cross-Phase Validation Notes); confirmed clean solo `scripts\validate.cmd` pass.
