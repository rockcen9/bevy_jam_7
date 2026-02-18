use crate::prelude::*;

pub(crate) fn plugin(app: &mut bevy::app::App) {
    app.add_systems(Update, tick_block_regen);
}

#[derive(Clone, Debug, Reflect)]
pub struct BlockBuffData {
    pub current_stacks: u32,
    pub max_stacks: u32,
    pub regen_timer: Timer,
}
impl Default for BlockBuffData {
    fn default() -> Self {
        Self {
            current_stacks: 0,
            max_stacks: 5,
            regen_timer: Timer::from_seconds(5.0, TimerMode::Once),
        }
    }
}
fn tick_block_regen(time: Res<Time>, mut query: Query<&mut ActiveBuffs>) {
    for mut buffs in &mut query {
        for buff in &mut buffs.list {
            if let BuffEffect::Block(data) = buff {
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
