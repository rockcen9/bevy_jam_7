use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin, FrameTimeGraphConfig},
    text::FontSmoothing,
};

use crate::prelude::*;

pub(crate) fn plugin(app: &mut App) {
    app.add_plugins(FpsOverlayPlugin {
        config: FpsOverlayConfig {
            text_config: TextFont {
                // Here we define size of our overlay
                font_size: 42.0,
                // If we want, we can use a custom font
                font: default(),
                // We could also disable font smoothing,
                font_smoothing: FontSmoothing::default(),
                ..default()
            },
            // We can also change color of the overlay
            text_color: OverlayColor::GREEN,
            // We can also set the refresh interval for the FPS counter
            refresh_interval: core::time::Duration::from_millis(100),
            enabled: false,
            frame_time_graph_config: FrameTimeGraphConfig {
                enabled: false,
                // The minimum acceptable fps
                min_fps: 30.0,
                // The target fps
                target_fps: 144.0,
            },
        },
    });
    app.add_systems(Update, toggle_fps_panel);
}
struct OverlayColor;

impl OverlayColor {
    const GREEN: Color = Color::srgb(0.0, 1.0, 0.0);
}
fn toggle_fps_panel(keyboard: Res<ButtonInput<KeyCode>>, mut overlay: ResMut<FpsOverlayConfig>) {
    if keyboard.pressed(KeyCode::SuperLeft) && keyboard.just_pressed(KeyCode::KeyF) {
        overlay.frame_time_graph_config.enabled = !overlay.frame_time_graph_config.enabled;
        overlay.enabled = !overlay.enabled;
    }
}
