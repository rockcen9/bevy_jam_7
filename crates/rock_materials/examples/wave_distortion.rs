use bevy::{prelude::*, sprite_render::MeshMaterial2d};
use rock_materials::WaveDistortionMaterial;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(rock_materials::plugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<WaveDistortionMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Spawn camera
    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(Color::srgb(0.02, 0.02, 0.05)),
            ..default()
        },
    ));

    // Load the image texture
    let texture_handle: Handle<Image> = asset_server.load("procreate/BigEye.png");

    // Create wave distortion material with default settings
    let wave_material = materials.add(WaveDistortionMaterial {
        texture: texture_handle.clone(),
        wave_center: Vec2::new(0.5, 0.5), // Center of the texture
        wave_params: Vec3::new(10.0, 0.8, 0.1), // [frequency, falloff, thickness]
        alpha: 1.0,
    });

    // Spawn mesh with wave distortion effect
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(500.0, 500.0))),
        MeshMaterial2d(wave_material.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Spawn a second mesh with different wave parameters
    let wave_material_2 = materials.add(WaveDistortionMaterial {
        texture: texture_handle.clone(),
        wave_center: Vec2::new(0.5, 0.5),
        wave_params: Vec3::new(15.0, 0.6, 0.15), // Faster, weaker falloff, thicker wave
        alpha: 1.0,
    });

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(400.0, 400.0))),
        MeshMaterial2d(wave_material_2),
        Transform::from_xyz(550.0, 0.0, 0.0),
    ));

    // Spawn info text
    commands.spawn((
        Text::new("Wave Distortion Effect\nLeft: Default (10.0, 0.8, 0.1)\nRight: Custom (15.0, 0.6, 0.15)\nPulsing ripple distortion from center"),
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
