use bevy_tweening::{Tween, TweenAnim, lens::TransformPositionLens};

use crate::prelude::*;

pub(crate) fn plugin(app: &mut bevy::app::App) {
    app.add_systems(Update, (start_shake, cleanup_marker).chain());
}

/// Mark an entity to shake left and right, then return to original position.
#[derive(Component)]
pub struct ShakeLeftRight;

/// Internal marker added after the tween starts.
#[derive(Component)]
struct Shaking;

fn start_shake(
    q: Query<(Entity, &Transform), (With<ShakeLeftRight>, Without<Shaking>)>,
    mut commands: Commands,
) {
    for (entity, transform) in q.iter() {
        // Shake by moving left, then right, then back to center
        let shake_offset = Vec3::new(40.0, 0.0, 0.0);

        let tween = Tween::new(
            EaseFunction::QuadraticOut,
            std::time::Duration::from_millis(50),
            TransformPositionLens {
                start: transform.translation,
                end: transform.translation - shake_offset,
            },
        )
        .then(Tween::new(
            EaseFunction::QuadraticInOut,
            std::time::Duration::from_millis(100),
            TransformPositionLens {
                start: transform.translation - shake_offset,
                end: transform.translation + shake_offset,
            },
        ))
        .then(Tween::new(
            EaseFunction::QuadraticIn,
            std::time::Duration::from_millis(50),
            TransformPositionLens {
                start: transform.translation + shake_offset,
                end: transform.translation,
            },
        ));

        commands
            .entity(entity)
            .remove::<ShakeLeftRight>()
            .insert((TweenAnim::new(tween), Shaking));
    }
}

fn cleanup_marker(
    q: Query<Entity, (With<Shaking>, Without<TweenAnim>)>,
    mut commands: Commands,
) {
    for entity in q.iter() {
        commands.entity(entity).remove::<Shaking>();
    }
}
