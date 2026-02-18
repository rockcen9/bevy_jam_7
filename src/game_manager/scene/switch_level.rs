use bevy_ecs_ldtk::{
    LdtkProjectHandle, LevelSelection, assets::LdtkProject, prelude::RawLevelAccessor,
};

use crate::{prelude::*, screens::Screen};

pub(crate) fn plugin(app: &mut bevy::app::App) {
    app.add_systems(OnEnter(GameState::Preparing), switch_to_next_level);
    // app.add_systems(OnEnter(GameState::Preparing), switch_to_prebattle);
    app.add_systems(
        Update,
        increment_round_on_key9.run_if(in_state(GameState::Battle)),
    );
}

fn increment_round_on_key9(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut campaign: ResMut<GameProgress>,
    mut next_state: ResMut<NextState<GameState>>,
    current_state: Res<State<GameState>>,
) {
    if std::env::var("ENABLE_SWITCH_LEVEL").is_ok() {
        if keyboard.just_pressed(KeyCode::Digit9) {
            info!(
                "Cheat: simulating battle win (current state: {:?})",
                current_state.get()
            );
            campaign.record_battle(true);
            next_state.set(GameState::WinAndNextDay);
            info!("Cheat: set next state to WinAndNextDay");
        }
    }
}

fn zoom_for_round(round: usize) -> f32 {
    match round {
        1 => 2.0,
        2 => 2.2,
        3 => 2.5,
        _ => 2.5,
    }
}

fn switch_to_next_level(
    mut level_selection: ResMut<LevelSelection>,
    progression: Res<GameProgress>,
    mut screen: ResMut<NextState<Screen>>,
    ldtk_projects: Query<&LdtkProjectHandle>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    mut zoom_writer: MessageWriter<CameraZoomMessage>,
    mut player_gold: ResMut<crate::game_manager::shop::PlayerGold>,
) {
    debug!("=== LEVEL SWITCHING DEBUG ===");

    // List all available levels from LDTK
    if let Ok(ldtk_handle) = ldtk_projects.single() {
        if let Some(ldtk_project) = ldtk_project_assets.get(ldtk_handle) {
            let level_identifiers: Vec<String> = ldtk_project
                .iter_raw_levels()
                .map(|level| level.identifier.clone())
                .collect();
            debug!("Available levels from LDTK: {:?}", level_identifiers);
        } else {
            debug!("LDTK project not loaded yet");
        }
    } else {
        debug!("No LDTK project handle found");
    }

    debug!("Current round: {}", progression.current_round);
    debug!("Current level selection: {:?}", *level_selection);

    if progression.current_round == 1 {
        debug!("Switching to Level1");
        *level_selection = LevelSelection::Identifier("Level1".to_string());
        zoom_writer.write(CameraZoomMessage(zoom_for_round(1)));
        player_gold.amount = 3 * 20;
    } else if progression.current_round == 2 {
        debug!("Switching to Level2");
        *level_selection = LevelSelection::Identifier("Level2".to_string());
        zoom_writer.write(CameraZoomMessage(zoom_for_round(2)));
        player_gold.amount = 7 * 20;
    } else if progression.current_round == 3 {
        debug!("Switching to Level3");
        *level_selection = LevelSelection::Identifier("Level3".to_string());
        zoom_writer.write(CameraZoomMessage(zoom_for_round(3)));
        player_gold.amount = 10 * 20;
    } else {
        debug!("All levels completed, returning to Title screen");
        screen.set(Screen::Title);
    }

    debug!("New level selection: {:?}", *level_selection);
    debug!("=== END LEVEL SWITCHING ===");
}
