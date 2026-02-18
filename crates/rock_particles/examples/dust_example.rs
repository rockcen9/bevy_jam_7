use bevy::prelude::*;
use rock_particles::VfxEvent;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Haniba VFX - Dust Cloud".to_string(),
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
    // Spawn camera
    commands.spawn(Camera2d);

    // Spawn instruction text
    commands.spawn((
        Text::new(
            "Dust Cloud Effect\n\n\
            Press SPACE - Spawn effect at cursor\n\
            Press ESC - Exit",
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
    window_q: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
) {
    // Exit on ESC
    if keyboard.just_pressed(KeyCode::Escape) {
        std::process::exit(0);
    }

    // Get cursor position for spawning effects
    let Ok(window) = window_q.single() else {
        return;
    };
    let Ok((camera, transform)) = camera_q.single() else {
        return;
    };

    let cursor_world_pos: Vec2 = window
        .cursor_position()
        .and_then(|cursor_pos: Vec2| camera.viewport_to_world_2d(transform, cursor_pos).ok())
        .unwrap_or(Vec2::ZERO);

    // Spawn dust effect
    if keyboard.just_pressed(KeyCode::Space) {
        info!("Spawning dust effect at {:?}", cursor_world_pos);
        commands.trigger(VfxEvent::dust(cursor_world_pos));
    }
}
