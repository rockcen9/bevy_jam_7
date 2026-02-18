use bevy::{prelude::*, sprite_render::MeshMaterial2d};
use rock_materials::PoisonMaterial;

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
    mut materials: ResMut<Assets<PoisonMaterial>>,
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

    // Create poison material with low intensity
    let poison_material_low = materials.add(PoisonMaterial {
        texture: texture_handle.clone(),
        poison_amount: 0.3, // Low poison
        pulse_speed: 3.0,
        poison_color: LinearRgba::new(0.3, 1.0, 0.3, 1.0), // Bright green
    });

    // Spawn mesh with low poison effect (top left)
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(300.0, 300.0))),
        MeshMaterial2d(poison_material_low),
        Transform::from_xyz(-250.0, 200.0, 0.0),
    ));

    // Create poison material with medium intensity
    let poison_material_med = materials.add(PoisonMaterial {
        texture: texture_handle.clone(),
        poison_amount: 0.6, // Medium poison
        pulse_speed: 3.0,
        poison_color: LinearRgba::new(0.3, 1.0, 0.3, 1.0), // Bright green
    });

    // Spawn mesh with medium poison effect (top right)
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(300.0, 300.0))),
        MeshMaterial2d(poison_material_med),
        Transform::from_xyz(250.0, 200.0, 0.0),
    ));

    // Create poison material with high intensity and slow pulse
    let poison_material_high = materials.add(PoisonMaterial {
        texture: texture_handle.clone(),
        poison_amount: 0.9, // High poison
        pulse_speed: 2.0,   // Slower pulse
        poison_color: LinearRgba::new(0.3, 1.0, 0.3, 1.0), // Bright green
    });

    // Spawn mesh with high poison effect (bottom left)
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(300.0, 300.0))),
        MeshMaterial2d(poison_material_high),
        Transform::from_xyz(-250.0, -200.0, 0.0),
    ));

    // Create poison material with custom color (purple toxic)
    let poison_material_purple = materials.add(PoisonMaterial {
        texture: texture_handle.clone(),
        poison_amount: 0.7,
        pulse_speed: 4.0, // Fast pulse
        poison_color: LinearRgba::new(0.8, 0.2, 1.0, 1.0), // Purple toxic
    });

    // Spawn mesh with purple poison effect (bottom right)
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(300.0, 300.0))),
        MeshMaterial2d(poison_material_purple),
        Transform::from_xyz(250.0, -200.0, 0.0),
    ));

    // Spawn info text
    commands.spawn((
        Text::new(
            "Poison Effect\n\
            Top Left: Low (0.3) - Bright Green\n\
            Top Right: Medium (0.6) - Bright Green\n\
            Bottom Left: High (0.9), Slow Pulse - Bright Green\n\
            Bottom Right: Custom (0.7), Fast Pulse - Purple Toxic\n\n\
            Pulsing green overlay effect for poisoned appearance"
        ),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        TextColor(Color::WHITE),
        TextFont {
            font_size: 18.0,
            ..default()
        },
    ));
}
