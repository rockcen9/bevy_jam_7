use bevy_tweening::{RepeatCount, RepeatStrategy, Tween, TweenAnim, lens::TransformScaleLens};

use crate::prelude::*;

pub(crate) fn plugin(app: &mut bevy::app::App) {
    app.add_systems(
        Update,
        velocity_based_squash_animation.run_if(in_state(GameState::Battle)),
    );

    // Cleanup: reset all squash animations when leaving Battle state
    app.add_systems(OnExit(GameState::Battle), cleanup_squash_animations);
}

/// Marker component indicating this Model entity is currently playing an infinite squash animation.
/// Used to distinguish between active squashing and one-shot return-to-normal tweens.
#[derive(Component)]
struct ActiveSquashAnimation;

#[derive(Component)]
pub struct BaseScale(pub Vec3);

/// Velocity-based squash animation system.
/// Applies squash-and-stretch animation to units while they're moving.
/// - Moving units (velocity > threshold): plays infinite squash animation
/// - Stopped units: smoothly returns to normal scale
///
/// Performance: O(N) using HashMap lookup instead of nested loops

/// Velocity-based squash animation system.
/// Performance: Native O(M) using ECS O(1) direct lookup. Zero heap allocation per frame.
fn velocity_based_squash_animation(
    q_units: Query<&Velocity, Or<(With<PlayerUnit>, With<EnemyUnit>)>>,
    q_model: Query<(Entity, &Transform, &BelongTo, Option<&BaseScale>), With<Model>>,
    q_squashing: Query<(), With<ActiveSquashAnimation>>,
    mut commands: Commands,
) {
    const MIN_VELOCITY_FOR_SQUASH: f32 = 10.0;
    const MIN_VELOCITY_SQUARED: f32 = MIN_VELOCITY_FOR_SQUASH * MIN_VELOCITY_FOR_SQUASH;

    for (model_entity, transform, belong_to, base_scale_opt) in q_model.iter() {
        let Ok(velocity) = q_units.get(belong_to.0) else {
            continue;
        };

        let velocity_magnitude_sq = velocity.0.length_squared();
        let is_moving = velocity_magnitude_sq >= MIN_VELOCITY_SQUARED;
        let is_squashing = q_squashing.contains(model_entity);

        let base_scale_abs = if let Some(bs) = base_scale_opt {
            bs.0
        } else {
            let abs_scale = transform.scale.abs();
            commands.entity(model_entity).insert(BaseScale(abs_scale));
            abs_scale
        };

        let facing_sign = transform.scale.x.signum();

        if is_moving && !is_squashing {
            let squash_scale = Vec3::new(
                base_scale_abs.x * 1.15 * facing_sign,
                base_scale_abs.y * 0.7,
                base_scale_abs.z,
            );

            let tween = Tween::new(
                EaseFunction::SineInOut,
                std::time::Duration::from_millis(300),
                TransformScaleLens {
                    start: transform.scale,
                    end: squash_scale,
                },
            )
            .with_repeat_count(RepeatCount::Infinite)
            .with_repeat_strategy(RepeatStrategy::MirroredRepeat);

            commands
                .entity(model_entity)
                .insert((TweenAnim::new(tween), ActiveSquashAnimation));
        } else if !is_moving && is_squashing {
            let normal_scale = Vec3::new(
                base_scale_abs.x * facing_sign,
                base_scale_abs.y,
                base_scale_abs.z,
            );

            let return_tween = Tween::new(
                EaseFunction::QuadraticOut,
                std::time::Duration::from_millis(100),
                TransformScaleLens {
                    start: transform.scale,
                    end: normal_scale,
                },
            );

            commands
                .entity(model_entity)
                .remove::<ActiveSquashAnimation>()
                .insert(TweenAnim::new(return_tween));
        }
    }
}

/// Cleanup system that runs when exiting Battle state.
/// Smoothly returns all squashed units to their normal scale.
fn cleanup_squash_animations(
    q_models: Query<(Entity, &Transform, Option<&BaseScale>), With<ActiveSquashAnimation>>,
    mut commands: Commands,
) {
    for (model_entity, transform, base_scale_opt) in q_models.iter() {
        // Get base scale or fallback to normalized scale
        let base_scale_abs = base_scale_opt
            .map(|bs| bs.0)
            .unwrap_or_else(|| transform.scale.abs());

        let facing_sign = transform.scale.x.signum();
        let normal_scale = Vec3::new(
            base_scale_abs.x * facing_sign,
            base_scale_abs.y,
            base_scale_abs.z,
        );

        // Smoothly return to normal scale
        let return_tween = Tween::new(
            EaseFunction::QuadraticOut,
            std::time::Duration::from_millis(150),
            TransformScaleLens {
                start: transform.scale,
                end: normal_scale,
            },
        );

        commands
            .entity(model_entity)
            .remove::<ActiveSquashAnimation>()
            .insert(TweenAnim::new(return_tween));
    }
}
