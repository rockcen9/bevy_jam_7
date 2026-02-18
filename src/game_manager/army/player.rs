use crate::prelude::*;
pub(crate) fn plugin(_app: &mut bevy::app::App) {}

#[derive(Component, Default, Reflect)]
#[require(RequireShadowSprite)]
pub struct Unit;

#[derive(Component, Default, Reflect)]
#[require(Unit, Faction::Player, OriginalColor(Color::WHITE))]
pub struct PlayerUnit;

#[derive(Component, Reflect)]
pub struct OriginalColor(pub Color);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub enum Faction {
    Player,
    Enemy,
}
