# Scene Pop-In Fix Plan

## Metadata
- Feature slug: `scene-pop-in-fix`
- Feature area: `multi-area`
- Primary area: `engine`
- Root branch: `feature/scene-pop-in-investigation`
- Root branch status: `Existing; continuing directly from the investigation commit rather than creating a new branch (see Branch Note below)`
- Engine branch: `feature/scene-pop-in-investigation`
- Engine branch status: `Required before implementation; engine submodule is currently detached at 1bc59f9a0039dfe412b735c869a90f38a0d58582, which is engine origin/dev tip`
- Engine submodule pointer: `1bc59f9a0039dfe412b735c869a90f38a0d58582 (unchanged so far)`
- Status: `Expanded on 2026-07-22 with async scene loading and scene preloading/cache plan; awaiting user approval before implementation`
- Planning model: `gpt-5.5`
- Implementation model: `gpt-5.4`
- Review model: `gpt-5.5`
- Created: `2026-07-20`
- Last updated: `2026-07-22`

## Branch Note
`feature-plan-docs` normally requires a fresh `feature/*` branch per feature.
This plan continues on `feature/scene-pop-in-investigation` instead of
branching again, because the fix is a direct continuation of the
already-approved investigation on that branch, in the same conversation,
with no indication the user wants separate PRs for "investigate" and "fix."
The docs folder is still named after this plan's own slug
(`scene-pop-in-fix`) rather than the branch name, since the branch predates
the fix scope. Root branch was created from root `dev` at
`7cacf7cabfff058305c08d9988dc15bd935f49e4` (verified: `dev` is an ancestor of
this branch, only one extra commit — the investigation doc — sits on top).

## User Request
Fix the widget/font pop-in issue documented in
`docs/scene-pop-in-investigation.md`: scenes visibly build up in stages
(blank → full layout, then text re-fonts) for roughly the first ~100ms
after every scene open, including revisits.

## Feature Summary
Add readiness gating so Foundation scene-stack content becomes visible only
once it is fully built and styled, instead of the current behavior where
visibility is driven purely by scene-stack presence. This closes the gap
that produces the three additive pop-in stages identified in the
investigation: structural BSN apply, nested `LastBeaconBsnWidget` composition,
and font-handle reassignment.

## Feature Area Classification
- Area: `multi-area`
- Primary area: `engine`
- Rationale: The root mechanism — deciding when a scene-owned entity is
  actually ready to be shown — belongs to Foundation's scene stack and BSN
  bridge (`engine/crates/foundation-runtime-library`), which already owns
  `SceneRuntimeFlags`/`sync_scene_entity_visibility` and the BSN
  apply-pending lifecycle (`FoundationBsnApplyPending`). Last Beacon
  (`game/`) owns the two game-specific pieces that must participate in that
  readiness signal: `LastBeaconBsnWidget` nested-widget loading and the
  Last Beacon font-reassignment pass. Neither piece can fully solve the
  problem alone.

## Codebase Research
Carried over from `docs/scene-pop-in-investigation.md` (full detail there);
key facts repeated here for plan continuity:
- `engine/crates/foundation-runtime-library/src/scene_stack.rs`:
  `open_scene` marks a new stack entry `visible` in the same frame it's
  opened (`update_runtime_flags`), before any content exists.
  `sync_scene_entity_visibility` only ever acts on entities that already
  have a `Visibility` component; it does not hide anything proactively.
- `engine/crates/foundation-runtime-library/src/bsn_assets.rs`:
  `spawn_bsn_instance_with_asset_server` spawns the scene root with no
  `Node`/`Visibility`. `apply_pending_bsn_instances` inserts the entire
  authored subtree (via `scene_patch.apply(...)`) in one frame once the
  async `.bsn` load/parse/resolve finishes; Bevy's `Node` auto-populates
  `Visibility::Inherited`, so the whole subtree is visible the instant it's
  created, with no gating.
- `game/src/ui_widgets.rs`: `LastBeaconBsnWidget` is a second, independent,
  game-owned BSN load/apply pipeline for reusable widgets
  (`queue_last_beacon_bsn_widgets` / `apply_pending_last_beacon_bsn_widgets`),
  used directly in `main_menu.bsn` and `options_menu.bsn`, and used
  extensively by reusable widgets in general per `docs/ui-widgets.md`.
  `apply_last_beacon_ui_font` unconditionally reassigns every newly-added
  `TextFont` to `fonts/NotoSans-Regular.ttf` /
  `fonts/NotoSansSymbols2-Regular.ttf` via `Added<TextFont>`, which fires
  fresh every time a scene is (re)opened because `cleanup_removed_scene_entities`
  fully despawns scene-owned entities on close — there is no cross-open
  entity reuse to cache against.
- `engine/crates/foundation-runtime-library/src/splash_screen.rs` already
  has a narrow precedent for "wait before showing": `initialize_splash_screens`
  waits for authored `FoundationSplashUiRoot`/`FoundationSplashText` markers
  to exist before picking which UI to drive, instead of latching onto blank
  generated fallback UI. It does not wait for fonts or nested widgets, and
  it only decides *which* UI to use, not whether that UI's root `Visibility`
  should stay hidden until ready — so splash scenes are still subject to
  the same pop-in as any other scene.
- User-confirmed observations (recorded in
  `docs/scene-pop-in-investigation.md`, "Open questions" section): pop-in
  happens on every scene open including revisits (not just first-ever
  load), takes under ~100ms per scene, and both the structural pop and the
  font reflow are independently noticeable.

## External Research
No external online research was performed for this plan; the investigation
doc's research was entirely internal codebase tracing, and this fix works
within Bevy 0.19 APIs already in use elsewhere in the codebase (`Visibility`,
`Added<T>` queries, existing BSN apply-pending components). No new upstream
Bevy behavior needs to be researched.

## Affected Files And Systems
- `engine/crates/foundation-runtime-library/src/bsn_assets.rs`: spawn
  scene/prefab roots with an explicit not-ready state (e.g. `Visibility::Hidden`
  at spawn) instead of no `Visibility` at all; expose when a tracked
  instance's apply has completed.
- `engine/crates/foundation-runtime-library/src/scene_stack.rs`: extend or
  complement `sync_scene_entity_visibility` so a scene-owned root's
  effective visibility depends on both stack presentation (`is_visible`)
  and content readiness, not stack presentation alone.
- `engine/crates/foundation-runtime-library/src/splash_screen.rs`: reconcile
  the existing authored-marker wait with the new generic readiness gate so
  there is one mechanism, not two overlapping ones, for splash scenes.
- `engine/docs/scene-system.md`: document the new readiness-gating behavior
  and its interaction with `ScenePresentation`/covering.
- `game/src/ui_widgets.rs`: make `LastBeaconBsnWidgetPending` state
  queryable/exposable so a parent scene's readiness can account for
  in-flight nested widget slots; treat `LastBeaconBsnWidgetFailed` as
  settled (not perpetually pending) so a failed widget load can't block a
  scene from ever becoming visible.
- `game/src/ui_widgets.rs` or `game/src/lib.rs`: add a `Startup` system
  that preloads `fonts/NotoSans-Regular.ttf` and
  `fonts/NotoSansSymbols2-Regular.ttf` via `AssetServer::load` before the
  first scene opens.
- `game/src/scenes/mod.rs`: none expected, but verify `open_initial_scene`
  ordering still makes sense once font preload exists in `Startup`.
- `docs/scene-pop-in-investigation.md`: link forward to this plan once
  implementation starts, for traceability.

## Proposed Implementation Approach
1. **Engine: explicit not-ready state for BSN-spawned roots.**
   In `bsn_assets.rs`, insert `Visibility::Hidden` (or an equivalent
   explicit "not ready" marker) on the scene/prefab root at spawn time,
   alongside the existing `FoundationBsnApplyPending` marker, so the root
   has a real `Visibility` component from frame one instead of acquiring
   one implicitly when `.apply()` runs.
2. **Engine: gate scene-owned visibility on readiness, not just stack state.**
   In `scene_stack.rs`, change `sync_scene_entity_visibility` (or add a
   system that runs alongside it) so a scene-owned root only receives
   `Visibility::Inherited` once both (a) the scene stack says it should be
   visible (`stack.is_visible`) and (b) the entity's own content is ready
   (no `FoundationBsnApplyPending`, and — see step 3 — no outstanding
   Last Beacon nested-widget pending state for game-owned scenes). Keep the
   existing `covers_previous`-driven hide/show behavior intact; readiness
   is an additional AND-condition, not a replacement.
3. **Game: fold nested widget loading into the same readiness signal.**
   In `ui_widgets.rs`, expose whether a `LastBeaconBsnWidget` slot under a
   given scene root is still pending (and treat `LastBeaconBsnWidgetFailed`
   as resolved, not pending, so failures don't block visibility forever).
   Wire this into the engine readiness gate from step 2 — likely via a
   small trait/query Foundation can call generically, or via a
   Last-Beacon-owned system that participates in the same
   `Visibility`-flip decision after the generic engine gate would otherwise
   flip it. Keep the generic mechanism in engine and the
   `LastBeaconBsnWidget`-specific check in game, per
   `foundation-architecture`'s ownership rules.
4. **Game: preload shared fonts before first scene open.**
   Add a `Startup` system (ordered before `scenes::open_initial_scene`)
   that calls `asset_server.load(...)` for both Last Beacon fonts, so their
   handles are already loading (and, after the first successful session
   load, already cached) well before any BSN content that references them
   applies. This does not replace `apply_last_beacon_ui_font` — it removes
   most of the "handle not loaded yet" latency the reassignment currently
   introduces.
5. **Reconcile with splash screen's existing wait.**
   Once the generic readiness gate exists, evaluate whether
   `splash_screen.rs`'s authored-marker wait can defer entirely to it, or
   whether it still needs to also decide *which* UI (authored vs.
   generated fallback) to drive. The marker-selection logic likely still
   needs to stay; the visibility-hiding half of the problem should be
   subsumed by the new generic mechanism so splash scenes aren't special-cased
   twice.
6. **Tests.**
   - Engine: a scene-owned BSN root stays `Visibility::Hidden` while
     `FoundationBsnApplyPending` is present and flips to `Inherited` only
     after apply completes and the scene is stack-visible; a
     stack-covered-but-ready scene stays `Hidden` due to covering, not
     readiness.
   - Game: a scene with a pending `LastBeaconBsnWidget` slot does not
     become visible until that slot resolves (success or documented
     failure); font preload issues `AssetServer::load` for both fonts at
     `Startup`.
   - Manual: launch the game, open main menu, options, a Beacon page,
     ui_playground, and revisit main menu after navigating away — confirm
     no visible structural or font pop for any of them.

## Submodule Plan
- Engine changes required: `yes`
- Engine branch: `feature/scene-pop-in-investigation`
- Engine commit expectation: implement the readiness-gating change to
  `bsn_assets.rs`, `scene_stack.rs`, and `splash_screen.rs`, plus engine
  docs and tests, committed inside `engine/` first.
- Bound engine commit hash: `Pending; will be recorded once engine changes are committed`
- Root pointer update required: `yes; pending root commit after engine commit exists`
- Root game changes required: `yes — LastBeaconBsnWidget readiness participation, font preload, and any wiring needed to call into the new engine gate`

## Alternatives Considered
- **Pure game-side fix (no engine changes):** track BSN-apply/font
  readiness entirely in Last Beacon code and manually control `Visibility`
  outside Foundation's scene stack. Rejected: `sync_scene_entity_visibility`
  is already the single source of truth for scene-owned visibility
  (including `covers_previous` semantics for overlays); duplicating that
  logic in game code risks desyncing from stack-driven hide/show and would
  fight Foundation's ownership of scene-owned entities.
- **Cosmetic fade-in/transition instead of fixing timing:** mask the pop by
  animating opacity/scale on scene open regardless of readiness. Rejected
  as the primary fix — it would hide, not eliminate, the underlying
  reflow (a font swap mid-fade is still visible), and doesn't address the
  root cause. Worth noting as a possible complementary follow-up, but out
  of scope here unless the user asks for it separately.
- **Flatten `LastBeaconBsnWidget` composition into fully inlined `.bsn` scenes**
  to remove the nested-load pipeline entirely. Rejected: this would
  duplicate widget markup across every scene that uses a shared widget,
  undermining the reuse model `docs/ui-widgets.md` is built around, just to
  solve a timing problem that has a smaller, targeted fix available.

## Risks, Constraints, And Assumptions
- Changing BSN-spawned roots to start `Visibility::Hidden` must not break
  existing scene-stack tests/behavior around `covers_previous` (e.g.
  `covered_scene_entities_are_hidden_until_uncovered` in
  `scene_stack.rs`) — readiness and covering must combine correctly
  (visible only when both stack-visible AND ready).
  `sync_scene_entity_visibility` currently reacts to stack state changes
  every `PostUpdate`; the new logic must also react to readiness becoming
  true on a frame where the stack state didn't change, or a ready scene
  could stay hidden indefinitely.
- The `LastBeaconBsnWidgetFailed` fallback must count as "settled" in any
  readiness check, or a failed widget load would permanently hide its
  parent scene instead of just showing broken/missing content for that one
  widget, which would be a worse regression than the current pop-in.
- Font preload adds a small always-on `Startup` cost (two small `.ttf`
  files); expected to be negligible but should be sanity-checked so it
  doesn't itself delay the very first splash frame.
- This plan does not attempt to reduce the underlying `.bsn` parse/apply
  cost (LALRPOP parse time, disk I/O) — it only prevents that cost from
  being visible. If parse/apply ever takes long enough to be noticeable as
  a delay-before-appearing rather than a pop, that would need separate
  investigation (not expected at the ~100ms scale reported).

## Open Questions
- Should the engine readiness gate expose a generic "scene ready" query/API
  that game code can extend (preferred per `foundation-architecture`'s
  reusability guidance), or is a narrower Last-Beacon-specific hook
  sufficient for now? Plan recommendation: start with the narrowest
  mechanism that lets `bsn_assets.rs`/`scene_stack.rs` stay generic and
  `ui_widgets.rs` supply the Last-Beacon-specific piece, and generalize
  further only if a second game needs the same pattern.
- Should splash screens keep their own marker-wait system after the
  generic gate exists, or can it be simplified to rely entirely on the new
  mechanism? Plan recommendation: keep marker selection (authored vs.
  generated fallback) as-is; let the new mechanism own the
  Visibility-hiding half.

## Documentation Expectations
- `engine/docs/scene-system.md` must document the new readiness-gating
  behavior: what "ready" means, how it interacts with
  `ScenePresentation`/covering, and any new public API.
- New/changed public Foundation APIs (any new component, resource, or
  trait exposing readiness) must have Rustdoc explaining intent, since this
  becomes a reusable Foundation concept per the area classification above.
- `docs/ui-widgets.md` should get a short note if `LastBeaconBsnWidget`
  gains new pending/ready-query behavior that widget authors should know
  about.
- Generated documentation (`cargo doc`) must be produced for changed public
  APIs before the feature is considered complete.

## Implementation Handoff Notes
- Use `gpt-5.4` for implementation.
- Before implementation edits, read `.pi/skills/feature-tracker-update/SKILL.md`,
  `.pi/skills/rust-workspace-dev/SKILL.md`, `.pi/skills/rust-coding-standards/SKILL.md`,
  `.pi/skills/gitflow-workflow/SKILL.md`, and `.pi/skills/foundation-architecture/SKILL.md`.
- Create the engine submodule branch `feature/scene-pop-in-investigation`
  from engine `origin/dev` (currently `1bc59f9a0039dfe412b735c869a90f38a0d58582`)
  before touching any `engine/` files; engine is currently detached at that
  same commit.
- Keep the generic readiness mechanism in `engine/crates/foundation-runtime-library`;
  keep `LastBeaconBsnWidget`-specific readiness participation and font
  preload in `game/`.
- Preserve existing `covers_previous`/`blocks_previous_*` presentation
  semantics exactly; readiness is additive, not a replacement.
- Treat `LastBeaconBsnWidgetFailed` (and any equivalent engine-side failed
  state) as "settled," not "pending," in every readiness check.

## Optional Review Focus Areas
- Use `gpt-5.5` for review.
- Confirm covered-but-ready and visible-but-not-ready scenes both behave
  correctly (existing `scene_stack.rs` tests plus new readiness tests).
- Confirm a failed BSN or widget load can never permanently hide a scene.
- Confirm splash screens still work correctly with only one
  visibility-control mechanism instead of two overlapping ones.
- Confirm font preload doesn't measurably delay first splash frame.
- Manually confirm no visible pop-in across main menu, options, at least
  one Beacon page, ui_playground, and a main-menu revisit.

## 2026-07-22 Scope Expansion: Async Scene Loading And Scene Cache

### User Request
After profiling confirmed that transition stalls come from synchronous BSN `ScenePatch::apply` construction, the user requested two core scene-stack capabilities on the same feature branch:

1. A system that asynchronously loads target scenes, then displays them only when every target is ready. This should be the backbone for a future loading-screen solution.
2. A system that preloads and caches scenes so menus and other common UI elements can be added to the stack instantly. Scenes should be able to preload other scenes, and scene loading must be asynchronous; synchronous transition-time loads are not acceptable.

### Architectural Direction
This belongs primarily in `engine/crates/foundation-runtime-library` as a core scene-stack capability. Last Beacon should only provide game-specific preload registrations and manual validation coverage.

Foundation should split scene transitions into explicit stages:

1. **Request.** A scene command declares the desired stack mutation and target scene sources.
2. **Prepare.** Foundation resolves the target sources through an asynchronous preparation/cache resource instead of constructing visible scene content directly in the transition path.
3. **Ready.** Prepared scene content exists off-stack or in a cache in a hidden/non-interactive state, with its BSN asset load/resolve/apply work already settled.
4. **Activate.** The scene stack applies the stack mutation and attaches/retags prepared content with the new `SceneOwner` only after every target scene in the transition group is ready.
5. **Display.** Existing readiness gating then controls final visibility so game-side nested loads still have a chance to settle before the scene appears.

### Proposed Engine APIs And Systems
- Add a reusable scene preparation/cache resource, likely `ScenePreparationCache`, keyed by a stable `SceneCacheKey` derived from `SceneSource` and any future variant information needed to distinguish differently-authored instances.
- Add scene preparation states such as `Requested`, `LoadingAsset`, `Applying`, `Ready`, `Activating`, and `Failed`. For BSN scenes, `LoadingAsset` uses `AssetServer::load`; `Ready` means a hidden cached root has already received its `ScenePatch::apply` content or a failure state has settled.
- Add public scene-stack messages/events for preparation lifecycle, such as `ScenePreloadRequested`, `ScenePreloadReady`, `ScenePreloadFailed`, and/or `SceneTransitionReady`. Final names can be adjusted during implementation, but the behavior should be public and documented.
- Extend `SceneCommand` or add helper commands so callers can request preloading independently from opening, for example `SceneCommand::Preload { source }` and `SceneCommandsExt::preload_scene(...)`.
- Make ordinary `Open` / `ClearAndOpen` commands route through the async preparation path by default. If a requested scene is not ready, Foundation should keep the current stack stable or enter an explicit pending-transition state rather than spawning the target synchronously into the visible stack.
- Add transition-group support for multi-scene opens (for example startup override lists or console `open a b c`): every scene in the group should prepare first; activation happens only when all required targets are ready.
- Add a scene-authored preload mechanism so a scene can prime likely next scenes. Initial implementation can be data-driven through registered scene metadata (for example registry entries that list preload targets) rather than requiring `.bsn` syntax changes.
- Integrate cache invalidation with existing `.bsn` hot reload. If a cached asset reloads, the cached prepared root must be rebuilt or marked stale before activation.

### Important Constraint: Applying Is Still Main-Thread ECS Work
Bevy `World` mutation and `ScenePatch::apply` cannot be moved wholesale to arbitrary background threads. The async/cache design eliminates transition-time synchronous loading by moving preparation earlier and keeping target scenes hidden until ready. If profiling still shows unacceptable frame spikes while preloading happens in the background/loading-screen phase, a follow-up implementation should budget or chunk the apply step across multiple frames rather than calling `ScenePatch::apply` for a large scene in one frame.

The first implementation should therefore avoid any synchronous apply on the click-to-transition path. It is acceptable for a preload request to do main-thread apply work before the player asks to open that scene, because the cached result will make activation instant. For cold, uncached scene opens, the stack should wait in a pending/loading state until preparation completes instead of showing a black half-loaded target.

### Proposed Phases
1. **Engine design pass:** Add scene preparation/cache types, lifecycle messages, and command/API shape in `scene_stack.rs` and BSN bridge ownership in `bsn_assets.rs`.
2. **Async cold-open path:** Route scene opens through preparation and activate only when all requested scenes are ready.
3. **Preload/cache path:** Allow scenes/systems to preload scene sources, keep prepared hidden roots cached, activate cached scenes instantly, and cleanly release/rebuild stale cache entries.
4. **Last Beacon integration:** Register practical menu/UI preload relationships, such as main menu preloading options/credits and heavy UI playground targets where appropriate.
5. **Documentation/validation:** Document the lifecycle in `engine/docs/scene-system.md`, update root docs if Last Beacon gets preload metadata, and validate engine/root behavior.

### Risks And Open Questions
- Cache keys must avoid returning the wrong prepared instance for scenes that need unique runtime parameters in the future.
- Cached scene roots must not receive a final `SceneOwner` too early, or cleanup of an unrelated stack scene could despawn cached content. Activation should assign/propagate the actual stack `SceneOwner` when the prepared root is moved into the active stack.
- Some scenes may not be safe to cache indefinitely if they hold runtime state. Initial policy should favor static UI/menu scenes and provide explicit cache release/invalidation APIs.
- Cold opens without a configured loading screen need a clear behavior. Recommended initial behavior: keep the current stack visible/interactable policy explicit while target scenes prepare, then swap when ready. A future loading-screen scene can subscribe to the same transition lifecycle.
- If `ScenePatch::apply` remains too expensive during preload, budgeted/chunked apply may be required as a second stage. This plan should leave room for that without requiring it in the first cache implementation.

### Additional Success Criteria
- Opening a scene never constructs BSN content synchronously in the same transition path that makes the scene visible.
- Cold scene opens prepare asynchronously and only activate/display after all requested target scenes are ready.
- Preloaded cached scenes can be activated into the scene stack without re-running the expensive BSN asset load/resolve/apply path.
- A scene or scene metadata can declare likely next scenes to preload.
- Existing `ScenePresentation`, `SceneOwner` cleanup, readiness gating, and hot reload semantics remain coherent.
- Failed scene preparation produces an explicit failure state/message and does not leave the stack permanently stuck.

### Additional Testing Methodology
- Engine unit tests for scene preparation state transitions, cold open deferral, all-targets-ready activation, cached instant activation, cache invalidation on hot reload, and failed preparation behavior.
- Engine docs generation after adding public APIs.
- Last Beacon tests or smoke validation proving registered preload relationships request the expected scene sources.
- Manual profiling with `scripts/profile-scene.cmd`: compare a cold open versus a cached menu open and verify cached activation avoids `foundation_bsn_apply` on the transition frame.

## Success Criteria
- Opening any Last Beacon scene shows already-complete, already-styled
  content the moment it becomes visible — no visible structural build-up
  and no visible font/style change shortly after appearing.
- This holds on every open, including revisiting an already-opened scene
  in the same session (per user-confirmed behavior), not just first loads.
- Existing scene-stack presentation behavior (pause overlay, input-blocking
  overlay, covering) is unchanged.
- A failed `.bsn` or widget asset load degrades to visible-but-broken
  content for that piece, not a permanently hidden scene.
- Engine and game validation pass; generated docs are recorded; root
  records the updated engine submodule pointer.

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
- Manual/visual validation: launch the game and open/revisit each affected
  scene (main menu, options, at least one Beacon page, ui_playground,
  pixel-perfect/bevy splash, main-menu revisit) watching specifically for
  the structural pop and font reflow described in
  `docs/scene-pop-in-investigation.md`.
