# Foundation BSN Assets Tracker

## Metadata
- Feature slug: `foundation-bsn-assets`
- Feature area: `multi-area`
- Primary area: `engine`
- Root branch: `feature/foundation-bsn-assets`
- Engine branch: `feature/foundation-bsn-assets`
- Root branch base verification: `Created from current root dev at afa88b96bc0bb46336620af08713347d6871d52b on 2026-07-17`
- Engine branch base verification: `Pending; engine is currently detached at b4ff3107932e177a98ae1eee626578b1f05b2be9 and must be switched/created from engine dev before implementation edits`
- Engine submodule pointer: `Currently b4ff3107932e177a98ae1eee626578b1f05b2be9; update pending after engine feature commit`
- Overall status: `Planned`
- Planning model: `gpt-5.5`
- Preferred implementation model: `gpt-5.4`
- Optional final review model: `gpt-5.5`
- Current handoff state: `Ready for user review before gpt-5.4 implementation`
- Created: `2026-07-17`
- Last updated: `2026-07-17`

## Validation Rules
- Task complete only after required validation passes and documentation generation is recorded, unless a waiver is recorded.
- Phase complete only after required validation passes, documentation generation is recorded, required commits/pushes are complete, and required user confirmation is recorded.
- Engine work must be committed inside `engine/` before the root repository commits the updated `engine` submodule pointer.
- The exact engine commit hash bound to Last Beacon must be recorded before root completion.

## Repository State
- Root commit/push state: `Pending; planning docs uncommitted on feature/foundation-bsn-assets`
- Engine commit/push state: `Pending; no engine implementation branch or commit yet`
- Root game scene conversion state: `Pending; current Rust bsn! macro scenes should be converted to .bsn assets after engine support exists`
- Root submodule pointer update: `Pending; required after engine implementation commit`
- Root pull request state: `Pending`
- Engine pull request state: `Pending`

## Phase 1: Planning And Branch Setup
**Status:** In progress  
**Goal:** Capture an approved plan/tracker and prepare valid root/engine feature branches before implementation.

### Tasks
- [x] Create root feature branch `feature/foundation-bsn-assets` from current root `dev`.
  - Status: Complete
  - Repository: `root`
  - Notes: Created at root commit `afa88b96bc0bb46336620af08713347d6871d52b`.
- [x] Research Bevy BSN asset PRs and Foundation scene-stack context.
  - Status: Complete
  - Repository: `both`
  - Notes: Reviewed Bevy PR #23576, PR #23639, merged PR #23742, root/engine manifests, engine scene-system docs, `foundation-runtime-library` scene stack code, and current Last Beacon `game/src/scenes/` Rust `bsn!` macro scene catalog.
- [x] Create planning documents.
  - Status: Complete
  - Repository: `root`
  - Notes: Created `docs/plans/foundation-bsn-assets/plan.md` and `docs/plans/foundation-bsn-assets/tracker.md`.
- [ ] User review and approval to begin implementation.
  - Status: Pending
  - Repository: `root`
  - Notes: Required before implementation edits.
- [ ] Create or switch engine submodule to `feature/foundation-bsn-assets` from engine `dev`.
  - Status: Pending
  - Repository: `engine`
  - Notes: Must happen before implementation edits; engine is currently detached at `b4ff3107932e177a98ae1eee626578b1f05b2be9`.

### Validation
- Game validation: `N/A for planning-only docs`
- Engine validation: `N/A for planning-only docs`
- Documentation generation: `Pending; required after implementation changes public docs/APIs`
- User confirmation: `Pending`

## Phase 2: Temporary Foundation BSN Asset Bridge
**Status:** Planned  
**Goal:** Add isolated Foundation runtime support for loading `.bsn` files as spawnable level/prefab assets.

### Tasks
- [ ] Add a clearly temporary BSN asset module/plugin in `foundation-runtime-library`.
  - Status: Planned
  - Repository: `engine`
  - Notes: Keep API narrow and removable once Bevy supports official `.bsn` assets.
- [ ] Implement/adapt `.bsn` asset loading consistent with Bevy PR #23576 and Bevy 0.19 public APIs.
  - Status: Planned
  - Repository: `engine`
  - Notes: Prefer `ReflectConvert`/registered conversions where available; isolate parser dependencies and adapted code.
- [ ] Represent loaded BSN content as spawnable ECS level/prefab content.
  - Status: Planned
  - Repository: `engine`
  - Notes: Scope is levels and prefabs only, not arbitrary data assets.
- [ ] Add spawn APIs suitable for direct prefabs and scene-stack content.
  - Status: Planned
  - Repository: `engine`
  - Notes: Spawns should track asset source and instance context for reload replacement.

### Validation
- Engine validation: `Pending`
- Documentation generation: `Pending`
- User confirmation: `Not required until phase handoff unless implementation discovers scope changes`

## Phase 3: Scene Stack Integration And Hot Reload Replacement
**Status:** Planned  
**Goal:** Integrate BSN asset spawning with Foundation scene ownership and implement full root/children replacement on asset reload.

### Tasks
- [ ] Connect `SceneLoadRequested` / `SceneSource::BsnScene` to BSN asset spawning.
  - Status: Planned
  - Repository: `engine`
  - Notes: Direct asset-path interpretation is the recommended first pass unless implementation finds an existing catalog pattern to preserve.
- [ ] Tag spawned scene roots and descendants with `SceneOwner`.
  - Status: Planned
  - Repository: `engine`
  - Notes: Reuse existing scene cleanup and visibility systems.
- [ ] Track live BSN instances by source asset handle/path.
  - Status: Planned
  - Repository: `engine`
  - Notes: Tracking must support both scene-stack instances and direct prefab instances.
- [ ] Replace live instances on `.bsn` asset reload.
  - Status: Planned
  - Repository: `engine`
  - Notes: Despawn previous root entity/entities and children recursively, then spawn fresh replacement content. Do not preserve arbitrary gameplay-mutated state.
- [ ] Add automated tests for cleanup/replacement behavior.
  - Status: Planned
  - Repository: `engine`
  - Notes: Tests should prove old roots/children are removed and new roots/children are created for reload.

### Validation
- Engine validation: `Pending`
- Documentation generation: `Pending`
- User confirmation: `Not required until phase handoff unless implementation discovers scope changes`


## Phase 4: Convert Last Beacon BSN Macro Scenes To Asset Scenes
**Status:** Planned  
**Goal:** Convert the current Last Beacon Rust `bsn!` macro-authored scene catalog into full `.bsn` asset scenes as an end-to-end validation case for the Foundation bridge.

### Tasks
- [ ] Add game-owned `.bsn` asset files for the current Last Beacon scene flow.
  - Status: Planned
  - Repository: `root`
  - Notes: Cover splash screens, main menu, options, credits, pause menu, and sample gameplay level where static `.bsn` representation is practical.
- [ ] Replace direct Rust `bsn!` macro scene spawning with asset-backed scene loading.
  - Status: Planned
  - Repository: `root`
  - Notes: Preserve stable scene keys such as `last-beacon/main_menu` and existing scene-stack behavior.
- [ ] Keep only necessary Rust glue for behavior that cannot belong in static `.bsn` assets.
  - Status: Planned
  - Repository: `root`
  - Notes: Examples include systems, resources, runtime callbacks, or any interactions that require code.
- [ ] Use converted scenes to validate normal loading and hot-reload replacement.
  - Status: Planned
  - Repository: `both`
  - Notes: This is the main game-facing proof that the bridge works for real level/prefab content.

### Validation
- Game validation: `Pending`
- Engine validation: `Pending if conversion reveals engine issues`
- Documentation generation: `Pending`
- User confirmation: `Not required until phase handoff unless conversion exposes scope changes`

## Phase 5: Documentation, Validation, Commits, And Root Pointer Update
**Status:** Planned  
**Goal:** Document the temporary BSN bridge, validate engine/root behavior, commit/push both repositories, and record exact pointer state.

### Tasks
- [ ] Update engine documentation.
  - Status: Planned
  - Repository: `engine`
  - Notes: `engine/docs/scene-system.md` should explain `.bsn` level/prefab assets, hot-reload replacement semantics, limitations, and future removal.
- [ ] Run focused and/or full engine validation.
  - Status: Planned
  - Repository: `engine`
  - Notes: Use engine validation wrappers and record results.
- [ ] Commit and push engine changes.
  - Status: Planned
  - Repository: `engine`
  - Notes: Record exact engine commit hash and push/PR state.
- [ ] Update root `engine` submodule pointer and tracker.
  - Status: Planned
  - Repository: `root`
  - Notes: Commit pointer update after engine commit exists.
- [ ] Run root validation.
  - Status: Planned
  - Repository: `root`
  - Notes: Run `scripts/validate.cmd` after pointer update unless waived.
- [ ] Commit and push root changes.
  - Status: Planned
  - Repository: `root`
  - Notes: Include plan/tracker updates and `engine` pointer.

### Validation
- Engine validation: `Pending`
- Game validation: `Pending`
- Documentation generation: `Pending`
- User confirmation: `Pending before final completion`

## Implementation / Review Handoff Notes
- Use `gpt-5.4` for implementation.
- Never use Anthropic models.
- Read the mandatory workflow skills before implementation: `feature-tracker-update`, `feature-plan-docs`, `rust-workspace-dev`, `rust-coding-standards`, `gitflow-workflow`, and `foundation-architecture`.
- Do not edit engine files until `engine/` is on `feature/foundation-bsn-assets` from engine `dev`.
- Keep the implementation in `engine/`, primarily `foundation-runtime-library`.
- Convert Last Beacon root scene content from Rust `bsn!` macro scenes to game-owned `.bsn` asset scenes after the engine bridge is available.
- Hot reload must fully remove then replace root entity/entities and children for changed `.bsn` instances.
- Treat runtime state preservation, arbitrary data assets, editor save/export, and in-place diffing as postponed unless the user explicitly approves a separate scope expansion.

## Postponed Work
- Arbitrary `.bsn` data assets outside level/prefab ECS content: postponed because the approved scope is levels and prefabs.
- In-place hot-reload diffing or state preservation: postponed because full despawn/respawn is the accepted compromise.
- BSN writing/editor save support from Bevy PR #23639: postponed until runtime loading and hot reload are stable.
- A full scene/prefab catalog system: postponed unless direct asset-path use proves insufficient; the Last Beacon conversion may use a minimal key-to-asset mapping if needed to preserve current scene keys.

## Notes / Issues / Oversights
- Engine submodule is detached at planning time; branch setup is mandatory before implementation.
- Upstream Bevy dynamic BSN support is not finalized, so adapted code must stay isolated and replaceable.
- Reload replacement can invalidate entity references and reset gameplay state; this is expected and must be documented.

## Progress Log
- `2026-07-17`: User approved planning for Foundation `.bsn` level/prefab asset support with full root/children replacement on hot reload.
- `2026-07-17`: Created root branch `feature/foundation-bsn-assets` from current root `dev`.
- `2026-07-17`: Created plan and tracker under `docs/plans/foundation-bsn-assets/`.
- `2026-07-17`: Updated plan/tracker to include conversion of current Last Beacon Rust `bsn!` macro scenes into `.bsn` asset scenes as an end-to-end test of the system.
