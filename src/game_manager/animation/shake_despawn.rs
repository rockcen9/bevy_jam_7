use bevy_tweening::{Tween, TweenAnim, lens::TransformPositionLens};

use crate::prelude::*;

pub(crate) fn plugin(app: &mut bevy::app::App) {
    app.add_systems(Update, (start_shake, finish_despawn).chain());
}

/// Mark an entity to shake then despawn.
#[derive(Component)]
pub struct ShakeDespawn;

/// Internal marker added after the tween starts.
#[derive(Component)]
struct ShakingDespawn;

fn start_shake(
    q: Query<(Entity, &Transform), (With<ShakeDespawn>, Without<ShakingDespawn>)>,
    mut commands: Commands,
) {
    for (entity, transform) in q.iter() {
        // Shake by moving slightly to the right then back
        let shake_offset = Vec3::new(80.0, 0.0, 0.0);

        let tween = Tween::new(
            EaseFunction::QuadraticInOut,
            std::time::Duration::from_millis(100),
            TransformPositionLens {
                start: transform.translation,
                end: transform.translation + shake_offset,
            },
        )
        .then(Tween::new(
            EaseFunction::QuadraticInOut,
            std::time::Duration::from_millis(100),
            TransformPositionLens {
                start: transform.translation + shake_offset,
                end: transform.translation,
            },
        ));

        commands
            .entity(entity)
            .remove::<ShakeDespawn>()
            .insert((TweenAnim::new(tween), ShakingDespawn));
    }
}

fn finish_despawn(
    q: Query<Entity, (With<ShakingDespawn>, Without<TweenAnim>)>,
    mut commands: Commands,
) {
    for entity in q.iter() {
        commands.entity(entity).despawn();
    }
}
