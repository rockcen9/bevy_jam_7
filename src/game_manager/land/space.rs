
use crate::prelude::*;

#[derive(Component, Default, Prefab, Reflect)]
#[require(SpriteActor, SpriteLayer::Grid)]
pub struct Space;

#[derive(Component, Default, Prefab, Reflect)]
#[require(SpriteActor, SpriteLayer::Grid)]
pub struct OuterSpace;

pub fn plugin(_app: &mut App) {}
