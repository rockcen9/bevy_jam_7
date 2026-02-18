mod poison;
mod stun;

use crate::prelude::*;
pub(crate) fn plugin(app: &mut bevy::app::App) {
    poison::plugin(app);
    stun::plugin(app);
}

#[derive(Clone, Debug, Reflect)]
pub struct PoisonDeBuffData {
    pub stacks: u32,
    pub max_stacks: u32,
    pub damage_per_tick: f32,
    pub timer: Timer,
}
impl Default for PoisonDeBuffData {
    fn default() -> Self {
        Self {
            stacks: 0,
            max_stacks: 5,
            damage_per_tick: 1.0,
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        }
    }
}
#[derive(Clone, Debug, Reflect)]
pub struct StunDeBuffData {
    pub stunning: bool,
    pub duration: Timer,
}

// ==========================================
// ==========================================
#[derive(Clone, Debug, Reflect)]
pub enum DebuffEffect {
    Poison(PoisonDeBuffData),
    Stun(StunDeBuffData),
}

// ==========================================
// ==========================================
#[derive(Component, Default, Reflect)]
pub struct ActiveDeBuffs {
    pub list: Vec<DebuffEffect>,
}
