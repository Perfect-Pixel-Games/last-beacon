# Scene Pop-In Fix Tracker

## Metadata
- Feature slug: `scene-pop-in-fix`
- Feature area: `multi-area`
- Primary area: `engine`
- Root branch: `feature/scene-pop-in-investigation`
- Engine branch: `feature/scene-pop-in-investigation`
- Root branch base verification: `Verified: dev (7cacf7cabfff058305c08d9988dc15bd935f49e4) is an ancestor of this branch; only the investigation doc commit sits on top`
- Engine branch base verification: `Verified: created feature/scene-pop-in-investigation from engine origin/dev at 1bc59f9a0039dfe412b735c869a90f38a0d58582 on 2026-07-20`
- Engine submodule pointer: `Updated to splash completion message engine commit 8a675c6e825eb17eff6c6042f057282b91f95c58; root pointer committed and pushed`
- Overall status: `Phase 6 implemented; refining Last Beacon preload relationships to the intended three scene groups`
- Planning model: `gpt-5.5`
- Preferred implementation model: `gpt-5.4`
- Optional final review model: `gpt-5.5`
- Current handoff state: `Implementation refinement in progress with gpt-5.4 — main-menu root orchestration`
- Created: `2026-07-20`
- Last updated: `2026-07-22`

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
- Root commit/push state: `Main-menu root orchestration commit 1cc46f30e62e7c6ba90108950564e4b188c8b67e pushed to origin/feature/scene-pop-in-investigation`
- Engine commit/push state: `Readiness-gating commit 0874b9c4ac462a20adff2fec8ee1b07ab88c78fd, font-ordering-export commit 609ab9a6aa963abadc0e55cfa5e78a22334bd646, BSN profiling hooks commit 0b419a403373e7bf7dd42ea547660e4ec97b047a, async scene cache pipeline commit df69663a9224cd62fd715a7af3822a1af286e239, and splash completion message commit 8a675c6e825eb17eff6c6042f057282b91f95c58 pushed to origin/feature/scene-pop-in-investigation`
- Root submodule pointer update: `Committed in root main-menu root orchestration commit 1cc46f30e62e7c6ba90108950564e4b188c8b67e for engine commit 8a675c6e825eb17eff6c6042f057282b91f95c58`
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

## Phase 5: Scene Transition Profiling Setup
**Status:** Complete; pushed to origin/feature/scene-pop-in-investigation
**Goal:** Make the BSN scene-transition spike measurable in normal local builds and collect enough evidence to separate scene construction, nested widget construction, and font/rendering symptoms.

### Tasks
- [x] Add low-overhead Foundation BSN profiling hooks.
  - Status: Complete; committed and pushed in engine commit `0b419a403373e7bf7dd42ea547660e4ec97b047a`
  - Repository: `engine`
  - Notes: `apply_pending_bsn_instances` now emits Chrome/Tracy-compatible tracing spans for `foundation_bsn_instance` and `foundation_bsn_apply`, and logs slow resolve/apply steps when `FOUNDATION_BSN_PROFILE_MS` is set.
- [x] Add low-overhead Last Beacon widget profiling hooks.
  - Status: Complete; included in root commit `9ea72dd04742b87839e732abd7111e7fc08819c2`
  - Repository: `root`
  - Notes: `apply_pending_last_beacon_bsn_widgets` now emits tracing spans for `last_beacon_bsn_widget` and `last_beacon_bsn_widget_apply`, and logs slow resolve/apply steps when `LAST_BEACON_WIDGET_PROFILE_MS` is set.
- [x] Add a repeatable profiling launch path.
  - Status: Complete; included in root commit `9ea72dd04742b87839e732abd7111e7fc08819c2`
  - Repository: `root`
  - Notes: Added `scripts/profile-scene.cmd`, which runs the game with `bevy/trace_chrome` and `bevy/debug` under the optimized `foundation-test` profile and defaults both slow-step thresholds to 1ms.
- [x] Document how to capture and read scene-transition traces.
  - Status: Complete; engine docs committed in `0b419a403373e7bf7dd42ea547660e4ec97b047a`, root docs included in `9ea72dd04742b87839e732abd7111e7fc08819c2`
  - Repository: `both`
  - Notes: Added `docs/scene-transition-profiling.md` and extended `engine/docs/scene-system.md` with the Foundation-side profiling knobs.

### Validation
- Engine validation: `Passed: cargo fmt --manifest-path engine/Cargo.toml --all -- --check; cargo check --manifest-path engine/Cargo.toml -p foundation-runtime-library --all-features; cargo clippy --manifest-path engine/Cargo.toml -p foundation-runtime-library --all-targets --all-features -- -D warnings; cargo test --manifest-path engine/Cargo.toml -p foundation-runtime-library --all-features (97 passed); cargo doc --manifest-path engine/Cargo.toml -p foundation-runtime-library --all-features --no-deps`
- Game validation: `Passed: cargo fmt --manifest-path game/Cargo.toml -- --check; cargo check --manifest-path game/Cargo.toml --all-features; cargo check --manifest-path game/Cargo.toml --profile foundation-test --features "bevy/trace_chrome,bevy/debug"; cargo clippy --manifest-path game/Cargo.toml --all-targets --all-features -- -D warnings; cargo test --manifest-path game/Cargo.toml --all-features (13 lib + 2 integration tests passed); cargo doc --manifest-path game/Cargo.toml --all-features --no-deps`
- Documentation generation: `Passed for engine and game docs`
- User confirmation: `Pending — profiling setup is ready for local capture; spike reduction still requires a separate BSN apply optimization/chunking decision`

## Phase 6: Async Scene Loading And Scene Cache
**Status:** Complete; pending commit/push recording
**Goal:** Move Foundation scene opening to an asynchronous preparation/activation lifecycle and add preloaded scene caching so prepared menu/UI scenes can be added to the stack without transition-frame BSN construction.

### Tasks
- [x] Add core scene preparation/cache API in Foundation.
  - Status: Complete
  - Repository: `engine`
  - Notes: Add a resource/state model for requested/loading/applying/ready/failed prepared scenes, keyed by stable scene source/cache keys.
- [x] Route ordinary scene opens through async preparation.
  - Status: Complete
  - Repository: `engine`
  - Notes: Open/clear-and-open should not make target content visible or run transition-time synchronous construction; cold targets should activate only once all requested sources are ready.
- [x] Add explicit scene preload/cache commands and lifecycle messages.
  - Status: Complete
  - Repository: `engine`
  - Notes: Include public APIs for systems/scenes to request preloading and observe ready/failed states. This becomes the loading-screen backbone.
- [x] Support activating cached scenes instantly.
  - Status: Complete
  - Repository: `engine`
  - Notes: Prepared hidden roots should receive the active stack `SceneOwner` only at activation, then reuse existing readiness/visibility gates. Hot reload must invalidate/rebuild stale cached roots.
- [x] Add Last Beacon preload registrations for likely menu/UI transitions.
  - Status: Complete
  - Repository: `root`
  - Notes: Keep this game-specific; likely candidates include main menu preloading options/credits and any heavy UI test/menu scenes useful for validation.
- [x] Update engine/root docs and validation evidence.
  - Status: Complete
  - Repository: `both`
  - Notes: Document cold-open, preload, cache activation, failure, hot-reload, and future loading-screen semantics.

### Validation
- Engine validation: `Passed: cargo check --manifest-path engine/Cargo.toml -p foundation-runtime-library --all-features; cargo clippy --manifest-path engine/Cargo.toml -p foundation-runtime-library --all-targets --all-features -- -D warnings; cargo test --manifest-path engine/Cargo.toml -p foundation-runtime-library --all-features (100 passed); cargo doc --manifest-path engine/Cargo.toml -p foundation-runtime-library --all-features --no-deps; engine/scripts/validate-project.cmd`
- Game validation: `Passed: cargo check --manifest-path game/Cargo.toml --all-features; cargo clippy --manifest-path game/Cargo.toml --all-targets --all-features -- -D warnings; cargo test --manifest-path game/Cargo.toml --all-features (14 lib + 2 integration tests passed); cargo doc --manifest-path game/Cargo.toml --all-features --no-deps; scripts/validate.cmd`
- Documentation generation: `Passed for engine and game docs`
- User confirmation: `Not required for implementation completion; user already approved proceeding`

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
- `2026-07-22`: User asked to continue on the same feature/branch, re-investigate the massive scene-transition lag spikes, and ideally set up profiling. Resuming implementation with `gpt-5.4`; root branch and engine branch both match `feature/scene-pop-in-investigation`, and both still verify `dev` as an ancestor.
- `2026-07-21`: **Regression reported by user**: "large lag spike whenever opening a scene," after the readiness-gating fix landed. Investigated with `superpowers:systematic-debugging`.
  - Added temporary `FrameTimeDiagnosticsPlugin` + `EntityCountDiagnosticsPlugin` + a frame-time-threshold logger to `game/src/lib.rs` (never committed; reverted via `git checkout` after investigation).
  - Reproduced via `--scene last-beacon/<key>` startup override (GUI mouse-click automation proved unreliable in this environment — several techniques tried and abandoned: `mouse_event` + `SetForegroundWindow`, `AttachThreadInput` + Alt-key-unlock trick, `PostMessage` WM_LBUTTONDOWN/UP direct injection — none reliably registered clicks against the winit window; `--scene` override sidestepped the need for clicking entirely).
  - Confirmed two distinct spikes per fresh launch: (1) a ~290ms one-time spike on the very first frame (window/GPU/adapter creation — present at every launch regardless of which scene opens first; not related to this fix). (2) A ~130-185ms second spike occurring exactly on the frame `loading_markers` (entities with `SceneContentLoading`) drops to 0 — i.e., the readiness-reveal frame.
  - **Isolated Stage 2's contribution via a differential test**: temporarily disabled the `SceneContentLoading` insertion in `queue_last_beacon_bsn_widgets` (game-only change, no engine rebuild needed) and re-measured both `main_menu` (2 small nested widgets) and `ui_playground` (10+ nested widgets, the heaviest user). Result: the reveal-frame spike and entity count at that frame were **essentially unchanged** with Stage 2 disabled (main_menu: 174ms→164ms; ui_playground: 130ms→185ms, entity count identical in both cases). This rules out Stage 2 (nested-widget gating) as the dominant cause — nested `.bsn` widget files are small and load/apply at nearly the same speed regardless of whether anything waits for them, so they were already clustering together in time before gating was even a factor.
  - **Root cause conclusion**: this is very likely a **pre-existing** Bevy/wgpu cost (first-time UI render-pipeline specialization/compilation and/or text glyph-atlas building for a scene's specific set of materials/text, the first time that scene's content is ever rendered) that was **already being paid on the same frame as `.apply()`** in the original, pre-fix code too — confirmed by frame-timing analysis: Stage 1's `Visibility::Hidden → Inherited` flip happens in the exact same frame as `.apply()` completing, identically before and after this fix, since `SceneContentLoading` is removed within the same system call that runs `.apply()`. The fix did not add new computation; it changed *when the cost becomes visible*: previously this same stutter coincided with (and was masked by) the visible incremental pop-in, reading as "ugly popping." Now, with nothing visibly changing beforehand, the identical stutter reads as an isolated, unexplained freeze — "a large lag spike."
  - Not yet empirically confirmed (would require a full pre-fix baseline rebuild, judged not worth the ~15-20 min cost given the frame-timing reasoning above is already strong): whether this stutter recurs on every re-visit of an already-opened scene in the same session, or only on each scene's first-ever open (Bevy typically caches compiled pipelines and glyph atlases for the process lifetime, so a second open of the same scene should be far cheaper computationally — the pop-in itself recurs every time per earlier findings, but pop-in is an ECS-respawn cost, not a pipeline-compile cost, and the two are separable).
  - **User decision (2026-07-21): Accept as-is.** The stutter is understood as a pre-existing, one-time-per-scene-type Bevy pipeline/glyph-atlas warm-up cost that the pop-in fix unmasked rather than introduced. No code change made in response to this regression report. Revisit with pipeline pre-warming or a fade-in transition later if it proves bothersome after broader play-testing.

- `2026-07-21` (round 2): **User reopened this**, reporting the spike as "extreme" and that fonts are "still streaming in," asking for a real fix rather than acceptance. Continued investigation with `superpowers:systematic-debugging`, going further than round 1.
  - **Font streaming — root cause found and fixed.** Read Bevy 0.19 source directly (`bevy_ui-0.19.0/src/widget/text.rs`'s `text_system`, which calls `TextPipeline::update_text_layout_info` — the actual glyph rasterization step). It has **no `Visibility` filtering at all** — it runs for every changed text node regardless of hidden state. This means `SceneContentLoading` gating never affected font timing. The real bug: `apply_last_beacon_ui_font` (which reassigns every `Added<TextFont>` to the real NotoSans handle) had no ordering constraint relative to the systems that create `TextFont` components (engine's `apply_pending_bsn_instances`, game's `apply_pending_last_beacon_bsn_widgets`). Depending on Bevy's unconstrained parallel scheduling, it could run *before* those systems on the frame content is created, meaning `Added<TextFont>` wouldn't catch the new text until the following frame — one full frame of the wrong/default font, visible or not, then a visible re-render once corrected.
    - Fix: made `apply_pending_bsn_instances` `pub` (engine, exported via prelude) and added `.after(apply_pending_bsn_instances).after(ui_widgets::apply_pending_last_beacon_bsn_widgets)` to `apply_last_beacon_ui_font` in `game/src/lib.rs`. Now the font is always corrected before Bevy's `PostUpdate` text system ever rasterizes a glyph for that text — one rasterization pass, correct font, no visible swap.
  - **Lag spike — went deeper, found the real root cause, and it's out of scope for this fix.**
    - Verified via direct Bevy source reading (`bevy_ui_render-0.19.0/src/text.rs`'s `extract_text`/`extract_preedit_underlines`) that render extraction genuinely **is** skipped for `Visibility::Hidden` (`if !inherited_visibility.get() { continue; }`), confirming render-pipeline specialization is deferred while hidden — this part of the round-1 theory was correct.
    - Built and tested a pipeline/glyph pre-warm system (`prewarm_last_beacon_ui_pipelines`, Startup, game-only): spawns one off-screen instance of every distinct visual pattern Last Beacon uses (bordered+rounded rect, plain rect, text at all 13 authored font sizes, symbol glyphs) so their render pipelines and glyph atlases compile during the already-expected startup window instead of during scene navigation. Verified Last Beacon uses no gradients/box-shadows/images anywhere (`grep` for `Gradient|BoxShadow|ImageNode` returned nothing), so this covers the full pipeline surface.
    - **Result: no measurable improvement.** `main_menu`'s reveal-frame spike was 163ms with prewarm active vs. 164-174ms without — within noise, not a real reduction.
    - **Decisive test**: temporarily disabled `Visibility::Hidden` entirely in `bsn_assets.rs` (both the pre-insertion at spawn and the `SceneContentLoading` marker), making `spawn_bsn_instance_with_asset_server` behave byte-for-byte like the **original, pre-any-of-this-fix code** — content becomes visible immediately on `.apply()`, no gating, no hiding, exactly as it worked before this whole feature existed. Re-measured `main_menu`: **the same ~152ms spike still occurred on the exact same frame** (`entities` jumping to 485 on `.apply()` completing).
    - **This is conclusive**: the spike is not caused by `Visibility::Hidden`, not by the readiness-gating fix, not by render pipeline compilation being deferred, and not by anything introduced in this feature. It is intrinsic to `scene_patch.apply()` itself — Foundation's temporary, reflection-based BSN construction (`dynamic_bsn.rs`'s `DynamicStruct` rebuilding, `ResolvedSceneRoot::resolve`, `HandleTemplate`→`Handle` resolution, `insert_reflect` for ~50-100 entities each with several components), which runs synchronously on the main thread inside `apply_pending_bsn_instances`, unconditionally, regardless of Visibility. It was **always this slow**, in the original code too, for a scene this large — round 1's "unmasking" theory was correct in spirit but wrong in mechanism: the round 1 fix didn't unmask a *rendering* cost, it unmasked this *reflection/ECS-construction* cost.
    - Both diagnostic changes (pipeline prewarm, hiding-disabled test) reverted; only the font-ordering fix was kept.
  - **What this means going forward**: reducing `.apply()`'s own cost is a meaningfully different, larger piece of work than scene-pop-in readiness gating — it means either (a) profiling/optimizing the reflection-based construction path in `dynamic_bsn.rs`/Foundation's temporary BSN asset bridge itself (the module's own doc comment already calls this bridge temporary scaffolding meant to be replaced once Bevy ships official `.bsn` asset support, so heavy investment here has some waste risk), (b) restructuring `.apply()`'s call site to spread construction across multiple frames (budgeted/chunked apply) instead of one synchronous call, or (c) reducing how much a single scene's `.bsn` file constructs at once (an authoring-side change, e.g. splitting the heaviest scenes). None of these are small, and none are things the scene-stack readiness-gating fix on this branch can address — this needs its own scoped plan if pursued.

## Progress Log
- `2026-07-20`: Investigated scene/widget pop-in; wrote and committed `docs/scene-pop-in-investigation.md` (`8f224e4`) on `feature/scene-pop-in-investigation`; pushed to `origin/feature/scene-pop-in-investigation`.
- `2026-07-20`: User answered the investigation's three open questions via direct observation; recorded answers and their implications in `docs/scene-pop-in-investigation.md`.
- `2026-07-20`: Created `docs/plans/scene-pop-in-fix/plan.md` and this tracker, continuing on `feature/scene-pop-in-investigation`. Awaiting user review and approval to begin implementation.
- `2026-07-20`: User approved the plan and asked that the engine branch be created before implementation. Created engine submodule branch `feature/scene-pop-in-investigation` from `origin/dev` at `1bc59f9a0039dfe412b735c869a90f38a0d58582`. Starting Phase 2 implementation with gpt-5.4.
- `2026-07-20`: Implemented Phase 2 (engine readiness gating: `SceneContentLoading` marker, `bsn_assets.rs` spawn/apply/failure wiring, `reveal_ready_standalone_bsn_instances`, four new `scene_stack.rs` tests, three extended/new `bsn_assets.rs` tests, `engine/docs/scene-system.md` update) and Phase 3 (game readiness participation: `queue_last_beacon_bsn_widgets`/`apply_pending_last_beacon_bsn_widgets`/`mark_widget_failed` wiring, `.after(propagate_loaded_bsn_scene_owners)` ordering, `preload_last_beacon_ui_fonts`, four new `ui_widgets.rs` tests). All engine (97) and game (13 lib + 2 integration) tests pass; format, clippy, and doc generation clean on both crates.
- `2026-07-20`: Ran full `engine/scripts/validate-project.cmd` and `scripts/validate.cmd` (both exit 0). Launched the game via `scripts/run.cmd`, screenshotted the main menu and, after navigating to it, the UI Playground scene — both render fully and correctly with no elements stuck hidden. Noted as a tracker caveat that a static screenshot can't itself prove the sub-~100ms pop-in is gone; that needs the user's own play-test.
- `2026-07-20`: Committed and pushed engine readiness-gating changes as `0874b9c4ac462a20adff2fec8ee1b07ab88c78fd` on `origin/feature/scene-pop-in-investigation`. Updating root submodule pointer and committing root implementation changes next.
- `2026-07-21`: User reported a large lag spike when opening scenes, post-fix. Investigated with `superpowers:systematic-debugging`; see Notes/Issues for the full evidence trail and root-cause conclusion. All temporary diagnostic code reverted (never committed).
- `2026-07-21`: User decided to accept the stutter as-is, understanding it as a pre-existing Bevy pipeline/glyph-atlas warm-up cost unmasked (not introduced) by the pop-in fix. Feature considered complete. Root and engine branches remain pushed to `origin/feature/scene-pop-in-investigation`; no PRs opened yet.
- `2026-07-21` (round 2): User reopened the lag spike as "extreme" and reported fonts still streaming in, asking for a real fix. Investigated further with `superpowers:systematic-debugging`; see Notes/Issues for the full evidence trail. Found and fixed the font-streaming root cause (missing system ordering between `apply_last_beacon_ui_font` and the systems that create `TextFont` components — engine commit `609ab9a6aa963abadc0e55cfa5e78a22334bd646`). For the lag spike, built and empirically disproved a pipeline pre-warm mitigation, then conclusively isolated the true cause via a decisive differential test (disabling all Visibility gating, reproducing byte-for-byte pre-fix behavior): the spike is intrinsic to `scene_patch.apply()`'s reflection-based construction cost, unrelated to Visibility/rendering/this feature entirely, and was always present. This is out of scope for the scene-pop-in-fix branch; reporting findings to user for direction on next steps.
- `2026-07-22`: User asked to continue on the same feature/branch and set up profiling for the massive scene-transition lag spikes. Added permanent profiling hooks around Foundation BSN resolve/apply and Last Beacon nested-widget resolve/apply, a `scripts/profile-scene.cmd` Chrome-trace launch path, and profiling docs. Focused engine/game format, check, clippy, test, and doc generation passed. Engine profiling hooks committed and pushed as `0b419a403373e7bf7dd42ea547660e4ec97b047a`; root profiling implementation commit `9ea72dd04742b87839e732abd7111e7fc08819c2` and tracker follow-up pushed to `origin/feature/scene-pop-in-investigation`.
- `2026-07-22`: User approved the diagnosis and requested core scene-stack async loading plus preloaded/cached scene activation on the same branch. Updated `plan.md` with the async scene loading/cache architecture and added Phase 6 to this tracker.
- `2026-07-22`: Implemented Phase 6. Foundation scene-stack mutations now queue transition batches, BSN scenes prepare through `ScenePreloadRequested` / `ScenePreloadReady` / `ScenePreloadFailed`, transition status is exposed through `SceneTransitionStatus`, prepared BSN roots activate from cache and immediately begin a background refill, and Last Beacon registers preload relationships for splash/menu/gameplay/beacon flows. Engine/root focused validation plus `engine/scripts/validate-project.cmd` and `scripts/validate.cmd` passed. Engine changes committed and pushed as `df69663a9224cd62fd715a7af3822a1af286e239`; root integration and submodule pointer committed and pushed as `af91da1aaafba944bbc044617a526d4db13313e3`.
- `2026-07-22`: User requested a narrower preload policy with three clear areas to avoid over-caching: Main Menu area should only warm splash_pixel_perfect, splash_bevy, main_menu, and options_menu; Beacon should only warm Beacon pages plus options_menu; Gameplay should only warm pause_menu plus options_menu. Refining `game/src/scenes/mod.rs` and its tests to match that exact grouping and remove unintended cross-area warming.
- `2026-07-22`: User clarified that the Main Menu area should have a dedicated main-menu root scene that owns the preload group and splash ordering. Splash scenes should still define their timing/visuals but should not know which scene comes next. Implemented this as a Last Beacon runtime root scene plus a Foundation `FoundationSplashCompleted` message. Moved splash timing data into `pixel_perfect_splash.bsn` and `bevy_splash.bsn`; removed game-side splash driver next-scene configuration. Engine commit `8a675c6e825eb17eff6c6042f057282b91f95c58` and root commit `1cc46f30e62e7c6ba90108950564e4b188c8b67e` were pushed after `engine/scripts/validate-project.cmd` and `scripts/validate.cmd` passed.
