# Performance Diagnostics Tooling Tracker

## Metadata
- Feature slug: `performance-diagnostics`
- Feature area: `multi-area`
- Primary area: `engine`
- Branch: `feature/performance-diagnostics` (root and `engine/` submodule)
- Overall status: `In Progress`
- Planning model: `gpt-5.5` (Claude Sonnet 5, per durable override)
- Preferred implementation model: `gpt-5.4` (Claude Sonnet 5, per durable override)
- Optional final review model: `gpt-5.5` (Claude Sonnet 5, per durable override)
- Current handoff state: `Implementing`
- Created: `2026-09-09`
- Last updated: `2026-09-09`

## Validation Rules
- Task complete only after required Rust validation passes and documentation generation is recorded, unless a waiver is recorded.
- Phase complete only after required validation passes, documentation generation is recorded, and required user confirmation is recorded.

## Phase 1: Dev profile fix
**Status:** In Progress
**Goal:** `cargo run --manifest-path game/Cargo.toml` uses `opt-level = 1` for workspace crates and `opt-level = 3` for dependencies, and the user confirms an fps improvement.

### Tasks
- [x] Set `[profile.dev] opt-level = 1` and `[profile.dev.package."*"] opt-level = 3` in `game/Cargo.toml`
  - Status: Done (exploratory build in progress)
  - Notes: Applied on `feature/performance-diagnostics` before plan/tracker existed, as a build-time experiment to gather evidence for the design. `cargo build --manifest-path game/Cargo.toml` kicked off in background to confirm it compiles.
- [x] Set the same profile fix in `engine/Cargo.toml`
  - Status: Done
  - Notes: `game/Cargo.toml` is its own workspace root (path deps don't inherit `engine/Cargo.toml` profiles), so this only affects engine-local dev workflows (`engine/scripts/*`), not the game's own `cargo run` — fixed anyway for consistency.
- [ ] Confirm background build succeeded
  - Status: Pending
- [ ] User runs the game and reports fps via the `stat.perf` overlay to confirm the improvement
  - Status: Pending

### Validation
- Format: Pending
- Lint: Pending
- Tests: Pending
- Build: In progress (background build running)
- Documentation generation: N/A (no public API changed)
- Full validation wrapper: Pending
- User confirmation: Pending — required (fps must be confirmed improved)

## Phase 2: Tracy integration
**Status:** Planned
**Goal:** `cargo run --manifest-path game/Cargo.toml --features profiling` connects to a running Tracy capture and shows per-system spans.

### Tasks
- [ ] Add `profiling = ["bevy/trace_tracy"]` to `game/Cargo.toml` `[features]`
  - Status: Planned
- [ ] Write `docs/performance-profiling.md`
  - Status: Planned

### Validation
- Format: Pending
- Lint: Pending
- Tests: Pending
- Build: Pending
- Documentation generation: Pending
- Full validation wrapper: Pending
- User confirmation: Pending / Not required yet

## Phase 3: Extend `stat.perf` overlay
**Status:** Planned
**Goal:** Overlay shows a frame-time history graph; `stat.perf.dump` logs a one-shot snapshot.

### Tasks
- [ ] Add frame-time history ring-buffer resource and bar-graph rendering to `perf_overlay.rs`
  - Status: Planned
- [ ] Add `stat.perf.dump` console command
  - Status: Planned
- [ ] Add/extend unit tests following the file's existing test style
  - Status: Planned
- [ ] Commit engine submodule work, record exact engine commit hash, update root submodule pointer
  - Status: Planned

### Validation
- Format: Pending
- Lint: Pending
- Tests: Pending
- Build: Pending
- Documentation generation: Pending
- Full validation wrapper: Pending
- User confirmation: Pending / Not required yet

## Implementation / Review Handoff Notes
- None yet.

## Postponed Work
- GPU frame capture (RenderDoc/PIX) tooling: explicitly deferred by the user for this round (see plan's Alternatives Considered).

## Progress Log
- `2026-09-09`: Brainstormed with user; confirmed 40fps was observed via plain `cargo run` (unoptimized dev profile). User approved fixing the dev profile first, then scoped diagnostic tooling to Tracy integration + extending the `stat.perf` overlay (declined GPU capture tooling for now).
- `2026-09-09`: Created `feature/performance-diagnostics` branch in both root and `engine/` submodule from their respective `dev` branches.
- `2026-09-09`: Applied dev-profile fix to `game/Cargo.toml` and `engine/Cargo.toml`; kicked off background build to verify.
- `2026-09-09`: Plan and tracker created; user approved proceeding with full implementation ("lets do it").
