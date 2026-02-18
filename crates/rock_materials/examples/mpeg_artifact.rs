use bevy::{prelude::*, sprite_render::MeshMaterial2d};
use rock_materials::MpegArtifactMaterial;

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
    mut materials: ResMut<Assets<MpegArtifactMaterial>>,
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

    // Create MPEG artifact material with moderate effect
    let mpeg_material = materials.add(MpegArtifactMaterial {
        texture: texture_handle.clone(),
        intensity: 0.7, // Try adjusting 0.0 ~ 1.0
        alpha: 1.0,
    });

    // Spawn mesh with MPEG artifact effect
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(500.0, 500.0))),
        MeshMaterial2d(mpeg_material.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Spawn a second mesh with stronger effect
    let mpeg_material_2 = materials.add(MpegArtifactMaterial {
        texture: texture_handle.clone(),
        intensity: 1.0, // Full intensity
        alpha: 1.0,
    });

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(400.0, 400.0))),
        MeshMaterial2d(mpeg_material_2),
        Transform::from_xyz(550.0, 0.0, 0.0),
    ));

    // Spawn info text
    commands.spawn((
        Text::new("MPEG Artifact Effect\nLeft: Moderate (0.7)\nRight: Strong (1.0)\nNoise-based distortion, scanlines, and chromatic aberration"),
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
