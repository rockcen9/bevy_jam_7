use bevy::{prelude::*, sprite_render::MeshMaterial2d, window::WindowResolution};

use rock_materials::SoulMaterial;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Soul Effect".to_string(),

                resolution: WindowResolution::new(800, 600),

                ..default()
            }),

            ..default()
        }))
        .add_plugins(rock_materials::plugin)
        .add_systems(Startup, setup)
        .add_systems(Update, cycle_souls)
        .run();
}

/// Marks each soul quad (unused field kept for future preset switching).

#[derive(Component)]
#[allow(dead_code)]

struct SoulQuad(usize);

/// Keeps the material handles so `cycle_souls` can modify them at runtime.

#[derive(Resource)]

struct SoulHandles(Vec<Handle<SoulMaterial>>);

fn setup(
    mut commands: Commands,

    mut meshes: ResMut<Assets<Mesh>>,

    mut materials: ResMut<Assets<SoulMaterial>>,
) {
    commands.spawn(Camera2d);

    // Four presets arranged in a row

    let presets: &[(Color, f32, f32, &str)] = &[
        (Color::hsl(195.0, 0.8, 0.7), 1.0, 1.0, "Default\ncyan"),
        (Color::hsl(270.0, 1.0, 0.6), 1.8, 1.3, "Purple\nfast"),
        (Color::hsl(140.0, 0.9, 0.55), 0.6, 0.8, "Green\nslow"),
        (Color::hsl(10.0, 1.0, 0.55), 2.5, 1.6, "Fire\nred"),
    ];

    let size = 128.0_f32;

    let spacing = 180.0_f32;

    let total_width = spacing * (presets.len() as f32 - 1.0);

    let start_x = -total_width / 2.0;

    let mut handles: Vec<Handle<SoulMaterial>> = Vec::new();

    for (i, (color, speed, intensity, _label)) in presets.iter().enumerate() {
        let handle = materials.add(SoulMaterial {
            color: (*color).into(),

            speed: *speed,

            intensity: *intensity,

            ..default()
        });

        handles.push(handle.clone());

        let x = start_x + spacing * i as f32;

        // Soul quad

        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(size, size))),
            MeshMaterial2d(handle),
            Transform::from_xyz(x, 20.0, 0.0),
            SoulQuad(i),
        ));

        // Label

        commands.spawn((
            Text2d::new(*_label),
            TextFont {
                font_size: 14.0,

                ..default()
            },
            TextColor(Color::WHITE),
            Transform::from_xyz(x, -90.0, 1.0),
        ));
    }

    commands.insert_resource(SoulHandles(handles));
}

/// Gently pulses each soul's intensity to show runtime material mutation.

fn cycle_souls(
    time: Res<Time>,

    handles: Res<SoulHandles>,

    mut materials: ResMut<Assets<SoulMaterial>>,
) {
    for (i, handle) in handles.0.iter().enumerate() {
        if let Some(mat) = materials.get_mut(handle) {
            let phase = time.elapsed_secs() + i as f32 * 1.5;

            mat.intensity = 0.8 + phase.sin().abs() * 0.8;
        }
    }
}
