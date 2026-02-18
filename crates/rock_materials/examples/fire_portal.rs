use bevy::{prelude::*, sprite_render::MeshMaterial2d, window::WindowResolution};
use rock_materials::FirePortalMaterial;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Fire Portal Effect - +/- intensity, [/] alpha".to_string(),
                resolution: WindowResolution::new(800, 600),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(rock_materials::plugin)
        .add_systems(Startup, setup)
        .add_systems(Update, handle_input)
        .run();
}

#[derive(Component)]
struct PortalQuad {
    material_handle: Handle<FirePortalMaterial>,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<FirePortalMaterial>>,
) {
    commands.spawn(Camera2d);

    let mat = materials.add(FirePortalMaterial::default());

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(500.0, 500.0))),
        MeshMaterial2d(mat.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0),
        PortalQuad {
            material_handle: mat,
        },
    ));

    commands.spawn((
        Text::new(
            "Fire Portal\n\
            +/- : intensity\n\
            [/] : alpha",
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

fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    portals: Query<&PortalQuad>,
    mut materials: ResMut<Assets<FirePortalMaterial>>,
) {
    let intensity_delta = if keyboard.pressed(KeyCode::Equal) || keyboard.pressed(KeyCode::NumpadAdd) {
        0.02
    } else if keyboard.pressed(KeyCode::Minus) || keyboard.pressed(KeyCode::NumpadSubtract) {
        -0.02
    } else {
        0.0
    };

    let alpha_delta = if keyboard.pressed(KeyCode::BracketLeft) {
        -0.01
    } else if keyboard.pressed(KeyCode::BracketRight) {
        0.01
    } else {
        0.0
    };

    if intensity_delta == 0.0 && alpha_delta == 0.0 {
        return;
    }

    for portal in portals.iter() {
        if let Some(mat) = materials.get_mut(&portal.material_handle) {
            mat.intensity = (mat.intensity + intensity_delta).clamp(0.0, 5.0);
            mat.alpha = (mat.alpha + alpha_delta).clamp(0.0, 1.0);
        }
    }
}
