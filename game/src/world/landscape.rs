//! Procedural mountain-landscape mesh generation for the temporary World testbed.
//!
//! This is throwaway gameplay-testbed content (see [`crate::world`]'s module
//! docs and `docs/plans/world-landscape-testbed/plan.md`): a single low/mid-res
//! mesh built once per scene, not a shippable terrain system. No chunking,
//! LOD, or collision -- just enough visual scale and variety for gameplay
//! systems to be tested against.
//!
//! The height field itself is a faithful port of runevision's "Fast and
//! Gorgeous Erosion Filter" Shadertoy (<https://www.shadertoy.com/view/wXcfWn>)
//! -- see [`crate::world::shader_erosion`] for the ported math and
//! [`LandscapeGenerationSettings`] for the same runtime-tunable parameters
//! the shader itself exposes.

use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};

use super::shader_erosion::{heightmap_sample, HeightmapParams};

/// Side length of the square terrain footprint, in meters.
///
/// This is also the world-space width mapped onto one full cycle of the
/// shader's `[0, 1]` UV domain, so the whole 5km terrain reproduces exactly
/// the single mountain composition the Shadertoy source shows, just scaled
/// up -- not a tiled repeat of it.
pub const LANDSCAPE_SIZE_METERS: f32 = 5000.0;

/// Number of vertices along each edge of the terrain grid.
///
/// 512x512 (~522k triangles) keeps generation cost and triangle count
/// reasonable for a one-shot build at scene-spawn time; see the plan's
/// "Alternatives Considered" section for why a single mesh at this
/// resolution was chosen over chunking.
pub const LANDSCAPE_GRID_RESOLUTION: usize = 512;

const LANDSCAPE_GRASS_COLOR: Vec3 = Vec3::new(0.24, 0.42, 0.18);
const LANDSCAPE_ROCK_COLOR: Vec3 = Vec3::new(0.38, 0.36, 0.34);
const LANDSCAPE_SNOW_COLOR: Vec3 = Vec3::new(0.92, 0.93, 0.95);

/// Runtime-tunable terrain-generation parameters, mirroring the exact knobs
/// the Shadertoy source exposes (see [`HeightmapParams`]) plus two extra
/// parameters needed only because our terrain lives in world-space meters
/// instead of the shader's normalized `[0, 1]` box: a height scale and
/// offset to map the shader's raw output onto a sensible world-space range.
///
/// Driven by the `landscape.set`/`landscape.get`/`landscape.reset` debug
/// console commands (`game/src/world/mod.rs`, `dev-tools` feature) so the
/// same exploration the shader's own animated demo does can be done
/// interactively instead.
#[derive(Resource, Clone, Copy, Debug, Reflect)]
#[reflect(Resource)]
pub struct LandscapeGenerationSettings {
    pub heightmap: HeightmapParams,
    /// Multiplies the shader's raw (roughly unit-scale) height output to
    /// produce world-space meters.
    pub world_height_scale_meters: f32,
    /// Added to the scaled height, in meters.
    pub world_height_offset_meters: f32,
    /// World height, in meters, at and below which terrain is blended toward
    /// its smoother pre-erosion shape at full [`lowland_flatten_strength`],
    /// giving low-lying terrain a plains-like look. Not a shader parameter --
    /// the shader has no height-based flattening, only slope-based erosion
    /// masking, which doesn't target "low areas" specifically.
    pub lowland_flatten_height_meters: f32,
    /// Height range, in meters, above [`lowland_flatten_height_meters`] over
    /// which the flatten blend fades back out to no flattening.
    pub lowland_flatten_range_meters: f32,
    /// Maximum blend strength toward the smoothed shape at the lowest
    /// elevations. `0.0` disables flattening; `1.0` fully replaces eroded
    /// detail with the smooth base terrain shape at those elevations (which
    /// still has its own gentle rolling variation from the base FBM, so even
    /// at `1.0` the result isn't a perfectly flat plane).
    pub lowland_flatten_strength: f32,
    /// Fraction (`0`-`1`) of this mesh's own steepest vertex slope at which
    /// the green-to-grey terrain color blend starts. `0` is perfectly flat,
    /// `1` is exactly as steep as the single steepest vertex in the mesh --
    /// this is relative to the actual terrain, not an absolute slope angle,
    /// since raw `1 - normal.y` values are small at this grid resolution and
    /// world scale (a fixed absolute threshold left almost everything green).
    pub color_rock_slope_start: f32,
    /// Fraction of the mesh's steepest slope at which the blend reaches full
    /// grey. See [`color_rock_slope_start`](Self::color_rock_slope_start).
    pub color_rock_slope_end: f32,
    /// Controls how sharply the color blends (green-to-grey by slope, and
    /// the snow cap by height) switch between colors, independent of where
    /// [`color_rock_slope_start`](Self::color_rock_slope_start)/
    /// [`color_rock_slope_end`](Self::color_rock_slope_end) place that
    /// transition. `1.0` leaves the smoothstep's natural S-curve unchanged;
    /// values above `1.0` make the transition snap more abruptly around its
    /// midpoint; values below `1.0` spread it out more gradually. Either
    /// way, the blend still starts and ends at exactly the same points --
    /// only the shape of the curve between them changes.
    pub color_blend_sharpness: f32,
    /// How much a vertex's slope shifts its *effective* height for the
    /// snow-cap blend, as a fraction of the mesh's own height range. `0.0`
    /// disables this (snow appears at exactly the same height regardless of
    /// slope -- a flat, unnaturally uniform contour band around the
    /// mountain). Positive values make steeper terrain need more elevation
    /// before showing snow, while flat ground still starts snowing at the
    /// unshifted height thresholds -- so the snowline follows the terrain's
    /// own slope variation instead of a flat band, and steep faces can stay
    /// bare much closer to the peak. Kept at or below `0.15` (`1.0` minus
    /// the snow blend's own end threshold, `0.85`), this mesh's single
    /// highest vertex is still guaranteed pure white regardless of its
    /// slope; higher values trade that guarantee for a stronger slope
    /// effect at the very top too.
    pub snow_slope_bias: f32,
}

impl Default for LandscapeGenerationSettings {
    fn default() -> Self {
        Self {
            heightmap: HeightmapParams::default(),
            world_height_scale_meters: 3000.0,
            world_height_offset_meters: 0.0,
            lowland_flatten_height_meters: 1300.0,
            lowland_flatten_range_meters: 250.0,
            lowland_flatten_strength: 0.65,
            color_rock_slope_start: 0.01,
            color_rock_slope_end: 0.3,
            color_blend_sharpness: 16.0,
            snow_slope_bias: 0.12,
        }
    }
}

/// Sets one named terrain-generation parameter to `value`.
///
/// Names use kebab-case and mirror the shader's own parameter names (see
/// Buffer A's `Heightmap` function). Returns an error naming the unknown
/// parameter rather than panicking, since this is reachable from the debug
/// console with arbitrary user input.
pub fn apply_named_parameter(
    settings: &mut LandscapeGenerationSettings,
    name: &str,
    value: f32,
) -> Result<(), String> {
    let heightmap = &mut settings.heightmap;
    match name {
        "erosion-scale" => heightmap.erosion_scale = value,
        "erosion-strength" => heightmap.erosion_strength = value,
        "erosion-gully-weight" => heightmap.erosion_gully_weight = value,
        "erosion-detail" => heightmap.erosion_detail = value,
        "erosion-rounding-ridge" => heightmap.erosion_rounding.x = value,
        "erosion-rounding-crease" => heightmap.erosion_rounding.y = value,
        "erosion-rounding-height-multiplier" => heightmap.erosion_rounding.z = value,
        "erosion-rounding-octave-multiplier" => heightmap.erosion_rounding.w = value,
        "erosion-onset-initial" => heightmap.erosion_onset.x = value,
        "erosion-onset-octave" => heightmap.erosion_onset.y = value,
        "erosion-onset-ridge-map-initial" => heightmap.erosion_onset.z = value,
        "erosion-onset-ridge-map-octave" => heightmap.erosion_onset.w = value,
        "erosion-assumed-slope-value" => heightmap.erosion_assumed_slope.x = value,
        "erosion-assumed-slope-amount" => heightmap.erosion_assumed_slope.y = value,
        "erosion-cell-scale" => heightmap.erosion_cell_scale = value,
        "erosion-normalization" => heightmap.erosion_normalization = value,
        "erosion-octaves" => heightmap.erosion_octaves = value.max(0.0).round() as u32,
        "erosion-lacunarity" => heightmap.erosion_lacunarity = value,
        "erosion-gain" => heightmap.erosion_gain = value,
        "erosion-enabled" => heightmap.erosion_enabled = value >= 0.5,
        "terrain-height-offset-value" => heightmap.terrain_height_offset.x = value,
        "terrain-height-offset-erosion-mix" => heightmap.terrain_height_offset.y = value,
        "height-frequency" => heightmap.height_frequency = value,
        "height-amplitude" => heightmap.height_amplitude = value,
        "height-octaves" => heightmap.height_octaves = value.max(0.0).round() as u32,
        "height-lacunarity" => heightmap.height_lacunarity = value,
        "height-gain" => heightmap.height_gain = value,
        "world-height-scale-meters" => settings.world_height_scale_meters = value,
        "world-height-offset-meters" => settings.world_height_offset_meters = value,
        "lowland-flatten-height-meters" => settings.lowland_flatten_height_meters = value,
        "lowland-flatten-range-meters" => settings.lowland_flatten_range_meters = value,
        "lowland-flatten-strength" => settings.lowland_flatten_strength = value,
        "color-rock-slope-start" => settings.color_rock_slope_start = value,
        "color-rock-slope-end" => settings.color_rock_slope_end = value,
        "color-blend-sharpness" => settings.color_blend_sharpness = value,
        "snow-slope-bias" => settings.snow_slope_bias = value,
        _ => return Err(format!("unknown landscape parameter '{name}'")),
    }
    Ok(())
}

/// Returns the current value of one named terrain-generation parameter. See
/// [`apply_named_parameter`] for the list of valid names.
pub fn named_parameter_value(
    settings: &LandscapeGenerationSettings,
    name: &str,
) -> Result<f32, String> {
    let heightmap = &settings.heightmap;
    let value = match name {
        "erosion-scale" => heightmap.erosion_scale,
        "erosion-strength" => heightmap.erosion_strength,
        "erosion-gully-weight" => heightmap.erosion_gully_weight,
        "erosion-detail" => heightmap.erosion_detail,
        "erosion-rounding-ridge" => heightmap.erosion_rounding.x,
        "erosion-rounding-crease" => heightmap.erosion_rounding.y,
        "erosion-rounding-height-multiplier" => heightmap.erosion_rounding.z,
        "erosion-rounding-octave-multiplier" => heightmap.erosion_rounding.w,
        "erosion-onset-initial" => heightmap.erosion_onset.x,
        "erosion-onset-octave" => heightmap.erosion_onset.y,
        "erosion-onset-ridge-map-initial" => heightmap.erosion_onset.z,
        "erosion-onset-ridge-map-octave" => heightmap.erosion_onset.w,
        "erosion-assumed-slope-value" => heightmap.erosion_assumed_slope.x,
        "erosion-assumed-slope-amount" => heightmap.erosion_assumed_slope.y,
        "erosion-cell-scale" => heightmap.erosion_cell_scale,
        "erosion-normalization" => heightmap.erosion_normalization,
        "erosion-octaves" => heightmap.erosion_octaves as f32,
        "erosion-lacunarity" => heightmap.erosion_lacunarity,
        "erosion-gain" => heightmap.erosion_gain,
        "erosion-enabled" => {
            if heightmap.erosion_enabled {
                1.0
            } else {
                0.0
            }
        }
        "terrain-height-offset-value" => heightmap.terrain_height_offset.x,
        "terrain-height-offset-erosion-mix" => heightmap.terrain_height_offset.y,
        "height-frequency" => heightmap.height_frequency,
        "height-amplitude" => heightmap.height_amplitude,
        "height-octaves" => heightmap.height_octaves as f32,
        "height-lacunarity" => heightmap.height_lacunarity,
        "height-gain" => heightmap.height_gain,
        "world-height-scale-meters" => settings.world_height_scale_meters,
        "world-height-offset-meters" => settings.world_height_offset_meters,
        "lowland-flatten-height-meters" => settings.lowland_flatten_height_meters,
        "lowland-flatten-range-meters" => settings.lowland_flatten_range_meters,
        "lowland-flatten-strength" => settings.lowland_flatten_strength,
        "color-rock-slope-start" => settings.color_rock_slope_start,
        "color-rock-slope-end" => settings.color_rock_slope_end,
        "color-blend-sharpness" => settings.color_blend_sharpness,
        "snow-slope-bias" => settings.snow_slope_bias,
        _ => return Err(format!("unknown landscape parameter '{name}'")),
    };
    Ok(value)
}

/// Builds the procedural mountain-landscape mesh for the World testbed scene.
///
/// Generates a [`LANDSCAPE_GRID_RESOLUTION`]-by-[`LANDSCAPE_GRID_RESOLUTION`]
/// vertex grid over a [`LANDSCAPE_SIZE_METERS`]-by-[`LANDSCAPE_SIZE_METERS`]
/// footprint centered on the origin, with heights from the ported erosion
/// shader, computed smooth normals, and per-vertex colors blended by height
/// and slope (grass low/flat, rock on steep slopes, snow at peaks).
pub fn build_landscape_mesh(seed: u32, settings: &LandscapeGenerationSettings) -> Mesh {
    let positions = build_landscape_positions(seed, settings);
    let triangle_indices = build_landscape_triangle_indices();
    let normals = compute_landscape_normals(&positions, &triangle_indices);
    let colors = compute_landscape_vertex_colors(&positions, &normals, settings);

    let mut landscape_mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    );
    landscape_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    landscape_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    landscape_mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    landscape_mesh.insert_indices(Indices::U32(triangle_indices));
    landscape_mesh
}

/// Returns the terrain surface height, in meters, at the given world-space
/// XZ position, for the given seed and settings.
///
/// Exposed so callers (e.g. the World scene's camera spawn) can place things
/// safely above the actual generated surface instead of guessing a fixed
/// height that might land underground.
pub fn landscape_height_at(
    seed: u32,
    world_x: f32,
    world_z: f32,
    settings: &LandscapeGenerationSettings,
) -> f32 {
    sample_landscape_height(seed, world_x, world_z, settings)
}

/// Maps a world-space seed to a deterministic offset into the (otherwise
/// infinite) shader noise field.
///
/// The shader itself has no concept of a seed -- it always samples the same
/// fixed patch of noise. This offset is a small, deliberate addition on top
/// of the faithful port so that [`LastBeaconLandscapeTestScene`]'s existing
/// `seed` field continues to produce visibly different terrain compositions,
/// by shifting which patch of the infinite noise field our one [0, 1] tile
/// samples from.
///
/// The magnitude (0.3) is deliberately *not* large: the shader's `hash`
/// function (see `shader_erosion.rs`) is a cheap `fract()`-based hash whose
/// quality collapses once its input magnitude grows much past the shader's
/// own native range. The erosion
/// filter's highest octave multiplies `p` by a frequency up to ~152 (see
/// `EROSION_SCALE`/`EROSION_CELL_SCALE`/`EROSION_LACUNARITY` in
/// [`HeightmapParams::default`]), so this offset gets amplified by up to
/// ~152x before it ever reaches `hash()`. An earlier version of this offset
/// used a magnitude of 500, which pushed `hash()`'s input into the tens of
/// thousands -- far enough to blow through `f32` precision in `hash()`'s
/// internal `fract()` and make it return a *constant* value across wide
/// swaths of the domain, collapsing all of Phacelle Noise's per-cell
/// randomization and producing visibly grid-aligned (axis-aligned) erosion
/// gullies instead of ones that follow the terrain's slope. Keeping this
/// offset at the same order of magnitude as the shader's own `[0, 1]` domain
/// keeps the worst-case `hash()` input in the same range the shader itself
/// already operates in at its own domain edges.
///
/// [`LastBeaconLandscapeTestScene`]: super::LastBeaconLandscapeTestScene
fn seed_offset(seed: u32) -> Vec2 {
    let seed_f = seed as f32;
    Vec2::new(
        (seed_f * 12.9898).sin() * 0.3,
        (seed_f * 78.233).sin() * 0.3,
    )
}

/// Maps a world-space XZ position to the shader's `[0, 1]` UV domain (see
/// [`LANDSCAPE_SIZE_METERS`]'s docs for why one tile covers the whole world).
fn world_to_shader_p(seed: u32, world_x: f32, world_z: f32) -> Vec2 {
    Vec2::new(world_x, world_z) / LANDSCAPE_SIZE_METERS + Vec2::splat(0.5) + seed_offset(seed)
}

fn sample_landscape_height(
    seed: u32,
    world_x: f32,
    world_z: f32,
    settings: &LandscapeGenerationSettings,
) -> f32 {
    let p = world_to_shader_p(seed, world_x, world_z);
    let sample = heightmap_sample(p, &settings.heightmap);
    let eroded_height =
        sample.height * settings.world_height_scale_meters + settings.world_height_offset_meters;
    let smooth_height = sample.base_height * settings.world_height_scale_meters
        + settings.world_height_offset_meters;

    // Blend toward the smoother pre-erosion shape as elevation drops below
    // `lowland_flatten_height_meters`, so low-lying terrain reads as gently
    // rolling plains instead of carrying the same erosion detail as the
    // mountains above it.
    let flatten_range = settings.lowland_flatten_range_meters.max(1e-3);
    let lowness =
        ((settings.lowland_flatten_height_meters - eroded_height) / flatten_range).clamp(0.0, 1.0);
    let blend = lowness * settings.lowland_flatten_strength.clamp(0.0, 1.0);
    eroded_height + (smooth_height - eroded_height) * blend
}

fn build_landscape_positions(seed: u32, settings: &LandscapeGenerationSettings) -> Vec<Vec3> {
    let vertex_spacing = LANDSCAPE_SIZE_METERS / (LANDSCAPE_GRID_RESOLUTION - 1) as f32;
    let half_size = LANDSCAPE_SIZE_METERS * 0.5;

    let mut positions = Vec::with_capacity(LANDSCAPE_GRID_RESOLUTION * LANDSCAPE_GRID_RESOLUTION);
    for row_index in 0..LANDSCAPE_GRID_RESOLUTION {
        for column_index in 0..LANDSCAPE_GRID_RESOLUTION {
            let world_x = column_index as f32 * vertex_spacing - half_size;
            let world_z = row_index as f32 * vertex_spacing - half_size;
            let world_y = sample_landscape_height(seed, world_x, world_z, settings);
            positions.push(Vec3::new(world_x, world_y, world_z));
        }
    }
    positions
}

fn landscape_vertex_index(row_index: usize, column_index: usize) -> u32 {
    (row_index * LANDSCAPE_GRID_RESOLUTION + column_index) as u32
}

fn build_landscape_triangle_indices() -> Vec<u32> {
    let quad_count_per_edge = LANDSCAPE_GRID_RESOLUTION - 1;
    let mut triangle_indices = Vec::with_capacity(quad_count_per_edge * quad_count_per_edge * 6);

    for row_index in 0..quad_count_per_edge {
        for column_index in 0..quad_count_per_edge {
            let top_left = landscape_vertex_index(row_index, column_index);
            let top_right = landscape_vertex_index(row_index, column_index + 1);
            let bottom_left = landscape_vertex_index(row_index + 1, column_index);
            let bottom_right = landscape_vertex_index(row_index + 1, column_index + 1);

            // Winding order matters, not just topology: this order gives an
            // upward-facing (+Y) front face under Bevy's counter-clockwise
            // front-face convention, matching the normals computed below.
            triangle_indices.extend_from_slice(&[top_left, bottom_left, top_right]);
            triangle_indices.extend_from_slice(&[top_right, bottom_left, bottom_right]);
        }
    }
    triangle_indices
}

fn compute_landscape_normals(positions: &[Vec3], triangle_indices: &[u32]) -> Vec<Vec3> {
    let mut accumulated_normals = vec![Vec3::ZERO; positions.len()];

    for triangle_vertex_indices in triangle_indices.chunks_exact(3) {
        let vertex_a = positions[triangle_vertex_indices[0] as usize];
        let vertex_b = positions[triangle_vertex_indices[1] as usize];
        let vertex_c = positions[triangle_vertex_indices[2] as usize];

        // The cross product's magnitude is proportional to triangle area, so
        // accumulating it directly area-weights the average at each vertex.
        let face_normal = (vertex_b - vertex_a).cross(vertex_c - vertex_a);

        for vertex_index in triangle_vertex_indices {
            accumulated_normals[*vertex_index as usize] += face_normal;
        }
    }

    accumulated_normals
        .into_iter()
        .map(Vec3::normalize_or_zero)
        .collect()
}

fn compute_landscape_vertex_colors(
    positions: &[Vec3],
    normals: &[Vec3],
    settings: &LandscapeGenerationSettings,
) -> Vec<[f32; 4]> {
    // Normalized against the mesh's own min/max height rather than a fixed
    // constant, since `world_height_scale_meters` is runtime-tunable and can
    // change what range of world-space heights this terrain actually spans.
    let (min_height, max_height) = positions
        .iter()
        .fold((f32::MAX, f32::MIN), |(lowest, highest), position| {
            (lowest.min(position.y), highest.max(position.y))
        });
    let height_range = (max_height - min_height).max(f32::EPSILON);

    // Likewise normalized against the mesh's own steepest vertex: at this
    // grid resolution and world scale, raw `1 - normal.y` slope values are
    // small (the default terrain's steepest vertex is only ~0.56, with the
    // median under 0.05), so fixed absolute thresholds left almost the whole
    // mesh green. Scaling relative to the actual observed maximum keeps
    // `color_rock_slope_start`/`color_rock_slope_end` meaningful (as
    // fractions of "how steep this terrain actually gets") regardless of
    // erosion/height parameters or world scale.
    let max_slope_fraction = normals
        .iter()
        .map(|normal| 1.0 - normal.y.clamp(0.0, 1.0))
        .fold(f32::MIN, f32::max)
        .max(f32::EPSILON);

    positions
        .iter()
        .zip(normals)
        .map(|(position, normal)| {
            let height_fraction = ((position.y - min_height) / height_range).clamp(0.0, 1.0);
            // 0 for a flat, upward-facing normal; approaches 1 as the surface steepens.
            let slope_fraction = 1.0 - normal.y.clamp(0.0, 1.0);
            let normalized_slope_fraction = (slope_fraction / max_slope_fraction).clamp(0.0, 1.0);

            // Base color is purely slope-driven: green on flat ground, grey on
            // steep slopes, with a smooth gradient between the two.
            let rock_fraction = sharpen_blend_fraction(
                smoothstep(
                    settings.color_rock_slope_start,
                    settings.color_rock_slope_end,
                    normalized_slope_fraction,
                ),
                settings.color_blend_sharpness,
            );
            let sloped_color = LANDSCAPE_GRASS_COLOR.lerp(LANDSCAPE_ROCK_COLOR, rock_fraction);

            // White snow caps still layer on by altitude, but steeper ground
            // needs more of it: shifting each vertex's *effective* height
            // down in proportion to its own slope means flat shoulders start
            // showing snow at the unmodified height, while steep faces need
            // to climb higher before they do too. That makes the snowline
            // follow the terrain's own (already-noisy) slope variation
            // instead of tracing a single flat, unnaturally uniform contour.
            let snow_height_penalty = normalized_slope_fraction * settings.snow_slope_bias;
            let effective_snow_height_fraction = (height_fraction - snow_height_penalty).max(0.0);
            let snow_fraction = sharpen_blend_fraction(
                smoothstep(0.65, 0.85, effective_snow_height_fraction),
                settings.color_blend_sharpness,
            );
            let blended_color = sloped_color.lerp(LANDSCAPE_SNOW_COLOR, snow_fraction);

            [blended_color.x, blended_color.y, blended_color.z, 1.0]
        })
        .collect()
}

/// Hermite smoothstep, used to blend terrain colors without hard edges.
fn smoothstep(edge0: f32, edge1: f32, value: f32) -> f32 {
    let normalized_value = ((value - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    normalized_value * normalized_value * (3.0 - 2.0 * normalized_value)
}

/// Steepens or softens a `0..=1` blend fraction around its midpoint, without
/// moving where it reaches `0` or `1` -- so callers can control how sharply
/// a blend transitions without touching whatever thresholds decided *where*
/// it transitions. `sharpness == 1.0` is the identity (no change); `> 1.0`
/// snaps the transition harder around the midpoint; `< 1.0` spreads it out
/// more gradually.
fn sharpen_blend_fraction(fraction: f32, sharpness: f32) -> f32 {
    let sharpness = sharpness.max(1e-4);
    if fraction < 0.5 {
        0.5 * (2.0 * fraction).powf(sharpness)
    } else {
        1.0 - 0.5 * (2.0 * (1.0 - fraction)).powf(sharpness)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn landscape_mesh_has_expected_vertex_and_triangle_counts() {
        let landscape_mesh = build_landscape_mesh(1337, &LandscapeGenerationSettings::default());

        let expected_vertex_count = LANDSCAPE_GRID_RESOLUTION * LANDSCAPE_GRID_RESOLUTION;
        assert_eq!(landscape_mesh.count_vertices(), expected_vertex_count);

        let expected_triangle_count =
            (LANDSCAPE_GRID_RESOLUTION - 1) * (LANDSCAPE_GRID_RESOLUTION - 1) * 2;
        let Some(Indices::U32(triangle_indices)) = landscape_mesh.indices() else {
            panic!("landscape mesh should have u32 indices");
        };
        assert_eq!(triangle_indices.len(), expected_triangle_count * 3);
    }

    #[test]
    fn landscape_heights_vary_across_the_domain_and_are_finite() {
        let settings = LandscapeGenerationSettings::default();
        let half_size = LANDSCAPE_SIZE_METERS * 0.5;
        let sample_step = LANDSCAPE_SIZE_METERS / 32.0;

        let mut minimum_height = f32::MAX;
        let mut maximum_height = f32::MIN;

        let mut sample_x = -half_size;
        while sample_x <= half_size {
            let mut sample_z = -half_size;
            while sample_z <= half_size {
                let height = sample_landscape_height(7, sample_x, sample_z, &settings);
                assert!(
                    height.is_finite(),
                    "height at ({sample_x}, {sample_z}) should be finite"
                );
                minimum_height = minimum_height.min(height);
                maximum_height = maximum_height.max(height);
                sample_z += sample_step;
            }
            sample_x += sample_step;
        }

        assert!(
            maximum_height - minimum_height > 10.0,
            "expected substantial height variation across the domain, got min={minimum_height}, max={maximum_height}"
        );
    }

    #[test]
    fn different_seeds_produce_different_terrain() {
        let settings = LandscapeGenerationSettings::default();
        let height_with_seed_one = sample_landscape_height(1, 123.0, -456.0, &settings);
        let height_with_seed_two = sample_landscape_height(2, 123.0, -456.0, &settings);
        assert_ne!(
            height_with_seed_one, height_with_seed_two,
            "different seeds should sample different regions of the noise field"
        );
    }

    #[test]
    fn landscape_normals_point_generally_upward() {
        let landscape_mesh = build_landscape_mesh(42, &LandscapeGenerationSettings::default());
        let Some(bevy::render::mesh::VertexAttributeValues::Float32x3(normals)) =
            landscape_mesh.attribute(Mesh::ATTRIBUTE_NORMAL)
        else {
            panic!("landscape mesh should have Float32x3 normals");
        };

        let upward_normal_count = normals.iter().filter(|normal| normal[1] > 0.0).count();
        assert_eq!(
            upward_normal_count,
            normals.len(),
            "every landscape normal should point at least partially upward"
        );
    }

    #[test]
    fn apply_and_read_back_named_parameters_round_trips() {
        let mut settings = LandscapeGenerationSettings::default();
        let names = [
            "erosion-scale",
            "erosion-strength",
            "erosion-gully-weight",
            "erosion-detail",
            "erosion-rounding-ridge",
            "erosion-rounding-crease",
            "erosion-rounding-height-multiplier",
            "erosion-rounding-octave-multiplier",
            "erosion-onset-initial",
            "erosion-onset-octave",
            "erosion-onset-ridge-map-initial",
            "erosion-onset-ridge-map-octave",
            "erosion-assumed-slope-value",
            "erosion-assumed-slope-amount",
            "erosion-cell-scale",
            "erosion-normalization",
            "erosion-lacunarity",
            "erosion-gain",
            "terrain-height-offset-value",
            "terrain-height-offset-erosion-mix",
            "height-frequency",
            "height-amplitude",
            "height-lacunarity",
            "height-gain",
            "world-height-scale-meters",
            "world-height-offset-meters",
            "lowland-flatten-height-meters",
            "lowland-flatten-range-meters",
            "lowland-flatten-strength",
            "color-rock-slope-start",
            "color-rock-slope-end",
            "color-blend-sharpness",
            "snow-slope-bias",
        ];

        for name in names {
            apply_named_parameter(&mut settings, name, 1.25)
                .unwrap_or_else(|error| panic!("setting '{name}' should succeed: {error}"));
            let read_back = named_parameter_value(&settings, name)
                .unwrap_or_else(|error| panic!("reading '{name}' should succeed: {error}"));
            assert_eq!(
                read_back, 1.25,
                "'{name}' should round-trip through set/get"
            );
        }

        apply_named_parameter(&mut settings, "erosion-octaves", 3.0).unwrap();
        assert_eq!(
            named_parameter_value(&settings, "erosion-octaves").unwrap(),
            3.0
        );
        apply_named_parameter(&mut settings, "height-octaves", 4.0).unwrap();
        assert_eq!(
            named_parameter_value(&settings, "height-octaves").unwrap(),
            4.0
        );

        apply_named_parameter(&mut settings, "erosion-enabled", 0.0).unwrap();
        assert_eq!(
            named_parameter_value(&settings, "erosion-enabled").unwrap(),
            0.0
        );
        apply_named_parameter(&mut settings, "erosion-enabled", 1.0).unwrap();
        assert_eq!(
            named_parameter_value(&settings, "erosion-enabled").unwrap(),
            1.0
        );
    }

    #[test]
    fn unknown_parameter_name_is_rejected() {
        let mut settings = LandscapeGenerationSettings::default();
        assert!(apply_named_parameter(&mut settings, "not-a-real-parameter", 1.0).is_err());
        assert!(named_parameter_value(&settings, "not-a-real-parameter").is_err());
    }

    #[test]
    fn lowland_flatten_strength_zero_matches_unflattened_height() {
        let settings = LandscapeGenerationSettings {
            lowland_flatten_strength: 0.0,
            ..Default::default()
        };
        let world_x = 250.0;
        let world_z = -800.0;

        let flatten_disabled = sample_landscape_height(1337, world_x, world_z, &settings);

        let p = world_to_shader_p(1337, world_x, world_z);
        let sample = heightmap_sample(p, &settings.heightmap);
        let plain_eroded = sample.height * settings.world_height_scale_meters
            + settings.world_height_offset_meters;

        assert_eq!(flatten_disabled, plain_eroded);
    }

    #[test]
    fn lowland_flatten_at_full_strength_and_range_matches_the_smooth_base_shape() {
        // Force full saturation everywhere so this test doesn't depend on
        // exactly where a given sample point's height happens to fall.
        let settings = LandscapeGenerationSettings {
            lowland_flatten_strength: 1.0,
            lowland_flatten_height_meters: 1_000_000.0,
            lowland_flatten_range_meters: 1.0,
            ..Default::default()
        };
        let world_x = 250.0;
        let world_z = -800.0;

        let flattened = sample_landscape_height(1337, world_x, world_z, &settings);

        let p = world_to_shader_p(1337, world_x, world_z);
        let sample = heightmap_sample(p, &settings.heightmap);
        let expected_smooth = sample.base_height * settings.world_height_scale_meters
            + settings.world_height_offset_meters;

        assert!(
            (flattened - expected_smooth).abs() < 1e-3,
            "flattened height {flattened} should match the smooth base shape {expected_smooth}"
        );
    }

    #[test]
    fn lowland_flatten_does_not_touch_the_highest_terrain() {
        // At default settings the flatten threshold sits well below the
        // terrain's peak heights, so the highest sampled point should be
        // identical whether flattening is enabled or not.
        let mut settings = LandscapeGenerationSettings::default();
        let half_size = LANDSCAPE_SIZE_METERS * 0.5;
        let sample_step = LANDSCAPE_SIZE_METERS / 32.0;

        let peak_height_with = |settings: &LandscapeGenerationSettings| {
            let mut peak = f32::MIN;
            let mut sample_x = -half_size;
            while sample_x <= half_size {
                let mut sample_z = -half_size;
                while sample_z <= half_size {
                    peak = peak.max(sample_landscape_height(1337, sample_x, sample_z, settings));
                    sample_z += sample_step;
                }
                sample_x += sample_step;
            }
            peak
        };

        let peak_with_flatten = peak_height_with(&settings);
        settings.lowland_flatten_strength = 0.0;
        let peak_without_flatten = peak_height_with(&settings);

        assert_eq!(peak_with_flatten, peak_without_flatten);
    }

    #[test]
    fn rock_coloring_is_visible_at_default_settings() {
        // Regression guard: raw `1 - normal.y` slope values are small at this
        // grid resolution and world scale (the default terrain's steepest
        // vertex is only ~0.56), so thresholds expressed in absolute slope
        // units left almost every vertex pure grass with no visible grey.
        // `color_rock_slope_start`/`end` are fractions of the mesh's own
        // steepest slope for exactly this reason -- this checks a meaningful
        // share of vertices actually reach a visible rock blend, not just
        // the single steepest one.
        let settings = LandscapeGenerationSettings::default();
        let positions = build_landscape_positions(1337, &settings);
        let triangle_indices = build_landscape_triangle_indices();
        let normals = compute_landscape_normals(&positions, &triangle_indices);

        let max_slope_fraction = normals
            .iter()
            .map(|normal| 1.0 - normal.y.clamp(0.0, 1.0))
            .fold(f32::MIN, f32::max)
            .max(f32::EPSILON);

        let visibly_rocky_count = normals
            .iter()
            .filter(|normal| {
                let slope_fraction = 1.0 - normal.y.clamp(0.0, 1.0);
                let normalized_slope_fraction =
                    (slope_fraction / max_slope_fraction).clamp(0.0, 1.0);
                let rock_fraction = smoothstep(
                    settings.color_rock_slope_start,
                    settings.color_rock_slope_end,
                    normalized_slope_fraction,
                );
                rock_fraction > 0.1
            })
            .count();

        assert!(
            visibly_rocky_count > normals.len() / 100,
            "expected at least 1% of vertices to show a visible rock blend, got {visibly_rocky_count}/{}",
            normals.len()
        );
    }

    #[test]
    fn sharpen_blend_fraction_is_identity_at_default_sharpness() {
        for fraction in [0.0, 0.1, 0.25, 0.5, 0.75, 0.9, 1.0] {
            let sharpened = sharpen_blend_fraction(fraction, 1.0);
            assert!(
                (sharpened - fraction).abs() < 1e-5,
                "sharpness 1.0 should leave {fraction} unchanged, got {sharpened}"
            );
        }
    }

    #[test]
    fn sharpen_blend_fraction_keeps_endpoints_fixed() {
        for sharpness in [0.1, 0.5, 1.0, 2.0, 8.0] {
            assert!(
                sharpen_blend_fraction(0.0, sharpness).abs() < 1e-5,
                "sharpness {sharpness} should not move the 0.0 endpoint"
            );
            assert!(
                (sharpen_blend_fraction(1.0, sharpness) - 1.0).abs() < 1e-5,
                "sharpness {sharpness} should not move the 1.0 endpoint"
            );
        }
    }

    #[test]
    fn higher_sharpness_pushes_the_midpoint_region_away_from_center() {
        let soft = sharpen_blend_fraction(0.25, 1.0);
        let sharp = sharpen_blend_fraction(0.25, 4.0);
        assert!(
            sharp < soft,
            "sharpness above 1.0 should pull a below-midpoint fraction closer to 0, got soft={soft} sharp={sharp}"
        );

        let soft = sharpen_blend_fraction(0.75, 1.0);
        let sharp = sharpen_blend_fraction(0.75, 4.0);
        assert!(
            sharp > soft,
            "sharpness above 1.0 should push an above-midpoint fraction closer to 1, got soft={soft} sharp={sharp}"
        );
    }

    #[test]
    fn lower_sharpness_pulls_values_toward_the_midpoint() {
        let neutral = sharpen_blend_fraction(0.25, 1.0);
        let gradual = sharpen_blend_fraction(0.25, 0.25);
        assert!(
            gradual > neutral,
            "sharpness below 1.0 should pull a below-midpoint fraction toward 0.5, got neutral={neutral} gradual={gradual}"
        );
    }

    /// Five positions/normals set up so `min`/`max` height and `max` slope
    /// are pinned to known values: index 0/1 anchor the height range at 0
    /// and 100, index 2 is a perfectly flat vertex (an exact fixed point of
    /// the rock blend, regardless of sharpness), index 3 sits at
    /// `normalized_slope_fraction` `0.15` -- squarely inside the default
    /// `color_rock_slope_start`/`color_rock_slope_end` range, so its
    /// pre-sharpen rock fraction is a non-trivial ~`0.47`, not close to
    /// either endpoint -- and index 4 is the steepest vertex in the set
    /// (`normalized_slope_fraction` is defined relative to it). All test
    /// vertices share a height of `30.0` (fraction `0.3`), far below the
    /// snow blend's own range, so the snow layer can't interfere.
    fn rock_blend_test_positions_and_normals() -> (Vec<Vec3>, Vec<Vec3>) {
        let positions = vec![
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 100.0, 0.0),
            Vec3::new(1.0, 30.0, 0.0),
            Vec3::new(2.0, 30.0, 0.0),
            Vec3::new(3.0, 50.0, 0.0),
        ];
        let normals = vec![
            Vec3::Y,
            Vec3::Y,
            Vec3::Y,
            Vec3::new(0.38, 0.925, 0.0),
            Vec3::new(0.866, 0.5, 0.0),
        ];
        (positions, normals)
    }

    #[test]
    fn color_blend_sharpness_leaves_a_perfectly_flat_vertex_pure_grass() {
        let (positions, normals) = rock_blend_test_positions_and_normals();
        let pure_grass = [
            LANDSCAPE_GRASS_COLOR.x,
            LANDSCAPE_GRASS_COLOR.y,
            LANDSCAPE_GRASS_COLOR.z,
            1.0,
        ];

        for sharpness in [0.25, 1.0, 6.0, 16.0] {
            let settings = LandscapeGenerationSettings {
                color_blend_sharpness: sharpness,
                ..Default::default()
            };
            let colors = compute_landscape_vertex_colors(&positions, &normals, &settings);
            assert_eq!(
                colors[2], pure_grass,
                "a perfectly flat vertex should stay pure grass at sharpness {sharpness}"
            );
        }
    }

    #[test]
    fn color_blend_sharpness_changes_a_partially_blended_vertex_color() {
        let (positions, normals) = rock_blend_test_positions_and_normals();
        let low_sharpness_settings = LandscapeGenerationSettings {
            color_blend_sharpness: 1.0,
            ..Default::default()
        };
        let high_sharpness_settings = LandscapeGenerationSettings {
            color_blend_sharpness: 16.0,
            ..Default::default()
        };

        let low_colors =
            compute_landscape_vertex_colors(&positions, &normals, &low_sharpness_settings);
        let high_colors =
            compute_landscape_vertex_colors(&positions, &normals, &high_sharpness_settings);

        assert_ne!(
            low_colors[3], high_colors[3],
            "a partially rock-blended vertex's color should change with blend sharpness"
        );
    }

    /// Four positions/normals set up so `min`/`max` height and `max` slope
    /// are pinned to known values: index 0/1 anchor the height range at 0
    /// and 100, index 2 is a flat vertex at height-fraction 0.8, and index 3
    /// is a steep vertex (the steepest in the set, so its
    /// `normalized_slope_fraction` is exactly `1.0`) at the same
    /// height-fraction.
    fn snow_bias_test_positions_and_normals() -> (Vec<Vec3>, Vec<Vec3>) {
        let positions = vec![
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 100.0, 0.0),
            Vec3::new(1.0, 80.0, 0.0),
            Vec3::new(2.0, 80.0, 0.0),
        ];
        let normals = vec![
            Vec3::Y,
            Vec3::Y,
            Vec3::Y,
            Vec3::new(0.8, 0.6, 0.0).normalize(),
        ];
        (positions, normals)
    }

    /// `color_rock_slope_start`/`color_rock_slope_end` thresholds that
    /// saturate the rock blend to `1.0` for every `normalized_slope_fraction`
    /// in `0..=1`, so the flat and steep test vertices in
    /// [`snow_bias_test_positions_and_normals`] get an identical
    /// `sloped_color` (pure rock) -- isolating the snow layer as the only
    /// possible source of any color difference between them.
    fn rock_blend_saturated_to_rock_settings() -> LandscapeGenerationSettings {
        LandscapeGenerationSettings {
            color_rock_slope_start: -1.0,
            color_rock_slope_end: -0.5,
            ..Default::default()
        }
    }

    #[test]
    fn snow_slope_bias_zero_matches_pure_height_based_snow() {
        let (positions, normals) = snow_bias_test_positions_and_normals();
        let settings = LandscapeGenerationSettings {
            snow_slope_bias: 0.0,
            color_blend_sharpness: 1.0,
            ..rock_blend_saturated_to_rock_settings()
        };

        let colors = compute_landscape_vertex_colors(&positions, &normals, &settings);

        // With no slope bias, the flat and steep vertices sit at the same
        // height-fraction (0.8) and should therefore get the exact same
        // snow blend, regardless of their very different slopes.
        assert_eq!(
            colors[2], colors[3],
            "with snow_slope_bias at 0.0, slope should not affect the snow blend"
        );
    }

    #[test]
    fn steeper_terrain_needs_more_height_to_show_snow() {
        let (positions, normals) = snow_bias_test_positions_and_normals();
        let settings = LandscapeGenerationSettings {
            color_blend_sharpness: 1.0,
            snow_slope_bias: 0.12,
            ..rock_blend_saturated_to_rock_settings()
        };

        let colors = compute_landscape_vertex_colors(&positions, &normals, &settings);
        let flat_vertex_whiteness = colors[2][0] + colors[2][1] + colors[2][2];
        let steep_vertex_whiteness = colors[3][0] + colors[3][1] + colors[3][2];

        assert!(
            flat_vertex_whiteness > steep_vertex_whiteness,
            "at the same height, flatter ground should show more snow than steep ground: \
             flat={flat_vertex_whiteness}, steep={steep_vertex_whiteness}"
        );
    }
}
