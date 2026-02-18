use crate::prelude::*;
mod attack;

#[derive(Component, Default, Prefab, Reflect)]
#[require(
    UnitGameName("x".into()),
    SpriteActor,
    Pawn,
    Melee,
    UnitStats::melee(UnitKind::Shield),
    Health::new_full(100.),
    ActiveBuffs { list: vec![BuffEffect::Block(BlockBuffData::default())] }
)]
pub struct Shield;

pub(crate) fn plugin(app: &mut bevy::app::App) {
    attack::plugin(app);
}
