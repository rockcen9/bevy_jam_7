use crate::prelude::*;

pub(crate) fn plugin(app: &mut bevy::app::App) {
    app.add_systems(
        Update,
        (tick_stun_debuff, prevent_attack_when_stunned.before(AttackSet::Attack)),
    );
}

/// Ticks the stun debuff duration timer. When it finishes, clears the stun.
fn tick_stun_debuff(time: Res<Time>, mut query: Query<&mut ActiveDeBuffs>) {
    for mut debuffs in &mut query {
        for debuff in &mut debuffs.list {
            if let DebuffEffect::Stun(data) = debuff {
                if !data.stunning {
                    continue;
                }
                data.duration.tick(time.delta());
                if data.duration.just_finished() {
                    data.stunning = false;
                }
            }
        }
    }
}

/// Prevents stunned units from attacking by forcing them out of Attacking state.
fn prevent_attack_when_stunned(mut query: Query<(&ActiveDeBuffs, &mut UnitState)>) {
    for (debuffs, mut state) in &mut query {
        if *state != UnitState::Attacking {
            continue;
        }
        let stunned = debuffs.list.iter().any(|d| matches!(d, DebuffEffect::Stun(data) if data.stunning));
        if stunned {
            *state = UnitState::Idle;
        }
    }
}
