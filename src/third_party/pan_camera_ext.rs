use bevy::camera_controller::pan_camera::PanCameraPlugin;

use crate::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins(PanCameraPlugin);
}
