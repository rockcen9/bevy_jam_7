use bevy::{prelude::*, sprite_render::MeshMaterial2d, window::WindowResolution};

use rock_materials::LiquidMaterial;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Liquid Effect".to_string(),
                resolution: WindowResolution::new(800, 600),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(rock_materials::plugin)
        .add_systems(Startup, setup)
        .add_systems(Update, pulse_blobs)
        .run();
}

#[derive(Component)]
struct LiquidBlob(usize);

#[derive(Resource)]
struct LiquidHandles(Vec<Handle<LiquidMaterial>>);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<LiquidMaterial>>,
) {
    commands.spawn(Camera2d);

    let presets: &[(Color, f32, &str)] = &[
        (Color::hsl(210.0, 0.8, 0.55), 1.0, "Water\nnormal"),
        (Color::hsl(120.0, 0.9, 0.4), 1.8, "Slime\nfast"),
        (Color::hsl(270.0, 0.9, 0.5), 0.5, "Poison\nslow"),
        (Color::hsl(10.0, 1.0, 0.5), 2.5, "Lava\nfrenzy"),
    ];

    let size = 128.0_f32;
    let spacing = 180.0_f32;
    let total_width = spacing * (presets.len() as f32 - 1.0);
    let start_x = -total_width / 2.0;

    let mut handles: Vec<Handle<LiquidMaterial>> = Vec::new();

    for (i, (color, time_scale, label)) in presets.iter().enumerate() {
        let handle = materials.add(LiquidMaterial {
            color: (*color).into(),
            time_scale: *time_scale,
        });

        handles.push(handle.clone());

        let x = start_x + spacing * i as f32;

        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(size, size))),
            MeshMaterial2d(handle),
            Transform::from_xyz(x, 20.0, 0.0),
            LiquidBlob(i),
        ));

        commands.spawn((
            Text2d::new(*label),
            TextFont {
                font_size: 14.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Transform::from_xyz(x, -90.0, 1.0),
        ));
    }

    commands.insert_resource(LiquidHandles(handles));
}

/// Gently bobs each blob's time_scale to show runtime material mutation.
fn pulse_blobs(
    time: Res<Time>,
    handles: Res<LiquidHandles>,
    mut materials: ResMut<Assets<LiquidMaterial>>,
) {
    for (i, handle) in handles.0.iter().enumerate() {
        if let Some(mat) = materials.get_mut(handle) {
            let phase = time.elapsed_secs() + i as f32 * 1.5;
            mat.time_scale = 0.5 + phase.sin().abs() * 2.0;
        }
    }
}
