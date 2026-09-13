//! Free-fly debug camera for Last Beacon's World gameplay testbed.
//!
//! Game-owned rather than Foundation-owned: only the World landscape testbed
//! scene uses this, so it doesn't belong in shared engine infrastructure. Add
//! [`LastBeaconFreeFlyCameraPlugin`] and include
//! [`last_beacon_free_fly_camera_bundle`] in the camera's spawn bundle to use
//! it.

use std::f32::consts::FRAC_PI_2;

use bevy::{
    prelude::*,
    window::{CursorGrabMode, CursorOptions, PrimaryWindow},
};
use bevy_enhanced_input::prelude::*;
use foundation_runtime_library::prelude::{
    add_enhanced_input_plugin_if_missing, foundation_is_not_paused, FoundationPauseState,
};

/// Keeps the camera from pitching perfectly vertical, which would make yaw
/// direction ambiguous (gimbal lock at the poles).
const LAST_BEACON_FREE_FLY_CAMERA_PITCH_LIMIT_MARGIN: f32 = 0.01;

/// Installs the free-fly debug camera's input context and movement system.
///
/// Movement stops while [`foundation_runtime_library::prelude::FoundationPauseState`]
/// reports the game as paused, matching how other Foundation-owned per-frame
/// systems respect pause. The primary window's cursor is also locked and
/// hidden while flying, then unlocked and shown while paused (see
/// [`lock_cursor_for_last_beacon_free_fly_cameras`]).
#[derive(Default)]
pub struct LastBeaconFreeFlyCameraPlugin;

impl Plugin for LastBeaconFreeFlyCameraPlugin {
    fn build(&self, app: &mut App) {
        // Enhanced input must exist before `add_input_context` runs, and this
        // plugin is also used on its own in tests that skip `FoundationPlugin`.
        add_enhanced_input_plugin_if_missing(app);

        app.add_input_context::<LastBeaconFreeFlyCameraInput>()
            .add_systems(
                Update,
                (
                    move_last_beacon_free_fly_cameras.run_if(foundation_is_not_paused),
                    lock_cursor_for_last_beacon_free_fly_cameras,
                ),
            );
    }
}

/// Input context for Last Beacon's free-fly debug camera.
///
/// Spawned together with a camera's other components via
/// [`last_beacon_free_fly_camera_bundle`], not added to an existing entity
/// after the fact, since its actions/bindings must be spawned in the same
/// command as the context component.
#[derive(Component)]
#[require(
    LastBeaconFreeFlyCameraSettings,
    LastBeaconFreeFlyCameraOrientation,
    LastBeaconFreeFlyCameraVelocity
)]
pub struct LastBeaconFreeFlyCameraInput;

/// Tunable speed/sensitivity for a free-fly camera entity.
#[derive(Component, Clone, Copy, Debug)]
pub struct LastBeaconFreeFlyCameraSettings {
    /// Top movement speed, in world units per second, that the camera
    /// accelerates toward. Adjustable at runtime by scrolling the mouse
    /// wheel; see [`LastBeaconFreeFlyCameraSpeedAdjust`].
    pub move_speed: f32,
    /// Radians of rotation applied per pixel of mouse motion.
    pub look_sensitivity: f32,
    /// Seconds it takes the camera to accelerate from a stop up to
    /// `move_speed` (and to decelerate back down to a stop). Acceleration is
    /// derived from this and `move_speed` rather than stored separately, so
    /// a camera scrolled to twice the speed still reaches full speed in the
    /// same amount of time, not twice as long -- scrolling to change speed
    /// changes the camera's acceleration along with it.
    pub acceleration_time_seconds: f32,
    /// Multiplier applied to `move_speed` per unit of mouse wheel scroll
    /// (one "line" of scroll on most mice/trackpads).
    pub scroll_speed_multiplier: f32,
    /// Lower bound `move_speed` is clamped to when adjusted via scroll.
    pub min_move_speed: f32,
    /// Upper bound `move_speed` is clamped to when adjusted via scroll.
    pub max_move_speed: f32,
}

impl Default for LastBeaconFreeFlyCameraSettings {
    fn default() -> Self {
        Self {
            move_speed: 10.0,
            look_sensitivity: 0.002,
            acceleration_time_seconds: 0.25,
            scroll_speed_multiplier: 1.15,
            min_move_speed: 1.0,
            max_move_speed: 250.0,
        }
    }
}

/// Accumulated yaw/pitch for a free-fly camera entity.
///
/// `Transform.rotation` alone can't be incrementally nudged by mouse deltas
/// without re-deriving yaw/pitch from a quaternion each frame, so this
/// component tracks them directly instead.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct LastBeaconFreeFlyCameraOrientation {
    /// Rotation around the world Y axis, in radians.
    pub yaw: f32,
    /// Rotation around the camera's local X axis, in radians.
    pub pitch: f32,
}

/// Current velocity of a free-fly camera entity, in world units per second.
///
/// Tracked separately from `Transform` so movement can accelerate toward (and
/// decelerate away from) the input-driven target velocity instead of
/// snapping directly to it, which would otherwise make every start, stop, and
/// direction change feel instantaneous and jerky.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct LastBeaconFreeFlyCameraVelocity(pub Vec3);

/// Horizontal movement action (WASD). Output `x` is strafe, `y` is forward/back.
#[derive(InputAction)]
#[action_output(Vec2)]
pub struct LastBeaconFreeFlyCameraMove;

/// Vertical movement action. Space is `+1`, either Shift key is `-1`.
#[derive(InputAction)]
#[action_output(f32)]
pub struct LastBeaconFreeFlyCameraVertical;

/// Mouse-look action, bound to raw mouse motion.
#[derive(InputAction)]
#[action_output(Vec2)]
pub struct LastBeaconFreeFlyCameraLook;

/// Mouse wheel scroll action, used to adjust the camera's speed (and, since
/// acceleration is derived from it, its acceleration too). Output `y` is
/// positive when scrolling up/away from the user.
#[derive(InputAction)]
#[action_output(Vec2)]
pub struct LastBeaconFreeFlyCameraSpeedAdjust;

/// Returns the bundle a scene spawns to make an entity a free-fly camera.
///
/// Combine with [`Camera3d`] and any other camera components:
///
/// ```ignore
/// commands.spawn((
///     Camera3d::default(),
///     last_beacon_free_fly_camera_bundle(),
/// ));
/// ```
pub fn last_beacon_free_fly_camera_bundle() -> impl Bundle {
    (
        LastBeaconFreeFlyCameraInput,
        actions!(LastBeaconFreeFlyCameraInput[
            (
                Action::<LastBeaconFreeFlyCameraMove>::new(),
                DeadZone::default(),
                Bindings::spawn(Cardinal::wasd_keys()),
            ),
            (
                Action::<LastBeaconFreeFlyCameraVertical>::new(),
                bindings![
                    KeyCode::Space,
                    (KeyCode::ShiftLeft, Negate::all()),
                    (KeyCode::ShiftRight, Negate::all()),
                ],
            ),
            (
                Action::<LastBeaconFreeFlyCameraLook>::new(),
                bindings![Binding::mouse_motion()],
            ),
            (
                Action::<LastBeaconFreeFlyCameraSpeedAdjust>::new(),
                bindings![Binding::mouse_wheel()],
            ),
        ]),
    )
}

type LastBeaconFreeFlyCameraQuery<'world, 'state> = Query<
    'world,
    'state,
    (
        &'static mut Transform,
        &'static mut LastBeaconFreeFlyCameraOrientation,
        &'static mut LastBeaconFreeFlyCameraVelocity,
        &'static mut LastBeaconFreeFlyCameraSettings,
        &'static Actions<LastBeaconFreeFlyCameraInput>,
    ),
>;

fn move_last_beacon_free_fly_cameras(
    time: Res<Time>,
    mut free_fly_cameras: LastBeaconFreeFlyCameraQuery,
    move_actions: Query<&Action<LastBeaconFreeFlyCameraMove>>,
    vertical_actions: Query<&Action<LastBeaconFreeFlyCameraVertical>>,
    look_actions: Query<&Action<LastBeaconFreeFlyCameraLook>>,
    speed_adjust_actions: Query<&Action<LastBeaconFreeFlyCameraSpeedAdjust>>,
) {
    let elapsed_seconds = time.delta_secs();
    for (mut transform, mut orientation, mut velocity, mut settings, camera_actions) in
        &mut free_fly_cameras
    {
        // `Action<A>` lives on a separate entity related to the context entity
        // via the `Actions<C>`/`ActionOf<C>` relationship, not as a component
        // on the context entity itself, hence the `iter_many` lookups below.
        let Some(move_action) = move_actions.iter_many(camera_actions).next() else {
            continue;
        };
        let Some(vertical_action) = vertical_actions.iter_many(camera_actions).next() else {
            continue;
        };
        let Some(look_action) = look_actions.iter_many(camera_actions).next() else {
            continue;
        };
        let Some(speed_adjust_action) = speed_adjust_actions.iter_many(camera_actions).next()
        else {
            continue;
        };

        // Scroll wheel adjusts top speed multiplicatively, so it feels
        // proportionate across the whole speed range instead of a fixed
        // per-notch amount that's tiny at high speeds or huge at low ones.
        let scroll_amount = speed_adjust_action.y;
        if scroll_amount != 0.0 {
            let scale = settings.scroll_speed_multiplier.powf(scroll_amount);
            settings.move_speed = (settings.move_speed * scale)
                .clamp(settings.min_move_speed, settings.max_move_speed);
        }

        // Mouse motion is in screen pixels; negate so moving the mouse right
        // yaws right and moving it up pitches up.
        orientation.yaw -= look_action.x * settings.look_sensitivity;
        orientation.pitch = (orientation.pitch - look_action.y * settings.look_sensitivity).clamp(
            -FRAC_PI_2 + LAST_BEACON_FREE_FLY_CAMERA_PITCH_LIMIT_MARGIN,
            FRAC_PI_2 - LAST_BEACON_FREE_FLY_CAMERA_PITCH_LIMIT_MARGIN,
        );
        transform.rotation =
            Quat::from_euler(EulerRot::YXZ, orientation.yaw, orientation.pitch, 0.0);

        // WASD's "forward" (+Y from the Cardinal preset) maps to -Z in Bevy's
        // right-handed space, per `bevy_enhanced_input`'s own preset docs.
        let local_move_direction = Vec3::new(move_action.x, 0.0, -move_action.y);
        let mut world_move_direction = transform.rotation * local_move_direction;
        // Vertical movement is handled separately in world space below, so
        // strip any vertical component that came from pitching the camera.
        world_move_direction.y = 0.0;
        if world_move_direction.length_squared() > 0.0 {
            world_move_direction = world_move_direction.normalize();
        }

        let vertical_move_amount: f32 = **vertical_action;
        let target_direction = world_move_direction + Vec3::Y * vertical_move_amount;
        let target_velocity = target_direction * settings.move_speed;

        // Accelerate/decelerate current velocity toward the target velocity,
        // capped by how far `acceleration` can move it within this frame,
        // rather than snapping straight to it.
        let acceleration = settings.move_speed / settings.acceleration_time_seconds.max(1e-4);
        let velocity_delta = target_velocity - velocity.0;
        let max_delta_this_frame = acceleration * elapsed_seconds;
        if velocity_delta.length_squared() > max_delta_this_frame * max_delta_this_frame {
            velocity.0 += velocity_delta.normalize() * max_delta_this_frame;
        } else {
            velocity.0 = target_velocity;
        }

        transform.translation += velocity.0 * elapsed_seconds;
    }
}

/// Locks and hides the primary window's cursor while a free-fly camera is
/// active and gameplay isn't paused, so mouse-look isn't interrupted by the
/// cursor hitting the edge of the window. Releases and shows it again while
/// paused (e.g. the pause menu needs a free, visible cursor to click its
/// buttons), even though [`move_last_beacon_free_fly_cameras`] itself stops
/// running at that point.
fn lock_cursor_for_last_beacon_free_fly_cameras(
    pause_state: Res<FoundationPauseState>,
    free_fly_cameras: Query<(), With<LastBeaconFreeFlyCameraInput>>,
    mut primary_windows: Query<&mut CursorOptions, With<PrimaryWindow>>,
) {
    let Ok(mut cursor_options) = primary_windows.single_mut() else {
        return;
    };

    let should_lock_cursor = !pause_state.paused && !free_fly_cameras.is_empty();
    let (desired_visible, desired_grab_mode) = if should_lock_cursor {
        (false, CursorGrabMode::Locked)
    } else {
        (true, CursorGrabMode::None)
    };

    if cursor_options.visible != desired_visible {
        cursor_options.visible = desired_visible;
    }
    if cursor_options.grab_mode != desired_grab_mode {
        cursor_options.grab_mode = desired_grab_mode;
    }
}

#[cfg(test)]
mod tests {
    use bevy::input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll};

    use super::*;

    fn test_app_with_free_fly_camera() -> (App, Entity) {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.init_resource::<ButtonInput<KeyCode>>();
        app.init_resource::<AccumulatedMouseMotion>();
        app.init_resource::<AccumulatedMouseScroll>();
        app.init_resource::<FoundationPauseState>();
        app.world_mut()
            .spawn((PrimaryWindow, CursorOptions::default()));
        app.add_plugins(LastBeaconFreeFlyCameraPlugin);
        // `bevy_enhanced_input` finishes context setup in `Plugin::finish`, which
        // only runs automatically through `App::run()`. Tests that drive the app
        // with bare `update()` calls must invoke it manually first.
        app.finish();
        app.cleanup();

        let free_fly_camera_entity = app
            .world_mut()
            .spawn((Transform::default(), last_beacon_free_fly_camera_bundle()))
            .id();
        // Let the freshly spawned context/action/binding entities settle
        // before the test starts asserting on action values.
        app.update();

        (app, free_fly_camera_entity)
    }

    #[test]
    fn pressing_w_accelerates_camera_forward() {
        let (mut app, free_fly_camera_entity) = test_app_with_free_fly_camera();

        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyW);
        app.update();

        // `TimePlugin` (part of `MinimalPlugins`) recomputes `Time`'s delta from
        // the real clock every frame, so this test can't force an exact elapsed
        // duration -- it instead checks the acceleration formula against
        // whatever real (small, non-deterministic) delta actually elapsed,
        // assuming that delta stays well under `acceleration_time_seconds`
        // (true for any realistic single frame), so the camera is still
        // ramping up rather than already clamped to `move_speed`.
        let elapsed_seconds = app.world().resource::<Time>().delta_secs();
        let settings = LastBeaconFreeFlyCameraSettings::default();
        let acceleration = settings.move_speed / settings.acceleration_time_seconds;
        let expected_z = -acceleration * elapsed_seconds * elapsed_seconds;

        let camera_transform = app
            .world()
            .get::<Transform>(free_fly_camera_entity)
            .expect("free-fly camera should have a Transform");
        // Facing the default -Z direction, "forward" (W) should decrease Z
        // while ramping up toward move_speed.
        assert!(
            (camera_transform.translation.z - expected_z).abs() < 0.0001,
            "expected the camera to move to z={expected_z}, got {:?}",
            camera_transform.translation
        );
        assert!(camera_transform.translation.x.abs() < 0.0001);
        assert!(camera_transform.translation.y.abs() < 0.01);
    }

    #[test]
    fn pausing_stops_camera_movement() {
        let (mut app, free_fly_camera_entity) = test_app_with_free_fly_camera();

        app.world_mut()
            .resource_mut::<FoundationPauseState>()
            .paused = true;
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyW);
        app.update();

        let camera_transform = app
            .world()
            .get::<Transform>(free_fly_camera_entity)
            .expect("free-fly camera should have a Transform");
        assert_eq!(
            camera_transform.translation,
            Vec3::ZERO,
            "camera should not move while Foundation gameplay is paused"
        );
    }

    #[test]
    fn mouse_motion_rotates_camera_yaw() {
        let (mut app, free_fly_camera_entity) = test_app_with_free_fly_camera();

        app.world_mut()
            .resource_mut::<AccumulatedMouseMotion>()
            .delta = Vec2::new(100.0, 0.0);
        app.update();

        let orientation = app
            .world()
            .get::<LastBeaconFreeFlyCameraOrientation>(free_fly_camera_entity)
            .expect("free-fly camera should have an orientation");
        // Moving the mouse right (+X) should yaw the camera right, which this
        // module implements as a negative yaw delta.
        assert!(
            orientation.yaw < 0.0,
            "expected negative yaw after rightward mouse motion, got {}",
            orientation.yaw
        );
    }

    #[test]
    fn scrolling_up_increases_move_speed() {
        let (mut app, free_fly_camera_entity) = test_app_with_free_fly_camera();
        let initial_speed = LastBeaconFreeFlyCameraSettings::default().move_speed;

        app.world_mut()
            .resource_mut::<AccumulatedMouseScroll>()
            .delta = Vec2::new(0.0, 1.0);
        app.update();

        let settings = app
            .world()
            .get::<LastBeaconFreeFlyCameraSettings>(free_fly_camera_entity)
            .expect("free-fly camera should have settings");
        assert!(
            settings.move_speed > initial_speed,
            "expected scrolling up to increase move speed above {initial_speed}, got {}",
            settings.move_speed
        );
    }

    #[test]
    fn scrolling_down_decreases_move_speed() {
        let (mut app, free_fly_camera_entity) = test_app_with_free_fly_camera();
        let initial_speed = LastBeaconFreeFlyCameraSettings::default().move_speed;

        app.world_mut()
            .resource_mut::<AccumulatedMouseScroll>()
            .delta = Vec2::new(0.0, -1.0);
        app.update();

        let settings = app
            .world()
            .get::<LastBeaconFreeFlyCameraSettings>(free_fly_camera_entity)
            .expect("free-fly camera should have settings");
        assert!(
            settings.move_speed < initial_speed,
            "expected scrolling down to decrease move speed below {initial_speed}, got {}",
            settings.move_speed
        );
    }

    #[test]
    fn move_speed_is_clamped_to_configured_bounds() {
        let (mut app, free_fly_camera_entity) = test_app_with_free_fly_camera();

        for _ in 0..500 {
            app.world_mut()
                .resource_mut::<AccumulatedMouseScroll>()
                .delta = Vec2::new(0.0, 1.0);
            app.update();
        }

        let settings = app
            .world()
            .get::<LastBeaconFreeFlyCameraSettings>(free_fly_camera_entity)
            .expect("free-fly camera should have settings");
        assert_eq!(
            settings.move_speed, settings.max_move_speed,
            "move speed should clamp at max_move_speed after scrolling far past it"
        );
    }

    fn primary_window_cursor_options(app: &mut App) -> CursorOptions {
        app.world_mut()
            .query_filtered::<&CursorOptions, With<PrimaryWindow>>()
            .single(app.world())
            .expect("test app should have a primary window")
            .clone()
    }

    #[test]
    fn cursor_locks_and_hides_while_flying_unpaused() {
        let (mut app, _free_fly_camera_entity) = test_app_with_free_fly_camera();

        app.update();

        let cursor_options = primary_window_cursor_options(&mut app);
        assert!(!cursor_options.visible);
        assert_eq!(cursor_options.grab_mode, CursorGrabMode::Locked);
    }

    #[test]
    fn cursor_unlocks_and_shows_while_paused() {
        let (mut app, _free_fly_camera_entity) = test_app_with_free_fly_camera();

        app.world_mut()
            .resource_mut::<FoundationPauseState>()
            .paused = true;
        app.update();

        let cursor_options = primary_window_cursor_options(&mut app);
        assert!(cursor_options.visible);
        assert_eq!(cursor_options.grab_mode, CursorGrabMode::None);
    }

    #[test]
    fn cursor_stays_unlocked_without_a_free_fly_camera() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.init_resource::<ButtonInput<KeyCode>>();
        app.init_resource::<AccumulatedMouseMotion>();
        app.init_resource::<AccumulatedMouseScroll>();
        app.init_resource::<FoundationPauseState>();
        app.world_mut()
            .spawn((PrimaryWindow, CursorOptions::default()));
        app.add_plugins(LastBeaconFreeFlyCameraPlugin);
        app.finish();
        app.cleanup();

        app.update();

        let cursor_options = primary_window_cursor_options(&mut app);
        assert!(cursor_options.visible);
        assert_eq!(cursor_options.grab_mode, CursorGrabMode::None);
    }
}
