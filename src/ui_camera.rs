//! The UI camera is a 2D camera that renders all UI elements in front of everything else.
//! We use a dedicated camera for this because our other two cameras, namely the world and view model cameras,
//! don't exist during non-gameplay screens such as the main menu.

use bevy::camera::visibility::RenderLayers;

use crate::prelude::*;

pub(super) fn plugin(_app: &mut App) {
    // app.add_systems(Startup, spawn_ui_camera);
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub(crate) struct UiCamera;

fn _spawn_ui_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("UI Camera"),
        UiCamera,
        Camera2d,
        IsDefaultUiCamera,
        // UiCameraMarker,
        RenderLayers::layer(1),
        Camera {
            order: 100,

            clear_color: ClearColorConfig::None,

            ..default()
        },
    ));
}
