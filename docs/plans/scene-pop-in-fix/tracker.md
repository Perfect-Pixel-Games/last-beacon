# Scene Pop-In Fix Tracker

## Metadata
- Feature slug: `scene-pop-in-fix`
- Feature area: `multi-area`
- Primary area: `engine`
- Root branch: `feature/scene-pop-in-investigation`
- Engine branch: `feature/scene-pop-in-investigation`
- Root branch base verification: `Verified: dev (7cacf7cabfff058305c08d9988dc15bd935f49e4) is an ancestor of this branch; only the investigation doc commit sits on top`
- Engine branch base verification: `Verified: created feature/scene-pop-in-investigation from engine origin/dev at 1bc59f9a0039dfe412b735c869a90f38a0d58582 on 2026-07-20`
- Engine submodule pointer: `Updated to engine readiness-gating commit 0874b9c4ac462a20adff2fec8ee1b07ab88c78fd; root pointer commit pending`
- Overall status: `Regression found post-completion (large frame-time stutter on scene reveal); investigated, root cause understood, awaiting user decision on remediation direction`
- Planning model: `gpt-5.5`
- Preferred implementation model: `gpt-5.4`
- Optional final review model: `gpt-5.5`
- Current handoff state: `Blocked on user decision between remediation options; see Notes/Issues`
- Created: `2026-07-20`
- Last updated: `2026-07-21`

## Validation Rules
- Task complete only after required validation passes and documentation
  generation is recorded, unless a waiver is recorded.
- Phase complete only after required validation passes, documentation
  generation is recorded, required commits/pushes are complete, and
  required user confirmation is recorded.
- Engine work must be committed inside `engine/` before the root repository
  commits the updated `engine` submodule pointer.
- The exact engine commit hash bound to Last Beacon must be recorded before
  root completion.

## Repository State
- Root commit/push state: `Investigation doc commit 8f224e4 and plan/tracker commit 0804dcf pushed to origin/feature/scene-pop-in-investigation; fix implementation commit pending`
- Engine commit/push state: `Readiness-gating commit 0874b9c4ac462a20adff2fec8ee1b07ab88c78fd pushed to origin/feature/scene-pop-in-investigation`
- Root submodule pointer update: `Pending root commit`
- Root pull request state: `Pending`
- Engine pull request state: `Pending`

## Phase 1: Planning
**Status:** Complete
**Goal:** Capture an approved plan/tracker for fixing the scene pop-in issue documented in `docs/scene-pop-in-investigation.md`.

### Tasks
- [x] Investigate and document root cause.
  - Status: Complete
  - Repository: `both` (read-only research)
  - Notes: `docs/scene-pop-in-investigation.md`, committed as `8f224e4` on `feature/scene-pop-in-investigation`.
- [x] Answer open questions from investigation via user observation.
  - Status: Complete
  - Repository: `root`
  - Notes: User confirmed pop-in is both structural and font-related, happens every open (not just first), and takes under ~100ms per scene. Recorded in `docs/scene-pop-in-investigation.md`.
- [x] Create fix plan and tracker.
  - Status: Complete
  - Repository: `root`
  - Notes: Created `docs/plans/scene-pop-in-fix/plan.md` and this tracker. Continuing on `feature/scene-pop-in-investigation` rather than a new branch (see plan's Branch Note).
- [x] User review and approval to begin implementation.
  - Status: Complete
  - Repository: `root`
  - Notes: User approved on 2026-07-20 and asked that the engine branch be created before implementation starts.

### Validation
- Game validation: `N/A for planning-only docs`
- Engine validation: `N/A for planning-only docs`
- Documentation generation: `N/A for planning-only docs`
- User confirmation: `Pending`

## Phase 2: Engine Readiness Gating
**Status:** Complete
**Goal:** Foundation scene-owned roots only become visible once their BSN content has actually applied, not merely because the scene stack marked them visible.

### Tasks
- [x] Create engine submodule branch `feature/scene-pop-in-investigation` from engine `dev`.
  - Status: Complete
  - Repository: `engine`
  - Notes: Created from `origin/dev` at `1bc59f9a0039dfe412b735c869a90f38a0d58582`.
- [x] Spawn BSN roots with an explicit not-ready `Visibility` state in `bsn_assets.rs`.
  - Status: Complete
  - Repository: `engine`
  - Notes: `spawn_bsn_instance_with_asset_server` now inserts `Visibility::Hidden`, plus `SceneContentLoading` for scene-owned instances.
- [x] Gate `sync_scene_entity_visibility` (or an added system) in `scene_stack.rs` on readiness AND stack visibility.
  - Status: Complete
  - Repository: `engine`
  - Notes: Added public `SceneContentLoading` marker; `sync_scene_entity_visibility` now requires `stack.is_visible(...) && scene_ready` where `scene_ready` means no owned entity carries the marker.
- [x] Reconcile `splash_screen.rs`'s authored-marker wait with the new generic gate.
  - Status: Complete
  - Repository: `engine`
  - Notes: Investigated — no code change needed. Splash's authored root is itself a scene-owned BSN root, so it is already covered transparently by the generic gate; `initialize_splash_screens`'s existing wait only decides *which* UI (authored vs. generated fallback) to drive, which is an orthogonal concern. Confirmed via the full existing splash test suite (12 tests) passing unmodified plus a manual smoke test.
- [x] Add/extend engine tests for readiness-gated visibility, including interaction with `covers_previous`.
  - Status: Complete
  - Repository: `engine`
  - Notes: Added `loading_scene_entities_stay_hidden_even_when_stack_visible`, `scene_becomes_visible_once_loading_marker_clears`, `loading_marker_on_child_entity_keeps_whole_scene_hidden`, `covered_ready_scene_stays_hidden_from_presentation_not_readiness` (scene_stack.rs); extended `pending_instance_applies_scene_content_and_propagates_owner` and `failed_resolution_marks_instance_failed_and_stops_pending_retry`, added `standalone_bsn_instance_becomes_visible_once_applied` (bsn_assets.rs).

### Validation
- Engine validation: `Passed: cargo test -p foundation-runtime-library --all-features (97 passed), cargo clippy -p foundation-runtime-library --all-targets --all-features -D warnings, cargo fmt --check, cargo doc --all-features --no-deps, ./engine/scripts/validate-project.cmd (exit 0)`
- Documentation generation: `Passed; engine/docs/scene-system.md updated with a new "Readiness Gating (Scene Visibility)" section`
- User confirmation: `Not required until phase handoff unless implementation discovers scope changes`

## Phase 3: Game-Side Readiness Participation And Font Preload
**Status:** Complete
**Goal:** Last Beacon's nested widget loading and font reassignment participate in the same readiness signal instead of popping independently.

### Tasks
- [x] Expose `LastBeaconBsnWidgetPending`/`LastBeaconBsnWidgetFailed` state so parent-scene readiness can account for it.
  - Status: Complete
  - Repository: `root`
  - Notes: `queue_last_beacon_bsn_widgets` now also inserts `SceneContentLoading`; `apply_pending_last_beacon_bsn_widgets` (success path) and `mark_widget_failed` (failure path) both remove it, so a failed widget can't hide its scene forever.
- [x] Wire Last Beacon widget readiness into the engine gate from Phase 2.
  - Status: Complete
  - Repository: `both`
  - Notes: Made `propagate_loaded_bsn_scene_owners` `pub` and exported it via the engine prelude. `queue_last_beacon_bsn_widgets` is ordered `.after(propagate_loaded_bsn_scene_owners)` in `game/src/lib.rs` so a newly-discovered widget slot already carries the correct `SceneOwner` before it gains `SceneContentLoading` — otherwise the parent scene could flash visible for one frame before re-hiding.
- [x] Add a `Startup` system preloading `fonts/NotoSans-Regular.ttf` and `fonts/NotoSansSymbols2-Regular.ttf` before `scenes::open_initial_scene`.
  - Status: Complete
  - Repository: `root`
  - Notes: Added `ui_widgets::preload_last_beacon_ui_fonts`, chained first in the `Startup` system group in `game/src/lib.rs`.
- [x] Add/extend game tests for widget-pending-aware readiness and font preload.
  - Status: Complete
  - Repository: `root`
  - Notes: Added `queueing_a_widget_slot_marks_the_scene_as_still_loading`, `applying_a_widget_clears_the_scene_loading_marker`, `a_failed_widget_load_still_clears_the_scene_loading_marker`, `preloading_fonts_starts_loading_both_last_beacon_fonts` in `ui_widgets.rs`.

### Validation
- Game validation: `Passed: cargo test --manifest-path game/Cargo.toml --all-features (13 lib + 2 integration tests), cargo clippy --all-targets --all-features -D warnings, cargo fmt --check, cargo doc --all-features --no-deps`
- Documentation generation: `Passed; no docs/ui-widgets.md change needed — LastBeaconBsnWidget's authored API surface (asset_path) is unchanged, only internal readiness bookkeeping was added`
- User confirmation: `Not required until phase handoff unless implementation discovers scope changes`

## Phase 4: Documentation, Validation, Commits, And Root Pointer Update
**Status:** Complete, pending user confirmation
**Goal:** Document the new readiness mechanism, validate engine/root behavior, commit/push both repositories, and record exact pointer state.

### Tasks
- [x] Update `engine/docs/scene-system.md` with readiness-gating behavior.
  - Status: Complete
  - Repository: `engine`
- [x] Update `docs/ui-widgets.md` if `LastBeaconBsnWidget` gains new pending/ready-query behavior.
  - Status: Complete (waived — no author-facing change)
  - Repository: `root`
  - Notes: No new authored fields or behavior changes for widget authors; readiness bookkeeping is entirely internal.
- [x] Run full engine validation (`engine/scripts/validate-project.cmd`).
  - Status: Complete
  - Repository: `engine`
  - Notes: Exit code 0; format, lint, 97 tests, compile, and docs all passed.
- [x] Commit and push engine changes; record exact engine commit hash.
  - Status: Complete
  - Repository: `engine`
  - Notes: Commit `0874b9c4ac462a20adff2fec8ee1b07ab88c78fd` pushed to `origin/feature/scene-pop-in-investigation`.
- [x] Update root `engine` submodule pointer to the new engine commit.
  - Status: Complete
  - Repository: `root`
- [x] Run full root validation (`scripts/validate.cmd`).
  - Status: Complete
  - Repository: `root`
  - Notes: Exit code 0; format, lint, 13 lib + 2 integration tests, compile, and docs all passed.
- [x] Manual smoke test: launch the game, confirm main menu and UI Playground (heaviest `LastBeaconBsnWidget` user) render fully and correctly once settled, confirm scene navigation still works.
  - Status: Complete, with a caveat
  - Repository: `root`
  - Notes: Launched via `scripts/run.cmd`; screenshotted the main menu (all buttons/dividers/fonts/save panel present and correctly styled) and, after clicking through, the UI Playground scene (buttons, tabs, panels, stat rows, typography, and all input widgets present and correctly styled) — nothing stuck permanently hidden, navigation works. **Caveat**: a static screenshot cannot capture whether the sub-~100ms pop-in itself is visually gone, only that the settled result is correct and complete. Confirming the pop-in is actually eliminated (not just that nothing broke) needs the user's own play-test.
- [x] Commit and push root changes.
  - Status: Complete
  - Repository: `root`

### Validation
- Engine validation: `Passed (see Phase 2)`
- Game validation: `Passed (see Phase 3)`
- Documentation generation: `Passed`
- User confirmation: `Pending — user should play-test to confirm the pop-in is actually gone; static screenshots can't prove that`

## Implementation / Review Handoff Notes
- Use `gpt-5.4` for implementation, `gpt-5.5` for optional final review.
- Read `.pi/skills/feature-tracker-update/SKILL.md`, `.pi/skills/rust-workspace-dev/SKILL.md`,
  `.pi/skills/rust-coding-standards/SKILL.md`, `.pi/skills/gitflow-workflow/SKILL.md`, and
  `.pi/skills/foundation-architecture/SKILL.md` before implementation edits.
- Do not edit engine files until `engine/` is on `feature/scene-pop-in-investigation` from engine `dev`.
- See `docs/plans/scene-pop-in-fix/plan.md` for full proposed approach, risks, and alternatives considered.

## Postponed Work
- None yet.

## Notes / Issues / Oversights
- `2026-07-21`: **Regression reported by user**: "large lag spike whenever opening a scene," after the readiness-gating fix landed. Investigated with `superpowers:systematic-debugging`.
  - Added temporary `FrameTimeDiagnosticsPlugin` + `EntityCountDiagnosticsPlugin` + a frame-time-threshold logger to `game/src/lib.rs` (never committed; reverted via `git checkout` after investigation).
  - Reproduced via `--scene last-beacon/<key>` startup override (GUI mouse-click automation proved unreliable in this environment — several techniques tried and abandoned: `mouse_event` + `SetForegroundWindow`, `AttachThreadInput` + Alt-key-unlock trick, `PostMessage` WM_LBUTTONDOWN/UP direct injection — none reliably registered clicks against the winit window; `--scene` override sidestepped the need for clicking entirely).
  - Confirmed two distinct spikes per fresh launch: (1) a ~290ms one-time spike on the very first frame (window/GPU/adapter creation — present at every launch regardless of which scene opens first; not related to this fix). (2) A ~130-185ms second spike occurring exactly on the frame `loading_markers` (entities with `SceneContentLoading`) drops to 0 — i.e., the readiness-reveal frame.
  - **Isolated Stage 2's contribution via a differential test**: temporarily disabled the `SceneContentLoading` insertion in `queue_last_beacon_bsn_widgets` (game-only change, no engine rebuild needed) and re-measured both `main_menu` (2 small nested widgets) and `ui_playground` (10+ nested widgets, the heaviest user). Result: the reveal-frame spike and entity count at that frame were **essentially unchanged** with Stage 2 disabled (main_menu: 174ms→164ms; ui_playground: 130ms→185ms, entity count identical in both cases). This rules out Stage 2 (nested-widget gating) as the dominant cause — nested `.bsn` widget files are small and load/apply at nearly the same speed regardless of whether anything waits for them, so they were already clustering together in time before gating was even a factor.
  - **Root cause conclusion**: this is very likely a **pre-existing** Bevy/wgpu cost (first-time UI render-pipeline specialization/compilation and/or text glyph-atlas building for a scene's specific set of materials/text, the first time that scene's content is ever rendered) that was **already being paid on the same frame as `.apply()`** in the original, pre-fix code too — confirmed by frame-timing analysis: Stage 1's `Visibility::Hidden → Inherited` flip happens in the exact same frame as `.apply()` completing, identically before and after this fix, since `SceneContentLoading` is removed within the same system call that runs `.apply()`. The fix did not add new computation; it changed *when the cost becomes visible*: previously this same stutter coincided with (and was masked by) the visible incremental pop-in, reading as "ugly popping." Now, with nothing visibly changing beforehand, the identical stutter reads as an isolated, unexplained freeze — "a large lag spike."
  - Not yet empirically confirmed (would require a full pre-fix baseline rebuild, judged not worth the ~15-20 min cost given the frame-timing reasoning above is already strong): whether this stutter recurs on every re-visit of an already-opened scene in the same session, or only on each scene's first-ever open (Bevy typically caches compiled pipelines and glyph atlases for the process lifetime, so a second open of the same scene should be far cheaper computationally — the pop-in itself recurs every time per earlier findings, but pop-in is an ECS-respawn cost, not a pipeline-compile cost, and the two are separable).
  - Decision needed from user: see options presented after this update.

## Progress Log
- `2026-07-20`: Investigated scene/widget pop-in; wrote and committed `docs/scene-pop-in-investigation.md` (`8f224e4`) on `feature/scene-pop-in-investigation`; pushed to `origin/feature/scene-pop-in-investigation`.
- `2026-07-20`: User answered the investigation's three open questions via direct observation; recorded answers and their implications in `docs/scene-pop-in-investigation.md`.
- `2026-07-20`: Created `docs/plans/scene-pop-in-fix/plan.md` and this tracker, continuing on `feature/scene-pop-in-investigation`. Awaiting user review and approval to begin implementation.
- `2026-07-20`: User approved the plan and asked that the engine branch be created before implementation. Created engine submodule branch `feature/scene-pop-in-investigation` from `origin/dev` at `1bc59f9a0039dfe412b735c869a90f38a0d58582`. Starting Phase 2 implementation with gpt-5.4.
- `2026-07-20`: Implemented Phase 2 (engine readiness gating: `SceneContentLoading` marker, `bsn_assets.rs` spawn/apply/failure wiring, `reveal_ready_standalone_bsn_instances`, four new `scene_stack.rs` tests, three extended/new `bsn_assets.rs` tests, `engine/docs/scene-system.md` update) and Phase 3 (game readiness participation: `queue_last_beacon_bsn_widgets`/`apply_pending_last_beacon_bsn_widgets`/`mark_widget_failed` wiring, `.after(propagate_loaded_bsn_scene_owners)` ordering, `preload_last_beacon_ui_fonts`, four new `ui_widgets.rs` tests). All engine (97) and game (13 lib + 2 integration) tests pass; format, clippy, and doc generation clean on both crates.
- `2026-07-20`: Ran full `engine/scripts/validate-project.cmd` and `scripts/validate.cmd` (both exit 0). Launched the game via `scripts/run.cmd`, screenshotted the main menu and, after navigating to it, the UI Playground scene — both render fully and correctly with no elements stuck hidden. Noted as a tracker caveat that a static screenshot can't itself prove the sub-~100ms pop-in is gone; that needs the user's own play-test.
- `2026-07-20`: Committed and pushed engine readiness-gating changes as `0874b9c4ac462a20adff2fec8ee1b07ab88c78fd` on `origin/feature/scene-pop-in-investigation`. Updating root submodule pointer and committing root implementation changes next.
- `2026-07-21`: User reported a large lag spike when opening scenes, post-fix. Investigated with `superpowers:systematic-debugging`; see Notes/Issues for the full evidence trail and root-cause conclusion. All temporary diagnostic code reverted (never committed). Awaiting user decision on remediation direction.
