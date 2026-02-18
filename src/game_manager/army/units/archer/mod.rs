use crate::prelude::*;

mod attack;

pub(crate) fn plugin(app: &mut bevy::app::App) {
    attack::plugin(app);
}

#[derive(Component, Default, Prefab, Reflect)]
#[require(
    SpriteActor,
    UnitGameName("x".into()),
    Pawn,
    Ranged,
    UnitStats::ranged(UnitKind::Archer),
    Health::new_full(30.),
    ActiveBuffs{list: vec![BuffEffect::Poison(PoisonBuffData{stacks: 0, max_stack: 5, regen_timer: Timer::from_seconds(5.0, TimerMode::Once)})]},
)]
pub struct Archer;
