use bevy::prelude::*;
use rock_particles::{ComboRingColor, ComboRingEvent};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Hanabi VFX - Combo Ring".to_string(),
                resolution: (1280, 720).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(rock_particles::plugin)
        .add_systems(Startup, setup)
        .add_systems(Update, spawn_effect)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Text::new(
            "Combo Ring Effect\n\n\
            Click       - Purple ring at cursor\n\
            Right Click - Pink ring at cursor\n\
            Press SPACE - Purple ring at center\n\
            Press R     - 3 rings in formation\n\
            Press ESC   - Exit\n\n\
            Ring bursts outward then pulled inward.",
        ),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        TextColor(Color::WHITE),
        TextFont {
            font_size: 20.0,
            ..default()
        },
    ));
}

fn spawn_effect(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    window_q: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        std::process::exit(0);
    }

    let Ok(window) = window_q.single() else {
        return;
    };
    let Ok((camera, transform)) = camera_q.single() else {
        return;
    };

    let cursor_world_pos: Vec2 = window
        .cursor_position()
        .and_then(|pos| camera.viewport_to_world_2d(transform, pos).ok())
        .unwrap_or(Vec2::ZERO);

    // Purple ring at cursor
    if mouse.just_pressed(MouseButton::Left) {
        info!("Spawning purple combo ring at {:?}", cursor_world_pos);
        commands.trigger(ComboRingEvent::purple(cursor_world_pos));
    }

    // Pink ring at cursor
    if mouse.just_pressed(MouseButton::Right) {
        info!("Spawning pink combo ring at {:?}", cursor_world_pos);
        commands.trigger(ComboRingEvent::pink(cursor_world_pos));
    }

    // Purple ring at center
    if keyboard.just_pressed(KeyCode::Space) {
        info!("Spawning purple combo ring at center");
        commands.trigger(ComboRingEvent::purple(Vec2::ZERO));
    }

    // Multiple rings in a triangle formation - alternating colors
    if keyboard.just_pressed(KeyCode::KeyR) {
        info!("Spawning 3 combo rings in formation");
        let spacing = 150.0;
        let positions_colors = vec![
            (Vec2::new(0.0, spacing), ComboRingColor::Purple),
            (Vec2::new(-spacing, -spacing), ComboRingColor::Pink),
            (Vec2::new(spacing, -spacing), ComboRingColor::Purple),
        ];

        for (pos, color) in positions_colors {
            commands.trigger(ComboRingEvent::new(pos, color));
        }
    }
}
