use bevy::{prelude::*, sprite_render::MeshMaterial2d};
use rock_materials::ChromaticAberrationMaterial;

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
    mut materials: ResMut<Assets<ChromaticAberrationMaterial>>,
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

    // Create chromatic aberration material with moderate effect
    let chroma_material = materials.add(ChromaticAberrationMaterial {
        texture: texture_handle.clone(),
        amount: 0.05, // Try adjusting 0.0 ~ 0.1
    });

    // Spawn mesh with chromatic aberration effect
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(500.0, 500.0))),
        MeshMaterial2d(chroma_material.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Spawn a second mesh with stronger effect
    let chroma_material_2 = materials.add(ChromaticAberrationMaterial {
        texture: texture_handle.clone(),
        amount: 0.1, // Stronger effect
    });

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(400.0, 400.0))),
        MeshMaterial2d(chroma_material_2),
        Transform::from_xyz(550.0, 0.0, 0.0),
    ));

    // Spawn info text
    commands.spawn((
        Text::new("Chromatic Aberration Effect\nLeft: Moderate (0.05)\nRight: Strong (0.1)\nAnimated RGB channel separation"),
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
