use bevy::{prelude::*, sprite_render::MeshMaterial2d};
use rock_materials::GlitchSnakeMaterial;

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
    mut materials: ResMut<Assets<GlitchSnakeMaterial>>,
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

    // Create glitch snake material with default settings
    let snake_material = materials.add(GlitchSnakeMaterial {
        texture: texture_handle.clone(),
        strength: 0.01,  // Base oscillation amplitude
        frequency: 10.0, // Oscillation speed
        ..default()
    });

    // Spawn mesh with glitch snake effect
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(500.0, 500.0))),
        MeshMaterial2d(snake_material.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Spawn a second mesh with stronger effect and slower frequency
    let snake_material_2 = materials.add(GlitchSnakeMaterial {
        texture: texture_handle.clone(),
        strength: 0.02,  // Stronger effect
        frequency: 5.0,  // Slower oscillation
        ..default()
    });

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(400.0, 400.0))),
        MeshMaterial2d(snake_material_2),
        Transform::from_xyz(550.0, 0.0, 0.0),
    ));

    // Spawn a third mesh with very fast frequency
    let snake_material_3 = materials.add(GlitchSnakeMaterial {
        texture: texture_handle.clone(),
        strength: 0.015,
        frequency: 20.0, // Fast oscillation
        ..default()
    });

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(300.0, 300.0))),
        MeshMaterial2d(snake_material_3),
        Transform::from_xyz(-450.0, 0.0, 0.0),
    ));

    // Spawn info text
    commands.spawn((
        Text::new("Glitch Snake Effect\nLeft: Fast (freq=20)\nCenter: Default (freq=10)\nRight: Slow (freq=5)\nOscillating RGB channel separation"),
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
