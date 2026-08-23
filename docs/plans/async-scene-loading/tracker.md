# Async Scene Loading Tracker

## Metadata
- Feature slug: `async-scene-loading`
- Feature area: `multi-area`
- Primary area: `engine`
- Root branch: `feature/async-scene-loading`
- Engine branch: `feature/async-scene-loading`
- Root branch base verification: `Verified: created from root dev at 7cacf7cabfff058305c08d9988dc15bd935f49e4`
- Engine branch base verification: `Verified: created from engine dev at 1bc59f9a0039dfe412b735c869a90f38a0d58582`
- Engine submodule pointer: `1bc59f9a0039dfe412b735c869a90f38a0d58582 (unchanged so far)`
- Overall status: `Implementation in progress; user approved proceeding in full on 2026-08-23`
- Planning model: `gpt-5.5`
- Preferred implementation model: `gpt-5.4`
- Optional final review model: `gpt-5.5`
- Current handoff state: `Implementation in progress with gpt-5.4`
- Created: `2026-08-23`
- Last updated: `2026-08-23`

## Validation Rules
- Task complete only after required validation passes and documentation generation is recorded, unless a waiver is recorded.
- Phase complete only after required validation passes, documentation generation is recorded, required commits/pushes are complete, and required user confirmation is recorded.
- Engine work must be committed inside `engine/` before the root repository commits the updated `engine` submodule pointer.

## Repository State
- Root commit/push state: `Pending; branch created locally, no commits yet`
- Engine commit/push state: `Phase 1 commit f91712e pushed to origin/feature/async-scene-loading`
- Root submodule pointer update: `Pending; will be updated once a root commit needs to bind a specific engine commit`
- Root pull request state: `Pending`
- Engine pull request state: `Pending — will open once all engine phases land`

## Background
This feature replaces the approach previously attempted on `feature/scene-pop-in-investigation`, which was abandoned after accumulating a self-inflicted livelock (repeated despawn/respawn of the same startup scenes) that produced a black-screen startup hang. That branch's root cause was fully diagnosed (see `plan.md`'s Codebase Research section) before being abandoned in favor of rebuilding from the clean `dev` baseline with a deliberately smaller design. The abandoned branch's engine work is preserved in `engine` stash `wip: bsn self-modified suppression investigation` for reference only; it is not being applied.

## Phase 1: Fix pre-existing false-positive hot-reload detection
**Status:** Complete
**Goal:** `replace_reloaded_bsn_instances` never reacts to Foundation's own resolve-caching writes as if they were a real file edit, on the current `dev` baseline (this bug predates the abandoned branch).

### Tasks
- [x] Write a failing regression test proving an instance resolving itself does not despawn/replace itself across several `app.update()` cycles.
  - Status: Complete
  - Repository: `engine`
  - Notes: `resolving_an_instance_does_not_trigger_its_own_despawn_and_replace`; confirmed red (missing `FoundationBsnApplyPending` bug in the test itself caught first, then confirmed the real fix was needed) before the fix landed.
- [x] Implement grace-window suppression in `replace_reloaded_bsn_instances` (exclude `LoadedWithDependencies` entirely; suppress `Modified` within a short window after Foundation's own resolve).
  - Status: Complete
  - Repository: `engine`
  - Notes: Added `FoundationBsnSelfResolveSuppression` resource (500ms default window, configurable per-instance for tests) and `note_self_resolve`/`is_self_inflicted`. Updated `hot_reload_replaces_old_root_and_children` to use `Modified` instead of `LoadedWithDependencies` (the latter was never a genuine reload signal).
- [x] Add a second test proving a `Modified` event arriving after the grace window still replaces the instance (genuine hot reload still works).
  - Status: Complete
  - Repository: `engine`
  - Notes: `a_modified_event_after_the_suppression_window_is_treated_as_a_genuine_reload` uses a 5ms test window + real 20ms sleep to prove expiry.

### Validation
- Engine validation: `Passed: cargo test -p foundation-runtime-library --all-features (95 passed), cargo clippy --all-targets --all-features -D warnings (clean), cargo fmt --all -- --check (clean)`
- Documentation generation: `N/A for this phase (internal fix, no public API change)`
- User confirmation: `Not required until phase handoff unless implementation discovers scope changes`

## Phase 2: Reintroduce scene/widget readiness gating
**Status:** Complete
**Goal:** A scene-owned BSN root (and any nested Last Beacon widget) stays hidden until its content has actually applied, without any cache or token machinery.

### Tasks
- [x] Add `SceneContentLoading` marker in `scene_stack.rs`; spawn BSN roots `Visibility::Hidden` with the marker in `bsn_assets.rs`, cleared on apply success or failure.
  - Status: Complete
  - Repository: `engine`
  - Notes: Also added `reveal_ready_standalone_bsn_instances` since standalone (non-scene-owned) instances now start hidden too and have no scene-stack sync to reveal them — this path exists on the public `FoundationBsnCommandsExt::spawn_bsn_asset` API even though nothing in-tree currently calls it.
- [x] Extend `sync_scene_entity_visibility` to require `stack.is_visible(id) && !loading`.
  - Status: Complete
  - Repository: `engine`
- [x] Mark Last Beacon widget slots `SceneContentLoading` while pending in `ui_widgets.rs`; clear on success or failure (`LastBeaconBsnWidgetFailed` must count as settled).
  - Status: Complete
  - Repository: `root`
  - Notes: Made `propagate_loaded_bsn_scene_owners` `pub` and exported it via the engine prelude so `queue_last_beacon_bsn_widgets` can be ordered `.after()` it in `game/src/lib.rs` — a widget slot must already carry `SceneOwner` before it gains `SceneContentLoading`, or the marker briefly applies to no scene.
- [x] Engine + game tests for readiness gating, including a failed load never permanently hiding a scene.
  - Status: Complete
  - Repository: `both`
  - Notes: 6 new engine tests (`scene_stack.rs` + `bsn_assets.rs`), 4 new game tests (`ui_widgets.rs`).

### Validation
- Engine validation: `Passed: cargo test -p foundation-runtime-library --all-features (102 passed), cargo clippy --all-targets --all-features -D warnings (clean), cargo fmt --all -- --check (clean)`
- Game validation: `Passed: cargo test --manifest-path game/Cargo.toml --all-features (13 lib + 2 integration passed), cargo clippy --all-targets --all-features -D warnings (clean), cargo fmt -- --check (clean)`
- Documentation generation: `Waived for this phase — consolidated into Phase 6's single engine/docs/scene-system.md update covering readiness, load modes, and preload registry together`
- User confirmation: `Not required until phase handoff unless implementation discovers scope changes`
- Manual smoke test: launched via `cargo run --manifest-path game/Cargo.toml -- --log-inline`, ran the full 20s window, no ERROR lines, no early exit — matches the healthy `dev`-baseline log pattern exactly.

## Phase 3: `SceneLoadMode` and blocking transitions
**Status:** Planned
**Goal:** A scene open can request `Blocking` mode; the stack mutation is held until the target (and its blocking preload dependencies) are ready, without ever stalling the frame loop.

### Tasks
- [ ] Add `SceneLoadMode` (`Streaming` default, `Blocking`) to `OpenSceneOptions`.
  - Status: Planned
  - Repository: `engine`
- [ ] Implement off-stack spawn + pending-transition holding for `Blocking` opens; activate (assign `SceneOwner`, push stack entry) once ready.
  - Status: Planned
  - Repository: `engine`
- [ ] Failure handling: a failed blocking target resolves the pending transition to an explicit failure rather than waiting forever.
  - Status: Planned
  - Repository: `engine`
- [ ] Tests: blocking open does not appear on the stack until ready; stack stays interactive/rendering during the wait; failed blocking target does not hang the pending transition.
  - Status: Planned
  - Repository: `engine`

### Validation
- Engine validation: `Pending`
- Documentation generation: `Pending — engine/docs/scene-system.md load-mode section`
- User confirmation: `Not required until phase handoff unless implementation discovers scope changes`

## Phase 4: Per-scene preload declarations
**Status:** Planned
**Goal:** A scene can declare `Background` and `Blocking` preload targets on other scenes; no automatic refill after consumption.

### Tasks
- [ ] Add `ScenePreloadRegistry` / `ScenePreloadTarget` / `ScenePreloadMode` in `scene_stack.rs`.
  - Status: Planned
  - Repository: `engine`
- [ ] Trigger background preload on `SceneAdded`/`SceneFocused` for registered targets not already loading/loaded.
  - Status: Planned
  - Repository: `engine`
- [ ] Wire `Blocking`-mode preload targets into Phase 3's readiness check for the owning scene.
  - Status: Planned
  - Repository: `engine`
- [ ] Record Last Beacon's concrete preload registrations once the user specifies them (examples already given: `gameplay_level → pause_menu`, `pause_menu → options_menu`, both `Background`).
  - Status: Planned; blocked on user input
  - Repository: `root`

### Validation
- Engine validation: `Pending`
- Game validation: `Pending`
- Documentation generation: `Pending`
- User confirmation: `Required — exact preload registrations beyond the two given examples`

## Phase 5: Close the original pop-in investigation
**Status:** Planned
**Goal:** Splash → main menu uses `Blocking` mode; no visible pop-in on first-ever menu display.

### Tasks
- [ ] Switch the `splash_bevy → main_menu` transition in `game/src/scenes/mod.rs` to `SceneLoadMode::Blocking`.
  - Status: Planned
  - Repository: `root`
- [ ] Manual validation: launch repeatedly, confirm no structural or font pop-in on main menu appearance.
  - Status: Planned
  - Repository: `root`

### Validation
- Game validation: `Pending`
- Documentation generation: `N/A`
- User confirmation: `Pending — user play-test`

## Phase 6: Documentation and full validation
**Status:** Planned
**Goal:** Document the new model; run full engine/game validation; commit, push, update submodule pointer.

### Tasks
- [ ] Update `engine/docs/scene-system.md`.
  - Status: Planned
  - Repository: `engine`
- [ ] Run `engine/scripts/validate-project.cmd` and `scripts/validate.cmd`.
  - Status: Planned
  - Repository: `both`
- [ ] Commit and push engine changes; record exact engine commit hash.
  - Status: Planned
  - Repository: `engine`
- [ ] Update root submodule pointer; commit and push root changes.
  - Status: Planned
  - Repository: `root`
- [ ] Manual smoke test: full session covering cold start, splash→menu, navigation, and a repeated-resolve/apply log check.
  - Status: Planned
  - Repository: `root`

### Validation
- Engine validation: `Pending`
- Game validation: `Pending`
- Documentation generation: `Pending`
- User confirmation: `Pending`

## Implementation / Review Handoff Notes
- Use `gpt-5.4` for implementation, `gpt-5.5` for optional final review.
- Read `.pi/skills/feature-tracker-update/SKILL.md`, `.pi/skills/rust-workspace-dev/SKILL.md`, `.pi/skills/rust-coding-standards/SKILL.md`, `.pi/skills/gitflow-workflow/SKILL.md`, and `.pi/skills/foundation-architecture/SKILL.md` before implementation edits.
- Implement phases in order; do not start Phase 2 gating until Phase 1's tests are green, per `plan.md`'s explicit reasoning for why this ordering matters.
- Do not reintroduce a generic readiness-token API or automatic cache refill — see `plan.md`'s Implementation Handoff Notes.

## Postponed Work
- Automatic preload refill after a cached scene is consumed: deliberately deferred, not scheduled.
- Chunked/incremental `ScenePatch::apply`: deferred pending profiling evidence that Phase 3/5 alone is insufficient.

## Notes / Issues / Oversights
- `2026-08-23`: User interrupted an in-progress patch-forward attempt on `feature/scene-pop-in-investigation` (a time-window self-modified-event suppression fix, applied but not yet verified against the full livelock) and asked to instead verify `dev` directly. Confirmed by direct launch that `dev` boots cleanly to the main menu with no black screen. Root repo and engine submodule both reset to `dev` tip; the abandoned branch's in-progress engine diff is preserved only as `engine` stash `wip: bsn self-modified suppression investigation`.
- `2026-08-23`: While reading `dev`'s current `bsn_assets.rs` to establish the baseline, found the same `LoadedWithDependencies`/`Modified`-as-reload false positive already present on `dev`, predating the abandoned branch. Recorded as Phase 1 of this plan rather than a separate hotfix, since it is a direct prerequisite for Phase 2's readiness gating to be safe.

## Progress Log
- `2026-08-23`: Diagnosed and root-caused the black-screen hang on `feature/scene-pop-in-investigation` (self-inflicted `AssetEvent::Modified` livelock between `apply_pending_bsn_instances`'s resolve-caching and `replace_reloaded_bsn_instances`'s hot-reload detection). Attempted two forward-fixes on that branch; the second (time-window suppression) was in progress when the user asked to instead verify `dev` directly.
- `2026-08-23`: Verified `dev` boots cleanly. Reset root and engine to `dev` tip; abandoned branch's in-progress fix preserved in `engine` stash only. Created `feature/async-scene-loading` from `dev` in both repositories. Read `dev`'s current scene-stack/BSN baseline and confirmed the same hot-reload false positive exists there too, plus confirmed no readiness/pop-in gating exists on `dev` at all currently. Clarified design scope with the user (two-question round: no automatic preload cache/refill for v1; preload dependencies are background-only, never stack entries) and captured the full blocking/streaming/dependency design in `plan.md`. Awaiting user approval to begin implementation.
