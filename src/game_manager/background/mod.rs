use crate::{game_manager::GameState, prelude::*, screens::Screen};
use rock_materials::WaterColorMaterial;

#[derive(Component, Reflect)]
pub struct BackgroundQuad {
    material_handle: Handle<WaterColorMaterial>,
    /// Tracks ramp progress independently of pause state.
    logical_speed: f32,
    logical_iters: f32,
}

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), spawn_background);
    app.add_systems(Update, follow_camera.run_if(in_state(Screen::Gameplay)));
    app.add_systems(
        Update,
        increase_battle_speed.run_if(in_state(GameState::Battle)),
    );
    app.add_systems(OnExit(GameState::Battle), on_exit_battle);
}

fn spawn_background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<WaterColorMaterial>>,
    window: Query<&Window>,
) {
    let resolution = window
        .single()
        .map(|w| Vec2::new(w.width(), w.height()))
        .unwrap_or(Vec2::new(1920.0, 1080.0));

    let mat = materials.add(WaterColorMaterial {
        resolution,
        iters: 15.0,
        speed: 0.0,
        alpha: 0.7,
        _padding: 0.0,
    });

    commands.spawn((
        Name::new("Water Background"),
        BackgroundQuad {
            material_handle: mat.clone(),
            logical_speed: 0.0,
            logical_iters: 15.0,
        },
        Mesh2d(meshes.add(Rectangle::new(12000.0, 8000.0))),
        MeshMaterial2d(mat),
        // Do NOT use SpriteLayer here — SpriteLayerPlugin would override z to 50.0
        // which puts us inside the Mesh2d sort range and causes z-fighting with sprites.
        // Keep well below all game layers (lowest is Corpse at 55) but above the 2D
        // near clip plane (-1000) to avoid frustum culling on zoom-out.
        Transform::from_xyz(0.0, 0.0, -500.0),
        DespawnOnExit(Screen::Gameplay),
    ));
}

/// Keep the background centered on the camera so it always fills the view.
fn follow_camera(
    camera_q: Query<&Transform, With<MainCamera>>,
    mut bg_q: Query<&mut Transform, (With<BackgroundQuad>, Without<MainCamera>)>,
) {
    let Ok(cam) = camera_q.single() else { return };
    let Ok(mut bg) = bg_q.single_mut() else {
        return;
    };
    bg.translation.x = cam.translation.x;
    bg.translation.y = cam.translation.y;
}

fn increase_battle_speed(
    mut bg_q: Query<&mut BackgroundQuad>,
    mut materials: ResMut<Assets<WaterColorMaterial>>,
    real_time: Res<Time<Real>>,
    virtual_time: Res<Time<Virtual>>,
) {
    let Ok(mut bg) = bg_q.single_mut() else {
        return;
    };
    let Some(mat) = materials.get_mut(&bg.material_handle) else {
        return;
    };
    let paused = virtual_time.relative_speed() == 0.0;
    let dt = real_time.delta_secs();
    // Ramp logical values using real time — unaffected by 1x/2x/4x speed buttons.
    // bg.logical_speed = (bg.logical_speed + 0.01 * dt).clamp(0.0, 0.3);
    bg.logical_iters = (bg.logical_iters - 0.01 * dt * 12.).clamp(10., 15.);

    // Only suppress the visual output when paused; speed buttons have no effect.
    mat.speed = if paused { 0.0 } else { 0.10 };
    mat.iters = bg.logical_iters;
}

fn on_exit_battle(
    mut bg_q: Query<&mut BackgroundQuad>,
    mut materials: ResMut<Assets<WaterColorMaterial>>,
) {
    let Ok(mut bg) = bg_q.single_mut() else {
        return;
    };
    let Some(mat) = materials.get_mut(&bg.material_handle) else {
        return;
    };
    bg.logical_speed = 0.0;
    bg.logical_iters = 15.0;
    mat.speed = 0.0;
    mat.iters = 15.0;
}
