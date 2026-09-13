# World Landscape Testbed Tracker

## Metadata
- Feature slug: `world-landscape-testbed`
- Feature area: `multi-area` (originally `game`-only; expanded during manual QA)
- Primary area: `game`
- Root branch: `feature/world-landscape-testbed`
- Engine branch: `feature/world-landscape-testbed` (created from engine `dev` at `aba3c15f057e140a208cc525cc3d48b4b4ee26df`, added during manual QA)
- Root branch base verification: `Verified` (created from `dev` at commit `0cebbb1`; rebased onto `dev` at `92e5655` after `enhanced-input-adoption` merged)
- Engine branch base verification: `Verified` (created from `origin/dev` at `aba3c15f057e140a208cc525cc3d48b4b4ee26df`)
- Engine submodule pointer: `8585b3d9d316eb541a4ea70e90ac103fb09be97a`
- Overall status: `Implemented; two more bugs found and fixed during manual QA (rotated sky, pause not stopping camera)`
- Planning model: `gpt-5.5` (role fulfilled by Claude Sonnet 5)
- Preferred implementation model: `gpt-5.4` (role fulfilled by Claude Sonnet 5)
- Optional final review model: `gpt-5.5` (role fulfilled by Claude Sonnet 5)
- Current handoff state: `Ready for gpt-5.5 sanity review, or user acceptance`
- Created: `2026-09-12`
- Last updated: `2026-09-12`

## Validation Rules
- Task complete only after required validation passes and documentation generation is recorded, unless a waiver is recorded.
- Phase complete only after required validation passes, documentation generation is recorded, required commits/pushes are complete, and required user confirmation is recorded.

## Repository State
- Root commit/push state: `Pending` (this commit)
- Engine commit/push state: `Committed and pushed` -- `8585b3d9d316eb541a4ea70e90ac103fb09be97a` on `feature/world-landscape-testbed`
- Root submodule pointer update: `Pending` (this commit)

## Phase 1: Terrain generation
**Status:** Complete
**Goal:** A standalone function that builds a 5000m x 5000m, 512x512-vertex Bevy `Mesh` from 8-octave Perlin (`Fbm<Perlin>`) noise, with computed normals and height/slope-blended vertex colors.

### Tasks
- [x] Add the `noise` crate to `game/Cargo.toml` (confirm latest compatible version on crates.io first)
  - Status: Complete
  - Repository: `root`
  - Notes: Confirmed `noise = "0.9.0"` is current on crates.io; resolves cleanly. Read the actual crate source (`Fbm<T>`, `MultiFractal`, `NoiseFn`) to confirm exact API rather than trusting a web summary.
- [x] Implement the terrain-mesh-generation function (grid, noise sampling, normals, vertex colors) in `game/src/world/`
  - Status: Complete
  - Repository: `root`
  - Notes: New `game/src/world/landscape.rs`. Base frequency `1/1000` per meter with lacunarity 2.0 gives ~1000m ridges at octave 0 down to ~7.8m at octave 8 (close to the ~9.8m vertex spacing, so no wasted sub-mesh-resolution detail). Applied a `powf(1.6)` shaping curve on top of the normalized fBm output so the result reads as peaks-and-valleys rather than uniform rolling hills -- a deliberate addition beyond the literal "8 octaves of Perlin noise" ask, in service of the "resemble mountains" requirement. Vertex colors blend grass/rock/snow via two `smoothstep` calls keyed on height and slope. Normals computed via per-face cross-product accumulation (naturally area-weighted) then normalized. Three unit tests added (vertex/triangle counts, height bounds and variation, normals point upward) -- all passing.

### Validation
- Game validation: `Passed` -- `cargo test`/`cargo clippy --all-targets --all-features -- -D warnings`/`cargo build` all clean
- Engine validation: `N/A`
- Documentation generation: `Passed` -- `cargo doc --manifest-path game/Cargo.toml --all-features --no-deps`
- User confirmation: `Not required for this phase`

## Phase 2: Sky, lighting, and post-processing
**Status:** Complete
**Goal:** A camera + sun + atmosphere setup rendering Bevy's native physically-based sky with tonemapping, bloom, and SSAO active.

### Tasks
- [x] Add `bevy_pbr::AtmospherePlugin` (and SSAO plugin if not already implied by defaults) to `LastBeaconWorldGameplayPlugin::build`
  - Status: Complete -- **no action needed**. Confirmed by reading `bevy_pbr-0.19.1/src/lib.rs`: `PbrPlugin::build` unconditionally adds both `ScreenSpaceAmbientOcclusionPlugin` and `AtmospherePlugin` (plus `ScatteringMediumPlugin`), and `PbrPlugin` is part of `DefaultPlugins`. This plan step was based on an open question in the original plan ("confirm during implementation") -- now resolved.
  - Repository: `root`
  - Notes: None
- [x] Spawn `ScatteringMedium::default()` asset + `Atmosphere::earth(medium)` entity
  - Status: Complete
  - Repository: `root`
  - Notes: New `game/src/world/environment.rs`, `spawn_landscape_sky_and_sun`. `Atmosphere` and `ScatteringMedium` are re-exported through `bevy::light` (`bevy_light`), not `bevy::pbr` as the plan's research phase assumed -- corrected import path during implementation.
- [x] Spawn sun `DirectionalLight` (warm-white, ~20,000 lux, shadows enabled, ~35-45 degree angle)
  - Status: Complete
  - Repository: `root`
  - Notes: Position `(1, 1, 0.4) * 500` looking at the origin gives ~43 degrees of elevation, matching the placeholder-cube light's existing `from_translation(...).looking_at(...)` convention in `game/src/lib.rs`.
- [x] Spawn `Camera3d` with `Hdr`, `AtmosphereSettings`, `Tonemapping::TonyMcMapface`, `Bloom::default()`, `ScreenSpaceAmbientOcclusion::default()`, explicit `Msaa::Off`
  - Status: Complete
  - Repository: `root`
  - Notes: `environment::landscape_camera_rendering_bundle()` returns this as one `impl Bundle`. Import paths confirmed empirically (`bevy::camera::Hdr`, `bevy::post_process::bloom::Bloom`, `bevy::light::{Atmosphere, atmosphere::ScatteringMedium}`, `bevy::pbr::{AtmosphereSettings, ScreenSpaceAmbientOcclusion}`, `bevy::core_pipeline::tonemapping::Tonemapping`, `bevy::render::view::Msaa`) -- all compiled correctly on the first attempt.

### Validation
- Game validation: `Passed` -- compiles clean, clippy clean (no automated test for visual rendering output; covered by manual/smoke-test QA below)
- Engine validation: `N/A`
- Documentation generation: `Passed`
- User confirmation: `Not required for this phase`

## Phase 3: Scene wiring and free-fly camera
**Status:** Complete (pending final manual QA)
**Goal:** `GAMEPLAY_LEVEL_SCENE` spawns the new landscape/sky/camera content (replacing the placeholder cube) with correct `SceneOwner` propagation and a working free-fly camera controller, and dead placeholder-cube code is removed if unused elsewhere.

### Tasks
- [x] Add `LastBeaconLandscapeTestScene` marker component + `Added<...>` init system in `game/src/world/`, following `initialize_last_beacon_placeholder_cube_scenes`'s `SceneOwner` propagation pattern
  - Status: Complete
  - Repository: `root`
  - Notes: `game/src/world/mod.rs`. Marker has a literal `seed: u32` field (default `1337`), settable from BSN. `SceneOwner` is propagated to the terrain, atmosphere, sun, and camera entities, exactly mirroring `effective_placeholder_scene_owner`'s logic (duplicated locally rather than shared, matching this codebase's existing pattern of small per-module copies over cross-module coupling for this kind of helper).
- [x] Consume Foundation's `foundation_free_fly_camera_bundle()`/`FoundationFreeFlyCameraPlugin` for the World scene's camera, instead of building a bespoke free-fly controller
  - Status: Complete
  - Repository: `root`
  - Notes: `LastBeaconWorldGameplayPlugin::build` adds `FoundationFreeFlyCameraPlugin`; the camera entity spawns with `foundation_free_fly_camera_bundle()` alongside `Camera3d` and the rendering bundle. Camera keeps identity rotation (default -Z forward) so it doesn't need to also override `FoundationFreeFlyCameraOrientation`'s yaw/pitch (which would otherwise fight a non-identity initial `Transform` rotation on the first frame). See the two bugs found during manual QA below -- both fixed -- for why the camera's Y position and `Camera` component are more involved than a plain `Camera3d::default()`.
- [x] Update `game/assets/scenes/gameplay_level.bsn` to construct the new marker component instead of `LastBeaconPlaceholderCubeScene`
  - Status: Complete
  - Repository: `root`
  - Notes: `last_beacon::world::LastBeaconLandscapeTestScene { seed: 1337 }`, `FoundationPauseOpener` block unchanged.
- [x] Search workspace for other uses of `LastBeaconPlaceholderCubeScene`; remove dead placeholder-cube scaffolding from `game/src/lib.rs` if none remain
  - Status: Complete -- **scaffolding kept, not removed**. `beacon.bsn` (red cube) and `main_menu.bsn` (green cube) both still construct `LastBeaconPlaceholderCubeScene`, confirmed by direct search of `game/assets/scenes/`. Per the plan's explicit conditional instruction, nothing was removed from `game/src/lib.rs`. Updated `game/tests/bsn_asset_flow.rs`'s `register_bsn_test_types` to also register `LastBeaconLandscapeTestScene` (that test's minimal app registers types per-BSN-file it loads, separate from the full game plugin).
  - Repository: `root`
  - Notes: None
- [x] Manual QA: run the game, enter the World/gameplay-level scene, confirm sky/sun/shadows/bloom/SSAO/terrain scale, free-fly traversal, and no entity leaks on scene re-entry
  - Status: Partially complete -- user confirmed the scene renders correctly (terrain/sky visible, horizon horizontal) after bug fixes 1-3 below. Bug 4 (pause not stopping camera) also found and fixed. Full checklist (shadows/bloom/SSAO visibility, free-fly traversal feel, scene-reentry entity leaks) not yet individually confirmed by the user.
  - Repository: `root`
  - Notes: See bugs below.

#### Bugs found and fixed during manual QA
1. **Camera spawned inside/at ground level.** The camera's fixed spawn Y (200m) happened to be almost exactly the terrain's baseline elevation at that XZ for seed 1337 (fBm's statistical center, ~0 raw noise, maps to ~198m under the height-shaping curve -- not near 0m as might be assumed). Screen was black because the camera was embedded in or grazing the mesh. Fixed by adding `landscape::landscape_height_at(seed, x, z)` (extracted the existing noise-sampling logic so it's queryable outside mesh-building) and computing the camera's spawn Y as `terrain_height_at_camera + 400.0` instead of a hardcoded constant.
2. **Persistent UI camera clearing the 3D scene every frame.** `spawn_default_camera` (`game/src/lib.rs`) keeps an always-on `Camera2d` at order 100 for UI rendering, using the default `ClearColorConfig::Default`. Per Bevy's own documented behavior, `ClearColorConfig::Default` clears the *entire* viewport to `ClearColor` on every camera that uses it, regardless of render order -- it does not automatically skip clearing for a "later" camera. Since the World scene's `Camera3d` used the default order (0), it rendered *before* the UI camera (order 100), which then cleared the screen to black and painted over it with its own (empty, for this scene) UI content. Fixed by giving the World scene's camera an explicit `Camera { order: 200, clear_color: ClearColorConfig::None, ..default() }`, so it renders *after* the UI camera and draws on top without re-clearing. Safe specifically for this scene because the sky+terrain fill the entire frame (nothing needs to show through from the otherwise-empty UI layer underneath). **This is a pre-existing architecture gap, not something introduced by this feature** -- it would affect any future scene that pairs a 3D camera with the always-present UI camera at the default order; worth a Foundation-level fix or documented convention later if more scenes need this pattern (only `beacon.bsn` and `main_menu.bsn` currently also spawn their own 3D camera, via the placeholder-cube pattern, and neither had been visually confirmed to actually render before this session either).
3. **Sky/horizon rendered rotated relative to the terrain (repository: `root`, fix location: `game/src/world/environment.rs`).** The `Atmosphere` component's docs say to spawn it with `GlobalTransform`, not `Transform` -- easy to misread. `Atmosphere` requires `GlobalTransform` and has an `on_add` hook that places the "planet center" `inner_radius` (6,360km for Earth) units below the origin automatically, so the scene sits near the sphere's surface where curvature is locally negligible. Spawning with an explicit `Transform::default()` alongside it meant Bevy's normal transform-propagation system (which treats `Transform` as authoritative for un-parented entities) immediately overwrote that hook's placement back to the world origin on the next frame. With the "planet center" collapsed to the origin instead of far below it, the atmosphere's "up" (planet-center-to-camera direction) became highly sensitive to the camera's actual XZ position instead of a stable near-vertical direction -- visibly skewing/rotating the rendered sky relative to the actual Y-up terrain. Fixed by removing the explicit `Transform` from the atmosphere entity's spawn tuple entirely, letting the `on_add` hook's placement stand undisturbed.
4. **Pausing didn't stop the free-fly camera (repository: `engine`, fix location: `foundation-runtime-library/src/free_fly_camera.rs`).** `move_foundation_free_fly_cameras` had no awareness of `FoundationPauseState` at all -- it kept applying WASD/mouse-look input regardless of pause. Since the free-fly camera lives in the same crate as Foundation's pause system (added by the already-merged `enhanced-input-adoption` feature), and other Foundation per-frame systems (`spin_foundation_entities` in `menu.rs`) already gate on `foundation_is_not_paused`, this is a genuine bug in Foundation's reusable code, not something scoped to Last Beacon. Fixed by adding `.run_if(foundation_is_not_paused)` to the movement system, and `init_resource::<FoundationPauseState>()` in `FoundationFreeFlyCameraPlugin::build` so the condition works even if `FoundationMenuPlugin` isn't also present. Added a regression test (`pausing_stops_camera_movement`). **This required an engine-side fix**, expanding this feature's area from `game`-only to `multi-area` (see plan.md).

### Validation
- Game validation: `Passed` -- `scripts/validate.cmd` and a follow-up full `cargo fmt`/`clippy`/`test` pass after bug fixes 1-3, all green
- Engine validation: `Passed` -- `cargo fmt`/`clippy -D warnings`/`test` for `foundation-runtime-library` after bug fix 4 (157 tests passed, including the new `pausing_stops_camera_movement` test)
- Documentation generation: `Passed`
- User confirmation: `Partial` -- rendering and horizon orientation confirmed; full manual QA checklist and pause-fix confirmation still outstanding

## Implementation / Review Handoff Notes
- `Atmosphere`/`ScatteringMedium` live under `bevy::light`, not `bevy::pbr` -- the plan's codebase research had the crate right (`bevy_light`) but referenced it via `bevy_pbr`'s re-export path in a couple of places; the actual working import is `bevy::light::{Atmosphere, atmosphere::ScatteringMedium}`.
- `AtmospherePlugin` and `ScreenSpaceAmbientOcclusionPlugin` do not need to be added manually -- `PbrPlugin` (part of `DefaultPlugins`) already installs both unconditionally.
- The free-fly camera's initial `Transform` rotation and `FoundationFreeFlyCameraOrientation`'s yaw/pitch must agree, or the free-fly movement system will snap the camera to whatever the (possibly default, identity) orientation is on the very first frame it runs. Placing the camera at a position where the *default* -Z-forward rotation already faces the point of interest sidesteps this without needing to hand-compute matching yaw/pitch values.
- **Any new 3D content in this game must account for the always-present `Camera2d` (order 100, `game/src/lib.rs::spawn_default_camera`) clearing the screen by default.** Either render before it and accept being drawn under any active UI (fine if the UI has transparent regions, as `beacon.bsn`/`main_menu.bsn` assume), or render after it with `order` > 100 and `ClearColorConfig::None` (what this feature does, appropriate when the 3D content fills the whole frame).
- `noise` crate note for future terrain work: raw fBm output is centered on 0 and *not* uniformly distributed -- don't assume "0 noise" means "low terrain" after a reshaping curve is applied; check the actual mapped value at any position you plan to place something (e.g. a camera or spawn point) rather than assuming.
- **Read `Atmosphere`'s doc comment literally**: spawn it with `GlobalTransform`, not `Transform`. Adding a `Transform` component (even `Transform::default()`) lets normal transform propagation fight the component's own `on_add` placement hook, silently breaking the "planet center far below the scene" assumption the sky rendering depends on for a stable "up" direction.
- Any Foundation-provided per-frame system that affects the player/camera (like the free-fly camera) should be checked against `FoundationPauseState`/`foundation_is_not_paused` before being considered feature-complete -- it's easy to add a new interactive system and forget this, since nothing fails to compile or test if it's missing.

## Postponed Work
- Foundation-level fix or documented convention for mixing 3D and UI cameras (see bug #2 above) -- deferred; this feature's targeted per-camera fix is sufficient for now, but a third scene needing the same pattern should prompt revisiting this properly rather than copying the same workaround a third time.

## Progress Log
- `2026-09-12`: Plan and tracker created; `feature/world-landscape-testbed` branch created from `dev`.
- `2026-09-12`: `enhanced-input-adoption` feature landed (engine commit `58da948199f35c8662796a6609e0804a7f86bfb8`), unblocking this plan's free-fly camera task (Phase 3). Noted the dependency and updated task guidance above; no code changed on this branch yet.
- `2026-09-12`: `enhanced-input-adoption` merged to root `dev` (`92e5655`) and engine `dev` (`aba3c15`); rebased `feature/world-landscape-testbed` onto the new `dev` (clean, no conflicts) and force-pushed. Engine submodule now checked out at `aba3c15`, confirmed `foundation-runtime-library/src/free_fly_camera.rs` is present. User approved starting implementation. Starting Phase 1.
- `2026-09-12`: Phases 1-3 implemented (terrain generation, sky/light/post-processing, scene wiring with Foundation's free-fly camera). All game validation green (`scripts/validate.cmd`: fmt/clippy/test/build/doc).
- `2026-09-12`: Manual QA smoke-testing found and fixed two bugs (camera spawning at/inside ground level; UI camera clearing the 3D scene every frame). User confirmed the scene now renders. Re-ran fmt/clippy/test after the fixes, all green. Committed (`7a383a4`) and pushed to `feature/world-landscape-testbed` (root).
- `2026-09-12`: User reported the sky/horizon rendering rotated. Root cause: `Atmosphere` spawned with an explicit `Transform`, which let normal transform propagation override the component's own default-placement hook. Fixed by removing the `Transform`. User confirmed this fixed it. User then reported pausing doesn't stop the free-fly camera -- a bug in Foundation's own `free_fly_camera` module (from the already-merged `enhanced-input-adoption` feature), requiring an engine-side fix. Created engine branch `feature/world-landscape-testbed` from `dev` at `aba3c15`, added `.run_if(foundation_is_not_paused)` plus a regression test, ran full engine validation (157 tests, clippy, fmt all green), committed (`8585b3d`) and pushed. Root submodule pointer update and root commit/push for the atmosphere fix + docs in progress.
