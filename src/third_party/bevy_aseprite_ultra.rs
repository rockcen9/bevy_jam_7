use bevy_aseprite_ultra::AsepriteUltraPlugin;

use crate::prelude::*;
pub fn plugin(app: &mut App) {
    app.add_plugins(AsepriteUltraPlugin);
}
