use bevy::prelude::*;

use crate::prelude::*;

/// Resource to track battle performance and calculate score.
/// Score is calculated as: remaining_player_units * 10
#[derive(Resource, Default, Reflect, Debug)]
pub struct BattleScore {
    /// Virtual time when battle started (in seconds)
    pub battle_start_time: f64,
    /// Number of player units at battle start
    pub initial_player_units: usize,
    /// Number of player units remaining at battle end
    pub remaining_player_units: usize,
    /// Battle duration in seconds
    pub battle_duration: f64,
    /// Calculated score (higher is better)
    pub score: f64,
    pub player_name: Option<String>,
    pub score_amount: Option<u32>,
}

pub(crate) fn plugin(app: &mut App) {
    app.init_resource::<BattleScore>();
    app.register_type::<BattleScore>();
    app.add_systems(OnEnter(GameState::Battle), record_battle_start);
    app.add_systems(OnExit(GameState::Battle), calculate_battle_score);
}

fn record_battle_start(
    mut battle_score: ResMut<BattleScore>,
    time: Res<Time<Virtual>>,
    q_player_units: Query<(), With<PlayerUnit>>,
) {
    let player_count = q_player_units.iter().count();
    battle_score.battle_start_time = time.elapsed_secs_f64();
    battle_score.initial_player_units = player_count;
    battle_score.battle_duration = 0.0;
    battle_score.score = 0.0;

    info!(
        "Battle started: {} player units at time {:.2}s",
        player_count, battle_score.battle_start_time
    );
}

fn calculate_battle_score(
    mut battle_score: ResMut<BattleScore>,
    time: Res<Time<Virtual>>,
    q_player_units: Query<(), With<PlayerUnit>>,
    player_gold: Res<crate::game_manager::shop::PlayerGold>,
) {
    info!("=== calculate_battle_score called ===");
    let remaining = q_player_units.iter().count();
    let battle_duration = time.elapsed_secs_f64() - battle_score.battle_start_time;

    let unit_multiplier = 1.2;
    let gold_multiplier = 2.0;

    let unit_score = remaining as f64 * unit_multiplier;
    let gold_score = player_gold.amount as f64 * gold_multiplier;
    let round_score = unit_score + gold_score;

    battle_score.remaining_player_units = remaining;
    battle_score.battle_duration = battle_duration;
    battle_score.score = round_score;

    info!(
        "Battle ended: duration {:.2}s, initial units {}, remaining {}, gold {}, score {:.0}",
        battle_duration,
        battle_score.initial_player_units,
        remaining,
        player_gold.amount,
        battle_score.score
    );
}
