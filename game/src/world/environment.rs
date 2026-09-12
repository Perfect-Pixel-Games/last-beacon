//! Sky, sun, and camera/post-processing setup for the World testbed scene.
//!
//! Bevy's `PbrPlugin` (part of `DefaultPlugins`) already installs
//! `AtmospherePlugin` and `ScreenSpaceAmbientOcclusionPlugin` unconditionally,
//! so this module only needs to spawn entities/components, not register any
//! additional plugins.

use bevy::{
    camera::Hdr,
    core_pipeline::tonemapping::Tonemapping,
    light::{atmosphere::ScatteringMedium, Atmosphere},
    pbr::{AtmosphereSettings, ScreenSpaceAmbientOcclusion},
    post_process::bloom::Bloom,
    prelude::*,
    render::view::Msaa,
};

/// Warm-white sun color for the World testbed's directional light.
const LANDSCAPE_SUN_COLOR: Color = Color::srgb(1.0, 0.96, 0.88);

/// Directional light illuminance, in lux, roughly matching overcast-to-clear
/// daylight so the atmosphere and bloom read believably.
const LANDSCAPE_SUN_ILLUMINANCE: f32 = 20_000.0;

/// Spawns the atmosphere entity and the sun, returning their entity IDs so
/// the caller can propagate `SceneOwner` onto them for scene-stack cleanup.
pub fn spawn_landscape_sky_and_sun(
    commands: &mut Commands,
    scattering_media: &mut Assets<ScatteringMedium>,
) -> (Entity, Entity) {
    let scattering_medium = scattering_media.add(ScatteringMedium::default());
    let atmosphere_entity = commands
        .spawn((
            Atmosphere::earth(scattering_medium),
            Transform::default(),
            Name::new("Last Beacon Landscape Atmosphere"),
        ))
        .id();

    // Elevation ~43 degrees above the horizon, per the plan's 35-45 degree
    // target. Only rotation matters for a directional light, but placing the
    // translation "up where the sun would be" and looking at the origin
    // matches this codebase's existing light-setup convention (see
    // `initialize_last_beacon_placeholder_cube_scenes` in `game/src/lib.rs`).
    let sun_position = Vec3::new(1.0, 1.0, 0.4) * 500.0;
    let sun_entity = commands
        .spawn((
            DirectionalLight {
                color: LANDSCAPE_SUN_COLOR,
                illuminance: LANDSCAPE_SUN_ILLUMINANCE,
                shadow_maps_enabled: true,
                ..default()
            },
            Transform::from_translation(sun_position).looking_at(Vec3::ZERO, Vec3::Y),
            Name::new("Last Beacon Landscape Sun"),
        ))
        .id();

    (atmosphere_entity, sun_entity)
}

/// Returns the bundle of camera-side rendering components (HDR, atmosphere
/// rendering, tonemapping, bloom, SSAO) for the World testbed's free-fly
/// camera entity.
///
/// SSAO requires `Msaa::Off` -- Bevy silently disables it and logs a warning
/// otherwise -- so this bundle includes that too.
pub fn landscape_camera_rendering_bundle() -> impl Bundle {
    (
        Hdr,
        Msaa::Off,
        AtmosphereSettings::default(),
        Tonemapping::TonyMcMapface,
        Bloom::default(),
        ScreenSpaceAmbientOcclusion::default(),
    )
}
