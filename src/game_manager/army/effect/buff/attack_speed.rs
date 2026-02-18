use std::time::Duration;

use crate::prelude::*;

pub(crate) fn plugin(app: &mut bevy::app::App) {
    app.add_systems(Update, tick_attack_speed_regen);
    app.add_systems(
        Update,
        apply_attack_speed_buff.after(tick_attack_speed_regen),
    );
}
#[derive(Clone, Debug, Reflect)]
pub struct AttackSpeedBuffData {
    pub stacks: u32,
    pub max_stacks: u32,
    pub regen_timer: Timer,
}
impl Default for AttackSpeedBuffData {
    fn default() -> Self {
        Self {
            stacks: 0,
            max_stacks: 5,
            regen_timer: Timer::from_seconds(5.0, TimerMode::Once),
        }
    }
}
/// Ticks the attack speed buff regen timer and regenerates stacks over time.
fn tick_attack_speed_regen(time: Res<Time>, mut query: Query<&mut ActiveBuffs>) {
    for mut buffs in &mut query {
        for buff in &mut buffs.list {
            if let BuffEffect::AttackSpeed(data) = buff {
                data.regen_timer.tick(time.delta());
                if data.regen_timer.just_finished() {
                    if data.stacks < data.max_stacks {
                        data.stacks += 1;
                    }
                    data.regen_timer.reset();
                }
            }
        }
    }
}

/// Applies attack speed buff: each stack reduces attack timer duration by 5%.
fn apply_attack_speed_buff(mut query: Query<(&ActiveBuffs, &UnitStats, &mut AttackTimer)>) {
    for (buffs, stats, mut attack_timer) in &mut query {
        let mut total_stacks = 0u32;
        for buff in &buffs.list {
            if let BuffEffect::AttackSpeed(data) = buff {
                total_stacks += data.stacks;
            }
        }

        // 5% faster per stack: new_duration = base_duration * (1 - 0.05 * stacks)
        let multiplier = (1.0 - 0.05 * total_stacks as f32).max(0.25);
        let new_duration = stats.attack_speed * multiplier;
        attack_timer
            .0
            .set_duration(Duration::from_secs_f32(new_duration));
    }
}
