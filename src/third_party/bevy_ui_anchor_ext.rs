use bevy_ui_anchor::AnchorUiPlugin;

use crate::prelude::*;
pub fn plugin(app: &mut App) {
    app.add_plugins(AnchorUiPlugin::<WorldUIMarker>::new());
}

#[derive(Component)]
/// We need a marker for the camera, so the plugin knows which camera to perform position
/// calculations towards
pub struct WorldUIMarker;
