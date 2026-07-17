# Foundation BSN Assets Plan

## Metadata
- Feature slug: `foundation-bsn-assets`
- Feature area: `multi-area`
- Primary area: `engine`
- Root branch: `feature/foundation-bsn-assets`
- Root branch status: `Created from current root dev at afa88b96bc0bb46336620af08713347d6871d52b`
- Engine branch: `feature/foundation-bsn-assets`
- Engine branch status: `Required before implementation; engine submodule is currently detached at b4ff3107932e177a98ae1eee626578b1f05b2be9`
- Engine submodule pointer: `Currently b4ff3107932e177a98ae1eee626578b1f05b2be9; root pointer update required after engine implementation commit`
- Status: `Planned`
- Planning model: `gpt-5.5`
- Implementation model: `gpt-5.4`
- Review model: `gpt-5.5`
- Created: `2026-07-17`
- Last updated: `2026-07-17`

## User Request
Add temporary Foundation Engine support for `.bsn` asset files before Bevy ships official BSN asset support. The feature should support `.bsn` for levels and prefabs, not arbitrary data assets. Hot reload should entirely remove the spawned root entity and children, then replace them from the changed `.bsn` file. The implementation must live in Foundation Engine and remain easy to remove when Bevy implements its own `.bsn` asset pipeline. As part of validating the system, convert the current Last Beacon Rust `bsn!` macro-authored scenes into full `.bsn` asset scenes.

## Feature Summary
Build a Foundation-owned BSN asset bridge for Bevy 0.19 that lets games spawn level and prefab content from `.bsn` files. Foundation should treat loaded BSN files as spawnable ECS content, integrate with existing scene-stack ownership, and support deterministic development hot reload by despawning and respawning each live instance when its source asset reloads. After the engine bridge exists, migrate the current Last Beacon code-authored `bsn!` scenes to `.bsn` asset files so the existing splash/menu/gameplay flow becomes an end-to-end test of asset-backed BSN.

This is explicitly temporary compatibility infrastructure. It should wrap or mirror Bevy's in-progress dynamic BSN direction closely enough that Foundation can delete or replace the bridge when upstream Bevy provides first-party `.bsn` asset support.

## Feature Area Classification
- Area: `multi-area`
- Primary area: `engine`
- Rationale: The loader, spawn bridge, hot-reload policy, and reusable prefab/level support belong to Foundation Engine runtime infrastructure. Last Beacon is affected as the first consumer because its current Rust `bsn!` scene catalog should be converted to `.bsn` asset scenes after the engine bridge is available.

## Codebase Research
- Root `game/Cargo.toml` uses Bevy `0.19.0` with `file_watcher`, `reflect_documentation`, and `serialize`, and depends on `foundation-runtime-library` through `../engine/crates/foundation-runtime-library`.
- Engine `Cargo.toml` uses Bevy `0.19.0` with the same asset watcher and reflection-related features at workspace level.
- `engine/README.md` and `engine/docs/scene-system.md` state that TemplateGame currently uses Rust-authored Bevy 0.19 BSN macros because Bevy does not currently ship a first-party `.bsn` asset loader.
- `engine/crates/foundation-runtime-library/src/scene_stack.rs` already models `SceneSource::BsnScene { key }`, emits `SceneLoadRequested`, and documents that bridge systems should load the requested source and tag spawned entities with `SceneOwner`.
- Root `game/src/scenes/` currently contains the Last Beacon scene catalog as Rust modules using `bsn!` macros for splash screens, menus, credits, pause menu, and the sample gameplay level. `game/src/scenes/mod.rs` maps stable scene keys such as `last-beacon/main_menu` and `last-beacon/gameplay_level` to those Rust-authored scenes.
- `SceneOwner` cleanup already despawns top-level scene-owned entities when a scene leaves the stack, and visibility sync operates on scene-root owned entities. The BSN asset bridge should integrate with this ownership model instead of inventing a separate lifetime system for stacked scenes.
- `foundation-runtime-library` is the correct engine crate for reusable runtime support. A focused BSN asset module/plugin should be exported from this crate and installed from `FoundationPlugin` or through an explicitly named opt-in plugin.
- The engine submodule is currently detached at `b4ff3107932e177a98ae1eee626578b1f05b2be9`. Before implementation edits, create or switch to `feature/foundation-bsn-assets` inside `engine/` from engine `dev` and verify/record the branch base.

## External Research
- Bevy PR #23576 implements a dynamic `.bsn` asset loader as `DynamicBsnLoader`, an `AssetLoader` whose asset type is `ScenePatch` and whose extension list is `["bsn"]`. The loader reads text, lexes/parses BSN, converts it to a `ScenePatch`, and registers from the scene plugin. Sources: [`dynamic_bsn.rs`](https://github.com/bevyengine/bevy/blob/bbc8b57ef45059b57633d30d9a18958bcad1dca4/crates/bevy_scene2/src/dynamic_bsn.rs#L149-L190), [`lib.rs`](https://github.com/bevyengine/bevy/blob/bbc8b57ef45059b57633d30d9a18958bcad1dca4/crates/bevy_scene2/src/lib.rs#L50-L55).
- PR #23576's scene spawning path resolves `ScenePatch` assets after `AssetEvent::LoadedWithDependencies` and applies queued scene instances when the resolved asset is available. This supports an asset-driven load lifecycle, but does not by itself provide live in-place reconciliation for already-spawned entities. Source: [`spawn.rs`](https://github.com/bevyengine/bevy/blob/bbc8b57ef45059b57633d30d9a18958bcad1dca4/crates/bevy_scene2/src/spawn.rs#L583-L614).
- Bevy PR #23742, merged as `02aef1419910c8f02b0a19a5344e3488f6df7388`, introduced `ReflectConvert`, a generic reflected conversion mechanism intended for asset BSN. Bevy asset registration also registers `String -> HandleTemplate<A>`, allowing string asset paths in BSN to convert into handle templates without a permanent hard-coded loader hack. Sources: [`convert.rs`](https://github.com/bevyengine/bevy/blob/02aef1419910c8f02b0a19a5344e3488f6df7388/crates/bevy_reflect/src/convert.rs#L65-L96), [`bevy_asset/src/lib.rs`](https://github.com/bevyengine/bevy/blob/02aef1419910c8f02b0a19a5344e3488f6df7388/crates/bevy_asset/src/lib.rs#L681-L690).
- Bevy PR #23639 adds a BSN writer that serializes ECS worlds/assets to `.bsn` text. This is useful for future editor save/export workflows but is not required for the first runtime loader/hot-reload bridge. Sources: [`dynamic_bsn_writer.rs`](https://github.com/bevyengine/bevy/blob/c45268e88f346b40ccdfcc3bee50fba3e0392eea/crates/bevy_scene2/src/dynamic_bsn_writer.rs#L100-L112), [`dynamic_bsn_writer.rs`](https://github.com/bevyengine/bevy/blob/c45268e88f346b40ccdfcc3bee50fba3e0392eea/crates/bevy_scene2/src/dynamic_bsn_writer.rs#L181-L226).

## Affected Files And Systems
- `engine/crates/foundation-runtime-library/src/lib.rs`: export and install a temporary Foundation BSN asset plugin or opt-in module.
- `engine/crates/foundation-runtime-library/src/bsn_assets.rs` or `src/bsn_assets/mod.rs`: likely new module for the temporary `.bsn` asset type, loader, spawn handles, instance tracking, hot-reload replacement, and scene-stack integration.
- `engine/crates/foundation-runtime-library/Cargo.toml`: may need minimal parser dependencies if Bevy 0.19 does not expose a reusable BSN parser for asset files. Avoid broad dependencies unless clearly justified.
- `engine/Cargo.toml`: workspace dependency additions only if required by the selected parser approach.
- `engine/docs/scene-system.md`: update to explain Foundation temporary `.bsn` asset support for levels and prefabs, hot-reload semantics, and removal expectations once Bevy supports official `.bsn` assets.
- `engine/games/template-game/`: optionally add a small example `.bsn` prefab or level and integration smoke path if useful for validation, but keep game-specific content minimal.
- `game/src/scenes/`: convert current Rust `bsn!` macro scene content into `.bsn` asset-backed scenes or thin glue that delegates to `.bsn` assets.
- `game/assets/`: add `.bsn` level/prefab scene files for the current Last Beacon splash/menu/credits/pause/gameplay flow, using paths or catalog keys that exercise the Foundation loader.
- `game/`: update scene catalog glue and tests so existing `SceneSource::bsn_scene` keys resolve through asset-backed `.bsn` content instead of code-authored macro scenes where practical.
- `docs/plans/foundation-bsn-assets/`: root plan/tracker and eventual root submodule pointer tracking.

## Proposed Implementation Approach
1. Define the public Foundation contract:
   - `.bsn` support is for ECS levels and prefabs only.
   - A BSN asset represents spawnable ECS content with one or more root entities and optional children.
   - Hot reload replaces the whole live instance by despawning tracked root entities recursively and spawning a fresh instance from the reloaded asset.
   - Runtime gameplay state preservation is intentionally out of scope.
2. Add a temporary, clearly named Foundation module/plugin, for example `FoundationBsnAssetPlugin` in `foundation-runtime-library`.
   - The name and module docs should say this is a bridge pending official Bevy `.bsn` asset support.
   - Keep the API surface narrow so removal later is straightforward.
3. Implement or adapt a `.bsn` asset loader consistent with Bevy PR #23576.
   - Prefer using Bevy 0.19 public APIs where possible.
   - If code must be adapted from upstream PRs, keep it isolated in the temporary module and preserve license-compatible attribution in comments/docs where appropriate.
   - Use `ReflectConvert` and registered asset conversions where available rather than hard-coding `Handle<T>` behavior unnecessarily.
4. Represent loaded BSN files as a Foundation asset wrapper or scene/prefab patch type that can be spawned by Foundation systems.
   - Keep level and prefab semantics unified where possible; semantic difference should come from the caller/path/API, not from two divergent file formats.
   - Track source handles/paths for each live instance so asset reload events can find instances to replace.
5. Add spawn APIs for direct prefab/level use.
   - Provide a command/system-friendly way to spawn a `.bsn` asset as a root entity or set of root entities.
   - Ensure spawned roots and children can receive `SceneOwner` when spawned for a scene stack entry.
   - Ensure direct prefab instances can be hot-reload tracked even when not attached to the scene stack.
6. Integrate with Foundation scene stack.
   - Listen for `SceneLoadRequested` with `SceneSource::BsnScene` where the key resolves to a `.bsn` asset path or catalog entry.
   - Spawn the loaded content and tag roots/children with `SceneOwner { scene_id }`.
   - Reuse existing scene cleanup for normal scene removal.
7. Implement hot reload replacement.
   - Listen for the appropriate Bevy asset events indicating a `.bsn` asset was loaded/reloaded with dependencies.
   - For each live instance of that asset, record enough context to respawn it in the same parent/root context.
   - Despawn the previous root entity or root entities recursively before spawning the replacement.
   - Reapply `SceneOwner`, parent attachment, transforms/overrides that are part of the instance contract, and visibility expectations.
   - Avoid preserving gameplay-mutated component state unless a future plan explicitly adds that feature.
8. Convert the current Last Beacon Rust `bsn!` macro scenes into `.bsn` asset scenes.
   - Move the splash, main menu, options, credits, pause, and sample gameplay scene structure into `game/assets/**/*.bsn` or an equivalent game-owned asset layout.
   - Keep Rust glue only where behavior cannot reasonably live in static `.bsn`, such as registering systems, resources, or strongly typed runtime callbacks.
   - Preserve the current scene keys and gameplay flow so this migration tests the new loader without changing player-facing behavior.
9. Add tests and examples.
   - Unit-test path/key resolution, instance tracking, and reload replacement bookkeeping where possible.
   - Add ECS tests that verify a replacement removes old roots/children and creates fresh roots/children for the same asset.
   - Use the converted Last Beacon scenes as an end-to-end game-facing validation case.
10. Document the temporary nature and removal plan.
   - Explain that this is a Foundation bridge around Bevy 0.19 limitations.
   - Identify the likely future deletion/swap point once official Bevy `.bsn` asset support exists.

## Submodule Plan
- Engine changes required: `yes`
- Engine branch: `feature/foundation-bsn-assets`
- Engine commit expectation: Commit the Foundation BSN asset bridge, tests, documentation, and validation results inside `engine/` first.
- Bound engine commit hash: `Pending; record exact engine commit after engine implementation commit exists`
- Root pointer update required: `yes, after the engine commit is created and pushed`
- Root game changes required: `yes, convert current Last Beacon Rust bsn! macro scenes into .bsn asset scenes after the engine bridge is available`

## Alternatives Considered
- Keep all BSN content Rust-authored with `bsn!`: rejected because the user requires earlier `.bsn` file asset support for levels and prefabs.
- Implement only whole-scene `.bsn` loading: rejected because the user also needs prefabs such as characters, enemies, and loot crates.
- Implement arbitrary `.bsn` data assets: rejected/deferred because the approved scope is levels and prefabs, and arbitrary data is better handled by purpose-built asset formats until Bevy's official direction is clearer.
- Implement in-place hot-reload diffing: rejected because full root replacement is simpler, deterministic, easier to validate, and acceptable to the user.
- Fork or vendor broad Bevy scene internals throughout Foundation: rejected because the bridge must remain easy to remove when upstream Bevy support lands.
- Add BSN writing/editor save support now: deferred. PR #23639 is useful later, but runtime loading and hot reload are the immediate need.

## Risks, Constraints, And Assumptions
- Bevy 0.19's BSN macro and in-progress scene APIs may not expose all internals needed for a clean external loader. Some adaptation from upstream PRs may be necessary.
- Upstream PR #23576 is still open and work-in-progress, so its API shape may diverge from the eventual official Bevy implementation.
- The hot-reload policy deliberately resets gameplay state for affected instances. This is acceptable for development/editor iteration but should be documented so runtime gameplay does not rely on state preservation.
- Despawning and replacing roots can invalidate external entity references. Foundation should document this and keep reload behavior development-focused where appropriate.
- All component and asset types used by `.bsn` files must be registered for reflection/template conversion as required by the loader.
- Parser dependencies such as `lalrpop` or `nom` may be required if Bevy does not expose a reusable parser. Dependency additions must be isolated and justified.
- The engine submodule is detached at planning time. Implementation must not edit engine files until the engine repository is on a valid feature branch.

## Open Questions
- Should `SceneSource::BsnScene { key }` interpret `key` directly as an asset path such as `levels/test.bsn`, or should Foundation maintain a catalog that maps stable scene keys to asset paths? Plan recommendation: support direct asset paths first and allow a catalog wrapper later.
- Should hot reload be enabled in all builds where Bevy's file watcher is enabled, or only in dev/editor builds? Plan recommendation: tie it to Bevy asset events and document that practical hot reload depends on the watcher/dev setup.
- Should direct prefab spawns support caller-provided instance transforms/parents that survive reload? Plan recommendation: preserve only explicit instance context such as parent and root transform if the API records it; do not preserve arbitrary mutated state.

## Documentation Expectations
- Public APIs added by this feature must have Rustdoc comments explaining that the module is a temporary Foundation BSN asset bridge.
- `engine/docs/scene-system.md` should describe `.bsn` level/prefab asset support, scene stack integration, hot-reload replacement semantics, limitations, and future removal path.
- If example assets or APIs are added, include minimal usage documentation for loading/spawning `.bsn` prefabs and scenes.
- Generated documentation must be produced before the feature is considered complete.

## Implementation Handoff Notes
- Use `gpt-5.4` for implementation.
- Never use Anthropic models.
- Before implementation edits, read `.pi/skills/feature-tracker-update/SKILL.md`, `.pi/skills/feature-plan-docs/SKILL.md`, `.pi/skills/rust-workspace-dev/SKILL.md`, `.pi/skills/rust-coding-standards/SKILL.md`, `.pi/skills/gitflow-workflow/SKILL.md`, and `.pi/skills/foundation-architecture/SKILL.md`.
- Confirm root branch is `feature/foundation-bsn-assets` and create/switch engine submodule branch `feature/foundation-bsn-assets` from engine `dev` before touching engine files.
- Keep all reusable loader/spawn/reload code inside `engine/`.
- Keep the temporary BSN bridge isolated in a dedicated module/plugin with a narrow API and clear removal comments.
- Prefer full-instance despawn/respawn hot reload over any diffing or state preservation.
- Record engine commit hash, push state, root submodule pointer update, and validation results in the tracker.

## Optional Review Focus Areas
- Use `gpt-5.5` for review.
- Confirm the implementation is removable once official Bevy `.bsn` asset support lands.
- Confirm hot reload fully removes old roots/children before replacement and does not leak entities.
- Confirm scene-stack ownership and direct prefab ownership are coherent.
- Confirm dependencies are minimal and parser/adapted-code boundaries are isolated.
- Confirm public API docs clearly describe temporary status and limitations.

## Success Criteria
- Foundation can load `.bsn` files as asset-backed ECS levels/prefabs in Bevy 0.19.
- Foundation can spawn `.bsn` content through scene-stack requests and through a reusable prefab/level spawn path.
- Current Last Beacon Rust `bsn!` macro-authored scenes are converted to `.bsn` asset scenes while preserving the existing splash/menu/credits/pause/gameplay flow.
- Spawned BSN scene content is tagged with `SceneOwner` and participates in existing Foundation cleanup/visibility behavior.
- When a loaded `.bsn` asset reloads, each live instance from that asset fully despawns its previous root entity/entities and children, then spawns fresh replacement content.
- The implementation is isolated, documented as temporary, and easy to remove or replace when Bevy ships official `.bsn` asset support.
- Engine tests and validation pass, generated docs are recorded, and the root repository records the updated engine submodule pointer.

## Testing Methodology
- Engine validation:
  - `engine/scripts/format-project.cmd`
  - `engine/scripts/lint-project.cmd`
  - `engine/scripts/test-project.cmd`
  - `engine/scripts/compile-project.cmd`
  - `engine/scripts/doc-project.cmd`
  - `engine/scripts/validate-project.cmd` when full validation is needed
- Focused engine checks:
  - `cargo test --manifest-path engine/Cargo.toml -p foundation-runtime-library`
  - `cargo check --manifest-path engine/Cargo.toml -p foundation-runtime-library`
  - `cargo doc --manifest-path engine/Cargo.toml -p foundation-runtime-library --all-features --no-deps`
- Game/root validation after the engine pointer update and Last Beacon scene conversion:
  - `scripts/validate.cmd`
  - Focused game build/checks using `--manifest-path game/Cargo.toml` as needed.
  - Manual or automated smoke validation that the converted `.bsn` scenes load through the existing scene flow.
