//! Faithful Rust port of runevision's "Fast and Gorgeous Erosion Filter"
//! Shadertoy shader (<https://www.shadertoy.com/view/wXcfWn>), Common and
//! Buffer A tabs, as pasted into this project directly from the shader's own
//! source.
//!
//! This is a line-for-line port, not a reinterpretation: variable names,
//! operation order, and function boundaries mirror the GLSL source so a
//! mismatch can be diffed against the original rather than re-derived from
//! scratch. That's why this file reads less idiomatically than the rest of
//! the codebase (e.g. spelling out `.x`/`.y`/`.z` instead of using Rust
//! patterns GLSL doesn't have).
//!
//! Two things are intentionally *not* ported, because they're rendering
//! concerns rather than terrain-shape concerns: tree placement (`trees` in
//! the shader's `Heightmap`) and the packed-channel output format (`pack4`).
//! [`HeightmapSample`] exposes the same underlying values (height, ridge
//! map, erosion delta, debug) as plain fields instead.

use bevy::prelude::{Reflect, Vec2, Vec3, Vec4};

const TAU: f32 = std::f32::consts::TAU;

fn fract(x: f32) -> f32 {
    x - x.floor()
}

fn fract2(v: Vec2) -> Vec2 {
    Vec2::new(fract(v.x), fract(v.y))
}

fn clamp01(x: f32) -> f32 {
    x.clamp(0.0, 1.0)
}

/// GLSL `sign`: unlike [`f32::signum`], this returns exactly `0.0` for `0.0`.
fn sign(x: f32) -> f32 {
    if x > 0.0 {
        1.0
    } else if x < 0.0 {
        -1.0
    } else {
        0.0
    }
}

/// GLSL `mix(a, b, t)` for scalars.
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// Port of the Common tab's `hash(vec2) -> vec2`.
#[allow(clippy::approx_constant)] // Exact shader constants, not stand-ins for PI-derived values.
fn hash(x: Vec2) -> Vec2 {
    const K: Vec2 = Vec2::new(0.3183099, 0.3678794);
    let x = x * K + Vec2::new(K.y, K.x);
    let scalar = fract(x.x * x.y * (x.x + x.y));
    Vec2::splat(-1.0) + 2.0 * fract2(Vec2::splat(16.0) * K * scalar)
}

/// Port of the Common tab's `noised(vec2) -> vec3` gradient noise.
/// Returns `(value, d/dx, d/dy)` packed as `(x, y, z)`.
fn noised(p: Vec2) -> Vec3 {
    let i = Vec2::new(p.x.floor(), p.y.floor());
    let f = fract2(p);

    let u = f * f * f * (f * (f * 6.0 - Vec2::splat(15.0)) + Vec2::splat(10.0));
    let du = Vec2::splat(30.0) * f * f * (f * (f - Vec2::splat(2.0)) + Vec2::splat(1.0));

    let ga = hash(i + Vec2::new(0.0, 0.0));
    let gb = hash(i + Vec2::new(1.0, 0.0));
    let gc = hash(i + Vec2::new(0.0, 1.0));
    let gd = hash(i + Vec2::new(1.0, 1.0));

    let va = ga.dot(f - Vec2::new(0.0, 0.0));
    let vb = gb.dot(f - Vec2::new(1.0, 0.0));
    let vc = gc.dot(f - Vec2::new(0.0, 1.0));
    let vd = gd.dot(f - Vec2::new(1.0, 1.0));

    let value = va + u.x * (vb - va) + u.y * (vc - va) + u.x * u.y * (va - vb - vc + vd);
    let gradient = ga
        + u.x * (gb - ga)
        + u.y * (gc - ga)
        + u.x * u.y * (ga - gb - gc + gd)
        + du * (Vec2::new(u.y, u.x) * (va - vb - vc + vd) + Vec2::new(vb, vc) - Vec2::splat(va));

    Vec3::new(value, gradient.x, gradient.y)
}

/// Port of the Buffer A tab's `PhacelleNoise`.
/// Returns `(normalized.x, normalized.y, sideDir.x, sideDir.y)`.
fn phacelle_noise(p: Vec2, norm_dir: Vec2, freq: f32, offset: f32, normalization: f32) -> Vec4 {
    let side_dir = Vec2::new(norm_dir.y, norm_dir.x) * Vec2::new(-1.0, 1.0) * freq * TAU;
    let offset = offset * TAU;

    let p_int = Vec2::new(p.x.floor(), p.y.floor());
    let p_frac = fract2(p);
    let mut phase_dir = Vec2::ZERO;
    let mut weight_sum = 0.0_f32;

    for i in -1..=2 {
        for j in -1..=2 {
            let grid_offset = Vec2::new(i as f32, j as f32);
            let grid_point = p_int + grid_offset;
            let random_offset = hash(grid_point) * 0.5;
            let vector_from_cell_point = p_frac - grid_offset - random_offset;

            let sqr_dist = vector_from_cell_point.dot(vector_from_cell_point);
            let weight = ((-sqr_dist * 2.0).exp() - 0.011_11).max(0.0);
            weight_sum += weight;

            let wave_input = vector_from_cell_point.dot(side_dir) + offset;
            phase_dir += Vec2::new(wave_input.cos(), wave_input.sin()) * weight;
        }
    }

    let interpolated = phase_dir / weight_sum;
    let magnitude = interpolated
        .dot(interpolated)
        .sqrt()
        .max(1.0 - normalization);
    let normalized = interpolated / magnitude;
    Vec4::new(normalized.x, normalized.y, side_dir.x, side_dir.y)
}

fn pow_inv(t: f32, power: f32) -> f32 {
    1.0 - (1.0 - clamp01(t)).powf(power)
}

fn ease_out(t: f32) -> f32 {
    let v = 1.0 - clamp01(t);
    1.0 - v * v
}

fn smooth_start(t: f32, smoothing: f32) -> f32 {
    if t >= smoothing {
        t - 0.5 * smoothing
    } else {
        0.5 * t * t / smoothing
    }
}

fn safe_normalize(n: Vec2) -> Vec2 {
    let l = n.length();
    if l.abs() > 1e-10 {
        n / l
    } else {
        n
    }
}

/// Erosion-filter tunable parameters: one field per shader `EROSION_*`
/// constant from the "Demonstration" section of Buffer A's `Heightmap`, so a
/// debug console can drive the exact same knobs as the Shadertoy demo.
#[derive(Clone, Copy, Debug, PartialEq, Reflect)]
pub struct ErosionParams {
    pub strength: f32,
    pub gully_weight: f32,
    pub detail: f32,
    /// `x`: ridge rounding, `y`: crease rounding, `z`: initial-height-function
    /// multiplier, `w`: per-octave multiplier.
    pub rounding: Vec4,
    /// `x`: initial-height onset, `y`: per-octave onset, `z`: ridge-map
    /// initial onset, `w`: ridge-map per-octave onset.
    pub onset: Vec4,
    /// `x`: assumed slope value, `y`: blend amount between actual and assumed slope.
    pub assumed_slope: Vec2,
    pub scale: f32,
    pub octaves: u32,
    pub lacunarity: f32,
    pub gain: f32,
    pub cell_scale: f32,
    pub normalization: f32,
}

/// Result of [`erosion_filter`], mirroring the shader's `vec4` return value
/// plus its two `out` parameters.
pub struct ErosionOutput {
    /// `heightAndSlopeDelta` (x: height delta, yz: slope delta).
    pub height_and_slope_delta: Vec3,
    pub magnitude: f32,
    pub ridge_map: f32,
    pub debug: f32,
}

/// Port of the Buffer A tab's `ErosionFilter`.
pub fn erosion_filter(
    p: Vec2,
    height_and_slope: Vec3,
    fade_target: f32,
    params: &ErosionParams,
) -> ErosionOutput {
    let mut strength = params.strength * params.scale;
    let mut fade_target = fade_target.clamp(-1.0, 1.0);

    let input_height_and_slope = height_and_slope;
    let mut height_and_slope = height_and_slope;
    let mut freq = 1.0 / (params.scale * params.cell_scale);
    let slope = Vec2::new(height_and_slope.y, height_and_slope.z);
    let slope_length = slope.length().max(1e-10);
    let mut magnitude = 0.0_f32;
    let mut rounding_mult = 1.0_f32;

    let rounding_for_input = lerp(
        params.rounding.y,
        params.rounding.x,
        clamp01(fade_target + 0.5),
    ) * params.rounding.z;
    let mut combi_mask = ease_out(smooth_start(
        slope_length * params.onset.x,
        rounding_for_input * params.onset.x,
    ));

    let mut ridge_map_combi_mask = ease_out(slope_length * params.onset.z);
    let mut ridge_map_fade_target = fade_target;

    let mut gully_slope = slope.lerp(
        slope / slope_length * params.assumed_slope.x,
        params.assumed_slope.y,
    );

    for _ in 0..params.octaves {
        let mut phacelle = phacelle_noise(
            p * freq,
            safe_normalize(gully_slope),
            params.cell_scale,
            0.25,
            params.normalization,
        );
        // phacelle.zw *= -freq
        phacelle.z *= -freq;
        phacelle.w *= -freq;
        let sloping = phacelle.y.abs();

        gully_slope +=
            Vec2::new(phacelle.z, phacelle.w) * sign(phacelle.y) * strength * params.gully_weight;

        let gullies = Vec3::new(phacelle.x, phacelle.y * phacelle.z, phacelle.y * phacelle.w);
        let faded_gullies =
            Vec3::new(fade_target, 0.0, 0.0).lerp(gullies * params.gully_weight, combi_mask);
        height_and_slope += faded_gullies * strength;
        magnitude += strength;

        fade_target = faded_gullies.x;

        let rounding_for_octave = lerp(
            params.rounding.y,
            params.rounding.x,
            clamp01(phacelle.x + 0.5),
        ) * rounding_mult;
        let new_mask = ease_out(smooth_start(
            sloping * params.onset.y,
            rounding_for_octave * params.onset.y,
        ));
        combi_mask = pow_inv(combi_mask, params.detail) * new_mask;

        ridge_map_fade_target = lerp(ridge_map_fade_target, gullies.x, ridge_map_combi_mask);
        let new_ridge_map_mask = ease_out(sloping * params.onset.w);
        ridge_map_combi_mask *= new_ridge_map_mask;

        strength *= params.gain;
        freq *= params.lacunarity;
        rounding_mult *= params.rounding.w;
    }

    let ridge_map = ridge_map_fade_target * (1.0 - ridge_map_combi_mask);
    let debug = fade_target;
    let height_and_slope_delta = height_and_slope - input_height_and_slope;

    ErosionOutput {
        height_and_slope_delta,
        magnitude,
        ridge_map,
        debug,
    }
}

/// Port of the Buffer A tab's `FractalNoise`, used for the base terrain shape.
fn fractal_noise(p: Vec2, freq: f32, octaves: u32, lacunarity: f32, gain: f32) -> Vec3 {
    let mut n = Vec3::ZERO;
    let mut nf = freq;
    let mut na = 1.0_f32;
    for _ in 0..octaves {
        n += noised(p * nf) * na * Vec3::new(1.0, nf, nf);
        na *= gain;
        nf *= lacunarity;
    }
    n
}

/// All tunable parameters from the shader's `Heightmap` function (Buffer A's
/// "Demonstration" section), minus its `#define ANIMATE_PARAMETERS` time
/// sweeps. Those exist in the original purely to showcase each parameter's
/// range over time; here the same range is meant to be explored interactively
/// via a debug console instead. Defaults match the shader's own un-animated
/// starting values.
#[derive(Clone, Copy, Debug, PartialEq, Reflect)]
pub struct HeightmapParams {
    pub erosion_scale: f32,
    pub erosion_strength: f32,
    pub erosion_gully_weight: f32,
    pub erosion_detail: f32,
    pub erosion_rounding: Vec4,
    pub erosion_onset: Vec4,
    pub erosion_assumed_slope: Vec2,
    pub erosion_cell_scale: f32,
    pub erosion_normalization: f32,
    pub erosion_octaves: u32,
    pub erosion_lacunarity: f32,
    pub erosion_gain: f32,
    /// `x`: height offset applied proportional to erosion magnitude, `y`:
    /// blend towards `-fadeTarget` (raise valleys / lower peaks) instead.
    pub terrain_height_offset: Vec2,
    pub height_frequency: f32,
    pub height_amplitude: f32,
    pub height_octaves: u32,
    pub height_lacunarity: f32,
    pub height_gain: f32,
    /// Replaces the shader's Enter-key/comparison-slider erosion toggle.
    pub erosion_enabled: bool,
}

impl Default for HeightmapParams {
    fn default() -> Self {
        Self {
            erosion_scale: 0.15,
            erosion_strength: 0.22,
            erosion_gully_weight: 0.5,
            erosion_detail: 1.5,
            erosion_rounding: Vec4::new(0.1, 0.0, 0.1, 2.0),
            erosion_onset: Vec4::new(1.25, 1.25, 2.8, 1.5),
            erosion_assumed_slope: Vec2::new(0.7, 1.0),
            erosion_cell_scale: 0.7,
            erosion_normalization: 0.5,
            erosion_octaves: 5,
            erosion_lacunarity: 2.0,
            erosion_gain: 0.5,
            terrain_height_offset: Vec2::new(-0.65, 0.0),
            height_frequency: 3.0,
            height_amplitude: 0.125,
            height_octaves: 3,
            height_lacunarity: 2.0,
            height_gain: 0.1,
            erosion_enabled: true,
        }
    }
}

/// Result of [`heightmap_sample`]: the shader's `Heightmap` output values
/// that describe terrain shape (height, ridge map, erosion delta, debug),
/// excluding the tree-placement and packed-channel rendering concerns.
pub struct HeightmapSample {
    pub height: f32,
    /// The base FBM terrain shape (the shader's `n.x`) before erosion is
    /// applied, in the same units as `height`. Not part of the shader's own
    /// output -- exposed so callers can blend toward this smoother shape
    /// (e.g. to flatten low-lying terrain into plains) without re-deriving
    /// it outside this faithful port.
    pub base_height: f32,
    pub ridge_map: f32,
    pub erosion_delta: f32,
    pub debug: f32,
}

/// Port of the Buffer A tab's `Heightmap(vec2) -> vec4`.
pub fn heightmap_sample(p: Vec2, params: &HeightmapParams) -> HeightmapSample {
    let raw = fractal_noise(
        p,
        params.height_frequency,
        params.height_octaves,
        params.height_lacunarity,
        params.height_gain,
    ) * params.height_amplitude;

    let fade_target = (raw.x / (params.height_amplitude * 0.6)).clamp(-1.0, 1.0);

    let n = raw * 0.5 + Vec3::new(0.5, 0.0, 0.0);

    let erosion_params = ErosionParams {
        strength: params.erosion_strength,
        gully_weight: params.erosion_gully_weight,
        detail: params.erosion_detail,
        rounding: params.erosion_rounding,
        onset: params.erosion_onset,
        assumed_slope: params.erosion_assumed_slope,
        scale: params.erosion_scale,
        octaves: params.erosion_octaves,
        lacunarity: params.erosion_lacunarity,
        gain: params.erosion_gain,
        cell_scale: params.erosion_cell_scale,
        normalization: params.erosion_normalization,
    };

    let mut erosion = erosion_filter(p, n, fade_target, &erosion_params);
    let mut ridge_map = erosion.ridge_map;
    if !params.erosion_enabled {
        erosion.height_and_slope_delta = Vec3::ZERO;
        erosion.magnitude = 0.0;
        ridge_map = 1.0;
    }

    let offset = lerp(
        params.terrain_height_offset.x,
        -fade_target,
        params.terrain_height_offset.y,
    ) * erosion.magnitude;
    let eroded = n.x + erosion.height_and_slope_delta.x + offset;

    HeightmapSample {
        height: eroded,
        base_height: n.x,
        ridge_map,
        erosion_delta: erosion.height_and_slope_delta.x,
        debug: erosion.debug,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_is_deterministic_and_bounded() {
        for x in [0.0_f32, 1.0, -3.5, 42.25] {
            for y in [0.0_f32, -1.0, 7.75, 100.125] {
                let point = Vec2::new(x, y);
                let first = hash(point);
                let second = hash(point);
                assert_eq!(first, second, "hash should be a pure function");
                assert!(
                    first.x >= -1.0 && first.x <= 1.0 && first.y >= -1.0 && first.y <= 1.0,
                    "hash output {first:?} should stay within [-1, 1]"
                );
            }
        }
    }

    #[test]
    fn hash_stays_varied_within_the_shaders_native_coordinate_range() {
        // `hash`'s `fract()`-based implementation loses precision as its
        // input magnitude grows: past a few hundred units it degenerates to
        // a near-constant output, which collapses PhacelleNoise's per-cell
        // randomization and produces visibly grid-aligned (not slope-following)
        // erosion gullies. The erosion filter's highest octave multiplies its
        // input by a frequency up to ~152 (see `EROSION_SCALE`/
        // `EROSION_CELL_SCALE`/`EROSION_LACUNARITY` defaults), so any caller
        // combining `p` with an additive offset before that multiplication
        // must keep the offset small enough that this stays comfortably
        // varied. This guards the coordinate range callers actually reach.
        let mut unique = std::collections::HashSet::new();
        for i in 0..10 {
            for j in 0..10 {
                let point = Vec2::new(150.0 + i as f32, 150.0 + j as f32 * 1.37);
                let h = hash(point);
                unique.insert((h.x.to_bits(), h.y.to_bits()));
            }
        }
        assert!(
            unique.len() > 10,
            "hash() should stay varied at coordinates up to ~200, got only {} unique outputs of 100 -- \
             an offset upstream is pushing hash() into its degenerate range",
            unique.len()
        );
    }

    #[test]
    fn noised_value_matches_its_own_finite_difference_gradient() {
        // The analytic derivative returned in `noised`'s yz should agree with
        // a numeric finite-difference estimate of the same value function.
        let point = Vec2::new(1.37, -2.61);
        let step = 0.0005_f32;

        let sample = noised(point);
        let dx_numeric = (noised(point + Vec2::new(step, 0.0)).x
            - noised(point - Vec2::new(step, 0.0)).x)
            / (2.0 * step);
        let dy_numeric = (noised(point + Vec2::new(0.0, step)).x
            - noised(point - Vec2::new(0.0, step)).x)
            / (2.0 * step);

        assert!(
            (sample.y - dx_numeric).abs() < 0.01,
            "analytic dx {} should match numeric dx {dx_numeric}",
            sample.y
        );
        assert!(
            (sample.z - dy_numeric).abs() < 0.01,
            "analytic dy {} should match numeric dy {dy_numeric}",
            sample.z
        );
    }

    #[test]
    fn phacelle_noise_output_is_normalized_cosine_sine_pair() {
        let result = phacelle_noise(Vec2::new(3.3, -1.1), Vec2::new(1.0, 0.0), 1.0, 0.25, 0.5);
        let magnitude = (result.x * result.x + result.y * result.y).sqrt();
        // With normalization < 1.0, the output isn't forced to exactly unit
        // length, but it should never wildly exceed it.
        assert!(
            magnitude <= 1.01,
            "phacelle noise (cos, sin) pair should stay near unit length, got {magnitude}"
        );
    }

    #[test]
    fn heightmap_sample_varies_across_the_domain() {
        let params = HeightmapParams::default();
        let mut min_height = f32::MAX;
        let mut max_height = f32::MIN;

        let mut x = 0.0_f32;
        while x <= 1.0 {
            let mut y = 0.0_f32;
            while y <= 1.0 {
                let height = heightmap_sample(Vec2::new(x, y), &params).height;
                assert!(height.is_finite(), "height at ({x}, {y}) should be finite");
                min_height = min_height.min(height);
                max_height = max_height.max(height);
                y += 0.05;
            }
            x += 0.05;
        }

        assert!(
            max_height - min_height > 0.05,
            "expected meaningful height variation, got min={min_height}, max={max_height}"
        );
    }

    #[test]
    fn disabling_erosion_removes_erosion_delta() {
        let params = HeightmapParams {
            erosion_enabled: false,
            ..Default::default()
        };

        let sample = heightmap_sample(Vec2::new(0.4, 0.6), &params);
        assert_eq!(sample.erosion_delta, 0.0);
        assert_eq!(sample.ridge_map, 1.0);
    }

    #[test]
    fn erosion_filter_magnitude_matches_geometric_series_of_strength() {
        let params = ErosionParams {
            strength: 0.2,
            gully_weight: 0.5,
            detail: 1.5,
            rounding: Vec4::new(0.1, 0.0, 0.1, 2.0),
            onset: Vec4::new(1.25, 1.25, 2.8, 1.5),
            assumed_slope: Vec2::new(0.7, 1.0),
            scale: 0.15,
            octaves: 5,
            lacunarity: 2.0,
            gain: 0.5,
            cell_scale: 0.7,
            normalization: 0.5,
        };
        let result = erosion_filter(Vec2::new(0.2, 0.7), Vec3::new(0.5, 0.1, -0.1), 0.3, &params);

        let mut expected_magnitude = 0.0_f32;
        let mut strength = params.strength * params.scale;
        for _ in 0..params.octaves {
            expected_magnitude += strength;
            strength *= params.gain;
        }
        assert!(
            (result.magnitude - expected_magnitude).abs() < 1e-5,
            "magnitude {} should equal the geometric series {expected_magnitude}",
            result.magnitude
        );
    }
}
