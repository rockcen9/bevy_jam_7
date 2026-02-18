use bevy_tweening::{Tween, TweenAnim, lens::TransformScaleLens};

use crate::prelude::*;

pub(crate) fn plugin(app: &mut bevy::app::App) {
    app.add_systems(Update, (start_shrink, finish_despawn).chain());
}

/// Mark an entity to shrink to scale 0 over 0.2s then despawn.
#[derive(Component)]
pub struct ShrinkDespawn;

/// Internal marker added after the tween starts.
#[derive(Component)]
struct ShrinkingDespawn;

fn start_shrink(
    q: Query<(Entity, &Transform), (With<ShrinkDespawn>, Without<ShrinkingDespawn>)>,
    mut commands: Commands,
) {
    for (entity, transform) in q.iter() {
        let tween = Tween::new(
            EaseFunction::QuadraticIn,
            std::time::Duration::from_millis(200),
            TransformScaleLens {
                start: transform.scale,
                end: Vec3::ZERO,
            },
        );

        commands
            .entity(entity)
            .remove::<ShrinkDespawn>()
            .insert((TweenAnim::new(tween), ShrinkingDespawn));
    }
}

fn finish_despawn(
    q: Query<Entity, (With<ShrinkingDespawn>, Without<TweenAnim>)>,
    mut commands: Commands,
) {
    for entity in q.iter() {
        commands.entity(entity).despawn();
    }
}
