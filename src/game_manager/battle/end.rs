use bevy::prelude::*;

use crate::prelude::*;
use crate::screens::Screen;

#[derive(Resource)]
struct LeaderboardTimer(Timer);

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        check_battle_result.run_if(in_state(GameState::Battle)),
    );
    app.add_systems(
        Update,
        click_to_continue.run_if(
            in_state(GameState::WinAndNextDay)
                .and(|progress: Res<GameProgress>| progress.current_round < 4),
        ),
    );
    app.add_systems(
        Update,
        click_to_leaderboard.run_if(
            in_state(GameState::WinAndNextDay)
                .and(|progress: Res<GameProgress>| progress.current_round >= 4),
        ),
    );
    app.add_systems(
        Update,
        click_to_title_on_lose.run_if(in_state(GameState::Lose)),
    );
    app.add_systems(OnEnter(GameState::Leaderboard), setup_leaderboard_timer);
    app.add_systems(
        Update,
        click_to_title_from_leaderboard.run_if(in_state(GameState::Leaderboard)),
    );
    app.add_systems(OnExit(GameState::WinAndNextDay), despawn_all_pawns);
    app.add_systems(OnExit(GameState::Lose), despawn_all_pawns);
    app.add_systems(OnEnter(GameState::Preparing), trigger_fade_in_on_prepare);
}

fn click_to_continue(
    mouse: Res<ButtonInput<MouseButton>>,
    mut next_state: ResMut<NextState<GameState>>,
    _commands: Commands,
) {
    if mouse.just_pressed(MouseButton::Left) {
        next_state.set(GameState::Preparing);
    }
}

fn click_to_leaderboard(
    mouse: Res<ButtonInput<MouseButton>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        next_state.set(GameState::Leaderboard);
    }
}

fn click_to_title_on_lose(
    mouse: Res<ButtonInput<MouseButton>>,
    mut next_screen: ResMut<NextState<Screen>>,
    _commands: Commands,
) {
    if mouse.just_pressed(MouseButton::Left) {
        next_screen.set(Screen::Title);
    }
}

fn setup_leaderboard_timer(mut commands: Commands) {
    commands.insert_resource(LeaderboardTimer(Timer::from_seconds(3.0, TimerMode::Once)));
}

fn click_to_title_from_leaderboard(
    mouse: Res<ButtonInput<MouseButton>>,
    mut next_screen: ResMut<NextState<Screen>>,
    mut progress: ResMut<GameProgress>,
    time: Res<Time>,
    mut timer: ResMut<LeaderboardTimer>,
    _commands: Commands,
) {
    // Tick the timer
    timer.0.tick(time.delta());

    // Only allow transition after 3 seconds have elapsed AND mouse is clicked
    if timer.0.elapsed_secs() >= 3.0 && mouse.just_pressed(MouseButton::Left) {
        progress.current_round = 3;
        next_screen.set(Screen::Title);
    }
}

fn check_battle_result(
    q_player_units: Query<(), With<PlayerUnit>>,
    q_enemy_units: Query<(), With<EnemyUnit>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut campaign: ResMut<GameProgress>,
    _commands: Commands,
) {
    let player_count = q_player_units.iter().count();
    let enemy_count = q_enemy_units.iter().count();

    info!("Battle check: {} player units, {} enemy units", player_count, enemy_count);

    if enemy_count == 0 {
        info!("All enemies defeated - Win!");
        campaign.record_battle(true);
        next_state.set(GameState::WinAndNextDay);
    } else if player_count == 0 {
        info!("All player units lost - Lose!");
        campaign.record_battle(false);
        next_state.set(GameState::Lose);
    }
}

fn despawn_all_pawns(
    mut commands: Commands,
    q_pawns: Query<Entity, With<PlayerSquad>>,
    q_enemies: Query<Entity, With<EnemySquad>>,
    q_corpses: Query<Entity, With<Corpse>>,
) {
    for entity in q_pawns.iter() {
        commands.entity(entity).despawn();
    }
    for entity in q_enemies.iter() {
        commands.entity(entity).despawn();
    }
    for entity in q_corpses.iter() {
        commands.entity(entity).despawn();
    }
}

/// Triggers camera fade out when entering GameState::Preparing
fn trigger_fade_in_on_prepare(mut commands: Commands) {
    commands.trigger(FadeInEvent::default());
}
