use std::sync::Mutex;

use bevy::{prelude::*, scene::ScenePatch};
use foundation_runtime_library::prelude::*;
use last_beacon::{
    asset_root,
    ui_widgets::{
        LastBeaconBeaconPrimaryButton, LastBeaconBeaconTabButton, LastBeaconBsnWidget,
        LastBeaconMainMenuPrimaryButton, LastBeaconUiButton, LastBeaconUiTab,
        LastBeaconUiTextInput, LastBeaconUiValueButton, LastBeaconUiValueText,
    },
};

static BSN_ASSET_TEST_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn converted_bsn_scene_assets_load_as_scene_patches() {
    let _bsn_asset_test_guard = BSN_ASSET_TEST_LOCK
        .lock()
        .expect("BSN asset test lock should not be poisoned");
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(bevy::asset::AssetPlugin {
        file_path: asset_root().to_string_lossy().to_string(),
        ..default()
    });
    app.add_plugins(bevy::scene::ScenePlugin);
    app.add_message::<SceneLoadRequested>();
    app.add_plugins(FoundationBsnAssetPlugin);
    register_bsn_test_types(&mut app);

    let asset_server = app.world().resource::<AssetServer>().clone();
    let scene_asset_paths = [
        "scenes/pixel_perfect_splash.bsn",
        "scenes/bevy_splash.bsn",
        "scenes/main_menu.bsn",
        "scenes/options_menu.bsn",
        "scenes/dashboard.bsn",
        "scenes/hangar.bsn",
        "scenes/garage.bsn",
        "scenes/mission_control.bsn",
        "scenes/fabrication.bsn",
        "scenes/ui_playground.bsn",
        "scenes/silo_upgrades.bsn",
        "scenes/credits.bsn",
        "scenes/gameplay_level.bsn",
        "scenes/pause_menu.bsn",
        "ui/widgets/common/primary_button.bsn",
        "ui/widgets/common/secondary_button.bsn",
        "ui/widgets/common/tertiary_button.bsn",
        "ui/widgets/common/tabs.bsn",
        "ui/widgets/common/stat_rows_panel.bsn",
        "ui/widgets/common/text_field.bsn",
        "ui/widgets/common/text_box.bsn",
        "ui/widgets/common/radio_buttons.bsn",
        "ui/widgets/common/toggle_buttons.bsn",
        "ui/widgets/common/combo_box.bsn",
        "ui/widgets/common/number_field.bsn",
        "ui/widgets/common/slider.bsn",
        "ui/widgets/common/property_container.bsn",
        "ui/widgets/main_menu/brand.bsn",
        "ui/widgets/main_menu/continue_button.bsn",
        "ui/widgets/main_menu/quick_run_button.bsn",
        "ui/widgets/main_menu/new_game_button.bsn",
        "ui/widgets/main_menu/settings_button.bsn",
        "ui/widgets/main_menu/credits_button.bsn",
        "ui/widgets/main_menu/ui_playground_button.bsn",
        "ui/widgets/main_menu/quit_button.bsn",
        "ui/widgets/main_menu/current_save_panel.bsn",
        "ui/widgets/main_menu/footer.bsn",
    ];
    let scene_handles = scene_asset_paths
        .into_iter()
        .map(|scene_asset_path| {
            (
                scene_asset_path,
                asset_server.load::<ScenePatch>(scene_asset_path),
            )
        })
        .collect::<Vec<_>>();

    for _frame_number in 0..120 {
        app.update();
    }

    let scene_assets = app.world().resource::<Assets<ScenePatch>>();
    for (scene_asset_path, scene_handle) in scene_handles {
        assert!(
            scene_assets.get(&scene_handle).is_some(),
            "the converted .bsn asset `{scene_asset_path}` should load as a ScenePatch"
        );
    }
}

#[test]
fn converted_pixel_perfect_scene_spawns_authored_text_through_foundation_bridge() {
    let _bsn_asset_test_guard = BSN_ASSET_TEST_LOCK
        .lock()
        .expect("BSN asset test lock should not be poisoned");
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(bevy::asset::AssetPlugin {
        file_path: asset_root().to_string_lossy().to_string(),
        ..default()
    });
    app.add_plugins(bevy::scene::ScenePlugin);
    app.add_message::<SceneLoadRequested>();
    app.add_plugins(FoundationBsnAssetPlugin);
    register_bsn_test_types(&mut app);

    let scene_key = "last-beacon/splash_pixel_perfect";
    app.world_mut()
        .resource_mut::<FoundationBsnSceneRegistry>()
        .register_scene(scene_key, "scenes/pixel_perfect_splash.bsn");
    app.world_mut().write_message(SceneLoadRequested {
        scene_id: SceneId(7),
        source: SceneSource::bsn_scene(scene_key),
    });

    for _frame_number in 0..120 {
        app.update();
    }

    let mut text_query = app.world_mut().query::<(&Text, Option<&SceneOwner>)>();
    let texts = text_query
        .iter(app.world())
        .map(|(text, scene_owner)| (text.0.clone(), scene_owner.copied()))
        .collect::<Vec<_>>();

    assert!(
        texts.iter().any(|(text, scene_owner)| {
            text == "Pixel Perfect" && *scene_owner == Some(SceneOwner { scene_id: SceneId(7) })
        }),
        "the Foundation BSN bridge should spawn the authored Pixel Perfect text with scene ownership; found {texts:?}",
    );
}

fn register_bsn_test_types(app: &mut App) {
    app.register_type::<Node>()
        .register_type::<Val>()
        .register_type::<FlexDirection>()
        .register_type::<AlignItems>()
        .register_type::<JustifyContent>()
        .register_type::<PositionType>()
        .register_type::<Overflow>()
        .register_type::<OverflowAxis>()
        .register_type::<UiRect>()
        .register_type::<BorderRadius>()
        .register_type::<BackgroundColor>()
        .register_type::<BorderColor>()
        .register_type::<Button>()
        .register_type::<Text>()
        .register_type::<TextFont>()
        .register_type::<TextColor>()
        .register_type::<bevy::text::FontSize>()
        .register_type::<Color>()
        .register_type::<Srgba>()
        .register_type::<FoundationMenuButton>()
        .register_type::<FoundationOptionsMenu>()
        .register_type::<FoundationCloseOnEscape>()
        .register_type::<FoundationResumeOnEscape>()
        .register_type::<FoundationPauseOpener>()
        .register_type::<FoundationSimpleGameplayLevel>()
        .register_type::<FoundationCreditsRoll>()
        .register_type::<FoundationSplashUiRoot>()
        .register_type::<FoundationSplashText>()
        .register_type::<LastBeaconBsnWidget>()
        .register_type::<LastBeaconMainMenuPrimaryButton>()
        .register_type::<LastBeaconBeaconPrimaryButton>()
        .register_type::<LastBeaconBeaconTabButton>()
        .register_type::<LastBeaconUiButton>()
        .register_type::<LastBeaconUiTab>()
        .register_type::<LastBeaconUiTextInput>()
        .register_type::<LastBeaconUiValueButton>()
        .register_type::<LastBeaconUiValueText>();
}
