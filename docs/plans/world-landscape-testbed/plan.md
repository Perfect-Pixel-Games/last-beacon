# World Landscape Testbed Plan

## Metadata
- Feature slug: `world-landscape-testbed`
- Feature area: `multi-area` (originally `game`; expanded during manual QA, see below)
- Primary area: `game`
- Root branch: `feature/world-landscape-testbed`
- Engine branch: `feature/world-landscape-testbed` (added during manual QA; see Submodule Plan)
- Engine submodule pointer: `8585b3d9d316eb541a4ea70e90ac103fb09be97a`
- Status: `Implemented`
- Planning model: `gpt-5.5` (role fulfilled by Claude Sonnet 5, per the user's standing instruction that Claude/subagents replace GPT in this workflow)
- Implementation model: `gpt-5.4` (role fulfilled by Claude Sonnet 5)
- Review model: `gpt-5.5` (role fulfilled by Claude Sonnet 5)
- Created: `2026-09-12`
- Last updated: `2026-09-12`

## User Request
Now that Hub/World gameplay space separation is scaffolded, the user wants the World gameplay space made more visually interesting *temporarily* so upcoming gameplay features have something realistic to test against. Specifically: a new scene with a full PBR sky, a directional light, tonemapping, and post-processing, plus a 5km x 5km landscape generated from 8 octaves of Perlin noise that resembles mountains.

Clarifying decisions made during brainstorming:
- Scene integration: replace the placeholder blue cube currently spawned by `GAMEPLAY_LEVEL_SCENE` rather than adding a separate, independently-reachable scene. This gives the previously-empty `world` module its first real content.
- Terrain mesh: a single mesh over the full 5000m x 5000m area on a 512x512 vertex grid (~9.8m spacing, ~522k triangles), generated once at scene-spawn time. No chunking/LOD -- this is a temporary gameplay testbed, not a shippable level.
- Camera: the scene ships its own minimal free-fly debug camera (WASD + mouse-look), since no gameplay player/camera controller exists yet in `hub`/`world`.
- Post-processing: Bloom and screen-space ambient occlusion (SSAO), in addition to tonemapping. Depth of field was explicitly declined (undesirable for testing gameplay across open terrain at a distance).
- Terrain shading: per-vertex colors blended by height and slope (grass low/flat, rock on steep slopes, snow at peaks), rendered through a plain `StandardMaterial` -- no texture assets or custom shader.
- No terrain collision/physics: the workspace has no physics crate yet, and the free-fly camera doesn't need it. Out of scope.

## Feature Summary
Gives the `world` gameplay module (`game/src/world/mod.rs`, currently an empty stub per the gameplay-space-separation feature) its first real content: a procedurally generated 5km x 5km mountain landscape rendered under Bevy's native physically-based atmosphere, with a sun-like directional light, HDR tonemapping, bloom, and SSAO. The existing `GAMEPLAY_LEVEL_SCENE` BSN scene currently spawns a placeholder blue cube (`LastBeaconPlaceholderCubeScene`, spawned by `initialize_last_beacon_placeholder_cube_scenes` in `game/src/lib.rs:382-449`); this feature replaces that content with a new `LastBeaconLandscapeTestScene` marker component and moves its init system into the `world` module, so the scene lives with the gameplay space it represents.

This is explicitly temporary/testbed content, not a shippable level: no chunking/LOD, no terrain collision, no biome variety beyond a height/slope color blend. It exists so gameplay systems (movement, combat, AI, etc.) have a large, visually realistic space to be tested against instead of a bare cube.

## Feature Area Classification
- Area: `multi-area` (originally `game`-only at planning time)
- Primary area: `game`
- Rationale: All affected files were originally Last Beacon-owned gameplay/scene files under `game/src/` and `game/assets/`; Bevy's native atmosphere/bloom/SSAO/tonemapping components were expected to be used as-is with no Foundation Engine runtime changes needed. During manual QA (Phase 3), a real bug was found in Foundation's own `foundation-runtime-library::free_fly_camera` (added by the separate, already-merged `enhanced-input-adoption` feature): its movement system didn't respect `FoundationPauseState`, so pausing didn't stop camera movement. Since the bug lives in Foundation's reusable code (shared by any game using the free-fly camera, not Last Beacon-specific), the fix belongs in `engine/`, expanding this feature's area to `multi-area`.

## Codebase Research
- `game/src/world/mod.rs`: `LastBeaconWorldGameplayPlugin` is currently an empty-`build` stub (per `docs/plans/gameplay-space-separation/plan.md`), already wired into `LastBeaconPlugin::build` at `game/src/lib.rs:179`. This is where the new scene's spawn system and any per-frame systems (free-fly camera movement) will be registered.
- `game/src/lib.rs:349-467`: the `LastBeaconPlaceholderCubeScene` marker component, its `initialize_last_beacon_placeholder_cube_scenes` init system (`game/src/lib.rs:382-449`), `effective_placeholder_scene_owner` (`game/src/lib.rs:451-459`), and `placeholder_cube_color` (`game/src/lib.rs:461-471`) are the direct template to follow: a marker component constructed by BSN, an `Added<T>` query system that spawns real content and propagates `SceneOwner` to every spawned entity (`game/src/lib.rs:443-447`) so Foundation's scene-stack cleanup despawns everything when the scene closes. The new landscape scene's init system must follow the same `SceneOwner` propagation pattern or spawned entities will leak past scene teardown.
- `game/src/lib.rs:139-140,207`: `LastBeaconPlaceholderCubeScene` is registered via `.register_type::<...>()` and its init system is added to the `Update` schedule inside `LastBeaconPlugin::build`. The new marker component and init system should instead be registered inside `LastBeaconWorldGameplayPlugin::build` (`game/src/world/mod.rs`), consistent with the Hub/World separation goal of keeping World-only types/systems out of the shared `LastBeaconPlugin::build`.
- `game/src/scenes/mod.rs:40,71`: `GAMEPLAY_LEVEL_SCENE` constant and its BSN registration (`registry.register_scene(GAMEPLAY_LEVEL_SCENE, "scenes/gameplay_level.bsn")`). No changes needed here -- only the BSN file's content and the spawned marker type change.
- `game/assets/scenes/gameplay_level.bsn`: currently `last_beacon::LastBeaconPlaceholderCubeScene { cube_color: "blue", cube_size: 2.0 }` plus a `FoundationPauseOpener`. Will be changed to construct the new `last_beacon::world::LastBeaconLandscapeTestScene { seed: <literal> }` (or a bare unit struct, depending on whether a configurable seed is worth exposing) alongside the unchanged `FoundationPauseOpener`. Per `[[project_bsn_grammar_no_function_calls]]`-equivalent constraint already known in this codebase, BSN can only construct registered components with literal field values -- no function calls -- so any parameters must be plain literals.
- `game/Cargo.toml:22-27`: current dependency list; Bevy `0.19.0` with `file_watcher`, `reflect_documentation`, `serialize` features (default Bevy features, which include `bevy_pbr`, `bevy_post_process`, `tonemapping_luts`, remain enabled). No noise-generation crate exists in the workspace yet.
- Confirmed by reading the vendored Bevy 0.19.1 source in the local Cargo registry cache (`~/.cargo/registry/src/.../bevy_light-0.19.1/src/atmosphere.rs`, `bevy_pbr-0.19.1/src/atmosphere/mod.rs`, `bevy_pbr-0.19.1/src/ssao/mod.rs`, `bevy_post_process-0.19.1/src/bloom/settings.rs`, `bevy_core_pipeline-0.19.1/src/tonemapping/mod.rs`):
  - `bevy_light::Atmosphere` is a full physically-based sky component (Hillaire 2020 model) with `Atmosphere::earth(medium: Handle<ScatteringMedium>)` and `ScatteringMedium::default()` (an Earth preset asset). Spawn on an entity with `GlobalTransform`; add `bevy_pbr::AtmospherePlugin` to the app; add `AtmosphereSettings` to each 3D camera that should render it.
  - `ScreenSpaceAmbientOcclusion` (`bevy_pbr::ssao`) auto-requires `DepthPrepass` + `NormalPrepass` via `#[require(...)]`, and only works with `Msaa::Off` on the camera (otherwise Bevy logs a warning and skips it) -- the camera entity must explicitly insert `Msaa::Off`.
  - `Bloom` lives in `bevy_post_process::bloom::settings` (re-exported through the Bevy prelude) and, like the atmosphere, needs an HDR-enabled camera (`bevy_camera::Hdr` component).
  - `Tonemapping` (`bevy_core_pipeline::tonemapping`) is a plain enum on the camera entity; `TonyMcMapface` is Bevy's filmic default and pairs well with a physical sky.
- No terrain/noise/heightmap code exists anywhere in the workspace (confirmed via full-workspace search for `perlin|noise|terrain|heightmap`).
- The `game` crate has no physics dependency (no `avian`/`rapier`/`bevy_xpbd` in any `Cargo.toml`), confirming terrain collision is correctly out of scope for this feature.

## External Research
No external online research was performed. All API details (Bevy `Atmosphere`/`ScatteringMedium`, `ScreenSpaceAmbientOcclusion`, `Bloom`, `Tonemapping`) were confirmed by reading the exact vendored Bevy 0.19.1 source already present in the local Cargo registry cache, which matches the workspace's pinned `bevy = "0.19.0"` dependency. The `noise` crate (noise-rs, for `Fbm<Perlin>` octave-based fractal noise) is a new dependency; its exact latest-compatible version should be confirmed against crates.io at implementation time.

## Affected Files And Systems
- `game/src/world/mod.rs`: add the `LastBeaconLandscapeTestScene` marker component, the terrain-mesh-generation function (noise sampling + mesh + vertex-color building), the sky/light/camera/post-processing spawn system, the free-fly camera movement system(s), and register all of it in `LastBeaconWorldGameplayPlugin::build` (type registration + `Startup`/`Update` systems as appropriate). Likely split into sub-modules (e.g. `world/landscape.rs`, `world/sky_camera.rs`) if `mod.rs` would otherwise grow too large -- exact split decided during implementation.
- `game/assets/scenes/gameplay_level.bsn`: swap `last_beacon::LastBeaconPlaceholderCubeScene { ... }` for the new marker component construction; keep the existing `FoundationPauseOpener` block unchanged.
- `game/src/lib.rs`: remove `LastBeaconPlaceholderCubeScene`, `SpinningCube`, `initialize_last_beacon_placeholder_cube_scenes`, `effective_placeholder_scene_owner`, `placeholder_cube_color`, and `spin_cube` (and their `register_type`/`add_systems` wiring) *only if* nothing else in the codebase still depends on the placeholder-cube scene. If any other BSN scene or test still constructs `LastBeaconPlaceholderCubeScene`, leave that scaffolding in place and only stop routing `GAMEPLAY_LEVEL_SCENE` through it. (Verify via a full-workspace search for `LastBeaconPlaceholderCubeScene` before removing.)
- `game/Cargo.toml`: add the `noise` crate dependency.
- `docs/plans/world-landscape-testbed/plan.md`, `docs/plans/world-landscape-testbed/tracker.md`: this plan and its tracker.

## Proposed Implementation Approach
1. Add the `noise` crate to `game/Cargo.toml`.
2. In `game/src/world/mod.rs` (or a new `game/src/world/landscape.rs` sub-module), write a terrain-mesh-generation function: build a 512x512 vertex grid over a 5000m x 5000m footprint, sample `Fbm<Perlin>` (8 octaves, tuned persistence/lacunarity/frequency so mountain-scale ridges of roughly 500-1500m appear across the domain) per vertex, scale to a height range of roughly 0-600m, compute vertex normals, and compute per-vertex colors by blending grass/rock/snow tones based on height and slope. Return a Bevy `Mesh` with position/normal/color attributes.
3. Add a `LastBeaconLandscapeTestScene` marker component (with a literal `seed: u32` field settable from BSN) and an `Added<LastBeaconLandscapeTestScene>` init system (following `initialize_last_beacon_placeholder_cube_scenes`'s `SceneOwner`-propagation pattern) that spawns:
   - The terrain mesh entity with a `StandardMaterial` (vertex colors, no texture).
   - A `ScatteringMedium::default()` asset handle and an `Atmosphere::earth(medium)` entity with `GlobalTransform`.
   - A sun `DirectionalLight` (warm-white color, ~20,000 lux illuminance, shadows enabled, angled ~35-45 degrees above the horizon).
   - A `Camera3d` entity with `Hdr`, `AtmosphereSettings`, `Tonemapping::TonyMcMapface`, `Bloom::default()`, `ScreenSpaceAmbientOcclusion::default()`, and an explicit `Msaa::Off`, plus a free-fly camera marker component.
4. Add `bevy_pbr::AtmospherePlugin` and `bevy_pbr::ssao::ScreenSpaceAmbientOcclusionPlugin` (if not already implied by Bevy's default plugin group -- confirm during implementation) to `LastBeaconWorldGameplayPlugin::build`.
5. Add a small free-fly camera movement system (WASD + Space/Shift for vertical + mouse-look while a mouse button is held) registered in `LastBeaconWorldGameplayPlugin::build`'s `Update` schedule.
6. Update `game/assets/scenes/gameplay_level.bsn` to construct `LastBeaconLandscapeTestScene` instead of `LastBeaconPlaceholderCubeScene`, keeping the `FoundationPauseOpener` block.
7. Search the workspace for any other use of `LastBeaconPlaceholderCubeScene`; if none remain, remove the now-dead placeholder-cube scaffolding from `game/src/lib.rs` (component, system, helpers, registrations). If other scenes still use it, leave it in place.
8. Manually run the game, navigate to the World/gameplay-level scene, and visually confirm: sky/atmosphere renders, sun casts shadows, terrain reads as mountains at the intended scale, bloom is visible around the sun, SSAO adds contact shadowing in terrain creases, and the free-fly camera can traverse the full 5km x 5km area.

## Submodule Plan
- Engine changes required: `yes` (discovered during manual QA; not anticipated at planning time)
- Engine branch: `feature/world-landscape-testbed` (created from engine `dev` at `aba3c15f057e140a208cc525cc3d48b4b4ee26df`)
- Engine commit expectation: gate `move_foundation_free_fly_cameras` (in `foundation-runtime-library::free_fly_camera`) on `foundation_is_not_paused`, matching how other Foundation per-frame systems already respect `FoundationPauseState`.
- Bound engine commit hash: `8585b3d9d316eb541a4ea70e90ac103fb09be97a`
- Root pointer update required: `yes`

## Alternatives Considered
- **Chunked terrain grid (multiple tile meshes)**: rejected for now. Would enable future per-chunk LOD/streaming/collision culling, but adds real complexity (chunk boundary seams, per-chunk asset management) for what is explicitly temporary testbed content. A single mesh at 512x512 resolution is simple and performant enough at this scale.
- **Custom procedural sky shader / HDRI skybox** instead of Bevy's native `Atmosphere`: rejected because Bevy 0.19 already ships a full physically-based atmospheric scattering implementation (Hillaire 2020) that satisfies "full PBR sky" with no custom shader work, and it integrates with time-of-day/directional-light color automatically.
- **Reusing an existing player/camera controller**: not applicable -- no such controller exists yet in `hub`/`world`, so a minimal scene-local free-fly camera is the only option that doesn't block this feature on unrelated gameplay work.
- **Texture-based terrain material** (splatmap/tiled textures) instead of per-vertex height/slope color blending: rejected as unnecessary asset/shader work for a temporary testbed; vertex colors read clearly as "mountains" at this scale with zero new texture assets.

## Risks, Constraints, And Assumptions
- Assumes Bevy 0.19's default feature set (already enabled via the workspace's plain `bevy = "0.19.0"` dependency) includes `bevy_pbr`, `bevy_post_process`, `bevy_light`, and `tonemapping_luts` without needing extra Cargo features -- confirmed by reading the vendored crate source, but must be re-verified if the `bevy` dependency's feature list changes.
- Building a 512x512-vertex mesh with 8-octave noise sampling happens once per scene-spawn on the main thread; this may cause a brief hitch on scene load. Acceptable for a temporary testbed; if it becomes noticeable, moving generation to an async task is a follow-up, not part of this feature.
- `AtmosphereSettings`/`Atmosphere` visual tuning (LUT resolution, sample counts, scattering coefficients) may need iteration after first visual pass to look convincing; the plan does not lock exact numeric constants beyond the ranges stated above.
- Removing the placeholder-cube scaffolding from `game/src/lib.rs` is conditional on no other scene depending on it; this must be re-verified at implementation time via a workspace-wide search, not assumed from this plan alone.

## Open Questions
- None outstanding; all decisions were resolved during brainstorming (see User Request section).

## Documentation Expectations
- Public APIs added by this feature (the `LastBeaconLandscapeTestScene` marker component and any newly-`pub` terrain-generation functions) must have Rustdoc comments explaining what they do and that this content is a temporary gameplay testbed, following the existing `LastBeaconPlaceholderCubeScene`/module-doc-comment style in `game/src/world/mod.rs` and `game/src/lib.rs`.
- No new `docs/` architecture documentation is expected beyond this plan; the feature is self-contained gameplay-scene content, not a reusable subsystem.
- Generated documentation (`cargo doc`) must be produced before the feature is considered complete, per the game validation defaults below.

## Implementation Handoff Notes
- Use `gpt-5.4` for implementation (role fulfilled by Claude Sonnet 5 per this project's standing instruction).
- Never use Anthropic models unless already directed otherwise by the user's standing instruction recorded above.
- Follow `initialize_last_beacon_placeholder_cube_scenes`'s `SceneOwner`-propagation pattern exactly (`game/src/lib.rs:382-449`) so the new scene's entities are correctly despawned by Foundation's scene-stack cleanup when the World scene closes.
- Keep all new types/systems inside `game/src/world/` (not the shared `LastBeaconPlugin::build` in `game/src/lib.rs`) to respect the Hub/World separation boundary from `docs/plans/gameplay-space-separation/plan.md`.
- Double-check the exact `noise` crate version on crates.io at implementation time (do not assume a version number from this plan).
- Verify no other BSN scene constructs `LastBeaconPlaceholderCubeScene` before deleting its scaffolding from `game/src/lib.rs`.

## Optional Review Focus Areas
- Use `gpt-5.5` for review (role fulfilled by Claude Sonnet 5).
- Confirm `SceneOwner` propagation actually despawns every spawned entity (terrain, atmosphere, light, camera) when leaving the World scene -- this is easy to get subtly wrong and would leak entities across scene transitions.
- Confirm the terrain mesh generation runs once (on `Added<LastBeaconLandscapeTestScene>`), not every frame.
- Sanity-check noise parameter choices (octaves/persistence/lacunarity/frequency/amplitude) actually produce mountain-scale features across 5km, not uniform noise or a single dome.

## Success Criteria
- Launching into the World/gameplay-level scene shows a 5km x 5km procedurally generated mountain landscape (visibly varied elevation, not flat or uniform) instead of the placeholder cube.
- The sky renders Bevy's native physically-based atmosphere with a visible sun; the directional light casts shadows onto the terrain.
- Tonemapping, bloom, and SSAO are all active and visually contribute (bloom around the sun/bright sky, contact shadowing in terrain creases).
- A free-fly camera can move across the full extent of the landscape via WASD + mouse-look.
- Leaving and re-entering the World scene does not leak entities (no duplicate terrain/sky/camera/light stacking up).

## Testing Methodology
- Game validation: `scripts/validate.cmd`.
- Focused checks: `cargo fmt`, `cargo clippy`, `cargo test`, and `cargo doc` using `--manifest-path game/Cargo.toml`.
- Manual QA (required, no automated visual test): run the game (`scripts/run.cmd` or equivalent), navigate into the World/gameplay-level scene, and visually confirm each item in Success Criteria above, including leaving/re-entering the scene to check for entity leaks.
- Engine validation: `N/A` (no engine changes).
