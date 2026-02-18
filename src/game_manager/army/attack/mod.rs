use bevy_rand::{global::GlobalRng, prelude::ChaCha8Rng};
use rand::Rng;

use crate::prelude::*;
mod corpse;
pub(crate) use corpse::*;
// ============================================================================
// Plugin
// ============================================================================

pub(crate) fn plugin(app: &mut bevy::app::App) {
    // Configure system ordering: Attack -> TakeDamage -> Death -> DeathRecord -> DeathDespawn
    // Run after movement systems
    app.configure_sets(
        Update,
        (
            AttackSet::Attack,
            AttackSet::TakeDamage,
            AttackSet::Corpse,
            AttackSet::Death,
            AttackSet::DeathRecord,
            AttackSet::DeathDespawn,
        )
            .chain()
            .after(MovementSet::Separation),
    );

    app.add_message::<TakeDamageMessage>();
    app.add_message::<UnitDeathMessage>();

    app.add_systems(
        Update,
        (
            take_damage_system.in_set(AttackSet::TakeDamage),
            death_message_system.in_set(AttackSet::Death),
            death_despawn_system.in_set(AttackSet::DeathDespawn),
        ),
    );
    app.add_observer(been_attack);

    corpse::plugin(app);
}

#[derive(Event)]
pub struct AttackEvent {
    pub _from: Entity,
    pub to: Entity,
    pub damage: f32,
}

/// Event emitted when a unit dies. Used for tracking kills.
#[derive(Message)]
pub struct UnitDeathMessage {
    pub _entity: Entity,
    pub is_enemy: bool,
    pub position: Vec2,
}

impl AttackEvent {
    pub fn new(from: Entity, to: Entity, damage: f32) -> Self {
        Self {
            _from: from,
            to,
            damage,
        }
    }
}

#[derive(Message)]
pub struct TakeDamageMessage {
    pub attacker: Option<Entity>,
    pub target: Entity,
    pub damage: f32,
}

// ============================================================================
// Components
// ============================================================================

/// Timer for attack cooldowns.
#[derive(Component, Reflect)]
pub struct AttackTimer(pub Timer);

impl AttackTimer {
    pub fn new(attack_speed: f32) -> Self {
        Self(Timer::from_seconds(attack_speed, TimerMode::Repeating))
    }
}

// ============================================================================
// System Sets for Ordering
// ============================================================================

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AttackSet {
    Attack,
    TakeDamage,
    Corpse,
    Death,
    DeathRecord,
    DeathDespawn,
}

// ============================================================================
// Systems
// ============================================================================

/// Sends death messages for units with zero health.
/// This runs BEFORE other systems process the death (like squad tracking).
fn death_message_system(
    q_units: Query<(Entity, &Health, &GlobalTransform, Option<&EnemyUnit>)>,
    mut ev_death: MessageWriter<UnitDeathMessage>,
) {
    for (entity, health, global_transform, enemy_unit) in &q_units {
        if !health.is_alive() {
            ev_death.write(UnitDeathMessage {
                _entity: entity,
                is_enemy: enemy_unit.is_some(),
                position: global_transform.translation().truncate(),
            });
        }
    }
}

/// Despawns units with zero health.
/// This runs AFTER all death recording systems have processed the death message.
fn death_despawn_system(q_units: Query<(Entity, &Health)>, mut commands: Commands) {
    for (entity, health) in &q_units {
        if !health.is_alive() {
            commands.entity(entity).despawn();
        }
    }
}
fn been_attack(
    trigger: On<AttackEvent>,
    q_stats: Query<&UnitStats>,
    q_belong: Query<&BelongToSquad>,
    q_bonus: Query<&BigEyeDamageBonus>,
    mut ev_damage: MessageWriter<TakeDamageMessage>,
) {
    let counter_mult = if let (Ok(attacker_stats), Ok(target_stats)) =
        (q_stats.get(trigger._from), q_stats.get(trigger.to))
    {
        if attacker_stats.counter == Some(target_stats.unity_kind) {
            1.2
        } else {
            1.0
        }
    } else {
        1.0
    };

    let big_eye_mult = q_belong
        .get(trigger._from)
        .ok()
        .and_then(|b| q_bonus.get(b.0).ok())
        .map(|bonus| 1.0 + bonus.0)
        .unwrap_or(1.0);

    ev_damage.write(TakeDamageMessage {
        attacker: Some(trigger._from),
        target: trigger.to,
        damage: trigger.damage * counter_mult * big_eye_mult,
    });
}

fn take_damage_system(
    mut ev_damage: MessageReader<TakeDamageMessage>,
    mut q_health: Query<(&mut Health, &UnitStats, Option<&mut ActiveBuffs>)>,
    q_transform: Query<&GlobalTransform>,
    mut commands: Commands,
    mut rng: Single<&mut ChaCha8Rng, With<GlobalRng>>,
) {
    for ev in ev_damage.read() {
        if let Ok((mut health, stats, active_buffs)) = q_health.get_mut(ev.target) {
            // Damage = max(1, Final Atk - Final Def)
            let base_damage = (ev.damage - stats.defense).max(1.0);

            // Block chance: 5% per stack, blocks reduce damage by 50%
            let block_mult = if let Some(mut buffs) = active_buffs {
                let mut blocked = false;
                for buff in &mut buffs.list {
                    if let BuffEffect::Block(data) = buff {
                        if data.current_stacks > 0 {
                            let block_chance = data.current_stacks as f32 * 0.05;
                            if rng.random::<f32>() < block_chance {
                                blocked = true;
                                data.current_stacks -= 1;
                                data.regen_timer.reset();
                                break;
                            }
                        }
                    }
                }
                if blocked { 0.5 } else { 1.0 }
            } else {
                1.0
            };

            let actual_damage = base_damage * block_mult;
            health.take_damage(actual_damage);
            let hit_pos = q_transform
                .get(ev.target)
                .map(|t| t.translation().truncate())
                .unwrap_or(Vec2::ZERO);
            commands.trigger(SFXEvent::space("hit", hit_pos).with_random_pitch(0.8, 1.2));
            commands.trigger(HitFlashVfxEvent { target: ev.target });
        }
    }
}
