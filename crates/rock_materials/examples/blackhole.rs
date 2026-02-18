use bevy::{prelude::*, sprite_render::MeshMaterial2d};
use rock_materials::BlackholeMaterial;

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
    mut materials: ResMut<Assets<BlackholeMaterial>>,
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
    let texture_handle: Handle<Image> = asset_server.load("procreate/RA.png");

    // Create blackhole material with moderate distortion
    let blackhole_material = materials.add(BlackholeMaterial {
        texture: texture_handle.clone(),
        distortion_strength: 0.3, // Try adjusting 0.1 ~ 0.8
        rotation_speed: 0.1,      // Rotation speed
    });

    // Spawn blackhole mesh with slight scale for better edge visibility
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(500.0, 500.0))),
        MeshMaterial2d(blackhole_material.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(1.2)),
    ));

    // Spawn a second blackhole with stronger distortion
    let blackhole_material_2 = materials.add(BlackholeMaterial {
        texture: texture_handle.clone(),
        distortion_strength: 0.6, // Stronger distortion
        rotation_speed: 3.5,      // Faster rotation
    });

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(400.0, 400.0))),
        MeshMaterial2d(blackhole_material_2),
        Transform::from_xyz(550.0, 0.0, 0.0),
    ));

    // Spawn info text
    commands.spawn((
        Text::new("Blackhole Distortion Effect\nLeft: Moderate distortion (0.3)\nRight: Strong distortion (0.6)"),
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
