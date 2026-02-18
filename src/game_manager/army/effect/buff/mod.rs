use crate::prelude::*;
mod attack_speed;
mod block;
mod poison;
mod stun;
pub(crate) use attack_speed::*;
pub(crate) use block::*;
pub(crate) use stun::*;
pub(crate) fn plugin(app: &mut bevy::app::App) {
    attack_speed::plugin(app);
    block::plugin(app);
    poison::plugin(app);
    stun::plugin(app);
}

#[derive(Clone, Debug, Reflect)]
pub struct PoisonBuffData {
    pub stacks: u32,
    pub max_stack: u32,
    pub regen_timer: Timer,
}
impl Default for PoisonBuffData {
    fn default() -> Self {
        Self {
            stacks: 0,
            max_stack: 5,
            regen_timer: Timer::from_seconds(5.0, TimerMode::Once),
        }
    }
}
// ==========================================
// ==========================================
#[derive(Clone, Debug, Reflect)]
pub enum BuffEffect {
    Poison(PoisonBuffData),
    Block(BlockBuffData),
    AttackSpeed(AttackSpeedBuffData),
    Stun(StunBuffData),
    Invincible,
}

// ==========================================
// ==========================================
#[derive(Component, Default, Reflect)]
pub struct ActiveBuffs {
    pub list: Vec<BuffEffect>,
}
