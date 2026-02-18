use bevy::prelude::*;
use rock_particles::VfxEvent;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Hanabi VFX - Explosion".to_string(),
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
            "Explosion Effect\n\n\
            Click       - Spawn explosion at cursor\n\
            Press SPACE - Spawn explosion at center\n\
            Press ESC   - Exit",
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

    if mouse.just_pressed(MouseButton::Left) {
        info!("Spawning explosion at {:?}", cursor_world_pos);
        commands.trigger(VfxEvent::explosion(cursor_world_pos));
    }

    if keyboard.just_pressed(KeyCode::Space) {
        info!("Spawning explosion at center");
        commands.trigger(VfxEvent::explosion(Vec2::ZERO));
    }
}
