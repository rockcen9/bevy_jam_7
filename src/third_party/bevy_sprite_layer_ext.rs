use extol_sprite_layer::{SpriteLayerOptions, SpriteLayerPlugin};

use crate::prelude::*;
pub(crate) fn plugin(app: &mut bevy::app::App) {
    app.add_plugins(SpriteLayerPlugin::<SpriteLayer>::default());
    app.insert_resource(SpriteLayerOptions { y_sort: false });
}
