use bevy_tweening::{Tween, TweenAnim, lens::TransformScaleLens};

use crate::{prelude::*, screens::Screen};

pub(crate) fn plugin(app: &mut bevy::app::App) {
    app.add_systems(
        Update,
        (put_animation, pick_animation).run_if(in_state(Screen::Gameplay)),
    );
}
#[derive(Component, Debug, Clone, Reflect, PartialEq)]
pub enum RequiredAnimation {
    Put,
    Pick,
}

#[derive(Component)]
pub struct RunWithNoModel;

pub fn pick_animation(
    q_animation: Query<(
        Entity,
        &RequiredAnimation,
        Option<&RunWithNoModel>,
        Option<&Transform>,
    )>,
    q_model: Query<(Entity, &BelongTo, &Transform), With<Model>>,
    mut commands: Commands,
) {
    for (entity, animation, run_no_model, transform) in q_animation.iter() {
        if *animation != RequiredAnimation::Pick {
            continue;
        }

        // If RunWithNoModel is present, play animation directly on this entity
        if run_no_model.is_some() {
            if let Some(transform) = transform {
                let original_scale = transform.scale;
                let invisible_scale = Vec3::ZERO;
                let overshoot_scale = original_scale * 1.2;

                // Phase 1: Start invisible (0s -> 0.1s = 100ms)
                let appear_tween = Tween::new(
                    EaseFunction::QuadraticOut,
                    std::time::Duration::from_millis(100),
                    TransformScaleLens {
                        start: invisible_scale,
                        end: overshoot_scale,
                    },
                );

                // Phase 2: Scale back to normal (0.1s -> 0.15s = 50ms)
                let settle_tween = Tween::new(
                    EaseFunction::QuadraticInOut,
                    std::time::Duration::from_millis(50),
                    TransformScaleLens {
                        start: overshoot_scale,
                        end: original_scale,
                    },
                );

                // Chain the animations: appear then settle
                let sequence = appear_tween.then(settle_tween);

                commands.entity(entity).insert(TweenAnim::new(sequence));
                commands
                    .entity(entity)
                    .remove::<(RequiredAnimation, RunWithNoModel)>();
                commands.trigger(SFXEvent::ui("pick"));
            }
            continue;
        }

        // Otherwise, find and animate the Model entity
        for (model_entity, link_ref, transform) in q_model.iter() {
            if link_ref.0 != entity {
                continue;
            }
            //put the animation code here
            let original_scale = transform.scale;
            let invisible_scale = Vec3::ZERO;
            let overshoot_scale = original_scale * 1.2;

            // Phase 1: Start invisible (0s -> 0.1s = 100ms)
            let appear_tween = Tween::new(
                EaseFunction::QuadraticOut,
                std::time::Duration::from_millis(100),
                TransformScaleLens {
                    start: invisible_scale,
                    end: overshoot_scale,
                },
            );

            // Phase 2: Scale back to normal (0.1s -> 0.15s = 50ms)
            let settle_tween = Tween::new(
                EaseFunction::QuadraticInOut,
                std::time::Duration::from_millis(50),
                TransformScaleLens {
                    start: overshoot_scale,
                    end: original_scale,
                },
            );

            // Chain the animations: appear then settle
            let sequence = appear_tween.then(settle_tween);

            commands
                .entity(model_entity)
                .insert(TweenAnim::new(sequence));
            commands.entity(entity).remove::<RequiredAnimation>();
            commands.trigger(SFXEvent::ui("pick"));
        }
    }
}

pub fn put_animation(
    q_animation: Query<(
        Entity,
        &RequiredAnimation,
        Option<&RunWithNoModel>,
        Option<&Transform>,
    )>,
    q_model: Query<(Entity, &BelongTo, &Transform), With<Model>>,
    mut commands: Commands,
) {
    for (entity, animation, run_no_model, transform) in q_animation.iter() {
        if *animation != RequiredAnimation::Put {
            continue;
        }

        // If RunWithNoModel is present, play animation directly on this entity
        if run_no_model.is_some() {
            if let Some(transform) = transform {
                let original_scale = transform.scale;

                // Phase 1: Squash on impact (0ms -> 50ms)
                let squash_scale = Vec3::new(
                    original_scale.x * 1.2, // Expand horizontally
                    original_scale.y * 0.8, // Compress vertically
                    original_scale.z,
                );

                let squash_tween = Tween::new(
                    EaseFunction::QuadraticOut,
                    std::time::Duration::from_millis(50),
                    TransformScaleLens {
                        start: original_scale,
                        end: squash_scale,
                    },
                );

                // Phase 2: Bounce back to normal (50ms -> 150ms = 100ms duration)
                let bounce_tween = Tween::new(
                    EaseFunction::ElasticOut,
                    std::time::Duration::from_millis(100),
                    TransformScaleLens {
                        start: squash_scale,
                        end: original_scale,
                    },
                );

                // Chain the animations: squash then bounce
                let sequence = squash_tween.then(bounce_tween);

                commands.entity(entity).insert(TweenAnim::new(sequence));
                commands
                    .entity(entity)
                    .remove::<(RequiredAnimation, RunWithNoModel)>();
                commands.trigger(SFXEvent::ui("put"));
            }
            continue;
        }

        // Otherwise, find and animate the Model entity
        for (model_entity, link_ref, transform) in q_model.iter() {
            if link_ref.0 != entity {
                continue;
            }
            //put the animation code here
            let original_scale = transform.scale;

            // Phase 1: Squash on impact (0ms -> 50ms)
            let squash_scale = Vec3::new(
                original_scale.x * 1.2, // Expand horizontally
                original_scale.y * 0.8, // Compress vertically
                original_scale.z,
            );

            let squash_tween = Tween::new(
                EaseFunction::QuadraticOut,
                std::time::Duration::from_millis(50),
                TransformScaleLens {
                    start: original_scale,
                    end: squash_scale,
                },
            );

            // Phase 2: Bounce back to normal (50ms -> 150ms = 100ms duration)
            let bounce_tween = Tween::new(
                EaseFunction::ElasticOut,
                std::time::Duration::from_millis(100),
                TransformScaleLens {
                    start: squash_scale,
                    end: original_scale,
                },
            );

            // Chain the animations: squash then bounce
            let sequence = squash_tween.then(bounce_tween);

            commands
                .entity(model_entity)
                .insert(TweenAnim::new(sequence));
            commands.entity(entity).remove::<RequiredAnimation>();
            commands.trigger(SFXEvent::ui("put"));
        }
    }
}
