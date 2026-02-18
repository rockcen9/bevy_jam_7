use bevy::{prelude::*, sprite_render::MeshMaterial2d, window::WindowResolution};
use rock_materials::DissolverMaterial;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Turbo Rainbow Dissolver - +/- speed  [/] scale  Space toggle".to_string(),
                resolution: WindowResolution::new(1280, 720),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(rock_materials::plugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (sync_resolution, handle_input, update_hud))
        .run();
}

// ── Components ────────────────────────────────────────────────────────────────

#[derive(Component)]
struct DissolverQuad {
    handle: Handle<DissolverMaterial>,
}

#[derive(Component)]
struct Hud;

// ── Setup ─────────────────────────────────────────────────────────────────────

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<DissolverMaterial>>,
) {
    commands.spawn(Camera2d);

    let mat = materials.add(DissolverMaterial {
        resolution: Vec2::new(1280.0, 720.0),
        ..default()
    });

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(1280.0, 720.0))),
        MeshMaterial2d(mat.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0),
        DissolverQuad { handle: mat },
    ));

    commands.spawn((
        Text::new(""),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        TextColor(Color::WHITE),
        TextFont { font_size: 16.0, ..default() },
        Hud,
    ));
}

// ── Systems ───────────────────────────────────────────────────────────────────

fn sync_resolution(
    window: Query<&Window>,
    quads: Query<&DissolverQuad>,
    mut materials: ResMut<Assets<DissolverMaterial>>,
) {
    if let Ok(win) = window.single() {
        let res = Vec2::new(win.width(), win.height());
        for q in &quads {
            if let Some(mat) = materials.get_mut(&q.handle) {
                mat.resolution = res;
            }
        }
    }
}

fn handle_input(
    keys: Res<ButtonInput<KeyCode>>,
    quads: Query<&DissolverQuad>,
    mut materials: ResMut<Assets<DissolverMaterial>>,
) {
    let speed_delta = if keys.pressed(KeyCode::Equal) || keys.pressed(KeyCode::NumpadAdd) {
        0.02
    } else if keys.pressed(KeyCode::Minus) || keys.pressed(KeyCode::NumpadSubtract) {
        -0.02
    } else {
        0.0
    };

    let scale_delta = if keys.pressed(KeyCode::BracketLeft) {
        -0.03
    } else if keys.pressed(KeyCode::BracketRight) {
        0.03
    } else {
        0.0
    };

    let pop_delta = if keys.pressed(KeyCode::Comma) {
        -0.005
    } else if keys.pressed(KeyCode::Period) {
        0.005
    } else {
        0.0
    };

    for q in &quads {
        if let Some(mat) = materials.get_mut(&q.handle) {
            mat.speed = (mat.speed + speed_delta).clamp(0.0, 10.0);
            mat.scale = (mat.scale + scale_delta).clamp(0.5, 20.0);
            mat.pop_width = (mat.pop_width + pop_delta).clamp(0.001, 0.3);
        }
    }
}

fn update_hud(
    quads: Query<&DissolverQuad>,
    materials: Res<Assets<DissolverMaterial>>,
    mut hud: Query<&mut Text, With<Hud>>,
) {
    let Ok(mut text) = hud.single_mut() else { return };

    let (speed, scale, pop) = quads
        .iter()
        .next()
        .and_then(|q| materials.get(&q.handle))
        .map(|m| (m.speed, m.scale, m.pop_width))
        .unwrap_or((1.0, 3.0, 0.05));

    text.0 = format!(
        "Turbo Rainbow Dissolver\n\n\
         +/–   speed    {speed:.2}\n\
         [/]   scale    {scale:.2}\n\
         ,/.   pop edge {pop:.3}"
    );
}
