mod boundary;
pub(crate) use boundary::*;

mod spawn;
mod switch_level;
mod tree;

use bevy::prelude::*;
use bevy_ecs_ldtk::assets::LdtkProject;
pub(crate) use spawn::*;

use crate::asset_tracking::LoadResource;
pub(crate) fn plugin(app: &mut bevy::app::App) {
    app.init_asset::<bevy_ecs_ldtk::prelude::LdtkProject>();
    app.load_resource::<SceneAssets>();
    spawn::plugin(app);
    tree::plugin(app);
    switch_level::plugin(app);
    boundary::plugin(app);
}

/// A [`Resource`] that contains all the assets needed to spawn the level.
/// We use this to preload assets before the level is spawned.
#[derive(Resource, Asset, Clone, TypePath)]
pub(crate) struct SceneAssets {
    #[dependency]
    pub(crate) level: Handle<LdtkProject>,
}

impl FromWorld for SceneAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();

        Self {
            level: assets.load("chaos_dream.ldtk"),
        }
    }
}
