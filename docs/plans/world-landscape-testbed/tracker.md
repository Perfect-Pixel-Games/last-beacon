# World Landscape Testbed Tracker

## Metadata
- Feature slug: `world-landscape-testbed`
- Feature area: `game`
- Primary area: `game`
- Root branch: `feature/world-landscape-testbed`
- Engine branch: `N/A`
- Root branch base verification: `Verified` (created from `dev` at commit `0cebbb1`)
- Engine branch base verification: `N/A`
- Engine submodule pointer: `01f0cfaaebfe8e193096994642ac2da1848ded9a` (unchanged)
- Overall status: `Planned`
- Planning model: `gpt-5.5` (role fulfilled by Claude Sonnet 5)
- Preferred implementation model: `gpt-5.4` (role fulfilled by Claude Sonnet 5)
- Optional final review model: `gpt-5.5` (role fulfilled by Claude Sonnet 5)
- Current handoff state: `Ready for implementation`
- Created: `2026-09-12`
- Last updated: `2026-09-12`

## Validation Rules
- Task complete only after required validation passes and documentation generation is recorded, unless a waiver is recorded.
- Phase complete only after required validation passes, documentation generation is recorded, required commits/pushes are complete, and required user confirmation is recorded.

## Repository State
- Root commit/push state: `Pending`
- Engine commit/push state: `N/A`
- Root submodule pointer update: `N/A`

## Phase 1: Terrain generation
**Status:** Planned
**Goal:** A standalone function that builds a 5000m x 5000m, 512x512-vertex Bevy `Mesh` from 8-octave Perlin (`Fbm<Perlin>`) noise, with computed normals and height/slope-blended vertex colors.

### Tasks
- [ ] Add the `noise` crate to `game/Cargo.toml` (confirm latest compatible version on crates.io first)
  - Status: Planned
  - Repository: `root`
  - Notes: None
- [ ] Implement the terrain-mesh-generation function (grid, noise sampling, normals, vertex colors) in `game/src/world/`
  - Status: Planned
  - Repository: `root`
  - Notes: Tune noise frequency/persistence/lacunarity so mountain-scale (~500-1500m) features appear across the 5km domain; height range roughly 0-600m.

### Validation
- Game validation: `Pending`
- Engine validation: `N/A`
- Documentation generation: Pending
- User confirmation: Not required yet

## Phase 2: Sky, lighting, and post-processing
**Status:** Planned
**Goal:** A camera + sun + atmosphere setup rendering Bevy's native physically-based sky with tonemapping, bloom, and SSAO active.

### Tasks
- [ ] Add `bevy_pbr::AtmospherePlugin` (and SSAO plugin if not already implied by defaults) to `LastBeaconWorldGameplayPlugin::build`
  - Status: Planned
  - Repository: `root`
  - Notes: None
- [ ] Spawn `ScatteringMedium::default()` asset + `Atmosphere::earth(medium)` entity
  - Status: Planned
  - Repository: `root`
  - Notes: None
- [ ] Spawn sun `DirectionalLight` (warm-white, ~20,000 lux, shadows enabled, ~35-45 degree angle)
  - Status: Planned
  - Repository: `root`
  - Notes: None
- [ ] Spawn `Camera3d` with `Hdr`, `AtmosphereSettings`, `Tonemapping::TonyMcMapface`, `Bloom::default()`, `ScreenSpaceAmbientOcclusion::default()`, explicit `Msaa::Off`
  - Status: Planned
  - Repository: `root`
  - Notes: SSAO requires `Msaa::Off` or Bevy silently skips it with a warning.

### Validation
- Game validation: `Pending`
- Engine validation: `N/A`
- Documentation generation: Pending
- User confirmation: Not required yet

## Phase 3: Scene wiring and free-fly camera
**Status:** Planned
**Goal:** `GAMEPLAY_LEVEL_SCENE` spawns the new landscape/sky/camera content (replacing the placeholder cube) with correct `SceneOwner` propagation and a working free-fly camera controller, and dead placeholder-cube code is removed if unused elsewhere.

### Tasks
- [ ] Add `LastBeaconLandscapeTestScene` marker component + `Added<...>` init system in `game/src/world/`, following `initialize_last_beacon_placeholder_cube_scenes`'s `SceneOwner` propagation pattern
  - Status: Planned
  - Repository: `root`
  - Notes: None
- [ ] Add free-fly camera movement system (WASD + Space/Shift + mouse-look) registered in `LastBeaconWorldGameplayPlugin::build`
  - Status: Planned
  - Repository: `root`
  - Notes: None
- [ ] Update `game/assets/scenes/gameplay_level.bsn` to construct the new marker component instead of `LastBeaconPlaceholderCubeScene`
  - Status: Planned
  - Repository: `root`
  - Notes: Keep the existing `FoundationPauseOpener` block unchanged.
- [ ] Search workspace for other uses of `LastBeaconPlaceholderCubeScene`; remove dead placeholder-cube scaffolding from `game/src/lib.rs` if none remain
  - Status: Planned
  - Repository: `root`
  - Notes: Do not remove if still referenced elsewhere.
- [ ] Manual QA: run the game, enter the World/gameplay-level scene, confirm sky/sun/shadows/bloom/SSAO/terrain scale, free-fly traversal, and no entity leaks on scene re-entry
  - Status: Planned
  - Repository: `root`
  - Notes: None

### Validation
- Game validation: `Pending`
- Engine validation: `N/A`
- Documentation generation: Pending
- User confirmation: Pending

## Implementation / Review Handoff Notes
- None

## Postponed Work
- None

## Progress Log
- `2026-09-12`: Plan and tracker created; `feature/world-landscape-testbed` branch created from `dev`.
