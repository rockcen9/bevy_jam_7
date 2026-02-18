use crate::prelude::*;

#[derive(Component, Default, Prefab, Reflect)]
#[require(
    UnitGameName("x".into()),
    SpriteActor,
    Pawn,
    Melee,
    UnitStats::melee(UnitKind::Cavalry),
    Health::new_full(80.),
    ActiveBuffs { list: vec![BuffEffect::Stun(StunBuffData::default())] }
)]
pub struct Cavalry;

pub(crate) fn plugin(_app: &mut bevy::app::App) {}
