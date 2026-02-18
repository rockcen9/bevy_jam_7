use std::time::Duration;

use bevy::{
    camera::{ScalingMode, visibility::RenderLayers},
    camera_controller::pan_camera::PanCamera,
};
#[cfg(feature = "backend")]
use bevy_seedling::prelude::SpatialListener2D;
use bevy_tweening::{Lens, Tween, TweenAnim};

use crate::prelude::*;

#[derive(Message)]
pub struct CameraZoomMessage(pub f32);

pub(crate) fn plugin(app: &mut bevy::app::App) {
    app.add_systems(Startup, setup_camera);
    app.add_message::<CameraZoomMessage>();
    app.add_systems(
        Update,
        (receive_zoom_message, fix_camera_zoom_to_projection),
    );
}

#[derive(Component)]
pub struct MainCamera;

fn setup_camera(mut commands: Commands) {
    let entity = commands.spawn((
        Name::new("World Camera"),
        Camera2d,
        MainCamera,
        Camera {
            // World camera renders first
            order: 0,
            // Don't clear on this camera - let it render the world
            ..default()
        },
        PanCamera {
            zoom_factor: 2.,
            min_zoom: 1.,
            max_zoom: 6.,
            ..default()
        },
        WorldUIMarker,
        RenderLayers::layer(0),
        // Start at origin for typical 2D games
        Transform::from_translation(Vec3::ZERO),
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::WindowSize,
            scale: 1.,
            ..OrthographicProjection::default_2d()
        }),
    )).id();
    #[cfg(feature = "backend")]
    commands.entity(entity).insert(SpatialListener2D);
}

struct PanCameraZoomLens {
    start: f32,
    end: f32,
}

impl Lens<PanCamera> for PanCameraZoomLens {
    fn lerp(&mut self, mut target: bevy::ecs::world::Mut<PanCamera>, ratio: f32) {
        target.zoom_factor = self.start.lerp(self.end, ratio);
    }
}

/// PanCamera sets zoom via `Transform.scale`, but Bevy's picking system requires zoom
/// to be expressed via `OrthographicProjection.scale`. This system transfers the scale
/// and resets `Transform.scale` to Vec3::ONE so picking works at all zoom levels.
fn fix_camera_zoom_to_projection(
    mut camera_q: Query<(&mut Transform, &mut Projection), With<MainCamera>>,
) {
    let Ok((mut transform, mut projection)) = camera_q.single_mut() else {
        return;
    };
    let scale = transform.scale.x;
    if (scale - 1.0).abs() > f32::EPSILON {
        transform.scale = Vec3::ONE;
        if let Projection::Orthographic(ref mut ortho) = *projection {
            ortho.scale = scale;
        }
    }
}

fn receive_zoom_message(
    mut messages: MessageReader<CameraZoomMessage>,
    mut commands: Commands,
    camera_q: Query<(Entity, &PanCamera), With<MainCamera>>,
) {
    for msg in messages.read() {
        let Ok((entity, pan)) = camera_q.single() else {
            return;
        };
        let start = pan.zoom_factor;
        let end = msg.0;
        let ease = if end > start {
            EaseFunction::QuadraticIn
        } else {
            EaseFunction::QuadraticOut
        };
        let tween = Tween::new(
            ease,
            Duration::from_millis(1000),
            PanCameraZoomLens { start, end },
        );
        commands.entity(entity).insert(TweenAnim::new(tween));
    }
}
