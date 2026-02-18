use bevy::{prelude::*, sprite_render::MeshMaterial2d};
use rock_materials::GlitchMaterial;

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
    mut materials: ResMut<Assets<GlitchMaterial>>,
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

    // Create glitch material with low intensity
    let glitch_material_low = materials.add(GlitchMaterial {
        texture: texture_handle.clone(),
        glitch_amount: 0.2, // Low glitch
        alpha: 1.0,
    });

    // Spawn mesh with low glitch effect
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(400.0, 400.0))),
        MeshMaterial2d(glitch_material_low),
        Transform::from_xyz(-300.0, 0.0, 0.0),
    ));

    // Create glitch material with medium intensity
    let glitch_material_med = materials.add(GlitchMaterial {
        texture: texture_handle.clone(),
        glitch_amount: 0.5, // Medium glitch
        alpha: 1.0,
    });

    // Spawn mesh with medium glitch effect
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(400.0, 400.0))),
        MeshMaterial2d(glitch_material_med),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Create glitch material with high intensity
    let glitch_material_high = materials.add(GlitchMaterial {
        texture: texture_handle.clone(),
        glitch_amount: 0.8, // High glitch
        alpha: 1.0,
    });

    // Spawn mesh with high glitch effect
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(400.0, 400.0))),
        MeshMaterial2d(glitch_material_high),
        Transform::from_xyz(300.0, 0.0, 0.0),
    ));

    // Spawn info text
    commands.spawn((
        Text::new("Glitch Effect\nLeft: Low (0.2)\nCenter: Medium (0.5)\nRight: High (0.8)\nChromatic aberration with random displacement"),
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
