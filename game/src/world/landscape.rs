//! Procedural mountain-landscape mesh generation for the temporary World testbed.
//!
//! This is throwaway gameplay-testbed content (see [`crate::world`]'s module
//! docs and `docs/plans/world-landscape-testbed/plan.md`): a single low/mid-res
//! mesh built once per scene, not a shippable terrain system. No chunking,
//! LOD, or collision -- just enough visual scale and variety for gameplay
//! systems to be tested against.

use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};
use noise::{Fbm, MultiFractal, NoiseFn, Perlin};

/// Side length of the square terrain footprint, in meters.
pub const LANDSCAPE_SIZE_METERS: f32 = 5000.0;

/// Number of vertices along each edge of the terrain grid.
///
/// 512x512 (~522k triangles) keeps generation cost and triangle count
/// reasonable for a one-shot build at scene-spawn time; see the plan's
/// "Alternatives Considered" section for why a single mesh at this
/// resolution was chosen over chunking.
pub const LANDSCAPE_GRID_RESOLUTION: usize = 512;

/// Maximum terrain relief, in meters.
const LANDSCAPE_MAX_HEIGHT_METERS: f32 = 600.0;

/// Fbm octave count, matching the "8 octaves of Perlin noise" request.
const LANDSCAPE_NOISE_OCTAVES: usize = 8;

/// Base noise frequency, in cycles per meter.
///
/// Tuned so the lowest octave produces mountain-range-scale ridges (roughly
/// 1000m wavelength) across the 5km domain, landing inside the plan's
/// 500-1500m target feature size. With `LACUNARITY` doubling frequency per
/// octave, the finest (8th) octave has a ~7.8m wavelength -- close to the
/// grid's own ~9.8m vertex spacing, so no noise detail is wasted below what
/// the mesh can actually represent.
const LANDSCAPE_NOISE_BASE_FREQUENCY: f64 = 1.0 / 1000.0;
const LANDSCAPE_NOISE_LACUNARITY: f64 = 2.0;
const LANDSCAPE_NOISE_PERSISTENCE: f64 = 0.5;

/// Exponent applied to normalized height before scaling to world units.
///
/// Raw fBm output reshaped linearly into [0, 1] reads as rolling hills; a
/// mild power curve flattens valleys and sharpens peaks so the result reads
/// as "mountains" rather than uniform noise.
const LANDSCAPE_HEIGHT_SHAPING_EXPONENT: f32 = 1.6;

const LANDSCAPE_GRASS_COLOR: Vec3 = Vec3::new(0.24, 0.42, 0.18);
const LANDSCAPE_ROCK_COLOR: Vec3 = Vec3::new(0.38, 0.36, 0.34);
const LANDSCAPE_SNOW_COLOR: Vec3 = Vec3::new(0.92, 0.93, 0.95);

/// Builds the procedural mountain-landscape mesh for the World testbed scene.
///
/// Generates a [`LANDSCAPE_GRID_RESOLUTION`]-by-[`LANDSCAPE_GRID_RESOLUTION`]
/// vertex grid over a [`LANDSCAPE_SIZE_METERS`]-by-[`LANDSCAPE_SIZE_METERS`]
/// footprint centered on the origin, with heights from 8-octave Perlin fBm
/// noise, computed smooth normals, and per-vertex colors blended by height
/// and slope (grass low/flat, rock on steep slopes, snow at peaks).
pub fn build_landscape_mesh(seed: u32) -> Mesh {
    let landscape_noise = landscape_noise_generator(seed);

    let positions = build_landscape_positions(&landscape_noise);
    let triangle_indices = build_landscape_triangle_indices();
    let normals = compute_landscape_normals(&positions, &triangle_indices);
    let colors = compute_landscape_vertex_colors(&positions, &normals);

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

fn landscape_noise_generator(seed: u32) -> Fbm<Perlin> {
    Fbm::<Perlin>::new(seed)
        .set_octaves(LANDSCAPE_NOISE_OCTAVES)
        .set_frequency(LANDSCAPE_NOISE_BASE_FREQUENCY)
        .set_lacunarity(LANDSCAPE_NOISE_LACUNARITY)
        .set_persistence(LANDSCAPE_NOISE_PERSISTENCE)
}

/// Returns the terrain surface height, in meters, at the given world-space
/// XZ position, for the given seed.
///
/// Exposed so callers (e.g. the World scene's camera spawn) can place things
/// safely above the actual generated surface instead of guessing a fixed
/// height that might land underground depending on the seed.
pub fn landscape_height_at(seed: u32, world_x: f32, world_z: f32) -> f32 {
    sample_landscape_height(&landscape_noise_generator(seed), world_x, world_z)
}

fn build_landscape_positions(landscape_noise: &Fbm<Perlin>) -> Vec<Vec3> {
    let vertex_spacing = LANDSCAPE_SIZE_METERS / (LANDSCAPE_GRID_RESOLUTION - 1) as f32;
    let half_size = LANDSCAPE_SIZE_METERS * 0.5;

    let mut positions = Vec::with_capacity(LANDSCAPE_GRID_RESOLUTION * LANDSCAPE_GRID_RESOLUTION);
    for row_index in 0..LANDSCAPE_GRID_RESOLUTION {
        for column_index in 0..LANDSCAPE_GRID_RESOLUTION {
            let world_x = column_index as f32 * vertex_spacing - half_size;
            let world_z = row_index as f32 * vertex_spacing - half_size;
            let world_y = sample_landscape_height(landscape_noise, world_x, world_z);
            positions.push(Vec3::new(world_x, world_y, world_z));
        }
    }
    positions
}

fn sample_landscape_height(landscape_noise: &Fbm<Perlin>, world_x: f32, world_z: f32) -> f32 {
    let raw_noise_value = landscape_noise.get([world_x as f64, world_z as f64]) as f32;
    // Raw fBm output is in roughly [-1, 1]; reshape into [0, 1] before scaling.
    let normalized_height = (raw_noise_value * 0.5 + 0.5).clamp(0.0, 1.0);
    normalized_height.powf(LANDSCAPE_HEIGHT_SHAPING_EXPONENT) * LANDSCAPE_MAX_HEIGHT_METERS
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

fn compute_landscape_vertex_colors(positions: &[Vec3], normals: &[Vec3]) -> Vec<[f32; 4]> {
    positions
        .iter()
        .zip(normals)
        .map(|(position, normal)| {
            let height_fraction = (position.y / LANDSCAPE_MAX_HEIGHT_METERS).clamp(0.0, 1.0);
            // 0 for a flat, upward-facing normal; approaches 1 as the surface steepens.
            let slope_fraction = 1.0 - normal.y.clamp(0.0, 1.0);

            // Snow needs both altitude and a slope gentle enough to hold it.
            let snow_fraction = smoothstep(0.55, 0.75, height_fraction)
                * (1.0 - smoothstep(0.5, 0.9, slope_fraction));
            // Steep slopes read as bare rock regardless of altitude, unless snow already claimed them.
            let rock_fraction = smoothstep(0.25, 0.55, slope_fraction) * (1.0 - snow_fraction);
            let grass_fraction = (1.0 - rock_fraction - snow_fraction).max(0.0);

            let blended_color = LANDSCAPE_GRASS_COLOR * grass_fraction
                + LANDSCAPE_ROCK_COLOR * rock_fraction
                + LANDSCAPE_SNOW_COLOR * snow_fraction;
            [blended_color.x, blended_color.y, blended_color.z, 1.0]
        })
        .collect()
}

/// Hermite smoothstep, used to blend terrain colors without hard edges.
fn smoothstep(edge0: f32, edge1: f32, value: f32) -> f32 {
    let normalized_value = ((value - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    normalized_value * normalized_value * (3.0 - 2.0 * normalized_value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn landscape_mesh_has_expected_vertex_and_triangle_counts() {
        let landscape_mesh = build_landscape_mesh(1337);

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
    fn landscape_heights_vary_across_the_domain_and_stay_within_bounds() {
        let landscape_noise = Fbm::<Perlin>::new(7)
            .set_octaves(LANDSCAPE_NOISE_OCTAVES)
            .set_frequency(LANDSCAPE_NOISE_BASE_FREQUENCY)
            .set_lacunarity(LANDSCAPE_NOISE_LACUNARITY)
            .set_persistence(LANDSCAPE_NOISE_PERSISTENCE);

        let mut minimum_height = f32::MAX;
        let mut maximum_height = f32::MIN;
        let half_size = LANDSCAPE_SIZE_METERS * 0.5;
        let sample_step = LANDSCAPE_SIZE_METERS / 32.0;

        let mut sample_x = -half_size;
        while sample_x <= half_size {
            let mut sample_z = -half_size;
            while sample_z <= half_size {
                let height = sample_landscape_height(&landscape_noise, sample_x, sample_z);
                assert!(
                    (0.0..=LANDSCAPE_MAX_HEIGHT_METERS).contains(&height),
                    "height {height} out of expected [0, {LANDSCAPE_MAX_HEIGHT_METERS}] range"
                );
                minimum_height = minimum_height.min(height);
                maximum_height = maximum_height.max(height);
                sample_z += sample_step;
            }
            sample_x += sample_step;
        }

        // Mountain-scale relief should produce a wide range, not a near-flat plane.
        assert!(
            maximum_height - minimum_height > LANDSCAPE_MAX_HEIGHT_METERS * 0.3,
            "expected substantial height variation across the domain, got min={minimum_height}, max={maximum_height}"
        );
    }

    #[test]
    fn landscape_normals_point_generally_upward() {
        let landscape_mesh = build_landscape_mesh(42);
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
}
