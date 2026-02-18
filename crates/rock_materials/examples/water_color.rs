use bevy::{prelude::*, sprite_render::MeshMaterial2d, window::WindowResolution};
use rock_materials::WaterColorMaterial;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title:
                    "Water Color Effect - SPACE: toggle mode  +/-: alpha  W/S: iters".to_string(),
                resolution: WindowResolution::new(1920, 1080),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(rock_materials::plugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (update_resolution, handle_input, update_info_text))
        .run();
}

#[derive(Component)]
struct WaterQuad {
    is_fullscreen: bool,
    material_handle: Handle<WaterColorMaterial>,
}

#[derive(Component)]
struct InfoText;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<WaterColorMaterial>>,
) {
    commands.spawn(Camera2d);

    // Fullscreen water quad (normal world map look, iters = 9)
    let fullscreen_mat = materials.add(WaterColorMaterial {
        resolution: Vec2::new(1920.0, 1080.0),
        iters: 9.0,
        speed: 0.0,
        alpha: 1.0,
        _padding: 0.0,
    });

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(1280.0, 720.0))),
        MeshMaterial2d(fullscreen_mat.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0),
        WaterQuad {
            is_fullscreen: true,
            material_handle: fullscreen_mat,
        },
    ));

    // // Left panel — normal world map (iters = 9)
    // let normal_mat = materials.add(WaterColorMaterial {
    //     resolution: Vec2::new(400.0, 600.0),
    //     iters: 9.0,
    //     speed: 1.0,
    //     alpha: 1.0,
    //     _padding: 0.0,
    // });

    // commands.spawn((
    //     Mesh2d(meshes.add(Rectangle::new(400.0, 600.0))),
    //     MeshMaterial2d(normal_mat.clone()),
    //     Transform::from_xyz(-440.0, 0.0, 0.0),
    //     WaterQuad {
    //         is_fullscreen: false,
    //         material_handle: normal_mat,
    //     },
    //     Visibility::Hidden,
    // ));

    // // Right panel — swamp world (iters = 12)
    // let swamp_mat = materials.add(WaterColorMaterial {
    //     resolution: Vec2::new(400.0, 600.0),
    //     iters: 12.0,
    //     speed: 1.0,
    //     alpha: 1.0,
    //     _padding: 0.0,
    // });

    // commands.spawn((
    //     Mesh2d(meshes.add(Rectangle::new(400.0, 600.0))),
    //     MeshMaterial2d(swamp_mat.clone()),
    //     Transform::from_xyz(440.0, 0.0, 0.0),
    //     WaterQuad {
    //         is_fullscreen: false,
    //         material_handle: swamp_mat,
    //     },
    //     Visibility::Hidden,
    // ));

    // commands.spawn((
    //     Text::new(""),
    //     Node {
    //         position_type: PositionType::Absolute,
    //         top: Val::Px(10.0),
    //         left: Val::Px(10.0),
    //         ..default()
    //     },
    //     TextColor(Color::WHITE),
    //     TextFont {
    //         font_size: 18.0,
    //         ..default()
    //     },
    //     InfoText,
    // ));
}

fn update_resolution(
    window: Query<&Window>,
    quads: Query<&WaterQuad>,
    mut materials: ResMut<Assets<WaterColorMaterial>>,
) {
    if let Ok(window) = window.single() {
        let resolution = Vec2::new(window.width(), window.height());
        for quad in quads.iter() {
            if quad.is_fullscreen {
                if let Some(mat) = materials.get_mut(&quad.material_handle) {
                    mat.resolution = resolution;
                }
            }
        }
    }
}

fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut quads: Query<(&WaterQuad, &mut Visibility)>,
    mut materials: ResMut<Assets<WaterColorMaterial>>,
) {
    // Toggle fullscreen / side-by-side comparison
    if keyboard.just_pressed(KeyCode::Space) {
        println!("SPACE pressed - Toggling fullscreen/side-by-side mode");
        for (quad, mut vis) in quads.iter_mut() {
            if quad.is_fullscreen {
                *vis = if *vis == Visibility::Visible {
                    Visibility::Hidden
                } else {
                    Visibility::Visible
                };
            } else {
                *vis = if *vis == Visibility::Hidden {
                    Visibility::Visible
                } else {
                    Visibility::Hidden
                };
            }
        }
    }

    // Adjust alpha
    let alpha_delta = if keyboard.pressed(KeyCode::Equal) || keyboard.pressed(KeyCode::NumpadAdd) {
        0.01
    } else if keyboard.pressed(KeyCode::Minus) || keyboard.pressed(KeyCode::NumpadSubtract) {
        -0.01
    } else {
        0.0
    };

    // Adjust iters (W = more, S = less)
    let iters_delta = if keyboard.just_pressed(KeyCode::KeyW) {
        1.0
    } else if keyboard.just_pressed(KeyCode::KeyS) {
        -1.0
    } else {
        0.0
    };

    // Adjust speed (D = faster, A = slower)
    let speed_delta = if keyboard.pressed(KeyCode::KeyD) {
        0.02
    } else if keyboard.pressed(KeyCode::KeyA) {
        -0.02
    } else {
        0.0
    };

    if alpha_delta != 0.0 || iters_delta != 0.0 || speed_delta != 0.0 {
        for (quad, _) in quads.iter() {
            if let Some(mat) = materials.get_mut(&quad.material_handle) {
                if alpha_delta != 0.0 {
                    mat.alpha = (mat.alpha + alpha_delta).clamp(0.0, 1.0);
                    println!(
                        "Alpha adjusted: {:.2} (delta: {:.2})",
                        mat.alpha, alpha_delta
                    );
                }
                if iters_delta != 0.0 {
                    mat.iters = (mat.iters + iters_delta).clamp(1.0, 20.0);
                    println!(
                        "Iterations adjusted: {:.0} (delta: {:.0})",
                        mat.iters, iters_delta
                    );
                }
                if speed_delta != 0.0 {
                    mat.speed = (mat.speed + speed_delta).clamp(0.0, 3.0);
                    println!(
                        "Speed adjusted: {:.2} (delta: {:.2})",
                        mat.speed, speed_delta
                    );
                }
            }
        }
    }
}

fn update_info_text(
    quads: Query<(&WaterQuad, &Visibility)>,
    materials: Res<Assets<WaterColorMaterial>>,
    mut info_text: Query<&mut Text, With<InfoText>>,
) {
    let Ok(mut text) = info_text.single_mut() else {
        return;
    };

    let fullscreen_visible = quads
        .iter()
        .any(|(q, v)| q.is_fullscreen && *v == Visibility::Visible);

    let mode = if fullscreen_visible {
        "Fullscreen"
    } else {
        "Side-by-side (left: normal iters=9 | right: swamp iters=12)"
    };

    let (alpha, iters, speed) = quads
        .iter()
        .find(|(_, v)| **v == Visibility::Visible)
        .and_then(|(q, _)| materials.get(&q.material_handle))
        .map(|m| (m.alpha, m.iters, m.speed))
        .unwrap_or((1.0, 9.0, 1.0));

    text.0 = format!(
        "Water Color Effect\n\
        Bitwise integer pattern — procedural water animation\n\n\
        Controls:\n\
        SPACE - Toggle fullscreen / side-by-side\n\
        +/- - Adjust alpha ({:.2})\n\
        W/S - Adjust iters ({:.0})  [9=normal, 12=swamp]\n\
        A/D - Adjust speed ({:.2})  [0=frozen, 1=default]\n\n\
        Mode: {}",
        alpha, iters, speed, mode
    );
}
