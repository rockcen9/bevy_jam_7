use crate::prelude::*;

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(Update, track_squad_hit_count.in_set(AttackSet::TakeDamage));
}

/// Tracks the total number of successful hits dealt by units in a squad.
#[derive(Component, Default, Reflect, Debug)]
pub struct SquadHitCount(pub u32);

fn track_squad_hit_count(
    mut messages: MessageReader<TakeDamageMessage>,
    q_belong: Query<&BelongToSquad>,
    mut q_squad: Query<&mut SquadHitCount>,
) {
    for msg in messages.read() {
        let Some(attacker) = msg.attacker else {
            continue;
        };
        let Ok(belong) = q_belong.get(attacker) else {
            continue;
        };
        let Ok(mut hit_count) = q_squad.get_mut(belong.0) else {
            continue;
        };
        hit_count.0 += 1;
    }
}
