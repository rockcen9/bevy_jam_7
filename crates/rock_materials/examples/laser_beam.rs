use bevy::{prelude::*, sprite_render::MeshMaterial2d, window::WindowResolution};
use rock_materials::LaserBeamMaterial;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Laser Beam Effect".to_string(),
                resolution: WindowResolution::new(1280, 720),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(rock_materials::plugin)
        .add_systems(Startup, setup)
        .add_systems(Update, update_resolution)
        .run();
}

#[derive(Component)]
struct LaserQuad(Handle<LaserBeamMaterial>);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<LaserBeamMaterial>>,
) {
    commands.spawn(Camera2d);

    let handle = materials.add(LaserBeamMaterial {
        resolution: Vec2::new(1280.0, 720.0),
        alpha: 1.0,
        _padding: 0.0,
    });

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(1280.0, 720.0))),
        MeshMaterial2d(handle.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0),
        LaserQuad(handle),
    ));
}

fn update_resolution(
    window: Query<&Window>,
    quads: Query<&LaserQuad>,
    mut materials: ResMut<Assets<LaserBeamMaterial>>,
) {
    if let Ok(window) = window.single() {
        let res = Vec2::new(window.width(), window.height());
        for quad in quads.iter() {
            if let Some(mat) = materials.get_mut(&quad.0) {
                mat.resolution = res;
            }
        }
    }
}
