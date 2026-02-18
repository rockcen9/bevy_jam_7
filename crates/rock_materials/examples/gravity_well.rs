use bevy::{prelude::*, sprite_render::MeshMaterial2d};
use rock_materials::GravityWellMaterial;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Gravity Well Warp Effect".to_string(),
                resolution: (1280, 720).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(rock_materials::plugin)
        .add_systems(Startup, setup)
        .add_systems(Update, handle_input)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Text::new(
            "Gravity Well Warp Effect\n\n\
            Inverse-square gravitational lensing\n\
            Multi-well interference warping\n\
            Animated orbiting gravity wells\n\n\
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

fn handle_input(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    window_q: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<GravityWellMaterial>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        std::process::exit(0);
    }

    if keyboard.just_pressed(KeyCode::Space) {
        let Ok(window) = window_q.single() else {
            return;
        };
        let Ok((camera, transform)) = camera_q.single() else {
            return;
        };

        let pos: Vec2 = window
            .cursor_position()
            .and_then(|p| camera.viewport_to_world_2d(transform, p).ok())
            .unwrap_or(Vec2::ZERO);

        let material = materials.add(GravityWellMaterial {
            time: 0.0,
            center_position: pos,
            base_texture: None,
        });

        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(1920.0, 1080.0))),
            MeshMaterial2d(material),
            Transform::from_xyz(pos.x, pos.y, 100.0),
        ));
    }
}
