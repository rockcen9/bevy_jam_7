use bevy::{prelude::*, sprite_render::MeshMaterial2d, window::WindowResolution};
use rock_materials::VoidPatternMaterial;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Void Pattern Effect - Press Space to toggle fullscreen, +/- for alpha".to_string(),
                resolution: WindowResolution::new(1280, 720),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(rock_materials::plugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (update_resolution, handle_input, update_info_text))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<VoidPatternMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Spawn camera
    commands.spawn(Camera2d);

    // Load a texture (can be any texture - the shader will transform it)
    // For best results, use a simple texture or even a white texture
    let texture_handle: Handle<Image> = asset_server.load("procreate/BigEye.png");

    // Create void pattern material for fullscreen
    let void_material_fullscreen = materials.add(VoidPatternMaterial {
        texture: texture_handle.clone(),
        resolution: Vec2::new(1280.0, 720.0),
        alpha: 1.0,
        _padding: 0.0,
    });

    // Spawn a full-screen quad with the effect
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(1280.0, 720.0))),
        MeshMaterial2d(void_material_fullscreen.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0),
        VoidPatternQuad {
            is_fullscreen: true,
            material_handle: void_material_fullscreen,
        },
    ));

    // Create smaller void pattern materials with different alpha
    let void_material_small1 = materials.add(VoidPatternMaterial {
        texture: texture_handle.clone(),
        resolution: Vec2::new(300.0, 300.0),
        alpha: 0.8,
        _padding: 0.0,
    });

    let void_material_small2 = materials.add(VoidPatternMaterial {
        texture: texture_handle.clone(),
        resolution: Vec2::new(400.0, 400.0),
        alpha: 0.6,
        _padding: 0.0,
    });

    // Spawn smaller quads (hidden by default)
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(300.0, 300.0))),
        MeshMaterial2d(void_material_small1.clone()),
        Transform::from_xyz(-400.0, 0.0, 1.0),
        VoidPatternQuad {
            is_fullscreen: false,
            material_handle: void_material_small1,
        },
        Visibility::Hidden,
    ));

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(400.0, 400.0))),
        MeshMaterial2d(void_material_small2.clone()),
        Transform::from_xyz(400.0, 0.0, 1.0),
        VoidPatternQuad {
            is_fullscreen: false,
            material_handle: void_material_small2,
        },
        Visibility::Hidden,
    ));

    // Spawn info text
    commands.spawn((
        Text::new(""),
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
        InfoText,
    ));
}

#[derive(Component)]
struct VoidPatternQuad {
    is_fullscreen: bool,
    material_handle: Handle<VoidPatternMaterial>,
}

#[derive(Component)]
struct InfoText;

/// Update material resolution when window is resized
fn update_resolution(
    window: Query<&Window>,
    quads: Query<&VoidPatternQuad>,
    mut materials: ResMut<Assets<VoidPatternMaterial>>,
) {
    if let Ok(window) = window.single() {
        let resolution = Vec2::new(window.width() as f32, window.height() as f32);

        // Update only fullscreen materials with window resolution
        for quad in quads.iter() {
            if quad.is_fullscreen {
                if let Some(material) = materials.get_mut(&quad.material_handle) {
                    material.resolution = resolution;
                }
            }
        }
    }
}

/// Handle keyboard input for toggling display modes and adjusting alpha
fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut quads: Query<(&VoidPatternQuad, &mut Visibility)>,
    mut materials: ResMut<Assets<VoidPatternMaterial>>,
) {
    // Toggle between fullscreen and multi-quad mode with Space
    if keyboard.just_pressed(KeyCode::Space) {
        for (quad, mut visibility) in quads.iter_mut() {
            if quad.is_fullscreen {
                *visibility = if *visibility == Visibility::Visible {
                    Visibility::Hidden
                } else {
                    Visibility::Visible
                };
            } else {
                *visibility = if *visibility == Visibility::Hidden {
                    Visibility::Visible
                } else {
                    Visibility::Hidden
                };
            }
        }
    }

    // Adjust alpha with +/- keys
    let alpha_change = if keyboard.pressed(KeyCode::Equal) || keyboard.pressed(KeyCode::NumpadAdd) {
        0.01
    } else if keyboard.pressed(KeyCode::Minus) || keyboard.pressed(KeyCode::NumpadSubtract) {
        -0.01
    } else {
        0.0
    };

    if alpha_change != 0.0 {
        for (quad, _) in quads.iter() {
            if let Some(material) = materials.get_mut(&quad.material_handle) {
                material.alpha = (material.alpha + alpha_change).clamp(0.0, 1.0);
            }
        }
    }
}

/// Update info text with current state
fn update_info_text(
    quads: Query<(&VoidPatternQuad, &Visibility)>,
    materials: Res<Assets<VoidPatternMaterial>>,
    mut info_text: Query<&mut Text, With<InfoText>>,
) {
    if let Ok(mut text) = info_text.single_mut() {
        let fullscreen_visible = quads
            .iter()
            .any(|(q, v)| q.is_fullscreen && *v == Visibility::Visible);

        let mode = if fullscreen_visible {
            "Fullscreen Mode"
        } else {
            "Multi-Quad Mode"
        };

        // Get alpha value from first visible material
        let alpha = quads
            .iter()
            .find(|(_, v)| **v == Visibility::Visible)
            .and_then(|(q, _)| materials.get(&q.material_handle))
            .map(|m| m.alpha)
            .unwrap_or(1.0);

        text.0 = format!(
            "Void Pattern Effect\n\
            Complex mathematical visual pattern\n\
            Animates with time automatically\n\n\
            Controls:\n\
            SPACE - Toggle fullscreen/multi-quad mode\n\
            +/- - Adjust alpha (opacity)\n\n\
            Mode: {}\n\
            Alpha: {:.2}",
            mode, alpha
        );
    }
}
