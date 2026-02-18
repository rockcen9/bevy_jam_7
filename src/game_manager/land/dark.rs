
use crate::prelude::*;

#[derive(Component, Default, Prefab, Reflect)]
#[require(SpriteActor, SpriteLayer::Dark)]
pub struct Dark;

pub(crate) fn plugin(_app: &mut bevy::app::App) {}
