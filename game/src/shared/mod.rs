//! Cross-space communication layer shared by Last Beacon's Hub and World gameplay.
//!
//! [`crate::hub`] (Beacon dashboard/hangar/garage/etc.) and [`crate::world`]
//! (launched-vehicle/expedition) gameplay are separate modules that never depend
//! on each other. Anything one space needs to hand to the other -- and anything
//! the options menu needs to read or write -- flows through the resource and
//! message types in this module instead of a direct dependency between the two.

use bevy::prelude::*;
use foundation_runtime_library::prelude::*;

use crate::scenes::{BEACON_SCENE, GAMEPLAY_LEVEL_SCENE};

/// Installs Last Beacon's shared cross-space gameplay state and messaging.
#[derive(Default)]
pub struct LastBeaconSharedGameplayPlugin;

impl Plugin for LastBeaconSharedGameplayPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LastBeaconSharedGameplayState>()
            .init_resource::<LastBeaconGameplaySpace>()
            .register_type::<LastBeaconSharedGameplayState>()
            .register_type::<LastBeaconGameplaySpace>()
            .add_message::<LastBeaconGameplayEvent>()
            // Runs in `Last`, after scene-stack command processing settles in
            // `PostUpdate`, so it always reflects this frame's final stack --
            // the same ordering `hide_last_beacon_menu_ui_behind_settings` uses
            // in `game/src/lib.rs` for the same reason.
            .add_systems(Last, resolve_last_beacon_gameplay_space);
    }
}

/// Persistent cross-space gameplay state.
///
/// Hub and World gameplay read and write this directly to hand data across the
/// space boundary -- for example, a robot loadout Hub assembled that World later
/// needs to spawn, or an expedition result World produced that Hub needs to show.
/// It starts empty; fields land here as real gameplay features need them, rather
/// than being guessed at ahead of any concrete gameplay design.
#[derive(Clone, Debug, Default, Reflect, Resource)]
#[reflect(Resource)]
pub struct LastBeaconSharedGameplayState;

/// Discrete cross-space gameplay occurrences.
///
/// Use this for one-shot "this just happened" moments -- for example, an
/// expedition launching or a robot being destroyed -- rather than persistent
/// state (see [`LastBeaconSharedGameplayState`] for that). It starts with no
/// variants; the first real gameplay event adds one.
#[derive(Clone, Debug, Message, Reflect)]
pub enum LastBeaconGameplayEvent {}

/// Which Last Beacon gameplay space is currently active.
///
/// Recomputed every frame from the scene stack (by a private system in this
/// module), so it is never a second source of truth that can drift from what
/// is actually on screen.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Reflect, Resource)]
#[reflect(Resource)]
pub enum LastBeaconGameplaySpace {
    /// Neither the Hub nor the World scene is on the scene stack -- for example,
    /// the main menu, a splash screen, or credits.
    #[default]
    Neither,
    /// The Beacon hub scene is on the scene stack.
    Hub,
    /// The launched-vehicle/expedition world scene is on the scene stack.
    World,
}

/// Run condition: true while [`LastBeaconGameplaySpace::Hub`] is active.
///
/// Mirrors `foundation_is_not_paused`'s shape so Hub gameplay systems can gate
/// themselves with `.run_if(last_beacon_is_in_hub)`.
pub fn last_beacon_is_in_hub(gameplay_space: Res<LastBeaconGameplaySpace>) -> bool {
    *gameplay_space == LastBeaconGameplaySpace::Hub
}

/// Run condition: true while [`LastBeaconGameplaySpace::World`] is active.
pub fn last_beacon_is_in_world(gameplay_space: Res<LastBeaconGameplaySpace>) -> bool {
    *gameplay_space == LastBeaconGameplaySpace::World
}

/// Recomputes [`LastBeaconGameplaySpace`] from the current scene stack.
///
/// Checks for the Beacon shell scene and the World gameplay scene by key rather
/// than by presentation flags, since both scenes persist on the stack beneath
/// pause/options overlays -- an overlay opening or closing must never change the
/// resolved space.
fn resolve_last_beacon_gameplay_space(
    scene_stack: Option<Res<SceneStack>>,
    mut gameplay_space: ResMut<LastBeaconGameplaySpace>,
) {
    let Some(scene_stack) = scene_stack else {
        return;
    };

    let beacon_scene_is_open = scene_stack_contains_bsn_scene(&scene_stack, BEACON_SCENE);
    let world_scene_is_open = scene_stack_contains_bsn_scene(&scene_stack, GAMEPLAY_LEVEL_SCENE);

    let resolved_gameplay_space = if beacon_scene_is_open {
        LastBeaconGameplaySpace::Hub
    } else if world_scene_is_open {
        LastBeaconGameplaySpace::World
    } else {
        LastBeaconGameplaySpace::Neither
    };

    if *gameplay_space != resolved_gameplay_space {
        *gameplay_space = resolved_gameplay_space;
    }
}

fn scene_stack_contains_bsn_scene(scene_stack: &SceneStack, target_scene_key: &str) -> bool {
    scene_stack
        .entries()
        .iter()
        .any(|scene_stack_entry| match &scene_stack_entry.source {
            SceneSource::BsnScene { key } => key == target_scene_key,
            SceneSource::Runtime { .. } => false,
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scenes::PAUSE_MENU_SCENE;

    fn gameplay_space_test_app() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(FoundationSceneStackPlugin);
        app.init_resource::<LastBeaconGameplaySpace>();
        app.add_systems(Last, resolve_last_beacon_gameplay_space);
        app
    }

    #[test]
    fn resolves_to_neither_when_no_gameplay_scene_is_open() {
        let mut app = gameplay_space_test_app();
        app.update();

        assert_eq!(
            *app.world().resource::<LastBeaconGameplaySpace>(),
            LastBeaconGameplaySpace::Neither,
        );
    }

    #[test]
    fn resolves_to_hub_when_the_beacon_scene_is_open() {
        let mut app = gameplay_space_test_app();
        app.world_mut()
            .write_message(SceneCommand::open(SceneSource::bsn_scene(BEACON_SCENE)));
        app.update();

        assert_eq!(
            *app.world().resource::<LastBeaconGameplaySpace>(),
            LastBeaconGameplaySpace::Hub,
        );
    }

    #[test]
    fn resolves_to_world_when_the_gameplay_level_scene_is_open() {
        let mut app = gameplay_space_test_app();
        app.world_mut()
            .write_message(SceneCommand::open(SceneSource::bsn_scene(
                GAMEPLAY_LEVEL_SCENE,
            )));
        app.update();

        assert_eq!(
            *app.world().resource::<LastBeaconGameplaySpace>(),
            LastBeaconGameplaySpace::World,
        );
    }

    #[test]
    fn pause_overlay_does_not_change_the_resolved_hub_space() {
        let mut app = gameplay_space_test_app();
        app.world_mut()
            .write_message(SceneCommand::open(SceneSource::bsn_scene(BEACON_SCENE)));
        app.update();

        let pause_overlay_options =
            OpenSceneOptions::default().with_presentation(ScenePresentation::PAUSE_OVERLAY);
        app.world_mut()
            .write_message(SceneCommand::open_with_options(
                SceneSource::bsn_scene(PAUSE_MENU_SCENE),
                pause_overlay_options,
            ));
        app.update();

        assert_eq!(
            *app.world().resource::<LastBeaconGameplaySpace>(),
            LastBeaconGameplaySpace::Hub,
        );
    }
}
