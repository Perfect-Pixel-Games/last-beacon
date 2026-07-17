use bevy::{prelude::*, scene::ScenePatch};
use foundation_runtime_library::prelude::*;
use last_beacon::asset_root;

#[test]
fn converted_bsn_scene_assets_load_as_scene_patches() {
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
        "scenes/credits.bsn",
        "scenes/gameplay_level.bsn",
        "scenes/pause_menu.bsn",
    ];
    let scene_handles = scene_asset_paths
        .into_iter()
        .map(|scene_asset_path| asset_server.load::<ScenePatch>(scene_asset_path))
        .collect::<Vec<_>>();

    for _frame_number in 0..60 {
        app.update();
    }

    let scene_assets = app.world().resource::<Assets<ScenePatch>>();
    for scene_handle in scene_handles {
        assert!(
            scene_assets.get(&scene_handle).is_some(),
            "the converted .bsn asset should load as a ScenePatch"
        );
    }
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
        .register_type::<BackgroundColor>()
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
        .register_type::<FoundationSplashText>();
}
