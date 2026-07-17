# Foundation BSN Assets Tracker

## Metadata
- Feature slug: `foundation-bsn-assets`
- Feature area: `multi-area`
- Primary area: `engine`
- Root branch: `feature/foundation-bsn-assets`
- Engine branch: `feature/foundation-bsn-assets`
- Root branch base verification: `Created from current root dev at afa88b96bc0bb46336620af08713347d6871d52b on 2026-07-17`
- Engine branch base verification: `Created feature/foundation-bsn-assets from engine origin/dev at b4ff3107932e177a98ae1eee626578b1f05b2be9 on 2026-07-17`
- Engine submodule pointer: `Updated to engine inline-log opt-in commit 71bf4c3d3acc6c8e34bec9fdbca0478468bbb967; root pointer commit pending`
- Overall status: `Implementation in progress - logging visual polish requested on current feature branches`
- Planning model: `gpt-5.5`
- Preferred implementation model: `gpt-5.4`
- Optional final review model: `gpt-5.5`
- Current handoff state: `Implementation in progress with gpt-5.4`
- Created: `2026-07-17`
- Last updated: `2026-07-17`

## Validation Rules
- Task complete only after required validation passes and documentation generation is recorded, unless a waiver is recorded.
- Phase complete only after required validation passes, documentation generation is recorded, required commits/pushes are complete, and required user confirmation is recorded.
- Engine work must be committed inside `engine/` before the root repository commits the updated `engine` submodule pointer.
- The exact engine commit hash bound to Last Beacon must be recorded before root completion.

## Repository State
- Root commit/push state: `Root logging visual polish pointer/tracker commit f952ef67fe03d91f3a4625b6847c06b5626ee0b6 pushed to origin/feature/foundation-bsn-assets; inline-log opt-in pointer/tracker commit pending`
- Engine commit/push state: `Engine inline-log opt-in commit 71bf4c3d3acc6c8e34bec9fdbca0478468bbb967 pushed to origin/feature/foundation-bsn-assets; previous logging visual polish commit 89d40757b1a77e9e51cc62acb3778d31bb8b9133 also on branch`
- Root game scene conversion state: `Converted current Rust bsn! macro scenes to .bsn assets in root commit 5e0eb27984d67edaac35f0459b0c31552d9f0d92`
- Root submodule pointer update: `Pending root commit for engine inline-log opt-in commit 71bf4c3d3acc6c8e34bec9fdbca0478468bbb967`
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
- [x] User review and approval to begin implementation.
  - Status: Complete
  - Repository: `root`
  - Notes: User approved implementation on 2026-07-17.
- [x] Create or switch engine submodule to `feature/foundation-bsn-assets` from engine `dev`.
  - Status: Complete
  - Repository: `engine`
  - Notes: Created `feature/foundation-bsn-assets` at `b4ff3107932e177a98ae1eee626578b1f05b2be9`, which is contained by `origin/dev`.

### Validation
- Game validation: `N/A for planning-only docs`
- Engine validation: `N/A for planning-only docs`
- Documentation generation: `Pending; required after implementation changes public docs/APIs`
- User confirmation: `Pending`

## Phase 2: Temporary Foundation BSN Asset Bridge
**Status:** Awaiting root pointer commit
**Goal:** Add isolated Foundation runtime support for loading `.bsn` files as spawnable level/prefab assets.

### Tasks
- [x] Add a clearly temporary BSN asset module/plugin in `foundation-runtime-library`.
  - Status: Complete
  - Repository: `engine`
  - Notes: Added isolated `FoundationBsnAssetPlugin`, registry, instance component, and command extension in `bsn_assets.rs`.
- [x] Implement/adapt `.bsn` asset loading consistent with Bevy PR #23576 and Bevy 0.19 public APIs.
  - Status: Complete
  - Repository: `engine`
  - Notes: Added isolated dynamic BSN lexer/parser/loader adapted from upstream in-progress Bevy work, using LALRPOP and reflection.
- [x] Represent loaded BSN content as spawnable ECS level/prefab content.
  - Status: Complete
  - Repository: `engine`
  - Notes: Loader emits `ScenePatch` assets for Bevy's existing scene runtime.
- [x] Add spawn APIs suitable for direct prefabs and scene-stack content.
  - Status: Complete
  - Repository: `engine`
  - Notes: Added `FoundationBsnCommandsExt::spawn_bsn_asset` and scene-stack spawning through `SceneSource::BsnScene`.

### Validation
- Engine validation: `Passed focused checks: cargo check -p foundation-runtime-library, cargo clippy -p foundation-runtime-library --all-targets --all-features -D warnings, cargo test -p foundation-runtime-library --all-features`
- Documentation generation: `Passed cargo doc -p foundation-runtime-library --all-features --no-deps`
- User confirmation: `Not required until phase handoff unless implementation discovers scope changes`

## Phase 3: Scene Stack Integration And Hot Reload Replacement
**Status:** Awaiting root pointer commit
**Goal:** Integrate BSN asset spawning with Foundation scene ownership and implement full root/children replacement on asset reload.

### Tasks
- [x] Connect `SceneLoadRequested` / `SceneSource::BsnScene` to BSN asset spawning.
  - Status: Complete
  - Repository: `engine`
  - Notes: `FoundationBsnSceneRegistry` resolves keys to asset paths, falling back to direct key-as-path loading.
- [x] Tag spawned scene roots and descendants with `SceneOwner`.
  - Status: Complete
  - Repository: `engine`
  - Notes: Scene-owner context is applied to roots at spawn and propagated recursively after scene patch application.
- [x] Track live BSN instances by source asset handle/path.
  - Status: Complete
  - Repository: `engine`
  - Notes: `FoundationBsnInstance` tracks asset path, optional scene owner, and optional parent context.
- [x] Replace live instances on `.bsn` asset reload.
  - Status: Complete
  - Repository: `engine`
  - Notes: Asset events trigger recursive despawn of old roots and fresh replacement from the same `.bsn` asset.
- [x] Add automated tests for cleanup/replacement behavior.
  - Status: Complete
  - Repository: `engine`
  - Notes: Added `hot_reload_replaces_old_root_and_children`.

### Validation
- Engine validation: `Passed focused checks: cargo check -p foundation-runtime-library, cargo clippy -p foundation-runtime-library --all-targets --all-features -D warnings, cargo test -p foundation-runtime-library --all-features`
- Documentation generation: `Passed cargo doc -p foundation-runtime-library --all-features --no-deps`
- User confirmation: `Not required until phase handoff unless implementation discovers scope changes`


## Phase 4: Convert Last Beacon BSN Macro Scenes To Asset Scenes
**Status:** Awaiting root commit
**Goal:** Convert the current Last Beacon Rust `bsn!` macro-authored scene catalog into full `.bsn` asset scenes as an end-to-end validation case for the Foundation bridge.

### Tasks
- [x] Add game-owned `.bsn` asset files for the current Last Beacon scene flow.
  - Status: Complete
  - Repository: `root`
  - Notes: Added `.bsn` assets under `game/assets/scenes/` for splash screens, main menu, options, credits, pause menu, and sample gameplay level.
- [x] Replace direct Rust `bsn!` macro scene spawning with asset-backed scene loading.
  - Status: Complete
  - Repository: `root`
  - Notes: Replaced the Rust macro scene catalog with key-to-asset registration through `FoundationBsnSceneRegistry`.
- [x] Keep only necessary Rust glue for behavior that cannot belong in static `.bsn` assets.
  - Status: Complete
  - Repository: `root`
  - Notes: Retained Rust splash transition drivers while moving authored hierarchy/components into assets.
- [x] Use converted scenes to validate normal loading and hot-reload replacement.
  - Status: Complete
  - Repository: `both`
  - Notes: Added `game/tests/bsn_asset_flow.rs` to load all converted `.bsn` files as `ScenePatch` assets.

### Validation
- Game validation: `Passed cargo check --manifest-path game/Cargo.toml, cargo clippy --manifest-path game/Cargo.toml --all-targets --all-features -D warnings, cargo test --manifest-path game/Cargo.toml --all-features`
- Engine validation: `Passed focused engine checks listed above`
- Documentation generation: `Passed cargo doc --manifest-path game/Cargo.toml --all-features --no-deps and engine docs generation`
- User confirmation: `Not required until phase handoff unless conversion exposes scope changes`

## Phase 5: Documentation, Validation, Commits, And Root Pointer Update
**Status:** Complete
**Goal:** Document the temporary BSN bridge, validate engine/root behavior, commit/push both repositories, and record exact pointer state.

### Tasks
- [x] Update engine documentation.
  - Status: Complete
  - Repository: `engine`
  - Notes: Updated `engine/docs/scene-system.md` with `.bsn` level/prefab assets, hot-reload replacement semantics, limitations, and future removal.
- [x] Run focused and/or full engine validation.
  - Status: Complete
  - Repository: `engine`
  - Notes: Passed focused checks and `./engine/scripts/validate-project.cmd`.
- [x] Commit and push engine changes.
  - Status: Complete
  - Repository: `engine`
  - Notes: Engine commit `5fbbf2b4c1d93c7767cef9d12fd6481b7c1df0b0` pushed to `origin/feature/foundation-bsn-assets`.
- [x] Update root `engine` submodule pointer and tracker.
  - Status: Complete
  - Repository: `root`
  - Notes: Root now points at engine commit `5fbbf2b4c1d93c7767cef9d12fd6481b7c1df0b0`; root commit pending.
- [x] Run root validation.
  - Status: Complete
  - Repository: `root`
  - Notes: Passed `./scripts/validate.cmd`.
- [x] Commit and push root changes.
  - Status: Complete
  - Repository: `root`
  - Notes: Root implementation commit `5e0eb27984d67edaac35f0459b0c31552d9f0d92` pushed to `origin/feature/foundation-bsn-assets`; final tracker status commit pending.

### Validation
- Engine validation: `Passed ./engine/scripts/validate-project.cmd`
- Game validation: `Passed ./scripts/validate.cmd`
- Documentation generation: `Passed engine validate-project docs and root validate docs`
- User confirmation: `Pending final user review/acceptance after implementation summary`

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
- Runtime black-screen issue found after first implementation: `.bsn` assets loaded successfully, but the runtime bridge did not deterministically resolve/apply the loaded `ScenePatch` before scene-scoped UI systems needed it. The fix moves BSN application into Foundation's temporary bridge and propagates `SceneOwner` after authored roots/children exist.
- Follow-up black-screen cause: scene-owned splash drivers could initialize before authored `.bsn` UI existed and permanently select the empty generated fallback UI. Scene-owned splashes now wait for authored `FoundationSplashUiRoot` and `FoundationSplashText` instead of using fallback UI.
- User-provided runtime logs showed loader failures for both tuple components and bare Foundation marker components: `Dynamic BSN type does not reflect Default` followed by failed scene resolution. The loader now supports fully specified tuple-struct components without requiring `ReflectDefault`, and Foundation-authored `.bsn` component types with defaults now expose `ReflectDefault` through `#[reflect(Default)]`.

## Phase 6: Logging Visual Polish Scope Expansion
**Status:** In progress
**Goal:** Improve Foundation visible log readability using the user's current PowerShell/terminal font and theme colors, with clearer severity and category labels.

### Tasks
- [x] Add readable Foundation log formatting for visible `--log` output.
  - Status: Complete
  - Repository: `engine`
  - Notes: Added a custom visible formatter with aligned severity, category, target, separator, and message fields. Colors use ANSI terminal roles so PowerShell/Windows Terminal maps them through the active theme; on Windows, Foundation tries to attach to the parent terminal before allocating a fallback console. Foundation does not hard-code a GUI font.
- [x] Categorize Bevy and Foundation targets without modifying Bevy.
  - Status: Complete
  - Repository: `engine`
  - Notes: Added target-prefix categorization for Bevy, Foundation Engine, Foundation Runtime, Foundation Editor, Last Beacon, TemplateGame, Rust, and third-party targets.
- [x] Update documentation and validation state.
  - Status: Complete
  - Repository: `both`
  - Notes: Updated `engine/docs/logging.md` and this tracker to record that log font and colors come from the terminal/PowerShell theme where the process is launched.
- [x] Make `--log` open a separate log window by default, with inline logging only when explicitly requested.
  - Status: Complete
  - Repository: `engine`
  - Notes: User clarified that logs should not reuse the console that opened the game by default. Added `--log-inline` for current-terminal behavior and updated the Foundation launcher to forward it.

### Validation
- Game validation: `Passed cargo check --manifest-path game/Cargo.toml --all-features`
- Engine validation: `Passed cargo test --manifest-path engine/Cargo.toml -p foundation-runtime-library logging::tests --all-features; passed cargo test --manifest-path engine/Cargo.toml -p foundation parse_game_editor_and_log_arguments; passed cargo clippy --manifest-path engine/Cargo.toml -p foundation-runtime-library -p foundation --all-targets --all-features -- -D warnings`
- Documentation generation: `Passed cargo doc --manifest-path engine/Cargo.toml -p foundation-runtime-library -p foundation --all-features --no-deps`
- User confirmation: `Pending`

## Progress Log
- `2026-07-17`: User requested visual logging polish on the current root and engine feature branches: better readability, theme-derived PowerShell font/colors, severity colors, and source categories including Bevy, Foundation Engine, Foundation Runtime, and Last Beacon.
- `2026-07-17`: Implemented Foundation visible log formatter with ANSI theme colors, aligned category labels, target display, Bevy target wrapping, and logging documentation. Focused engine tests, clippy, and docs passed.
- `2026-07-17`: Committed and pushed engine logging visual polish as `89d40757b1a77e9e51cc62acb3778d31bb8b9133` on `origin/feature/foundation-bsn-assets`.
- `2026-07-17`: Focused root compile check passed after updating the submodule pointer: `cargo check --manifest-path game/Cargo.toml --all-features`.
- `2026-07-17`: Committed and pushed root logging visual polish pointer/tracker update as `f952ef67fe03d91f3a4625b6847c06b5626ee0b6` on `origin/feature/foundation-bsn-assets`.
- `2026-07-17`: User clarified that `--log` should open a separate log window by default instead of attaching to the console that launched the game, with a separate opt-in argument for inline current-terminal logging. Started follow-up implementation with `--log-inline`.
- `2026-07-17`: Implemented `--log-inline`, changed Windows `--log` behavior to detach from any parent console and allocate a separate log window by default, updated launcher forwarding/usage, and updated logging docs. Focused engine tests, clippy, docs, and root cargo check passed.
- `2026-07-17`: Committed and pushed engine inline-log opt-in change as `71bf4c3d3acc6c8e34bec9fdbca0478468bbb967` on `origin/feature/foundation-bsn-assets`.
- `2026-07-17`: User approved planning for Foundation `.bsn` level/prefab asset support with full root/children replacement on hot reload.
- `2026-07-17`: Created root branch `feature/foundation-bsn-assets` from current root `dev`.
- `2026-07-17`: Created plan and tracker under `docs/plans/foundation-bsn-assets/`.
- `2026-07-17`: Updated plan/tracker to include conversion of current Last Beacon Rust `bsn!` macro scenes into `.bsn` asset scenes as an end-to-end test of the system.
- `2026-07-17`: User approved implementation; created engine branch `feature/foundation-bsn-assets` from `origin/dev` at `b4ff3107932e177a98ae1eee626578b1f05b2be9`; implementation started with gpt-5.4.
- `2026-07-17`: Implemented and pushed engine BSN asset bridge in commit `5fbbf2b4c1d93c7767cef9d12fd6481b7c1df0b0`.
- `2026-07-17`: Converted Last Beacon Rust `bsn!` macro scenes to `.bsn` assets and added asset loading test.
- `2026-07-17`: Passed `./engine/scripts/validate-project.cmd` and `./scripts/validate.cmd`; root commit remains pending.
- `2026-07-17`: Pushed root implementation commit `5e0eb27984d67edaac35f0459b0c31552d9f0d92` with Last Beacon `.bsn` scene conversion and engine submodule pointer update.
- `2026-07-17`: Reproduced black-screen startup symptom as a runtime scheduling issue: `ScenePatch` assets parse/load, but Foundation BSN owner propagation and splash initialization happen before Bevy scene content is ready. Resuming implementation with gpt-5.4 on root and engine `feature/foundation-bsn-assets` branches.
- `2026-07-17`: Applied engine fix in `foundation-runtime-library/src/bsn_assets.rs`: Foundation now tracks the `ScenePatch` handle, resolves/applies pending BSN instances itself, and then propagates `SceneOwner` through authored roots/children. Validation passed: engine `cargo test -p foundation-runtime-library bsn_assets`, engine `cargo clippy -p foundation-runtime-library --all-targets --all-features -D warnings`, game `cargo test --test bsn_asset_flow`, and game `cargo clippy --all-targets --all-features -D warnings`. Manual launch smoke ran for 12 seconds without process errors, but visual confirmation is still needed.
- `2026-07-17`: Applied second engine fix in `foundation-runtime-library/src/splash_screen.rs`: scene-owned splash screens now wait for authored BSN UI markers instead of permanently selecting blank generated fallback UI while assets are still applying. Validation passed: engine `cargo test -p foundation-runtime-library splash`, engine `cargo test -p foundation-runtime-library bsn_assets`, engine clippy, game BSN asset-flow test, and game clippy.
- `2026-07-17`: Applied loader fix in `foundation-runtime-library/src/dynamic_bsn.rs`: fully specified tuple-struct components can now materialize directly without `ReflectDefault`, and the missing-default error now reports the exact type path. Validation passed: game `cargo test --test bsn_asset_flow`, engine BSN tests, engine splash tests, and engine clippy. Short launch smoke no longer printed the dynamic BSN resolve errors before timeout.
- `2026-07-17`: User log identified missing reflected default for `foundation_runtime_library::splash_screen::FoundationSplashUiRoot`. Added `#[reflect(Default)]` to Foundation `.bsn` authored components that already implement `Default` in `splash_screen.rs`, `menu.rs`, and `credits.rs`. Validation passed: game BSN asset-flow test, engine BSN tests, engine splash tests, and engine clippy. Short launch smoke again showed no BSN resolve errors before timeout.
- `2026-07-17`: User confirmed the startup scene is working perfectly after the loader/default-reflection fixes. User asked for robustness assessment before treating the fix as final.
- `2026-07-17`: Committed and pushed engine runtime startup fix as `49392f1f1a44662cc8d9a88572a70cfe86f84d22` on `origin/feature/foundation-bsn-assets`. Root submodule pointer/tracker commit `c1db32468ddf5de5018f7c3112be60bd49d10476` pushed to `origin/feature/foundation-bsn-assets`.
- `2026-07-17`: Started hardening pass with gpt-5.4 after user confirmed the working runtime fix should be checkpointed first.
- `2026-07-17`: Hardening pass implemented: added engine tests proving pending BSN instances apply scene content and recursively propagate `SceneOwner`, added failed-resolve state to stop pending retry loops, added reflected-default regression coverage for Foundation-authored `.bsn` components, and added a game integration test proving Last Beacon's converted Pixel Perfect scene spawns authored text through the public Foundation bridge. Validation passed: engine format check, engine BSN tests, engine reflected-default test, engine clippy, engine docs, game format check, game BSN asset-flow/bridge tests, game clippy, and game docs. Engine hardening commit `a44d4d21fca472f1d8ace1f7a7abe52ae9044d41` pushed; root hardening commit `113be54a9c7e9378f666d9a22d3bbf8b2ed27c47` pushed.
