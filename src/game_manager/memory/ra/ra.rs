use crate::prelude::*;

use crate::game_manager::memory::{EnemyMemory, PlayerMemory};

pub(crate) fn plugin(app: &mut bevy::app::App) {
    app.add_systems(Update, spawn_ra_on_unit_reduction);
    app.add_systems(Update, tick_ra_despawn);
}

#[derive(Component, Default, Prefab)]
#[require(Actor, RequiredCustomMaterial::glitch_snake(), SpriteLayer::Memory)]
pub struct RA;

#[derive(Component)]
pub struct RALifetime {
    pub despawn_timer: Timer,
}

impl RALifetime {
    pub fn new() -> Self {
        Self {
            despawn_timer: Timer::from_seconds(5.0, TimerMode::Once),
        }
    }
}

fn spawn_ra_on_unit_reduction(
    mut commands: Commands,
    mut messages: MessageReader<UnitReductionMessage>,
    battle_status: Res<BattleStatus>,
) {
    for message in messages.read() {
        if matches!(message.percentage_lost, 30 | 60 | 90) {
            let scale = 1.;

            match message.faction {
                Faction::Player => {
                    let position = if let Some(pos) = battle_status.last_player_death_position {
                        Vec3::new(pos.x - 500., pos.y, 0.0)
                    } else {
                        error!(
                            "No player death position recorded! Spawning BigEye at (0,0,0) - Player lost {}%",
                            message.percentage_lost
                        );
                        Vec3::ZERO
                    };

                    commands.spawn((
                        RA,
                        PlayerMemory,
                        Transform::from_translation(position).with_scale(Vec3::splat(scale)),
                        RALifetime::new(),
                    ));
                    info!(
                        "Spawned BigEye with PlayerMemory at {:?} - Player lost {}% ({}/{} remaining)",
                        position,
                        message.percentage_lost,
                        message.remaining_units,
                        message.max_units
                    );
                }
                Faction::Enemy => {
                    let position = if let Some(pos) = battle_status.last_enemy_death_position {
                        Vec3::new(pos.x + 500., pos.y, 0.0)
                    } else {
                        error!(
                            "No enemy death position recorded! Spawning BigEye at (0,0,0) - Enemy lost {}%",
                            message.percentage_lost
                        );
                        Vec3::ZERO
                    };

                    commands.spawn((
                        RA,
                        EnemyMemory,
                        Transform::from_translation(position).with_scale(Vec3::splat(scale)),
                        RALifetime::new(),
                    ));
                    info!(
                        "Spawned BigEye with EnemyMemory at {:?} - Enemy lost {}% ({}/{} remaining)",
                        position,
                        message.percentage_lost,
                        message.remaining_units,
                        message.max_units
                    );
                }
            }
        }
    }
}

fn tick_ra_despawn(
    time: Res<Time>,
    mut commands: Commands,
    mut ra_query: Query<(Entity, &mut RALifetime)>,
) {
    for (entity, mut lifetime) in &mut ra_query {
        lifetime.despawn_timer.tick(time.delta());
        if lifetime.despawn_timer.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}
