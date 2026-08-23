# Async Scene Loading Plan

## Metadata
- Feature slug: `async-scene-loading`
- Feature area: `multi-area`
- Primary area: `engine`
- Root branch: `feature/async-scene-loading`
- Root branch status: `Created from root dev at 7cacf7cabfff058305c08d9988dc15bd935f49e4`
- Engine branch: `feature/async-scene-loading`
- Engine branch status: `Created from engine dev at 1bc59f9a0039dfe412b735c869a90f38a0d58582`
- Engine submodule pointer: `1bc59f9a0039dfe412b735c869a90f38a0d58582 (unchanged so far)`
- Status: `Planned; awaiting user approval before implementation`
- Planning model: `gpt-5.5`
- Implementation model: `gpt-5.4`
- Review model: `gpt-5.5`
- Created: `2026-08-23`
- Last updated: `2026-08-23`

## User Request
Starting from the (working) `dev` baseline — not the abandoned `feature/scene-pop-in-investigation` branch, which was reset back to `dev` after it accumulated a self-inflicted livelock and a black-screen startup hang — refactor scene loading so that:
1. Loading is asynchronous for every scene; the game must never freeze, even for a "blocking" load.
2. Each scene can opt into a **blocking load** (the game does not transition to that scene until it has fully finished loading) or a **streaming load** (the scene becomes visible while still loading, content pops in as it settles).
3. A scene can declare other scenes as **dependencies**, each independently marked either background-load (start loading, but does not gate the owning scene's readiness) or block-load (must finish before the owning scene is considered ready).
4. The user will specify which concrete scenes need which preload relationships once the mechanism exists (example given: `gameplay_level` should keep `pause_menu` warm; `pause_menu` should keep `options_menu` warm).

## Feature Summary
Add a small, explicit scene-readiness model to Foundation's scene stack and BSN bridge, replacing today's "push to stack and reveal immediately, load pops in whenever it's ready" behavior (current `dev` behavior) with two explicit, opt-in modes per scene open, plus a declarative per-scene preload/dependency registry. The design deliberately reuses the simplest primitives proven during the abandoned branch's early phases (a single `SceneContentLoading` marker, scanned recursively) and explicitly avoids the primitives that caused that branch's failure (a prepared-scene cache with automatic refill, and a generic cross-system "readiness token" API) — see `Codebase Research` below for the exact mechanism that caused the failure and why this design avoids it structurally, not just by being more careful.

## Feature Area Classification
- Area: `multi-area`
- Primary area: `engine`
- Rationale: Scene readiness, the load-mode enum, the pending-transition queue, and the preload registry are generic Foundation scene-stack/BSN-bridge concerns (`engine/crates/foundation-runtime-library`). Last Beacon (`game/`) only supplies concrete per-scene preload declarations and picks a load mode per `SceneCommand::open_with_options` call site — no game-specific readiness logic.

## Codebase Research

### Baseline (`dev`, confirmed working by direct launch)
- `scene_stack.rs` (1086 lines): `open_scene` pushes a new `SceneStackEntry` and marks it visible in the very next `update_runtime_flags` pass, in the same frame it's requested — there is no readiness concept at all. `sync_scene_entity_visibility` only ever consults `stack.is_visible(id)`.
- `bsn_assets.rs` (636 lines): `apply_pending_bsn_instances` polls `AssetServer`-loaded `ScenePatch` assets each frame and applies them in one synchronous `scene_patch.apply(&mut EntityWorldMut)` call once ready — this is genuinely async up to the apply step (no busy-waiting), but the apply step itself is one unbudgeted synchronous ECS mutation. Profiling on the abandoned branch measured this at ~1–15 ms for most Last Beacon scenes and ~60–150 ms for `options_menu.bsn`, confirmed intrinsic to `ScenePatch::apply`'s reflection-based construction, not something introduced by any later branch work.
- Game-side (`game/src/scenes/mod.rs`, `game/src/ui_widgets.rs`): splash sequencing (`spawn_splash_driver`) and nested `LastBeaconBsnWidget` composition both already exist on `dev` and are unchanged from the pre-investigation baseline. Neither participates in any readiness signal — a scene's authored widgets can visibly pop in after the scene itself is already shown. This is the exact "pop-in" symptom `docs/scene-pop-in-investigation.md` originally documented, and it is present on `dev` today.

### A latent bug found on `dev` (not introduced by this plan, must be fixed as a prerequisite)
`bsn_assets.rs::replace_reloaded_bsn_instances` treats `AssetEvent::LoadedWithDependencies` and `AssetEvent::Modified` identically as "the source `.bsn` file changed on disk, despawn and respawn this instance." `LoadedWithDependencies` fires on every normal first-time load completion, not just file edits, and `apply_pending_bsn_instances`'s own resolve step calls `Assets::get_mut` on the same asset (to cache the resolved form) — an unavoidable Bevy side effect that also fires `Modified`, indistinguishable at the event level from a real edit. This is present on `dev` right now: every BSN scene that has ever finished loading self-triggers a spurious despawn-and-respawn shortly after it applies.

On `dev` this is only a wasted-work/occasional-stutter bug, because nothing currently gates the whole app on a scene reaching a stable ready state. On the abandoned `feature/scene-pop-in-investigation` branch, a later phase added exactly that gate (wait for every required scene to reach `Ready` before activating a transition), and this pre-existing bug turned into a livelock: resolving a replacement instance re-triggers the same self-inflicted event, so a scene's readiness status kept resetting and the four scenes required for startup were never simultaneously ready — a black screen forever. Root-caused via direct reproduction, `FOUNDATION_BSN_PROFILE_MS=0` instrumentation showing the same four scene sources being resolved/applied repeatedly under new entity IDs, and temporary event-kind/despawn logging that confirmed the exact `AssetEvent` sequence (see the branch history for the full evidence trail, preserved in `engine` stash `wip: bsn self-modified suppression investigation` for reference).

This plan fixes the false-positive detection itself (Phase 1) before any readiness gating is reintroduced (Phase 2+), so the new gating is built on a correct foundation instead of repeating the same failure mode.

### External Research
No external online research was performed. The relevant constraint (`ScenePatch::apply` must run synchronously on the main thread; Bevy 0.19 has no first-party chunked/incremental apply API) was already established through direct Bevy source inspection during the abandoned branch's investigation and is treated as given here.

## Affected Files And Systems
- `engine/crates/foundation-runtime-library/src/bsn_assets.rs`: fix the false-positive hot-reload detection; reintroduce `Visibility::Hidden` + `SceneContentLoading` at BSN instance spawn; add off-stack instance spawning for blocking opens and preload targets; add readiness-check helper.
- `engine/crates/foundation-runtime-library/src/scene_stack.rs`: add `SceneContentLoading` marker (does not exist on `dev`); add `SceneLoadMode` (`Streaming` default / `Blocking`) on `OpenSceneOptions`; add a small pending-transition holding mechanism for blocking opens; add `ScenePreloadRegistry` / `ScenePreloadTarget` / `ScenePreloadMode` (`Background` / `Blocking`); extend `sync_scene_entity_visibility` to require readiness in addition to stack visibility.
- `engine/docs/scene-system.md`: document load modes, the preload registry, and the readiness model.
- `game/src/scenes/mod.rs`: pick a load mode for the splash → main menu handoff (blocking, closing the original pop-in investigation); leave preload registrations for the user to specify per-scene.
- `game/src/ui_widgets.rs`: mark widget slots `SceneContentLoading` while a nested `LastBeaconBsnWidget` is pending, clearing it on success or failure (mirrors the abandoned branch's Phase 3, which was correct and well-tested — the failure was introduced later, in Phase 7's token system, not here).
- `docs/scene-transition-profiling.md` (if reintroduced): optional, only if profiling hooks are restored; not required for this plan's success criteria.

## Proposed Implementation Approach

### Phase 1 — Fix the pre-existing false-positive hot-reload bug (prerequisite, engine-only)
Replace the `LoadedWithDependencies`/`Modified`-treated-as-reload match in `replace_reloaded_bsn_instances` with:
1. Never treat `LoadedWithDependencies` as a reload trigger (it fires on every first load, not just edits).
2. For `Modified`, suppress events that fall within a short grace window (e.g. 500 ms) after Foundation's own resolve step last wrote to that asset, tracked in a small `FoundationBsnSelfModifiedCredits`-style resource keyed by `AssetId<ScenePatch>` and a timestamp (not an exact event count — direct instrumentation during root-causing showed the exact self-inflicted event count is not reliably enumerable, since some of it is Bevy-internal bookkeeping outside Foundation's own two `get_mut` calls).

This phase ships alone, with a regression test proving an instance resolving itself does not trigger its own despawn/replace across several `app.update()` cycles, and a real second test proving a `Modified` event arriving *after* the grace window (simulating a genuine file edit) still replaces the instance.

### Phase 2 — Reintroduce scene/widget readiness gating (no cache, no tokens)
1. `bsn_assets.rs`: spawn BSN instances with `Visibility::Hidden`; insert `SceneContentLoading` on scene-owned roots at spawn, removed once `scene_patch.apply()` succeeds (or fails — a failed load must never hide a scene forever, matching the abandoned branch's existing, correct rule).
2. `scene_stack.rs`: add `SceneContentLoading` marker type (currently owned implicitly by whichever module needs it first; likely `scene_stack.rs` since it's the general-purpose readiness signal, re-exported for `bsn_assets.rs` and `game/`). Extend `sync_scene_entity_visibility` so a scene-owned root is visible only when `stack.is_visible(id) && no owned entity carries SceneContentLoading`.
3. `game/src/ui_widgets.rs`: `queue_last_beacon_bsn_widgets`-equivalent inserts `SceneContentLoading` on a widget slot while its nested `.bsn` is pending; the apply/failure paths remove it. No token registration API — a scene's readiness is simply "no `SceneContentLoading` anywhere in its owned subtree," computed by a plain recursive query scan, exactly as it worked in the abandoned branch before Phase 7 replaced it with a token registry.

### Phase 3 — `SceneLoadMode` and blocking transitions
1. Add `SceneLoadMode { Streaming, Blocking }` (default `Streaming`) to `OpenSceneOptions`.
2. `Streaming` (current `dev` behavior, unchanged): scene is pushed to the stack immediately; content becomes visible once Phase 2's readiness gate clears, same as any other scene.
3. `Blocking`: when a `SceneCommand::Open`/`ClearAndOpen` with `Blocking` mode is processed, Foundation spawns the target's content off-stack (no `SceneOwner` yet, so it doesn't render and isn't cleaned up as part of the current stack) and holds the stack mutation in a small pending-transition slot instead of applying it immediately. Every frame, Foundation checks whether the off-stack instance (plus any `Blocking`-mode preload dependencies from Phase 4) is ready; once ready, it assigns `SceneOwner` (reusing the existing `propagate_loaded_bsn_scene_owners` recursive-propagation system, which already re-runs unconditionally each frame and does not need changes) and pushes the stack entry. The currently-active stack keeps rendering, updating, and accepting input for every frame of the wait — this is the concrete meaning of "never freeze": the frame loop is never blocked waiting for a load, only the stack *mutation* is deferred.
4. A failed target's load must resolve the pending transition to a failure state (emit a message, drop the pending slot) rather than waiting forever — mirrors Phase 2's "a failure is a settled state" rule.

### Phase 4 — Per-scene preload declarations
1. Add `ScenePreloadRegistry` (`HashMap<SceneSource, Vec<ScenePreloadTarget>>`) and `ScenePreloadTarget { source: SceneSource, mode: ScenePreloadMode }` where `ScenePreloadMode` is `Background` (start loading, does not gate anything) or `Blocking` (must be ready before the *owning* scene's own readiness is reported — used when a scene command opens that owner with `SceneLoadMode::Blocking`).
2. When a scene becomes added or focused (`SceneAdded`/`SceneFocused`, matching the trigger the abandoned branch already used successfully), Foundation starts an off-stack load for each of its registered preload targets that isn't already loading/loaded. No automatic refill after a preloaded scene is consumed (opened) — if it's later closed and needs to be fast again, the owning scene re-triggers it naturally next time it's focused. This is the deliberate simplification that avoids the abandoned branch's refill-loop bug class entirely.
3. Last Beacon registers concrete preload relationships once the user specifies them (example already given: `gameplay_level` → `pause_menu` background preload; `pause_menu` → `options_menu` background preload). This plan does not hard-code those registrations; a follow-up task in the tracker records them once specified.

### Phase 5 — Close the loop on the original pop-in investigation
Use `SceneLoadMode::Blocking` for the existing `splash_bevy → main_menu` handoff in `game/src/scenes/mod.rs` (`spawn_splash_driver`'s `next_scene_key` transition), so `main_menu.bsn` and its nested widgets are guaranteed fully ready before the splash-to-menu transition happens. This directly closes `docs/scene-pop-in-investigation.md`'s original complaint using this much smaller mechanism, without reintroducing a dedicated "main menu root" orchestration scene.

### Phase 6 — Documentation and validation
Update `engine/docs/scene-system.md` with the readiness model, load modes, and preload registry. Run full engine and game validation. Manual play-test: launch the game, confirm splash → main menu shows fully-built content with no pop-in, confirm normal navigation (options, a Beacon-equivalent page if present, ui_playground) still works, confirm no repeated resolve/apply log lines for any scene across a full session (regression check for the Phase 1 fix).

## Submodule Plan
- Engine changes required: `yes`
- Engine branch: `feature/async-scene-loading`
- Engine commit expectation: Phases 1–4 land as engine commits (bsn_assets.rs, scene_stack.rs, engine docs).
- Bound engine commit hash: `Pending; recorded once engine changes are committed`
- Root pointer update required: `yes; pending root commit after engine commits exist`
- Root game changes required: `yes — SceneContentLoading widget participation, SceneLoadMode::Blocking on the splash→main-menu transition, and preload registrations once the user specifies them`

## Alternatives Considered
- **Reuse/repair the abandoned branch's prepared-scene cache instead of rebuilding smaller.** Rejected: that architecture's own tracker already documented "fewer game-side special cases after cleanup than before the reset, not more" as a goal it never reached across seven phases and 40+ commits; its core primitives (10-state lifecycle enum, generic readiness-token API, automatic cache refill) are exactly what produced the self-feedback livelock. Starting from the proven-simple `dev` baseline and adding only what the user actually asked for is both less risky and directly addresses why the previous attempt failed.
- **Chunked/incremental `ScenePatch::apply` to eliminate the ~150 ms options_menu hitch entirely.** Deferred, not rejected: this needs its own investigation into whether `ResolvedSceneRoot` exposes any partial-apply hook, and the abandoned branch's tracker already flagged it as "a meaningfully different, larger piece of work." This plan's blocking-load mechanism already moves that cost off the visible transition path in the common case; revisit only if profiling after this plan still shows a user-visible problem.
- **Automatic preload refill after a cached scene is consumed.** Rejected per the design-clarification conversation — deliberately deferred as a possible small follow-up, not included now, specifically because this was the mechanism most directly implicated in the abandoned branch's duplicate-activation and repeated-apply bugs.

## Risks, Constraints, And Assumptions
- `ScenePatch::apply` remains a synchronous, unbudgeted, main-thread operation; a single very heavy scene can still cost one visibly slower frame. This plan does not eliminate that cost, only prevents it from being paid redundantly (Phase 1) and moves it off the interactive path where possible (Phase 3/5).
- `SceneLoadMode::Blocking` must never wait forever: a failed target must resolve to an explicit failure, and (per Phase 3.4) the pending-transition slot must be released even on failure.
- Widget failure handling must continue to treat `LastBeaconBsnWidgetFailed` as settled, not pending, so a broken widget can never hide its parent scene forever — this rule already exists correctly in the abandoned branch's Phase 3 and is being ported forward unchanged.
- The Phase 1 fix's grace-window suppression is a heuristic (a real edit landing inside the ~500 ms window after a fresh load would be missed once); this is an explicit, accepted tradeoff versus an unbounded despawn/respawn loop, not a claim of perfect correctness.

## Open Questions
- Exact preload registrations beyond the two examples already given (`gameplay_level → pause_menu`, `pause_menu → options_menu`) — user will specify; tracked as a Phase 4 follow-up task rather than blocking plan approval.
- Whether any scene besides `main_menu` (via the splash handoff) should default to `Blocking` rather than `Streaming` — plan recommendation is to start every other `SceneCommand::open` call at the current implicit `Streaming` default and let the user opt specific transitions into `Blocking` once the mechanism exists and is easy to reason about in practice.

## Documentation Expectations
- `engine/docs/scene-system.md` must document `SceneLoadMode`, `SceneContentLoading`, and the preload registry as public Foundation concepts.
- All new public engine types/functions (`SceneLoadMode`, `ScenePreloadRegistry`, `ScenePreloadTarget`, `ScenePreloadMode`, `SceneContentLoading`) need Rustdoc explaining intent, mirroring the existing style in `scene_stack.rs`.
- Generated documentation (`cargo doc`) must be produced for changed public APIs before the feature is considered complete.

## Implementation Handoff Notes
- Use `gpt-5.4` for implementation.
- Read `.pi/skills/feature-tracker-update/SKILL.md`, `.pi/skills/rust-workspace-dev/SKILL.md`, `.pi/skills/rust-coding-standards/SKILL.md`, `.pi/skills/gitflow-workflow/SKILL.md`, and `.pi/skills/foundation-architecture/SKILL.md` before implementation edits.
- Both root and engine are already on `feature/async-scene-loading` from their respective `dev` tips (verified in this plan's metadata); no branch creation needed before implementation starts.
- Implement and land Phase 1 fully (with its own tests) before starting Phase 2 — the whole point of this plan is not repeating the abandoned branch's pattern of layering new gating logic on top of an unverified foundation.
- Do not add a generic cross-system "readiness token" API. If a future case genuinely can't be expressed with a single `SceneContentLoading` marker per pending entity, stop and raise it as a plan revision rather than generalizing preemptively.
- Do not add automatic cache refill/invalidation for preloaded scenes in this pass.

## Optional Review Focus Areas
- Use `gpt-5.5` for review.
- Confirm Phase 1's regression test actually fails without the fix (verify red-then-green, not just green).
- Confirm a scene stuck loading forever (simulated failure) cannot permanently block a `Blocking` transition.
- Confirm `Streaming` scenes retain exactly today's `dev` behavior (no regression for call sites that don't opt into `Blocking`).
- Confirm no `.bsn` scene is ever resolved/applied more than once per activation during a full manual play session (the Phase 1 regression class).

## Success Criteria
- The game launches to the main menu with no black screen, no hang, and no repeated resolve/apply of any scene (verified via `FOUNDATION_BSN_PROFILE_MS=0` log inspection across a full session).
- Splash → main menu uses `SceneLoadMode::Blocking`; main menu (including nested widgets/fonts) shows fully built the instant it appears — no pop-in, closing `docs/scene-pop-in-investigation.md`.
- Every other existing scene transition keeps its current `Streaming` behavior unless explicitly changed.
- The frame loop never stalls for a `Blocking` load: input keeps being processed and the previous scene keeps rendering for the entire wait.
- A scene can declare `Background` and `Blocking` preload dependencies on other scenes; Last Beacon's concrete registrations are recorded once the user specifies them.
- Engine and game validation pass; generated docs are recorded; root records the updated engine submodule pointer.

## Testing Methodology
- Engine validation:
  - `cargo test --manifest-path engine/Cargo.toml -p foundation-runtime-library`
  - `cargo clippy --manifest-path engine/Cargo.toml -p foundation-runtime-library --all-targets --all-features -- -D warnings`
  - `cargo doc --manifest-path engine/Cargo.toml -p foundation-runtime-library --all-features --no-deps`
  - `engine/scripts/validate-project.cmd` before final engine commit
- Game validation:
  - `cargo test --manifest-path game/Cargo.toml --all-features`
  - `cargo clippy --manifest-path game/Cargo.toml --all-targets --all-features -- -D warnings`
  - `scripts/validate.cmd` after the engine submodule pointer update
- Manual/visual validation: launch the game repeatedly (cold start, and after navigating away and back) and confirm no black screen, no hang, no visible pop-in on the splash→main-menu transition, and normal navigation continues to work.

## 2026-08-23 Scope Expansion: Prepared-Instance Preload Caching

### User Request
After the original 6 phases shipped and the disclosed font-swap fizzle was fixed (persistent `LastBeaconUiFontHandles`, see tracker), the user reported that preloaded scenes still visibly fizzle and are not instant, and required: opening a preloaded scene must be instant with zero visible construction; every other scene (streaming/non-blocking) must never show partial content either — only reveal once the entire scene and its dependencies are loaded.

### Why This Needs Its Own Plan Section
`ScenePreloadRegistry` (Phase 4) was deliberately scoped to asset-byte warming only (`AssetServer::load`, no spawned content) specifically to avoid reintroducing prepared-instance caching — the exact mechanism responsible for the original `feature/scene-pop-in-investigation` livelock (a 10-state lifecycle enum, a generic cross-system readiness-token API, and automatic cache refill after consumption, compounding into a self-sustaining despawn/respawn loop). That scope cut is exactly what makes "instant" unreachable today: a "preloaded" scene still pays its *entire* construction cost (BSN apply, nested widgets, font settling) fresh at open time, hidden behind `SceneContentLoading` — asset warming only removes disk I/O latency, not that cost. Delivering true instant activation requires actually pre-constructing the scene off-stack ahead of time and reusing that exact entity tree on open. This is the same category of change that caused the original disaster, so it gets a plan and an explicit approval checkpoint rather than being folded into an ordinary follow-up commit.

### Design Principle: Reuse the Existing Machinery, Add the Thinnest Possible Cache Layer
Phase 3's `Blocking` transition mechanism already does almost everything a prepared-instance cache needs: it reserves a `SceneId`, spawns content off-stack tagged `SceneOwner { scene_id }` (via the existing `SceneLoadRequested` → `spawn_requested_bsn_scenes` → `apply_pending_bsn_instances` pipeline, unchanged), waits for `SceneContentLoading` to clear using the exact readiness gate every other scene already uses, then activates by pushing a `SceneStackEntry` with that same reserved id. The only genuinely new piece is: do that off-stack construction *ahead of time* (triggered by a preload registration, not by an open request), and when the real open request eventually arrives, check whether an entry is already sitting there ready — and if so, skip construction entirely and activate immediately.

This reuses 100% of the already-fixed, already-tested resolve/apply/readiness pipeline (including the Phase 1 self-inflicted-`Modified`-event suppression) and adds nothing new to it. The only new code is a small lookup/activation layer.

### Proposed Design
- **`PreparedSceneCache` resource** (`scene_stack.rs`), `HashMap<SceneSource, PreparedSceneCacheEntry { root_entity: Entity, scene_id: SceneId }>`. Small, flat, no lifecycle enum beyond what `SceneContentLoading`'s presence/absence already expresses.
- **`prepare_registered_scene_preloads`** (replaces `warm_registered_scene_preloads`): on `SceneAdded`/`SceneFocused`, for each registered target not already in the cache, reserve a `SceneId`, emit `SceneLoadRequested { scene_id, source }` (spawns off-stack exactly like a `Blocking` transition target does today), and record the cache entry. No behavior change to the spawn/apply/widget/font pipeline itself.
- **Opening a scene checks the cache first**, for both `Streaming` and `Blocking`:
  - Cache hit, ready (no `SceneContentLoading` anywhere under it), entity still exists → activate immediately: push the `SceneStackEntry` using the *caller's* requested key/presentation (prepared content never bakes in stack-level presentation, only ever its own structure), remove the cache entry. This is the true "instant" path.
  - Cache hit, still constructing → `Blocking` waits on the existing off-stack instance (reusing `advance_pending_scene_transitions`, referencing the already-reserved id instead of allocating a new one — no duplicate construction); `Streaming` pushes it immediately, same as today's behavior (readiness gate keeps it hidden until ready — no fizzle, just not instant, matching what the user actually asked for `Streaming` scenes: no fizzle, not necessarily zero latency).
  - Cache miss → falls through to exactly today's behavior (fresh spawn), unchanged.
- **Single-consume, no automatic refill**: a cache entry is removed the moment it's activated. If that scene is wanted warm again later, its owning scene must become focused/added again to re-trigger `prepare_registered_scene_preloads` — deliberately not automatic. This is the same "no refill" policy Phase 4 already committed to, now applied to real prepared instances instead of just asset handles; it's the single biggest lever against the original duplicate-activation bug class.
- **Stale-entity safety net**: dev-time hot reload can despawn/respawn a prepared root out from under the cache (`replace_reloaded_bsn_instances` doesn't know about `PreparedSceneCache`). Rather than wiring cross-module invalidation, activation always checks `world.get_entity(cached_root_entity).is_ok()` first; if the entity is gone, treat it as a cache miss and fall through to a fresh spawn. Simple, defensive, and avoids coupling `bsn_assets.rs` to a `scene_stack.rs`-owned cache.

### Explicitly Ruled Out (why this design avoids the original failure modes)
- No generic cross-system "readiness token" API — readiness is still just "no `SceneContentLoading` in the owned subtree," unchanged from Phase 2.
- No automatic refill-after-consume loop — the exact mechanism most directly implicated in the original repeated-apply/duplicate-activation bugs.
- No new resolve-caching code path — reuses `apply_pending_bsn_instances` and `FoundationBsnSelfResolveSuppression` exactly as they exist today; nothing new can reintroduce the self-inflicted-`Modified`-event livelock.
- No new lifecycle enum — `PreparedSceneCacheEntry` only needs a `root_entity` and `scene_id`; "is it ready" is still answered by the existing readiness query, not by tracked state on the cache entry itself.

### Affected Files And Systems
- `engine/crates/foundation-runtime-library/src/scene_stack.rs`: add `PreparedSceneCache`; extend `apply_scene_command`/`queue_pending_scene_transition`/`advance_pending_scene_transitions` to consult it for both `Streaming` and `Blocking` opens.
- `engine/crates/foundation-runtime-library/src/bsn_assets.rs`: rename/extend `warm_registered_scene_preloads` → `prepare_registered_scene_preloads` to spawn real off-stack content (reusing `spawn_bsn_instance_with_asset_server` via the existing `SceneLoadRequested` path) instead of only calling `AssetServer::load`.
- `engine/docs/scene-system.md`: document the cache, its single-consume policy, and the stale-entity fallback.

### Risks, Constraints, And Assumptions
- Memory: preloaded-but-never-opened scenes stay fully constructed and resident (not just asset bytes) until either consumed or the app closes. Acceptable for Last Beacon's current preload set (five lightweight UI pages under hangar, two menus) — worth revisiting only if a future preload target is heavy.
- `ScenePreloadMode::Blocking` remains unwired (unchanged from Phase 4) — this expansion is about making `Background` preloads genuinely instant, not about the still-unused blocking-dependency gate.
- This does not change `Streaming` scenes into "instant" — it changes them into "never fizzle, hidden until fully ready," which is what was actually requested for that mode. Only scenes that were *actually preloaded* get the zero-latency activation path.

### Testing Methodology
- Engine unit tests: cache hit activates without re-triggering `SceneLoadRequested`/re-applying BSN; cache hit still-constructing correctly waits (`Blocking`) or streams (`Streaming`) without duplicate construction; cache miss falls through to today's unchanged behavior; a despawned/stale cached entity is detected and falls back to a fresh spawn instead of panicking or reactivating a dead entity.
- Manual validation: preload a target (e.g. open `hangar`), wait for it to settle, then open one of its pages — confirm zero visible delay and no BSN-apply log activity on the transition frame (matching the "cached activation avoids reapplying BSN" check from the original Phase 3/Phase 6 plan).
- Repeat the full multi-launch smoke-test regimen used for every prior phase (no black screen, no hang, no repeated resolve/apply of any scene across a full session).
