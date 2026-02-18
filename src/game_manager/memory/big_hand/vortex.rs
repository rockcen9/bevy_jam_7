use bevy_rand::{global::GlobalRng, prelude::ChaCha8Rng};
use rand::Rng;

use crate::prelude::*;

use crate::game_manager::memory::{EnemyMemory, PlayerMemory};

use super::BigHand;

const VORTEX_RANGE: f32 = 200.0;
const VORTEX_DAMAGE: f32 = 5.0;

pub(crate) fn plugin(app: &mut bevy::app::App) {
    app.add_systems(Update, spawn_vortex_on_big_hand_appear);
    app.add_systems(Update, tick_big_hand_vortex);
    app.add_systems(Update, tick_vortex_damage);
}

#[derive(Component)]
pub struct BigHandVortexTimer(Timer);

impl BigHandVortexTimer {
    pub fn new() -> Self {
        Self(Timer::from_seconds(3.0, TimerMode::Repeating))
    }
}

/// Invisible game-logic entity that lives as long as the vortex is active (3s).
/// Queries nearby units and damages them every 0.5s.
#[derive(Component)]
struct VortexDamageZone {
    damage_timer: Timer,
    lifetime: Timer,
    /// Which faction to damage
    target_faction: Faction,
}

impl VortexDamageZone {
    fn new(target_faction: Faction) -> Self {
        Self {
            damage_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
            lifetime: Timer::from_seconds(3.0, TimerMode::Once),
            target_faction,
        }
    }
}

fn spawn_vortex_at(commands: &mut Commands, position: Vec2, target_faction: Faction, label: &str) {
    debug!(
        "[BigHand vortex] {}: {:?} -> vortex at {:?}",
        label, target_faction, position
    );
    commands.trigger(VfxEvent::vortex(position));
    commands.spawn((
        Name::new("VortexDamageZone"),
        Transform::from_translation(position.extend(0.0)),
        VortexDamageZone::new(target_faction),
    ));
}

fn spawn_vortex_on_big_hand_appear(
    mut commands: Commands,
    new_big_hand: Query<(Entity, Has<PlayerMemory>, Has<EnemyMemory>), Added<BigHand>>,
    player_units: Query<&GlobalTransform, With<PlayerUnit>>,
    enemy_units: Query<&GlobalTransform, With<EnemyUnit>>,
    mut rng: Single<&mut ChaCha8Rng, With<GlobalRng>>,
) {
    for (entity, is_player_memory, is_enemy_memory) in &new_big_hand {
        commands.entity(entity).insert(BigHandVortexTimer::new());

        if is_player_memory {
            let positions: Vec<Vec2> = enemy_units
                .iter()
                .map(|t| t.translation().truncate())
                .collect();
            if !positions.is_empty() {
                let idx = rng.random_range(0..positions.len());
                spawn_vortex_at(
                    &mut commands,
                    positions[idx],
                    Faction::Enemy,
                    "spawn on appear PlayerMemory",
                );
            }
        } else if is_enemy_memory {
            let positions: Vec<Vec2> = player_units
                .iter()
                .map(|t| t.translation().truncate())
                .collect();
            if !positions.is_empty() {
                let idx = rng.random_range(0..positions.len());
                spawn_vortex_at(
                    &mut commands,
                    positions[idx],
                    Faction::Player,
                    "spawn on appear EnemyMemory",
                );
            }
        }
    }
}

fn tick_big_hand_vortex(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(&mut BigHandVortexTimer, Has<PlayerMemory>, Has<EnemyMemory>)>,
    player_units: Query<&GlobalTransform, With<PlayerUnit>>,
    enemy_units: Query<&GlobalTransform, With<EnemyUnit>>,
    mut rng: Single<&mut ChaCha8Rng, With<GlobalRng>>,
) {
    for (mut timer, is_player_memory, is_enemy_memory) in &mut query {
        timer.0.tick(time.delta());
        if !timer.0.just_finished() {
            continue;
        }

        if is_player_memory {
            let positions: Vec<Vec2> = enemy_units
                .iter()
                .map(|t| t.translation().truncate())
                .collect();
            if !positions.is_empty() {
                let idx = rng.random_range(0..positions.len());
                spawn_vortex_at(
                    &mut commands,
                    positions[idx],
                    Faction::Enemy,
                    "periodic tick PlayerMemory",
                );
            }
        } else if is_enemy_memory {
            let positions: Vec<Vec2> = player_units
                .iter()
                .map(|t| t.translation().truncate())
                .collect();
            if !positions.is_empty() {
                let idx = rng.random_range(0..positions.len());
                spawn_vortex_at(
                    &mut commands,
                    positions[idx],
                    Faction::Player,
                    "periodic tick EnemyMemory",
                );
            }
        }
    }
}

fn tick_vortex_damage(
    time: Res<Time>,
    mut commands: Commands,
    mut zones: Query<(Entity, &Transform, &mut VortexDamageZone)>,
    player_units: Query<(Entity, &GlobalTransform), With<PlayerUnit>>,
    enemy_units: Query<(Entity, &GlobalTransform), With<EnemyUnit>>,
) {
    for (zone_entity, zone_transform, mut zone) in &mut zones {
        zone.lifetime.tick(time.delta());
        if zone.lifetime.just_finished() {
            commands.entity(zone_entity).despawn();
            continue;
        }

        zone.damage_timer.tick(time.delta());
        if !zone.damage_timer.just_finished() {
            continue;
        }

        let zone_pos = zone_transform.translation.truncate();

        let targets: Vec<Entity> = match zone.target_faction {
            Faction::Enemy => enemy_units
                .iter()
                .filter(|(_, gt)| gt.translation().truncate().distance(zone_pos) <= VORTEX_RANGE)
                .map(|(e, _)| e)
                .collect(),
            Faction::Player => player_units
                .iter()
                .filter(|(_, gt)| gt.translation().truncate().distance(zone_pos) <= VORTEX_RANGE)
                .map(|(e, _)| e)
                .collect(),
        };

        for target in targets {
            debug!(
                "[VortexDamageZone] dealing {VORTEX_DAMAGE} to {:?} at {:?}",
                target, zone_pos
            );
            commands.trigger(AttackEvent::new(zone_entity, target, VORTEX_DAMAGE));
        }
    }
}
