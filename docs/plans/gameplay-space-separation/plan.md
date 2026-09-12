# Gameplay Space Separation Plan

## Metadata
- Feature slug: `gameplay-space-separation`
- Feature area: `game`
- Primary area: `game`
- Root branch: `feature/gameplay-space-separation`
- Engine branch: `N/A`
- Engine submodule pointer: `N/A` (no engine changes planned)
- Status: `Planned`
- Planning model: `gpt-5.5` (role fulfilled by Claude Sonnet 5, per the user's standing instruction that Claude/subagents replace GPT in this workflow)
- Implementation model: `gpt-5.4` (role fulfilled by Claude Sonnet 5)
- Review model: `gpt-5.5` (role fulfilled by Claude Sonnet 5)
- Created: `2026-09-12`
- Last updated: `2026-09-12`

## User Request
The user wants to start building gameplay and asked to first set up an architectural separation of concerns before any gameplay logic is written: Beacon/hub gameplay mechanics and launched-vehicle/world gameplay mechanics must be "totally separate gameplay spaces," while still having a shared communication stream so hub gameplay can set data the world gameplay reads (and vice versa). The user also wants the options menu to use that same shared communication stream rather than a separate channel. This plan covers only the scaffolding/separation; concrete gameplay systems are explicitly deferred to follow-up features.

Clarifying decisions made during brainstorming:
- Space naming: **Hub** / **World** (generic naming, not tied to current fiction wording).
- Communication mechanism: a shared **resource** (persistent cross-space state) **plus messages** (discrete one-shot cross-space events), mirroring the existing `SceneStack`/`FoundationPauseState` + `SceneCommand` pattern already used in this codebase.
- Scope: **pure plumbing** — empty-ish plugins and shared types wired into `LastBeaconPlugin`, with no speculative gameplay fields, since no concrete gameplay design exists yet to hang fields on.

## Feature Summary
Adds three new Last Beacon modules under `game/src/`:
- `shared/` — owns the cross-space communication layer: a persistent-state resource, a discrete-event message type, and a "which gameplay space is currently active" resource + run-conditions, all with zero gameplay-specific fields yet.
- `hub/` — designated home for future Beacon/dashboard gameplay systems. Depends on `shared` only.
- `world/` — designated home for future launched-vehicle/expedition gameplay systems. Depends on `shared` only.

The separation is enforced structurally: `hub` and `world` never depend on each other (no `use crate::world` inside `hub`, no `use crate::hub` inside `world`); anything that looks like it needs to cross that boundary must go through `shared` instead. This mirrors the existing `LastBeaconUiLayoutWidgetsPlugin` pattern in `game/src/ui_widgets.rs` (one small `Plugin` per concern, bundling its own type registrations and systems), and reuses the existing scene-stack/run-condition idioms already established by `foundation-runtime-library` (`FoundationPauseState` + `foundation_is_not_paused`, and `hide_last_beacon_menu_ui_behind_settings`'s pattern of reading `SceneStack` directly).

Options-menu code (currently BSN-driven components in `game/src/ui_widgets.rs`) can depend on `shared` the same way `hub`/`world` do, satisfying the requirement that options use the same communication stream without a fourth, separate channel.

## Feature Area Classification
- Area: `game`
- Primary area: `game`
- Rationale: All affected files are Last Beacon-owned gameplay/module-structure files under `game/src/`. No Foundation Engine runtime behavior is generic enough to belong in `engine/` — this is Last Beacon's own gameplay-space concept, built on top of Foundation's existing generic `SceneStack`/pause primitives without modifying them.

## Codebase Research
- `game/src/lib.rs`: `LastBeaconPlugin::build` is the single place that wires all game-owned plugins (`ui_widgets::LastBeaconUiLayoutWidgetsPlugin`, type registrations, `Startup`/`Update`/`PostUpdate` system groups). New plugins are added here the same way.
- `game/src/ui_widgets.rs:2693-2712` (`LastBeaconUiLayoutWidgetsPlugin`): the established one-`Plugin`-per-concern shape — `register_type::<...>()` calls followed by `add_systems(...)` — used as the template for `LastBeaconSharedGameplayPlugin`, `LastBeaconHubGameplayPlugin`, and `LastBeaconWorldGameplayPlugin`.
- `engine/crates/foundation-runtime-library/src/menu.rs:57-77` (`FoundationPauseState` + `foundation_is_not_paused`/`foundation_is_paused`): the existing precedent for "a small `Resource` plus a paired `run_if` condition function" — the template for `LastBeaconGameplaySpace` + `last_beacon_is_in_hub`/`last_beacon_is_in_world`.
- `engine/crates/foundation-runtime-library/src/scene_stack.rs`: `SceneStack` is a `Resource` holding scene stack entries with `SceneSource` (`BsnScene { key }` or `Runtime { key }`) and derived presentation flags (`covers_previous`, `blocks_previous_input`, `blocks_previous_update`). This is the source of truth `LastBeaconGameplaySpace` reads to determine which space is active — no new engine-level state is needed.
- `game/src/scenes.rs:9-42` (scene key constants): `BEACON_SCENE` plus its pages (`DASHBOARD_SCENE`, `HANGAR_SCENE`, `GARAGE_SCENE`, `MISSION_CONTROL_SCENE`, `FABRICATION_SCENE`, `SILO_UPGRADES_SCENE`) identify the Hub space; `GAMEPLAY_LEVEL_SCENE` identifies the World space. `OPTIONS_MENU_SCENE`/`PAUSE_MENU_SCENE` are overlays and must not change the detected space.
- `game/src/lib.rs:472-499` (`hide_last_beacon_menu_ui_behind_settings`): existing precedent for a system that reads `Res<SceneStack>` / `scene_stack.entries()` directly to answer "what's on the stack right now," including the overlay-detection pattern (`SceneSource::BsnScene { key } if key == scenes::OPTIONS_MENU_SCENE`) that `LastBeaconGameplaySpace`'s resolution system will reuse for "ignore overlay entries."
- `game/src/lib.rs:501-563` (`settings_overlay_hides_marked_menu_ui_without_closing_lower_scene` test): precedent for building a minimal `App` with `MinimalPlugins` + `FoundationSceneStackPlugin`, writing `SceneCommand` messages, and asserting on resulting state — the template for the new `LastBeaconGameplaySpace` resolution test.
- Bevy's `Message`/`MessageReader`/`MessageWriter` (not `Event`) is this codebase's current terminology (confirmed via `FoundationExitRequested`, `SceneCommand`, etc., all registered with `add_message::<T>()`), so `LastBeaconGameplayEvent` will be registered the same way despite its "Event"-flavored name (kept because it describes discrete gameplay occurrences, distinct from Foundation's own `Scene*` message family).

## External Research
No external online research was performed. This feature only combines existing in-repo Bevy/Foundation patterns (resources, run-conditions, messages, scene stack); no new library or API research was needed.

## Affected Files And Systems
- `game/src/shared/mod.rs` (new): `LastBeaconSharedGameplayState` resource, `LastBeaconGameplayEvent` message enum, `LastBeaconGameplaySpace` resource + resolution system, `last_beacon_is_in_hub`/`last_beacon_is_in_world` run-conditions, `LastBeaconSharedGameplayPlugin`.
- `game/src/hub/mod.rs` (new): `LastBeaconHubGameplayPlugin` (empty `build`, doc comment marking it as the designated home for future hub gameplay systems).
- `game/src/world/mod.rs` (new): `LastBeaconWorldGameplayPlugin` (empty `build`, doc comment marking it as the designated home for future world/expedition gameplay systems).
- `game/src/lib.rs`: add `pub mod shared;`, `pub mod hub;`, `pub mod world;`; add the three new plugins to `LastBeaconPlugin::build` (`shared` first, since `hub`/`world` conceptually depend on it existing, even though nothing currently enforces load order at compile time).

## Proposed Implementation Approach
1. Add `game/src/shared/mod.rs` with `LastBeaconSharedGameplayState`, `LastBeaconGameplayEvent`, `LastBeaconGameplaySpace`, the resolution system, the two run-conditions, and `LastBeaconSharedGameplayPlugin`, following `LastBeaconUiLayoutWidgetsPlugin`'s structure and doc-comment style.
2. Add `game/src/hub/mod.rs` and `game/src/world/mod.rs`, each with an empty `LastBeaconHubGameplayPlugin`/`LastBeaconWorldGameplayPlugin` and a doc comment stating the one-way dependency rule (depends on `shared`, must never depend on the other space's module).
3. Wire `pub mod shared;`, `pub mod hub;`, `pub mod world;` and the three plugins into `game/src/lib.rs`.
4. Add a unit test for `LastBeaconGameplaySpace` resolution covering: Hub scenes on the stack, `GAMEPLAY_LEVEL_SCENE` on the stack, neither (menu/splash/credits), and an overlay (pause/options) on top not changing the resolved space — following the existing `settings_overlay_hides_marked_menu_ui_without_closing_lower_scene` test template.
5. Run validation (see Testing Methodology) and fix any issues.

## Submodule Plan
- Engine changes required: `no`
- Engine branch: `N/A`
- Engine commit expectation: `N/A`
- Bound engine commit hash: `N/A`
- Root pointer update required: `no`

## Alternatives Considered
- **Bevy `States`/`SubStates`**: rejected as the space-detection mechanism. This codebase deliberately uses its own `SceneStack`/`SceneCommand` primitives instead of Bevy's built-in state machine (confirmed: no `bevy::state` usage anywhere in `game/` or `engine/crates/foundation-runtime-library`), so introducing `States` here would create a second, competing source of truth for "what's currently showing" alongside the scene stack. Deriving `LastBeaconGameplaySpace` from `SceneStack` keeps one source of truth.
- **Messages only, no shared resource**: rejected per the user's explicit choice. A messages-only design pushes state-caching onto whichever system needs to remember something, which tends to duplicate state rather than avoid it.
- **Single shared resource only, no messages**: rejected per the user's explicit choice. Loses a clean way to react to one-shot "this just happened" occurrences without polling for resource changes every frame.
- **Flat files (`hub.rs`/`world.rs`/`shared.rs`) instead of folders**: considered for consistency with most of `game/src`'s current flat layout, but folders were chosen (matching the existing `scenes/` folder-module) since both `hub` and `world` are expected to grow substantially as real gameplay systems land in follow-up features.
- **Pre-populating example fields end-to-end (hub→world and world→hub)**: considered so the pattern would be proven with a real data flow before gameplay leans on it, but rejected per the user's explicit "pure plumbing" scope choice — no gameplay design exists yet to justify concrete field shapes, and speculative fields would likely need renaming/reshaping once real gameplay requirements exist.

## Risks, Constraints, And Assumptions
- **Risk**: nothing prevents a future contributor from adding `use crate::world` inside `hub` (or vice versa) — the separation is a convention enforced by code review, not the compiler. Mitigated by clear doc comments on both new modules stating the rule explicitly.
- **Assumption**: `LastBeaconGameplaySpace` resolves purely from scene keys already defined in `game/src/scenes.rs`; if new Beacon pages or world-adjacent scenes are added later, their keys must be added to the resolution match arms, or they will resolve to `Neither`.
- **Assumption**: pause/options overlays are correctly identified as overlays (not spaces) using the same `ScenePresentation`/stack-entry inspection pattern `hide_last_beacon_menu_ui_behind_settings` already uses; if that pattern is wrong today, this feature inherits the same gap rather than introducing a new one.
- **Constraint**: no gameplay behavior changes in this feature — `LastBeaconHubGameplayPlugin`/`LastBeaconWorldGameplayPlugin` add no systems, so there is no observable in-game behavior difference after this feature merges, only new compile-time structure and one new resolvable resource.

## Open Questions
- None outstanding. All scope/naming/mechanism questions were resolved during brainstorming.

## Documentation Expectations
- Every new public type (`LastBeaconSharedGameplayState`, `LastBeaconGameplayEvent`, `LastBeaconGameplaySpace`, `last_beacon_is_in_hub`, `last_beacon_is_in_world`, `LastBeaconSharedGameplayPlugin`, `LastBeaconHubGameplayPlugin`, `LastBeaconWorldGameplayPlugin`) gets a Rustdoc comment, matching the existing style in `ui_widgets.rs` and `menu.rs`.
- `hub/mod.rs` and `world/mod.rs` module-level doc comments must state the one-way dependency rule (depends on `shared`, never on each other) so it's visible to anyone opening either file.
- No `docs/` markdown page is needed beyond this plan — the module doc comments are sufficient given this is internal architecture, not a player- or tool-facing feature (unlike `docs/ui-widgets.md`, which documents an authoring surface).

## Implementation Handoff Notes
- Use `gpt-5.4` for implementation (role fulfilled by Claude Sonnet 5).
- Never use Anthropic models (per standing project instruction; role fulfilled by Claude Sonnet 5 per the user's durable override).
- Follow `.pi/skills/rust-coding-standards/SKILL.md` for naming and comment conventions.
- Keep `LastBeaconSharedGameplayState` and `LastBeaconGameplayEvent` genuinely empty of gameplay fields/variants in this feature — adding speculative fields here would violate the user's explicit "pure plumbing" scope decision.
- Match `LastBeaconUiLayoutWidgetsPlugin`'s structure exactly for the three new plugins (register types first, then systems) for consistency.

## Optional Review Focus Areas
- Use `gpt-5.5` for review (role fulfilled by Claude Sonnet 5).
- Confirm `hub/mod.rs` has no `use crate::world` and `world/mod.rs` has no `use crate::hub`.
- Confirm `LastBeaconGameplaySpace` resolution correctly ignores overlay scenes (pause/options) and correctly resolves all current Beacon page scene keys, not just `BEACON_SCENE` itself.

## Success Criteria
- `game/src/shared/`, `game/src/hub/`, `game/src/world/` exist and compile as part of `LastBeaconPlugin`.
- `LastBeaconGameplaySpace` correctly resolves to `Hub`, `World`, or `Neither` based on the scene stack, verified by a passing unit test.
- No `use` dependency exists from `hub` into `world` or from `world` into `hub`.
- `scripts\validate.cmd` passes.

## Testing Methodology
- Game validation: `scripts\validate.cmd`.
- Focused check: `cargo test --manifest-path game/Cargo.toml` (or the equivalent focused test filter) for the new `LastBeaconGameplaySpace` resolution test.
- Engine validation: `N/A` (no engine changes).
