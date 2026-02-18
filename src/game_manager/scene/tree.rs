use bevy_ecs_ldtk::app::LdtkEntityAppExt;

use crate::prelude::*;

#[derive(Component, Prefab)]
#[require(SpriteActor)]
pub struct Tree;

pub(crate) fn plugin(app: &mut App) {
    app.register_ldtk_entity::<LDTKBundle>("Tree");
}
