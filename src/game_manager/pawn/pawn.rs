use smol_str::SmolStr;

use crate::prelude::*;

#[derive(Component, Default)]
#[require(
    SpriteLayer::Pawn,
    UnitState::Idle,
    UnitCollider,
    Target,
    AttackTimer::new(1.),
    ActiveBuffs::default(),
    ActiveDeBuffs::default()
)]
pub struct Pawn;

#[derive(Component)]
pub struct UnitGameName(pub SmolStr);
