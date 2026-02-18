use bevy::{prelude::*, sprite_render::MeshMaterial2d};
use rock_materials::PaperCutoutMaterial;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(rock_materials::plugin)
        .add_systems(Startup, setup)
        .run();
}

/// The mesh must be slightly larger than the sprite to give outline pixels room
/// to render outside the alpha edge.
/// Formula: mesh_size = sprite_size * (1.0 + 2.0 * outline_width)
fn outline_mesh_size(sprite_size: f32, outline_width: f32) -> f32 {
    sprite_size * (1.0 + 2.0 * outline_width)
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<PaperCutoutMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(Color::srgb(0.15, 0.15, 0.20)),
            ..default()
        },
    ));

    let texture: Handle<Image> = asset_server.load("procreate/Archer.png");
    let sprite_size = 300.0_f32;

    // Thin outline
    let ow = 0.02_f32;
    let mesh_size = outline_mesh_size(sprite_size, ow);
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(mesh_size, mesh_size))),
        MeshMaterial2d(materials.add(PaperCutoutMaterial {
            texture: texture.clone(),
            outline_width: ow,
            alpha: 1.0,
        })),
        Transform::from_xyz(-350.0, 0.0, 0.0),
    ));

    // Medium outline (default)
    let ow = 0.05_f32;
    let mesh_size = outline_mesh_size(sprite_size, ow);
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(mesh_size, mesh_size))),
        MeshMaterial2d(materials.add(PaperCutoutMaterial {
            texture: texture.clone(),
            outline_width: ow,
            alpha: 1.0,
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Thick outline
    let ow = 0.10_f32;
    let mesh_size = outline_mesh_size(sprite_size, ow);
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(mesh_size, mesh_size))),
        MeshMaterial2d(materials.add(PaperCutoutMaterial {
            texture: texture.clone(),
            outline_width: ow,
            alpha: 1.0,
        })),
        Transform::from_xyz(350.0, 0.0, 0.0),
    ));

    commands.spawn((
        Text::new("Paper Cutout Effect \nLeft: thin (0.02)  Center: default (0.05)  Right: thick (0.10)\nMesh is sprite_size * (1 + 2 * outline_width) to fit the outline"),
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
