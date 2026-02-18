use crate::prelude::*;

use bevy::window::PrimaryWindow;

use bevy_ecs_ldtk::{LdtkIntCell, LevelEvent, app::LdtkIntCellAppExt};

use bevy_tweening::{Tween, TweenAnim, lens::TransformScaleLens};

// use rock_materials::SoulMaterial;

pub(crate) fn plugin(app: &mut bevy::app::App) {
    app.register_ldtk_int_cell::<BoundaryBundle>(1);

    app.insert_resource(InBoundary::default());

    app.insert_resource(LevelBounds::default());

    app.add_systems(
        Update,
        (
            // setup_soul_material,
            calculate_level_bounds,
            check_cursor_in_boundary,
        )
            .chain(),
    );

    app.add_systems(
        Update,
        spawn_boundary_overlay.run_if(in_state(GameState::Preparing)),
    );

    app.add_systems(
        OnEnter(GameState::Preparing),
        (reset_level_bounds, show_boundary).chain(),
    );

    app.add_systems(
        OnExit(GameState::Preparing),
        (hide_boundary, despawn_boundary_overlay),
    );
}

/// Marker component that triggers soul material setup on the boundary's MainMesh.

/// Color is sourced from `ColorPalette` at setup time.

#[derive(Component, Clone, Debug)]

pub struct RequiredSoulMaterial {
    pub _speed: f32,

    pub _intensity: f32,
}

impl Default for RequiredSoulMaterial {
    fn default() -> Self {
        Self {
            _speed: 1.0,

            _intensity: 1.0,
        }
    }
}

#[derive(Component, Clone, Debug, Default, Reflect)]
#[require(Name::new("Boundary"), Actor, RequiredSoulMaterial)]

pub struct Boundary;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]

pub struct BoundaryBundle {
    boundary: Boundary,
}

#[derive(Resource, Default, Reflect)]

pub struct InBoundary(pub bool);

#[derive(Resource, Default, Reflect)]

pub struct LevelBounds {
    min: Vec2,

    max: Vec2,

    initialized: bool,
}

// fn setup_soul_material(
//     q_actor: Query<(Entity, &RequiredSoulMaterial)>,

//     q_belong_to: Query<(Entity, &BelongTo), With<MainMesh>>,

//     palette: Res<ColorPalette>,

//     mut rng: Single<&mut ChaCha8Rng, With<GlobalRng>>,

//     mut commands: Commands,

//     mut meshes: ResMut<Assets<Mesh>>,

//     mut soul_materials: ResMut<Assets<SoulMaterial>>,
// ) {
//     for (main_mesh, belong_to) in q_belong_to.iter() {
//         let Ok((actor, required)) = q_actor.get(belong_to.0) else {
//             continue;
//         };

//         let seed: f32 = rng.random_range(0.0..1000.0);

//         let material = soul_materials.add(SoulMaterial {
//             color: palette.brown_light.into(),

//             speed: required.speed,

//             intensity: required.intensity,

//             seed,

//             ..default()
//         });

//         commands.entity(main_mesh).insert((
//             Mesh2d(meshes.add(Rectangle::new(64.0, 64.0))),
//             MeshMaterial2d(material),
//         ));

//         commands.entity(actor).remove::<RequiredSoulMaterial>();
//     }
// }

fn calculate_level_bounds(
    mut level_messages: MessageReader<LevelEvent>,

    mut bounds: ResMut<LevelBounds>,

    boundary_q: Query<&GlobalTransform, With<Boundary>>,
) {
    for message in level_messages.read() {
        debug!("LevelEvent received: {:?}", message);

        // Use Transformed (not Spawned) so GlobalTransforms are guaranteed up-to-date

        if !matches!(message, LevelEvent::Transformed(_)) {
            continue;
        }

        let count = boundary_q.iter().count();

        debug!("Calculating bounds from {} Boundary entities", count);

        let mut min = Vec2::new(f32::MAX, f32::MAX);

        let mut max = Vec2::new(f32::MIN, f32::MIN);

        for transform in boundary_q.iter() {
            let pos = transform.translation().truncate();

            min = min.min(pos);

            max = max.max(pos);
        }

        bounds.min = min;

        bounds.max = max;

        bounds.initialized = true;

        debug!(
            "LevelBounds updated: min={:?} max={:?}",
            bounds.min, bounds.max
        );
    }
}

fn check_cursor_in_boundary(
    mut in_boundary: ResMut<InBoundary>,

    bounds: Res<LevelBounds>,

    q_window: Query<&Window, With<PrimaryWindow>>,

    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    if !bounds.initialized {
        return;
    }

    let Ok(window) = q_window.single() else {
        return;
    };

    let Ok((camera, camera_transform)) = q_camera.single() else {
        return;
    };

    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) else {
        return;
    };

    in_boundary.0 = world_pos.x >= bounds.min.x
        && world_pos.x <= bounds.max.x
        && world_pos.y >= bounds.min.y
        && world_pos.y <= bounds.max.y;
}

/// Marker for boundary overlay visual entities (fill + corner brackets).
#[derive(Component)]
struct BoundaryOverlay;

const CORNER_LEN: f32 = 48.0;
const CORNER_THICKNESS: f32 = 3.0;
const CORNER_COLOR: Color = Color::srgba(1.0, 1.0, 1.0, 0.75);
const FILL_COLOR: Color = Color::srgba(1.0, 1.0, 1.0, 0.30);
// Just above Grid (75) but below Building (100)
const OVERLAY_Z: f32 = 78.0;

fn spawn_boundary_overlay(
    bounds: Res<LevelBounds>,

    q_overlay: Query<(), With<BoundaryOverlay>>,

    mut commands: Commands,
) {
    if !bounds.initialized || !q_overlay.is_empty() {
        return;
    }

    let center = (bounds.min + bounds.max) / 2.0;

    // Extend half a tile (32px) on each side to include the tile footprint
    let size = bounds.max - bounds.min + Vec2::splat(64.0);

    let half = size / 2.0;

    // Inner area fill
    commands.spawn((
        Name::new("BoundaryOverlayFill"),
        BoundaryOverlay,
        Sprite {
            color: FILL_COLOR,
            custom_size: Some(size),
            ..default()
        },
        Transform::from_xyz(center.x, center.y, OVERLAY_Z),
    ));

    // Four corner positions (outer corners of the boundary box)
    let corners = [
        Vec2::new(center.x - half.x, center.y + half.y), // top-left
        Vec2::new(center.x + half.x, center.y + half.y), // top-right
        Vec2::new(center.x - half.x, center.y - half.y), // bottom-left
        Vec2::new(center.x + half.x, center.y - half.y), // bottom-right
    ];

    // Inward X direction and inward Y direction per corner
    let inward_dirs: [(f32, f32); 4] = [
        (1.0, -1.0),  // top-left: right, down
        (-1.0, -1.0), // top-right: left, down
        (1.0, 1.0),   // bottom-left: right, up
        (-1.0, 1.0),  // bottom-right: left, up
    ];

    for (corner, (ix, iy)) in corners.iter().zip(inward_dirs.iter()) {
        // Horizontal bar
        commands.spawn((
            Name::new("BoundaryOverlayCorner"),
            BoundaryOverlay,
            Sprite {
                color: CORNER_COLOR,
                custom_size: Some(Vec2::new(CORNER_LEN, CORNER_THICKNESS)),
                ..default()
            },
            Transform::from_xyz(corner.x + ix * CORNER_LEN / 2.0, corner.y, OVERLAY_Z + 1.0),
        ));

        // Vertical bar
        commands.spawn((
            Name::new("BoundaryOverlayCorner"),
            BoundaryOverlay,
            Sprite {
                color: CORNER_COLOR,
                custom_size: Some(Vec2::new(CORNER_THICKNESS, CORNER_LEN)),
                ..default()
            },
            Transform::from_xyz(corner.x, corner.y + iy * CORNER_LEN / 2.0, OVERLAY_Z + 1.0),
        ));
    }
}

fn despawn_boundary_overlay(
    q_overlay: Query<Entity, With<BoundaryOverlay>>,

    mut commands: Commands,
) {
    for entity in q_overlay.iter() {
        commands.entity(entity).despawn();
    }
}

fn reset_level_bounds(mut bounds: ResMut<LevelBounds>) {
    *bounds = LevelBounds::default();
}

fn show_boundary(
    q_boundary: Query<Entity, With<Boundary>>,

    mut q_model: Query<(&BelongTo, &mut Transform), With<Model>>,
) {
    for (belong_to, mut transform) in q_model.iter_mut() {
        if q_boundary.get(belong_to.0).is_ok() {
            transform.scale = Vec3::ONE;
        }
    }
}

fn hide_boundary(
    q_boundary: Query<Entity, With<Boundary>>,

    q_model: Query<(Entity, &BelongTo, &Transform), With<Model>>,

    mut commands: Commands,
) {
    for (model_entity, belong_to, transform) in q_model.iter() {
        if q_boundary.get(belong_to.0).is_err() {
            continue;
        }

        let tween = Tween::new(
            EaseFunction::QuadraticIn,
            std::time::Duration::from_millis(200),
            TransformScaleLens {
                start: transform.scale,

                end: Vec3::ZERO,
            },
        );

        commands.entity(model_entity).insert(TweenAnim::new(tween));
    }
}
