use crate::prelude::*;

pub(crate) fn plugin(app: &mut bevy::app::App) {
    app.add_systems(Update, tick_poison_regen);
    app.add_observer(on_attack_apply_poison);
}

/// Ticks the poison buff regen timer and regenerates stacks over time.
fn tick_poison_regen(time: Res<Time>, mut query: Query<&mut ActiveBuffs>) {
    for mut buffs in &mut query {
        for buff in &mut buffs.list {
            if let BuffEffect::Poison(data) = buff {
                data.regen_timer.tick(time.delta());
                if data.regen_timer.just_finished() {
                    if data.stacks < data.max_stack {
                        data.stacks += 1;
                    }
                    data.regen_timer.reset();
                }
            }
        }
    }
}

/// When an attacker with poison buff attacks, apply poison debuff to target.
/// Debuff stacks are set based on the attacker's buff stacks.
fn on_attack_apply_poison(
    trigger: On<AttackEvent>,
    q_attacker: Query<&ActiveBuffs>,
    mut q_target: Query<&mut ActiveDeBuffs>,
) {
    let attacker = trigger._from;
    let target = trigger.to;

    let Ok(attacker_buffs) = q_attacker.get(attacker) else {
        return;
    };

    let mut poison_stacks = 0;
    for buff in &attacker_buffs.list {
        if let BuffEffect::Poison(data) = buff {
            poison_stacks = data.stacks;
            break;
        }
    }

    if poison_stacks == 0 {
        return;
    }

    let Ok(mut target_debuffs) = q_target.get_mut(target) else {
        return;
    };

    // If target already has poison debuff, add stacks (capped at max)
    for debuff in &mut target_debuffs.list {
        if let DebuffEffect::Poison(data) = debuff {
            data.stacks = (data.stacks + poison_stacks).min(data.max_stacks);
            data.timer.reset();
            return;
        }
    }

    // Add new poison debuff with stacks matching buff stacks
    target_debuffs
        .list
        .push(DebuffEffect::Poison(PoisonDeBuffData {
            stacks: poison_stacks,
            ..Default::default()
        }));
}
