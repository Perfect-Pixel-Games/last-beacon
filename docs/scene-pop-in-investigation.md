# Scene/Widget Pop-In Investigation

Status: Investigation only. No fixes applied. Written on `feature/scene-pop-in-investigation`.

## Reported symptom

When a scene opens, widgets and elements visibly "pop in" over roughly the
first second rather than appearing complete. Text is affected too: it
appears then visibly changes font/style shortly after. Not a hang or a
freeze — the scene is fully responsive well before the popping settles, it's
a layered visual pop-in during otherwise-normal frame time.

## Method

Traced the full path from `SceneCommand::Open` to a rendered, styled widget,
reading:
- `engine/crates/foundation-runtime-library/src/scene_stack.rs` (stack + visibility sync)
- `engine/crates/foundation-runtime-library/src/bsn_assets.rs` (Foundation's temporary `.bsn` asset bridge)
- `engine/crates/foundation-runtime-library/src/dynamic_bsn.rs` (the `.bsn` text loader/parser)
- `engine/crates/foundation-runtime-library/src/splash_screen.rs` (the one place with an existing timing workaround)
- `game/src/scenes/mod.rs`, `game/src/ui_widgets.rs`, `game/src/lib.rs` (Last Beacon's scene catalog, widget composition, and system ordering)
- `game/assets/scenes/main_menu.bsn` (representative scene content)
- `docs/plans/foundation-bsn-assets/tracker.md` (history — this exact area already had two rounds of black-screen fixes during the original BSN-asset migration)

No code was changed. This is a read-only trace plus reasoning about Bevy's
scheduling and asset-loading model.

## Conclusion up front

This is not one bug, it's three independent, additive loading stages, each of
which currently renders content the instant it becomes available with no
gating or transition. They stack in sequence, and each stage is a visible
"pop":

1. **Scene-shell pop-in** — the whole widget subtree for a scene is invisible/nonexistent, then appears complete in a single frame once its `.bsn` asset finishes loading and parsing.
2. **Nested-widget pop-in** — reusable widgets referenced via `LastBeaconBsnWidget { asset_path }` (buttons, dividers, panels) are a *second*, independent async load per widget, starting only after the parent scene shell already exists. This staggers individual widgets appearing after the shell, not with it.
3. **Font pop-in** — text renders once with Bevy's fallback font (whatever the `.bsn` asset specified, often nothing), then a Last Beacon system unconditionally reassigns every new `TextFont` to the real NotoSans handle a frame later, which itself has to load from disk and build a glyph atlas before text reflows to final size/shape.

None of this is "the scene stack is broken" or "the BSN loader is broken" —
both work as designed. The bug is an *absence*: nothing in the stack,
bridge, or game systems currently waits for "fully loaded" before marking a
scene visible. Visibility is driven purely by scene-stack presence, not by
asset readiness.

## Stage 1: Scene-shell pop-in

`scene_stack.rs`:
- `open_scene` (line ~544) pushes the new `SceneStackEntry` and calls `update_runtime_flags` in the same system call, so `flags.visible` becomes `true` in the very same frame the scene is opened — before any content exists.
- `sync_scene_entity_visibility` (line ~631) only touches entities that already have a `Visibility` component. It doesn't hide anything proactively; it just does nothing until an entity has one.

`bsn_assets.rs`:
- `spawn_bsn_instance_with_asset_server` (line ~191) spawns the scene-root entity with only `Name`, `FoundationBsnInstance`, `FoundationBsnApplyPending`, and (for scene-stack scenes) `SceneOwner` — **no `Node`, no `Visibility`**. The entity exists but renders nothing.
- `apply_pending_bsn_instances` (line ~221) polls every frame until the `ScenePatch` asset is loaded and resolved (`ResolvedSceneRoot::resolve`), then calls `scene_patch.apply(&mut instance_entity_mut)` in one shot. This single call inserts every authored component (`Node`, `Text`, `TextFont`, `BackgroundColor`, all children, ...) onto the entity tree at once. Bevy's UI `Node` auto-populates `Visibility` (default `Inherited`, i.e. visible) as part of its required-components graph, so the instant `.apply()` runs, the whole subtree is visible with no fade or reveal step.

Between spawn and apply there's `asset_server.load(asset_path)` — an async
disk read plus a full LALRPOP-based `.bsn` text parse (`dynamic_bsn.rs`,
`DynamicBsnLoader::load`, line ~170). This is not free: it's proportional to
file size/complexity, and it competes with every other in-flight asset load
(splash text, fonts, other scenes, nested widgets — see Stage 2) on Bevy's
IO/compute task pools. For a several-hundred-line scene like
`main_menu.bsn`, this easily spans multiple frames, especially on a cold
start where many assets are queued simultaneously.

Net effect: for however many frames the load/parse/resolve takes, the scene
area is empty (or shows whatever was there before), then the entire widget
tree appears complete in one frame. That "in one frame" part is why it
reads as a pop rather than a smooth build-up — there's no partial/staged
reveal, it's all-or-nothing per scene.

## Stage 2: Nested-widget pop-in (compounds Stage 1)

`game/src/ui_widgets.rs` defines `LastBeaconBsnWidget { asset_path }` — a
second, completely independent BSN-loading pipeline, parallel to
Foundation's own bridge:
- `queue_last_beacon_bsn_widgets` (line ~490) starts loading a widget's
  `.bsn` file only when a `LastBeaconBsnWidget` component is `Added` — which
  can't happen until the *parent* scene's own `.bsn` has already been
  applied (Stage 1 must complete first for the slot entity to exist at all).
- `apply_pending_last_beacon_bsn_widgets` (line ~1824) applies it the same
  way Stage 1 does: nothing, then everything, in one frame, once its own
  independent load finishes.

This means widget slots pop in strictly *after* the parent scene shell, and
independently of each other and of the parent. Confirmed in
`game/assets/scenes/main_menu.bsn`: `#BrandDivider` and `#FooterDivider`
are both `LastBeaconBsnWidget { asset_path: "ui/widgets/common/divider.bsn" }`
— two extra staggered pops on the main menu alone. `options_menu.bsn` and
`ui_playground.bsn` use this pattern much more heavily (per
`docs/ui-widgets.md`, most reusable widgets are widget-asset-backed), so
those scenes should show visibly more staggered popping than a scene that's
mostly self-contained BSN like `main_menu.bsn`.

This directly explains the "widgets" (plural, apparently independently)
popping in, as distinct from the whole scene appearing at once.

## Stage 3: Font pop-in

`game/src/ui_widgets.rs`, `apply_last_beacon_ui_font` (line ~514):

```rust
pub fn apply_last_beacon_ui_font(
    asset_server: Res<AssetServer>,
    mut text_fonts: Query<(&mut TextFont, Option<&LastBeaconUiSymbolIcon>), Added<TextFont>>,
) {
    let ui_font = asset_server.load("fonts/NotoSans-Regular.ttf");
    let symbol_font = asset_server.load("fonts/NotoSansSymbols2-Regular.ttf");
    for (mut text_font, symbol_icon) in &mut text_fonts {
        let font_handle = if symbol_icon.is_some() { symbol_font.clone() } else { ui_font.clone() };
        text_font.font = FontSource::Handle(font_handle);
    }
}
```

This runs every `Update` (registered in `game/src/lib.rs` line ~172) and
reassigns the font handle on every newly-added `TextFont`, regardless of
whatever font the `.bsn` asset itself specified (in practice the `.bsn`
files only set `font_size`, not `font`, e.g. `main_menu.bsn` line 24:
`TextFont { font_size: FontSize::Px(12.0) }` — so text starts on Bevy's
built-in fallback font). So text visibly renders once already (Stage 1/2's
pop), on the fallback font, then this system swaps in the real `Handle<Font>`
a frame later. That handle itself hasn't loaded yet — `asset_server.load()`
just returns a handle and kicks off the disk read/glyph-atlas build — so
Bevy re-renders the text again once the real font and its atlas are ready.
This is the exact "text changes as well" part of the report: it's not one
font swap, it's fallback-font render → handle-reassignment (still not
loaded) → real-font render once the atlas builds, three distinct visual
states for the same text.

Notable: this system is generic-looking (`Added<TextFont>`) but always
loads the same two Last Beacon fonts no matter what — it isn't reacting to
per-scene font choices, it's unconditionally overwriting them. That's
existing, working-as-designed behavior (per `docs/ui-widgets.md` all text
is meant to use these two fonts), just not one that accounts for load
timing.

## Why this wasn't caught by the earlier BSN black-screen fixes

The tracker for `docs/plans/foundation-bsn-assets/` shows this exact
area (async `.bsn` apply timing) already caused a full black-screen bug
during the original migration, fixed in two passes:
1. Foundation's bridge now resolves/applies `ScenePatch` itself before
   propagating `SceneOwner` (`bsn_assets.rs`).
2. `splash_screen.rs`'s `initialize_splash_screens` now waits for authored
   `FoundationSplashUiRoot`/`FoundationSplashText` markers to exist instead
   of falling back to blank generated UI while the asset was still applying.

Both fixes solved *correctness* (don't show broken/empty content, don't
permanently latch onto a fallback) but neither fix gates *visibility on
readiness* — they solve "don't show garbage," not "don't show the
build-up." The splash fix specifically only waits for two marker
components to exist; it doesn't wait for the splash's own font or any
nested widgets to finish loading, so a `.bsn`-authored splash scene is
still subject to Stages 1–3 like any other scene. Regular (non-splash)
scenes — main menu, Beacon pages, options — have no equivalent gating at
all; they rely entirely on `sync_scene_entity_visibility`, which (as shown
in Stage 1) makes no attempt to wait for content.

## Things ruled out

- **Not a scene-stack ordering bug.** `SceneStack`'s visibility/focus flag
  computation (`update_runtime_flags`) is synchronous, deterministic, and
  well covered by tests (`scene_stack.rs` test module). The stack always
  marks a newly-opened scene visible immediately by design (there's no
  concept of "pending" in the stack itself) — the gap is entirely in what
  happens between "marked visible" and "content exists."
- **Not a hot-reload bug.** `replace_reloaded_bsn_instances` (despawn +
  respawn on asset change) is a dev-time feature and wasn't observed to be
  involved in the reported symptom (it only fires on `AssetEvent::Modified`
  / `LoadedWithDependencies` for already-tracked instances, not on first
  open).
- **Not specific to one scene type.** The mechanism is generic to every
  `SceneSource::BsnScene` open — splash, main menu, Beacon pages, options,
  gameplay level, credits, ui_playground. Severity should scale with (a)
  `.bsn` file size/parse cost, (b) how many `LastBeaconBsnWidget` slots it
  composes, and (c) how many distinct fonts/symbol icons it uses.
- **Not resolved by Bevy's asset cache, contrary to this doc's original
  assumption.** The user confirmed (see "Open questions" below) that
  pop-in happens *every* time a scene opens, including revisiting a scene
  already opened earlier in the same session — not just on first load.
  This makes sense on closer reading: `cleanup_removed_scene_entities`
  (`scene_stack.rs`) fully despawns a scene's entities when it leaves the
  stack, so reopening it always spawns a *fresh* `FoundationBsnInstance`
  entity and goes through `apply_pending_bsn_instances` again — even if
  Bevy's `Assets<ScenePatch>` still holds the parsed asset (skipping the
  disk read + LALRPOP parse), `ResolvedSceneRoot::resolve` and
  `scene_patch.apply(...)` still run fresh each time, and every `Text`
  entity is brand new, so `Added<TextFont>` in `apply_last_beacon_ui_font`
  fires every time regardless of whether the font *asset* is cached. Stage
  3 in particular is structurally incapable of being a first-open-only
  cost: it's keyed on entity creation, not asset-load state.

## Open questions — answered by user observation (2026-07-20)

These were flagged as unanswerable from static reading alone; the user
answered them from watching the game run. Recorded verbatim below, with
what each answer implies for the trace above.

**Q1 — Does it look like a blank gap that fills in, or content that appears then reflows/re-fonts?**
Answer: **Both, roughly equally noticeable.**
Implication: confirms Stage 1 (structural pop) and Stage 3 (font pop) are
both live and both visible in practice, not just Stage 3 alone as a subtler
effect riding on top of an already-obvious Stage 1. Matches the original
report ("widgets... popping in" + "text changes") reading as two distinct
phenomena rather than one.

**Q2 — Does it happen every time a scene opens, or only the first time a given scene is visited in a session?**
Answer: **Every time, even revisiting the same scene.**
Implication: this is the most load-bearing new finding. The "Things ruled
out" section below originally assumed repeat opens of an already-loaded
`.bsn` would be "faster/instant... since Bevy caches loaded assets by
path." That assumption is **wrong in practice** — see the follow-up note
under "Things ruled out."

**Q3 — Roughly how long does the pop-in/settling take?**
Answer: **Under ~100ms (a few frames).**
Implication: revises the scale of the problem. The original report said
"overall less than a second," which was the *complete* multi-scene startup
impression (splash → splash → main menu, each with its own pop), not the
duration of a single scene's pop. A single scene's pop-in is brief (a few
frames) but, per Q1, is a *compound, multi-stage* brief pop (shell + font
both visibly settling in that same short window) rather than one clean
frame-perfect appearance — which is enough to read as distracting/unpolished
even though it's not a multi-hundred-millisecond stall.

## Possible directions for a future fix (not evaluated or planned)

Recorded only as candidate angles raised by this trace, for whoever plans
the actual fix — none of these have been designed, scoped, or validated:

- Gate a scene's `SceneRuntimeFlags::visible` (or a new readiness flag) on
  "root `.bsn` applied + all nested `LastBeaconBsnWidget` slots applied +
  referenced fonts loaded," rather than on stack presence alone, and hide
  scene-owned root entities (or the whole scene, behind a shared loading
  state) until that's true.
- Preload `fonts/NotoSans-Regular.ttf` and `fonts/NotoSansSymbols2-Regular.ttf`
  once at startup (e.g. in `Startup`) so by the time any scene's text
  applies, the font handle is already loaded and only the first scene of
  the whole session would ever show a font pop, if any.
- Extend the splash's "wait for authored markers" pattern into a general,
  reusable Foundation primitive (e.g. a `SceneOwner`-scoped "content ready"
  check) instead of it being splash-specific.

## Files referenced

- `engine/crates/foundation-runtime-library/src/scene_stack.rs`
- `engine/crates/foundation-runtime-library/src/bsn_assets.rs`
- `engine/crates/foundation-runtime-library/src/dynamic_bsn.rs`
- `engine/crates/foundation-runtime-library/src/splash_screen.rs`
- `game/src/scenes/mod.rs`
- `game/src/ui_widgets.rs`
- `game/src/lib.rs`
- `game/assets/scenes/main_menu.bsn`
- `docs/ui-widgets.md`
- `docs/plans/foundation-bsn-assets/plan.md`
- `docs/plans/foundation-bsn-assets/tracker.md`
- `engine/docs/scene-system.md`
