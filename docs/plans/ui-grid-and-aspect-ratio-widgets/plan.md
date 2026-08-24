# UI Grid And Aspect Ratio Widgets Plan

## Metadata
- Feature slug: `ui-grid-and-aspect-ratio-widgets`
- Feature area: `game`
- Primary area: `game`
- Root branch: `feature/ui-grid-and-aspect-ratio-widgets`
- Engine branch: `N/A`
- Engine submodule pointer: `N/A` (no engine changes planned)
- Status: `Planned`
- Planning model: `gpt-5.5` (role fulfilled by Claude Sonnet 5, per the user's standing instruction that Claude/subagents replace GPT in this workflow)
- Implementation model: `gpt-5.4` (role fulfilled by Claude Sonnet 5)
- Review model: `gpt-5.5` (role fulfilled by Claude Sonnet 5)
- Created: `2026-08-24`
- Last updated: `2026-08-24`

## User Request
Following the widget-library hardening work in `feature/ui-widget-feathers-alignment` (PR #16, still open), the user asked for a follow-up list of additional reusable UI widgets worth building for games on this framework. From that list, the user picked three layout primitives to build next, refining scope across the conversation:
1. **Uniform Grid** — a grid container widget.
2. **Span Grid** — a grid where individual child elements can span more than one cell on the X and/or Y axis.
3. **Aspect Ratio container** — a single widget (not two variants) that a developer configures with a minimum and maximum aspect ratio; it constrains its own size to whatever ratio the available space clamps to within that range (so a HUD authored at, say, 16:9 doesn't stretch edge-to-edge on an ultrawide monitor, and doesn't squash below a legible minimum on a narrow one). Setting `min == max` is how a developer gets a single fixed ratio — there is no separate "fixed" widget.
The user explicitly said context menus are out of scope (not needed for a game). This plan does not cover the rest of the earlier follow-up list (modal/toast/progress-bar/tooltip/accordion, drag-and-drop cargo grid, virtualized list, etc.) — those remain a separate future conversation.

Two additional requirements were added before approval:
4. **Showcase scene examples**: all three widgets must be demonstrated in `game/assets/scenes/ui_playground.bsn`, matching how every other reusable widget is already showcased there (per `docs/ui-widgets.md`'s "Most reusable widgets are demonstrated in `game/assets/scenes/ui_playground.bsn`").
5. **Designed with Bevy Feathers in mind**: the user asked that these components be "designed in a similar way" to Feathers. Interpreted here (see Feature Summary for the concrete design decision, flagged for the user to correct if this reads too narrow or too broad) as: each new widget family gets its own dedicated Bevy `Plugin` that owns its type registrations and systems — mirroring how real Feathers composes one plugin per widget module (`ButtonPlugin`, `RadioPlugin`, `SliderPlugin`, all added together by `ControlsPlugin`) rather than appending registrations directly into the monolithic `LastBeaconPlugin` builder chain — plus Feathers-style doc comments (a clear one-paragraph purpose statement, authoring caveats called out explicitly) and harmless/inert `Default` values (a widget left unconfigured should do nothing surprising, the same way an unconfigured Feathers `Props` struct is safe). This does **not** extend to adopting Feathers' `SceneComponent`/`bsn!`-macro construction, `bevy_ui_widgets` headless primitives, or its `ThemeToken` color system for these widgets — that fuller architectural adoption was already explicitly scoped out of the prior PR (#16) and nothing in this request re-opens that decision; these three widgets are also layout-only (no hover/press/focus states), so most of Feathers' state-driven-styling pattern does not apply to them regardless.

## Feature Summary
Adds three reusable layout primitives to `game/src/ui_widgets.rs` and `docs/ui-widgets.md`, all designed to sit underneath the existing widgets (HUD panels, stat grids, inventory-adjacent layouts) rather than replace anything:

- **Uniform Grid** needs no new Rust code at all: `bevy_ui` 0.19 already has full CSS Grid support (`Display::Grid`, `grid_template_columns`/`grid_template_rows` as `RepeatedGridTrack`), so a uniform grid is just a documented `Node` preset (`RepeatedGridTrack::flex(column_count, 1.0)` repeated per column), matching how `divider.bsn`/`typography_panel.bsn` are already documented conventions with no backing component.
- **Span Grid** adds one small component, `LastBeaconUiGridItem { column_span: u16, row_span: u16 }`, plus a reactive `Added<T>` system that translates it into Bevy's native `GridPlacement::span(...)` on `Node.grid_column`/`Node.grid_row` — the same shape as every other `apply_last_beacon_ui_*` reactive system already in this file.
- **Aspect Ratio container** adds `LastBeaconUiAspectRatioBounds { min_aspect_ratio: f32, max_aspect_ratio: f32 }` plus a system that reads the parent's computed content-box size (post-layout, matching the existing text-box scrollbar system's `ComputedNode`-reading pattern), clamps the current ratio into `[min, max]`, computes a "contain fit" size at that ratio, and writes it back as explicit `Val::Px` width/height on the widget's own `Node`. Centering is the parent's responsibility (`align_items: Center, justify_content: Center`), which is documented as a requirement rather than forced, since this widget does not own its parent.
- Both `LastBeaconUiGridItem` and `LastBeaconUiAspectRatioBounds`'s type registrations and systems are grouped into one new `LastBeaconUiLayoutWidgetsPlugin`, added from `LastBeaconPlugin::build` via `.add_plugins(...)` rather than inline `.register_type()`/`.add_systems()` calls — the "designed with Feathers in mind" structural echo described above. Uniform Grid needs no entry here since it has no backing Rust component.
- All three widgets get a demonstration gallery card in `game/assets/scenes/ui_playground.bsn`: one combined "GRID" card showing a uniform grid with one spanning cell (demonstrates both Uniform Grid and Span Grid together, since a span only means something inside a grid), and one "ASPECT RATIO" card showing the bounds container holding its band inside a fixed-size gallery panel.

## Feature Area Classification
- Area: `game`
- Primary area: `game`
- Rationale: All affected files (`game/src/ui_widgets.rs`, `game/src/lib.rs`, `docs/ui-widgets.md`, and one new `.bsn` asset) are Last Beacon-owned. Everything needed already exists in `bevy_ui` 0.19 (already a dependency); no `engine/` changes and no new dependencies are required.

## Codebase Research
- `bevy_ui-0.19.0/src/ui_node.rs:785-818`: `Node` already has `grid_auto_flow`, `grid_template_rows`/`grid_template_columns: Vec<RepeatedGridTrack>`, `grid_auto_rows`/`grid_auto_columns: Vec<GridTrack>`, and `grid_row`/`grid_column: GridPlacement` — full CSS Grid support is native, not something Last Beacon needs to build.
- `bevy_ui-0.19.0/src/ui_node.rs:1830-1918` (`RepeatedGridTrack`): `RepeatedGridTrack::flex(repetition: u16, value: f32)` constructs `repetition` equal-width `minmax(0, Nfr)` tracks in one call — exactly a "uniform N-column grid."
- `bevy_ui-0.19.0/src/ui_node.rs:2040-2110` (`GridPlacement`): `GridPlacement::span(span: u16)` places an item automatically while spanning `span` tracks; **panics if `span == 0`** (`NonZero<u16>` internally) — the new `LastBeaconUiGridItem` component must clamp `column_span`/`row_span` to at least `1` before calling it, since `.bsn`-authored values could be `0` by mistake (Bevy's own default is span `1`, meaning "no override" is naturally expressed as `1`, not `0`).
- `bevy_ui-0.19.0/src/ui_node.rs:26-33,401-405` (`ComputedNode`): `pub size: Vec2` is a public field, and `ComputedNode` derives `Default` — meaning it can be constructed directly in tests (`ComputedNode { size: Vec2::new(1600.0, 900.0), ..default() }`) without needing to run Bevy's real Taffy layout pass, the same way this file's existing scrollbar-layout math is unit-tested.
- `ui_widgets.rs`'s existing `refresh_last_beacon_ui_text_box_scrollbars` (registered `.after(bevy::ui::UiSystems::PostLayout)` in `game/src/lib.rs`) is the established precedent for "read `ComputedNode` after layout, compute a derived value, write it back to `Node`" — the aspect-ratio system follows the identical shape.
- Bevy's own `Node.aspect_ratio: Option<f32>` field (native single-ratio support, resolved by Taffy similarly to CSS `aspect-ratio`) was considered and rejected as the implementation mechanism for this widget: it only expresses one fixed ratio, not a `[min, max]` band, and the user was explicit that this must be **one** widget configured with both a min and a max (equal values degenerating to "fixed"), not two code paths. A single custom system handles both cases uniformly.
- `game/assets/ui/widgets/common/divider.bsn` and `typography_panel.bsn` (confirmed via `docs/ui-widgets.md`) are the existing precedent for a "pure layout/style preset, documented, no backing Rust component" widget — this is the pattern Uniform Grid follows.
- Confirmed via `gh pr view 16` that the prior feature (`feature/ui-widget-feathers-alignment`, style-enforcement fix + keyboard focus support) is still open/unmerged; this feature branches from current `origin/dev` (`018b317`) per gitflow-workflow rather than stacking on that unmerged branch, since these three widgets have no functional dependency on it.
- `game/assets/scenes/ui_playground.bsn`: read in full. It's a flex-wrap row (`#UiPlaygroundBody`) of bordered "gallery" cards (`#ButtonWidgetGallery`, `#NavigationWidgetGallery`, `#PanelWidgetGallery`, `#TypographyWidgetGallery`, `#InputWidgetGallery`), each a fixed-width `Node` with a section-label `Text` followed by one or more `LastBeaconBsnWidget { asset_path: "ui/widgets/common/....bsn" }` references. New gallery cards for these three widgets follow this exact existing pattern. Notably, every existing gallery card already has an explicit fixed pixel width (not sized by its children) — exactly the "parent must not size itself from this widget" precondition the Aspect Ratio container's design assumes, so no special-case container is needed to demo it safely.

## External Research
No web search/fetch was performed or needed. The relevant `bevy_ui` 0.19.0 grid, grid-placement, and computed-node APIs were read directly from the local Cargo registry cache (`C:\Users\jonla\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\bevy_ui-0.19.0\src\ui_node.rs`), the exact version pinned in this workspace's `Cargo.lock`.

## Affected Files And Systems
- `game/src/ui_widgets.rs`: add `LastBeaconUiGridItem` component + `apply_last_beacon_ui_grid_item_span` system; add `LastBeaconUiAspectRatioBounds` component + `apply_last_beacon_ui_aspect_ratio_bounds` system; add `LastBeaconUiLayoutWidgetsPlugin` bundling both.
- `game/src/lib.rs`: `LastBeaconPlugin::build` adds `LastBeaconUiLayoutWidgetsPlugin` via `.add_plugins(...)` (no per-type/per-system inline registration needed, since the new plugin owns that).
- `game/assets/ui/widgets/common/uniform_grid.bsn`: new preset asset (Uniform Grid needs no Rust, just a documented starting `.bsn`).
- `game/assets/scenes/ui_playground.bsn`: two new gallery cards demonstrating Uniform Grid + Span Grid together, and Aspect Ratio Container.
- `docs/ui-widgets.md`: three new sections (Uniform Grid, Span Grid, Aspect Ratio Container).
- No `engine/` changes. No changes to any existing widget `.bsn` asset under `common/`.

## Proposed Implementation Approach

### 1. Uniform Grid (documentation + preset asset only)
1. Add `game/assets/ui/widgets/common/uniform_grid.bsn`: a `Node` with `display: Display::Grid`, `grid_auto_flow: GridAutoFlow::Row`, and `grid_template_columns: RepeatedGridTrack::flex(3, 1.0)` (three equal-width columns as a starting default, matching `stat_rows_panel.bsn`'s "duplicate/remove rows as needed" precedent for "change the column count for your use case"), plus a documented empty `Children [...]` slot for cells.
2. Add a "Uniform Grid" section to `docs/ui-widgets.md` documenting: `grid_template_columns`/`grid_template_rows` are the author-updated properties (change the repetition count and/or swap `flex` for `px`/`percent`/`auto` per column as needed); children are placed left-to-right, wrapping automatically via `grid_auto_flow`.

### 2. Span Grid
1. Add a new component:
   ```rust
   /// Marks a grid cell that should span more than one column and/or row of its
   /// parent grid container.
   #[derive(Clone, Copy, Debug, Component, Reflect)]
   #[reflect(Component, Default)]
   pub struct LastBeaconUiGridItem {
       /// Number of grid columns this item should span. Values below `1` are
       /// treated as `1` (`GridPlacement::span` panics on `0`).
       pub column_span: u16,
       /// Number of grid rows this item should span. Values below `1` are
       /// treated as `1`.
       pub row_span: u16,
   }

   impl Default for LastBeaconUiGridItem {
       fn default() -> Self {
           Self {
               column_span: 1,
               row_span: 1,
           }
       }
   }
   ```
2. Add a reactive system, following this file's established `Added<T>` shape:
   ```rust
   /// Translates an authored span into Bevy's native grid placement.
   ///
   /// `GridPlacement::span` panics on a span of `0`, so a `.bsn`-authored `0`
   /// (or default-initialized value) is clamped up to `1` -- Bevy's own
   /// default placement already behaves as "span 1," so this never changes
   /// behavior for an unset value.
   pub fn apply_last_beacon_ui_grid_item_span(
       mut commands: Commands,
       grid_items: Query<(Entity, &LastBeaconUiGridItem), Added<LastBeaconUiGridItem>>,
   ) {
       for (grid_item_entity, grid_item) in &grid_items {
           commands.entity(grid_item_entity).insert((
               GridPlacement::span(grid_item.column_span.max(1)),
               // `Node.grid_column`/`Node.grid_row` are typed as `GridPlacement`
               // directly, not a bundle -- insert each explicitly.
           ));
       }
   }
   ```
   (Exact insertion targets `Node.grid_column`/`Node.grid_row`; since `GridPlacement` is a field of `Node`, not a standalone component, the implementation step must mutate the existing `Node` component rather than `.insert(GridPlacement)` as a bare component -- see the corrected step in the task breakdown below. This plan-level snippet is illustrative of the span-clamping logic; the task breakdown has the exact `Node` mutation.)
3. Add a "Span Grid" section to `docs/ui-widgets.md` documenting `LastBeaconUiGridItem { column_span, row_span }` as the author-updated property, and that it must be used inside a grid container (Uniform Grid or a custom one) to have any visible effect.
4. Add a combined "GRID" gallery card to `game/assets/scenes/ui_playground.bsn` (matching the existing `#XWidgetGallery` pattern): a `uniform_grid.bsn`-style 3-column grid with three sample cells, the middle cell carrying `LastBeaconUiGridItem { column_span: 2, row_span: 1 }` so the demo shows spanning working inside a uniform grid in one place.

### 3. Aspect Ratio Container
1. Add a new component:
   ```rust
   /// Constrains this widget's own size to a clamped aspect ratio band based on
   /// its parent's available content space, so authored UI (for example a HUD
   /// sized around 16:9) neither stretches edge-to-edge on an ultrawide
   /// monitor nor squashes below a legible minimum on a narrow one.
   ///
   /// Setting `min_aspect_ratio == max_aspect_ratio` locks to a single fixed
   /// ratio; this is the same widget in both cases, not a separate variant.
   #[derive(Clone, Copy, Debug, Component, Reflect)]
   #[reflect(Component, Default)]
   pub struct LastBeaconUiAspectRatioBounds {
       /// Minimum allowed width-over-height ratio.
       pub min_aspect_ratio: f32,
       /// Maximum allowed width-over-height ratio.
       pub max_aspect_ratio: f32,
   }

   impl Default for LastBeaconUiAspectRatioBounds {
       fn default() -> Self {
           // Inert by default -- a widget left unconfigured constrains
           // nothing, the same way an unconfigured Feathers `Props` struct
           // is harmless. `0.0..=INFINITY` can never actually clamp a real
           // (positive, finite) aspect ratio.
           Self {
               min_aspect_ratio: 0.0,
               max_aspect_ratio: f32::INFINITY,
           }
       }
   }
   ```
2. Add a system that:
   - Runs after `bevy::ui::UiSystems::PostLayout` (matching `refresh_last_beacon_ui_text_box_scrollbars`'s existing precedent).
   - For each `LastBeaconUiAspectRatioBounds` entity, looks up its parent via `ChildOf` and reads the parent's `ComputedNode::content_box()` size.
   - Swaps `min`/`max` defensively if authored the wrong way round (`min > max`), so a content typo degrades gracefully instead of producing an inverted clamp.
   - Clamps the parent's current ratio into `[min, max]`, computes the largest size at that ratio that still fits inside the parent's content box ("contain fit"), and writes the result to this widget's own `Node.width`/`Node.height` as `Val::Px`, guarded by an equality check against the current value so it doesn't rewrite (and re-trigger layout) every single frame when nothing changed.
3. Add an "Aspect Ratio Container" section to `docs/ui-widgets.md` documenting: `LastBeaconUiAspectRatioBounds { min_aspect_ratio, max_aspect_ratio }` as the author-updated property; the requirement that the immediate parent center this widget (`align_items: Center, justify_content: Center`) and must not size itself from this widget's own size (pin the parent to the window/viewport) to avoid a layout feedback loop.
4. Add an "ASPECT RATIO" gallery card to `game/assets/scenes/ui_playground.bsn`: a fixed-size gallery panel (matching the existing cards' explicit-width pattern) containing a `LastBeaconUiAspectRatioBounds { min_aspect_ratio: 1.0, max_aspect_ratio: 1.6 }` child with a visible background fill, so the demo shows the widget centering and holding its band inside the panel.

### 4. Feathers-inspired plugin structure
1. Add a new plugin in `ui_widgets.rs`:
   ```rust
   /// Plugin which registers Last Beacon's grid-span and aspect-ratio layout
   /// widgets.
   ///
   /// Grouped into its own plugin, rather than appended directly to
   /// `LastBeaconPlugin`'s builder chain, to mirror how `bevy_feathers`
   /// composes its own controls: one plugin per widget family, added
   /// together by a parent plugin (`ControlsPlugin` there, `LastBeaconPlugin`
   /// here).
   pub struct LastBeaconUiLayoutWidgetsPlugin;

   impl Plugin for LastBeaconUiLayoutWidgetsPlugin {
       fn build(&self, app: &mut App) {
           app.register_type::<LastBeaconUiGridItem>()
               .register_type::<LastBeaconUiAspectRatioBounds>()
               .add_systems(Update, apply_last_beacon_ui_grid_item_span)
               .add_systems(
                   PostUpdate,
                   apply_last_beacon_ui_aspect_ratio_bounds
                       .after(bevy::ui::UiSystems::PostLayout),
               );
       }
   }
   ```
2. In `game/src/lib.rs`, add `.add_plugins(ui_widgets::LastBeaconUiLayoutWidgetsPlugin)` to `LastBeaconPlugin::build`'s existing chain, instead of inline `.register_type()`/`.add_systems()` calls for these two components. This does not change how any existing widget is registered.

### 5. Tests (per component, added to `ui_widgets.rs`'s existing `#[cfg(test)] mod tests`)
- Span Grid: an entity spawned with `LastBeaconUiGridItem { column_span: 2, row_span: 3 }` ends up with a `Node.grid_column`/`Node.grid_row` matching `GridPlacement::span(2)`/`GridPlacement::span(3)`; an entity spawned with `column_span: 0` still gets `GridPlacement::span(1)` (proving the panic-avoiding clamp actually runs) rather than panicking.
- Aspect Ratio Container: a parent with a manually constructed `ComputedNode { size: Vec2::new(1600.0, 900.0), ..default() }` (a 16:9 available area) and a child with `min_aspect_ratio: 1.0, max_aspect_ratio: 1.0` (forcing square) ends up sized to `900x900` (the largest square that fits); the same parent with `min_aspect_ratio: 0.5, max_aspect_ratio: 5.0` (a band the 16:9 parent already falls inside) ends up sized to the full `1600x900` (no clamping needed); a narrow parent (`ComputedNode { size: Vec2::new(400.0, 900.0), .. }`, i.e. a tall/narrow ratio around 0.44) with `min_aspect_ratio: 1.0, max_aspect_ratio: 2.0` ends up clamped to the `1.0` minimum, sized to `400x400`; a test proving the min/max swap when authored backwards (`min_aspect_ratio: 2.0, max_aspect_ratio: 1.0`) still produces the same result as the correctly-ordered case; a test proving the `Default` impl is inert (an entity left at `LastBeaconUiAspectRatioBounds::default()` inside any parent size passes that size through unchanged).

### 6. Validation and PR readiness
1. Run `cargo fmt --manifest-path game/Cargo.toml -- --check`, `cargo clippy --manifest-path game/Cargo.toml --all-targets --all-features -- -D warnings`, `cargo test --manifest-path game/Cargo.toml --all-features`, `cargo doc --manifest-path game/Cargo.toml --all-features --no-deps`, and `scripts\validate.cmd`.
2. Launch the game (`cargo run --manifest-path game/Cargo.toml --all-features`) and visually check the two new UI Playground gallery cards render as expected -- this project has no GUI automation tool available, so this is a smoke check (does it render, does it panic), not a substitute for the user's own visual review.
3. Push the branch and prepare a pull request into `dev` per the gitflow-workflow skill (no local merge). Note in the PR description that it is independent of, and does not depend on, the still-open PR #16.

## Submodule Plan
- Engine changes required: `no`
- Engine branch: `N/A`
- Engine commit expectation: `N/A`
- Bound engine commit hash: `N/A`
- Root pointer update required: `no`

## Alternatives Considered
- **Two separate widgets (fixed-ratio vs. clamped-ratio aspect containers)** — this was my initial framing; the user explicitly corrected it to a single widget configured with `min`/`max` (equal values degenerating to fixed). Adopted the single-widget design across the whole plan.
- **Using Bevy's native `Node.aspect_ratio: Option<f32>` field for the aspect-ratio widget** — rejected because it only supports one fixed ratio; a `[min, max]` band requires reading the actual available space and computing the clamp manually, so the native field can't express the general case the user asked for.
- **A `GridTrackRepetition::AutoFill`-based "responsive" uniform grid** (columns that auto-add based on available width, like CSS `repeat(auto-fill, ...)`) — not requested; the user asked for a uniform grid, not a responsive reflow grid. Left as a possible future enhancement to the same `.bsn` preset (swap the fixed repetition count for `GridTrackRepetition::AutoFill`), not a separate widget.
- **Fuller Feathers-style architecture for just these 3 widgets** (`SceneComponent`-derive + `bsn!`-macro scene functions instead of hand-authored `.bsn` files, `Props` structs as the primary construction API, `bevy_ui_widgets`-style headless behavior) — considered and rejected as the reading of "designed in a similar way." That fuller adoption was already explicitly scoped out of the prior PR (#16) for the whole widget library, and introducing it for only 3 of ~20 widgets would leave two incompatible construction patterns in the same file. The per-widget-`Plugin` structure (see Proposed Implementation Approach, section 4) is the part of Feathers' design that transfers cleanly without that inconsistency.

## Risks, Constraints, And Assumptions
- `GridPlacement::span(0)` panics; `LastBeaconUiGridItem` must clamp both span fields to a minimum of `1` before calling it. Covered by a dedicated test.
- The Aspect Ratio container's parent must not size itself based on this widget's own size (must be pinned to a stable size, e.g. the window/viewport), or the system's write-back could feed into a layout oscillation. This is documented as an authoring requirement, not something the system can enforce generically, since it does not own its parent.
- This branch was created from current `origin/dev` (`018b317`), not from the still-open `feature/ui-widget-feathers-alignment` (PR #16) branch, per gitflow-workflow's "branches come from dev" rule. The three widgets in this plan have no functional dependency on PR #16's changes, so this is safe, but the two PRs will both need to merge into `dev` independently (normal parallel-feature-branch behavior, not a blocker).
- Uniform Grid intentionally ships with zero new Rust code; if that turns out to be too limited in practice (e.g. authors want a shared marker component for tooling/reflection purposes), a thin `LastBeaconUiUniformGrid` marker component could be added later without breaking the `.bsn` convention.

## Open Questions
None blocking. The one open question from the prior revision (whether "designed in a similar way" to Feathers meant more than the per-widget-`Plugin` structure + docs + inert defaults) is resolved: the user confirmed these widgets should be authored via `.bsn` "as much as possible, the same as the other components," i.e. no Rust-side `Props`/builder spawn-function API. This matches the plan as written -- Uniform Grid is a pure `.bsn` preset, and Span Grid / Aspect Ratio Container are authored in `.bsn` via `LastBeaconUiGridItem`/`LastBeaconUiAspectRatioBounds` components read by reactive systems, exactly like every other existing widget (`LastBeaconUiSlider`, `LastBeaconUiTab`, etc.).

## Documentation Expectations
- Every new public function/component gets a Rustdoc comment, matching the existing convention in `ui_widgets.rs` and Feathers' own clear-purpose-statement style.
- `docs/ui-widgets.md` gets three new sections (Uniform Grid, Span Grid, Aspect Ratio Container) in the same format as existing entries, each noting the new gallery card in `ui_playground.bsn`.
- `cargo doc --manifest-path game/Cargo.toml --all-features --no-deps` must succeed.

## Implementation Handoff Notes
- Use Claude Sonnet 5 for implementation (fulfilling the `gpt-5.4` role per the user's standing instruction).
- Read `game/src/ui_widgets.rs` in full before editing (it has grown past 2300 lines even before this feature) and match its established conventions: query type aliases, `Added<T>` reactive systems, equality-guarded writes.
- Read `game/assets/scenes/ui_playground.bsn` in full before adding gallery cards; match the existing `#XWidgetGallery` card structure (fixed-width bordered `Node`, uppercase section-label `Text`, then the widget reference(s)) exactly rather than inventing new card styling.
- The task breakdown's illustrative Rust snippets for Span Grid describe the clamping/translation logic; implementation must mutate the existing `Node` component's `grid_column`/`grid_row` fields directly (`Node` already exists on any spawned UI entity), not attempt to `.insert()` a bare `GridPlacement` as if it were its own component.
- Apply `.pi/skills/rust-coding-standards/SKILL.md`: descriptive names, named values before non-trivial calls, frequent "why" comments at points that cross layout/ownership boundaries (the span-clamping panic-avoidance, the parent-lookup for aspect ratio, the min/max swap).
- Commit at the end of each of the three widgets (or more often), following the commit message format in `.pi/skills/gitflow-workflow/SKILL.md`.

## Optional Review Focus Areas
- Use Claude Sonnet 5 for review (fulfilling the `gpt-5.5` role per the user's standing instruction).
- Confirm the aspect-ratio contain-fit math is correct at the boundary cases (parent wider than the max ratio allows, parent narrower than the min ratio allows, parent already inside the band).
- Confirm the equality guard on the aspect-ratio write-back actually prevents a layout thrash loop (i.e. that a converged size is not rewritten every frame).
- Confirm the `LastBeaconUiLayoutWidgetsPlugin` split reads as a natural first step toward more modular registration, not as an inconsistent one-off next to the monolithic `LastBeaconPlugin`.

## Success Criteria
- A grid container built from the Uniform Grid preset lays out children in evenly sized columns with no custom Rust code.
- A child marked `LastBeaconUiGridItem { column_span: 2, row_span: 1 }` inside such a grid visibly spans two columns.
- A widget marked `LastBeaconUiAspectRatioBounds { min_aspect_ratio, max_aspect_ratio }` stays within that ratio band and is centered, regardless of the window's actual aspect ratio.
- Both new gallery cards appear in the UI Playground scene alongside the existing ones.
- `cargo fmt --check`, `cargo clippy -D warnings`, `cargo test --all-features`, and `cargo doc --no-deps` all pass against `game/Cargo.toml`; `scripts\validate.cmd` passes.

## Testing Methodology
- Game validation: `scripts\validate.cmd`.
- Focused commands: `cargo fmt --manifest-path game/Cargo.toml -- --check`, `cargo clippy --manifest-path game/Cargo.toml --all-targets --all-features -- -D warnings`, `cargo test --manifest-path game/Cargo.toml --all-features`, `cargo doc --manifest-path game/Cargo.toml --all-features --no-deps`.
- New unit tests per component, using directly-constructed `ComputedNode`/`Node` values rather than a full Taffy layout pass, matching this file's existing testing style.
- A background smoke launch of the game (same approach used in PR #16) to confirm the new gallery cards render without panicking, with the same recorded limitation that full visual review needs the user (no GUI automation tool available in this environment).
- Engine validation: `N/A` (no engine changes).
