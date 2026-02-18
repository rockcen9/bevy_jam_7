use bevy_rand::{global::GlobalRng, prelude::ChaCha8Rng};
use rand::Rng;

use crate::prelude::*;

pub(crate) fn plugin(app: &mut bevy::app::App) {
    app.add_systems(Update, tick_stun_regen);
    app.add_observer(on_attack_apply_stun);
}

#[derive(Clone, Debug, Reflect)]
pub struct StunBuffData {
    pub current_stacks: u32,
    pub max_stacks: u32,
    pub regen_timer: Timer,
}
impl Default for StunBuffData {
    fn default() -> Self {
        Self {
            current_stacks: 0,
            max_stacks: 5,
            regen_timer: Timer::from_seconds(5.0, TimerMode::Once),
        }
    }
}
/// Ticks the stun buff regen timer and regenerates stacks over time.
fn tick_stun_regen(time: Res<Time>, mut query: Query<&mut ActiveBuffs>) {
    for mut buffs in &mut query {
        for buff in &mut buffs.list {
            if let BuffEffect::Stun(data) = buff {
                data.regen_timer.tick(time.delta());
                if data.regen_timer.just_finished() {
                    if data.current_stacks < data.max_stacks {
                        data.current_stacks += 1;
                    }
                    data.regen_timer.reset();
                }
            }
        }
    }
}

/// When an attacker with stun buff attacks, each stack gives 5% chance to stun the target.
fn on_attack_apply_stun(
    trigger: On<AttackEvent>,
    mut rng: Single<&mut ChaCha8Rng, With<GlobalRng>>,
    mut q_attacker: Query<&mut ActiveBuffs>,
    mut q_target: Query<&mut ActiveDeBuffs>,
) {
    let attacker = trigger._from;
    let target = trigger.to;

    // Check if attacker has stun buff
    let Ok(mut attacker_buffs) = q_attacker.get_mut(attacker) else {
        return;
    };

    let mut stun_stacks = 0;
    for buff in &mut attacker_buffs.list {
        if let BuffEffect::Stun(data) = buff {
            stun_stacks = data.current_stacks;
            break;
        }
    }

    if stun_stacks == 0 {
        return;
    }

    // 5% chance per stack
    let stun_chance = stun_stacks as f32 * 0.05;
    if rng.random::<f32>() >= stun_chance {
        return;
    }

    // Apply stun debuff to target
    let Ok(mut target_debuffs) = q_target.get_mut(target) else {
        return;
    };

    // Check if target already has a stun debuff, if so reset the duration
    for debuff in &mut target_debuffs.list {
        if let DebuffEffect::Stun(data) = debuff {
            data.stunning = true;
            data.duration.reset();
            return;
        }
    }

    // Add new stun debuff
    target_debuffs.list.push(DebuffEffect::Stun(StunDeBuffData {
        stunning: true,
        duration: Timer::from_seconds(1.0, TimerMode::Once),
    }));
}
