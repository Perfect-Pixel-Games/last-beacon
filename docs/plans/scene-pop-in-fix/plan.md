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
- Status: `Planned`
- Planning model: `gpt-5.5`
- Implementation model: `gpt-5.4`
- Review model: `gpt-5.5`
- Created: `2026-07-20`
- Last updated: `2026-07-20`

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
