use bevy_inspector_egui::quick::ResourceInspectorPlugin;

use crate::{
    dev_tools::inspect::command_key_toggle_active,
    prelude::*,
    screens::{Screen, loading::LoadingScreen},
};

pub fn debug_resource_plugin(app: &mut App) {
    // state
    app.add_plugins(
        bevy_inspector_egui::quick::StateInspectorPlugin::<Screen>::default()
            .run_if(command_key_toggle_active(false, KeyCode::Digit2)),
    );
    app.add_plugins(
        bevy_inspector_egui::quick::StateInspectorPlugin::<GameState>::default()
            .run_if(command_key_toggle_active(false, KeyCode::Digit2)),
    );
    app.add_plugins(
        bevy_inspector_egui::quick::StateInspectorPlugin::<LoadingScreen>::default()
            .run_if(command_key_toggle_active(false, KeyCode::Digit2)),
    );

    // resource
    app.add_plugins(
        ResourceInspectorPlugin::<BattleScore>::default()
            .run_if(command_key_toggle_active(false, KeyCode::Digit3)),
    );
    app.add_plugins(
        ResourceInspectorPlugin::<GameProgress>::default()
            .run_if(command_key_toggle_active(false, KeyCode::Digit3)),
    );
    // app.add_plugins(
    //     ResourceInspectorPlugin::<CombatFlux>::default()
    //         .run_if(command_key_toggle_active(false, KeyCode::Digit3)),
    // );
    app.add_plugins(
        ResourceInspectorPlugin::<BattleStatus>::default()
            .run_if(command_key_toggle_active(false, KeyCode::Digit3)),
    );
    app.add_plugins(
        ResourceInspectorPlugin::<InBoundary>::default()
            .run_if(command_key_toggle_active(false, KeyCode::Digit3)),
    );
    app.add_plugins(
        ResourceInspectorPlugin::<LevelBounds>::default()
            .run_if(command_key_toggle_active(false, KeyCode::Digit3)),
    );
    app.add_plugins(
        ResourceInspectorPlugin::<UnitStatsCache>::default()
            .run_if(command_key_toggle_active(false, KeyCode::Digit3)),
    );
    // app.add_plugins(
    //     ResourceInspectorPlugin::<FadeOverlay>::default()
    //         .run_if(command_key_toggle_active(false, KeyCode::Digit3)),
    // );
    // app.add_plugins(
    //     ResourceInspectorPlugin::<LoadedCollision>::default()
    //         .run_if(command_key_toggle_active(false, KeyCode::Digit2)),
    // );
    // app.add_plugins(
    //     ResourceInspectorPlugin::<CustomerAmountState>::default()
    //         .run_if(command_key_toggle_active(false, KeyCode::Digit2)),
    // );
    // app.add_plugins(
    //     ResourceInspectorPlugin::<TotalFoodDelivered>::default()
    //         .run_if(command_key_toggle_active(false, KeyCode::Digit2)),
    // );
    // app.add_plugins(
    //     ResourceInspectorPlugin::<PlayerCoins>::default()
    //         .run_if(command_key_toggle_active(false, KeyCode::Digit2)),
    // );
    // app.add_plugins(
    //     ResourceInspectorPlugin::<SpineTexturesConfig>::default()
    //         .run_if(command_key_toggle_active(false, KeyCode::Digit2)),
    // );
}
