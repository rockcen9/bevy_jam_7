use crate::prelude::*;

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        track_squad_take_hit_count.in_set(AttackSet::TakeDamage),
    );
}

/// Tracks the total number of hits received by units in a squad.
#[derive(Component, Default, Reflect, Debug)]
pub struct SquadTakeHitCount(pub u32);

fn track_squad_take_hit_count(
    mut messages: MessageReader<TakeDamageMessage>,
    q_belong: Query<&BelongToSquad>,
    mut q_squad: Query<&mut SquadTakeHitCount>,
) {
    for msg in messages.read() {
        let Ok(belong) = q_belong.get(msg.target) else {
            continue;
        };
        let Ok(mut count) = q_squad.get_mut(belong.0) else {
            continue;
        };
        count.0 += 1;
    }
}
