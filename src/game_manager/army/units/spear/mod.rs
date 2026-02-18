use crate::prelude::*;

#[derive(Component, Default, Prefab, Reflect)]
#[require(
    SpriteActor,
    UnitGameName("x".into()),
    Pawn,
    Melee,
    UnitStats::melee(UnitKind::Spear),
    Health::new_full(50.),
    ActiveBuffs{list: vec![BuffEffect::AttackSpeed(AttackSpeedBuffData{stacks: 1, max_stacks: 5, regen_timer: Timer::from_seconds(5.0, TimerMode::Once)})]}
)]
pub struct Spear;

pub(crate) fn plugin(_app: &mut bevy::app::App) {}
