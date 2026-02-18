use crate::prelude::*;

pub(crate) fn plugin(app: &mut bevy::app::App) {
    app.add_systems(Update, unit_wobble_animation_system);
}

// Wobble animation configuration
const WOBBLE_SPEED: f32 = 15.0; // Speed of wobble oscillation
const WOBBLE_AMOUNT: f32 = 0.15; // Rotation amplitude in radians (~8.5 degrees)
const MIN_VELOCITY_FOR_WOBBLE: f32 = 0.001; // Minimum velocity squared to trigger wobble
const RANDOM_OFFSET_MULTIPLIER: f32 = 0.371; // Magic number for pseudo-random offset
const BOUNCE_AMPLITUDE: f32 = 2.0; // Vertical bounce amount in pixels
const STOP_LERP_SPEED: f32 = 10.0; // How fast rotation lerps back to identity when stopped

/// Marker component to track if wobble animation is active
#[derive(Component)]
pub struct WobbleActive;

/// System that applies wobble animation to moving units with randomness per entity.
/// Each unit gets a unique wobble pattern based on its entity index.
fn unit_wobble_animation_system(
    time: Res<Time>,
    q_actor: Query<(Entity, &Velocity), Or<(With<PlayerUnit>, With<EnemyUnit>)>>,
    mut q_model: Query<(&mut Transform, &BelongTo), With<Model>>,
) {
    let t = time.elapsed_secs();

    for (actor_entity, velocity) in &q_actor {
        // Check if unit is moving
        if velocity.0.length_squared() > MIN_VELOCITY_FOR_WOBBLE {
            // Find the Model entity for this Actor
            for (mut transform, belong_to) in &mut q_model {
                if belong_to.0 != actor_entity {
                    continue;
                }

                // 🔥 Magic: Use Entity index to create pseudo-random offset
                // This makes each unit wobble at a slightly different phase
                let random_offset = (actor_entity.index_u32() as f32) * RANDOM_OFFSET_MULTIPLIER;

                // sin(time * speed + random offset) * amplitude
                let angle = (t * WOBBLE_SPEED + random_offset).sin() * WOBBLE_AMOUNT;

                // Apply rotation (Z-axis rotation = 2D plane wobble)
                transform.rotation = Quat::from_rotation_z(angle);

                // 💡 Bonus effect: Add vertical bounce while walking
                // Use cos with double frequency to match wobble rhythm
                // let bounce = (t * WOBBLE_SPEED * 2.0 + random_offset).cos().abs() * BOUNCE_AMPLITUDE;
                // Note: Uncomment the line below if you want bounce effect
                // Warning: Only use this if Model has a parent entity, or it will affect actual position
                // transform.translation.y = bounce;
            }
        } else {
            // Unit stopped - smoothly lerp back to upright position
            for (mut transform, belong_to) in &mut q_model {
                if belong_to.0 != actor_entity {
                    continue;
                }

                transform.rotation = transform
                    .rotation
                    .lerp(Quat::IDENTITY, time.delta_secs() * STOP_LERP_SPEED);
            }
        }
    }
}
