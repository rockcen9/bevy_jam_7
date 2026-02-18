use bevy::{prelude::*, sprite_render::MeshMaterial2d};
use rock_materials::MpegArtifactV2Material;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(rock_materials::plugin)
        .add_systems(Startup, setup)
        .add_systems(Update, animate_fill)
        .run();
}

#[derive(Component)]
struct AnimatedFill;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<MpegArtifactV2Material>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(Color::srgb(0.02, 0.02, 0.05)),
            ..default()
        },
    ));

    let texture: Handle<Image> = asset_server.load("procreate/BigEye.png");

    // Left: animated fill — blue tint, fill sweeps back and forth
    let mat_animated = materials.add(MpegArtifactV2Material {
        texture: texture.clone(),
        intensity: 1.0,
        alpha: 1.0,
        fill: 0.5,
        fill_color: LinearRgba::new(0.1, 0.5, 1.0, 0.6),
    });
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(400.0, 400.0))),
        MeshMaterial2d(mat_animated),
        Transform::from_xyz(-460.0, 0.0, 0.0),
        AnimatedFill,
    ));

    // Center: 50% fill — green tint
    let mat_half = materials.add(MpegArtifactV2Material {
        texture: texture.clone(),
        intensity: 0.8,
        alpha: 1.0,
        fill: 0.5,
        fill_color: LinearRgba::new(0.1, 1.0, 0.3, 0.5),
    });
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(400.0, 400.0))),
        MeshMaterial2d(mat_half),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Right: full fill — red tint at 100%
    let mat_full = materials.add(MpegArtifactV2Material {
        texture: texture.clone(),
        intensity: 0.6,
        alpha: 1.0,
        fill: 1.0,
        fill_color: LinearRgba::new(1.0, 0.2, 0.1, 0.4),
    });
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(400.0, 400.0))),
        MeshMaterial2d(mat_full),
        Transform::from_xyz(460.0, 0.0, 0.0),
    ));

    commands.spawn((
        Text::new(
            "MPEG Artifact V2 — Fill Bar (bottom → top)\n\
             Left:   animated fill (blue, 0.0→1.0)\n\
             Center: 50% fill (green)\n\
             Right:  100% fill (red)\n\
             fill_color.a controls blend strength",
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

fn animate_fill(
    time: Res<Time>,
    query: Query<&MeshMaterial2d<MpegArtifactV2Material>, With<AnimatedFill>>,
    mut materials: ResMut<Assets<MpegArtifactV2Material>>,
) {
    for handle in &query {
        if let Some(mat) = materials.get_mut(handle.id()) {
            mat.fill = (time.elapsed_secs().sin() * 0.5 + 0.5).clamp(0.0, 1.0);
        }
    }
}
