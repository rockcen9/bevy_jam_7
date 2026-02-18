use crate::prelude::*;

// ============================================================================
// Plugin
// ============================================================================

pub(crate) fn plugin(app: &mut bevy::app::App) {
    app.add_systems(
        Update,
        (
            attack_system.in_set(AttackSet::Attack),
            // debug_draw_targets,
        ),
    );
}

// ============================================================================
// Systems
// ============================================================================

/// Handles units attacking their targets when in Attacking state.
fn attack_system(
    time: Res<Time>,
    mut q_attackers: Query<
        (Entity, &UnitState, &Target, &UnitStats, &mut AttackTimer),
        With<Melee>,
    >,
    mut commands: Commands,
) {
    for (entity, state, target, stats, mut attack_timer) in &mut q_attackers {
        // Only attack when in Attacking state
        if *state != UnitState::Attacking {
            continue;
        }

        // Tick the attack timer
        attack_timer.0.tick(time.delta());

        // Check if attack is ready
        if !attack_timer.0.just_finished() {
            continue;
        }

        // Get target entity
        let Some(target_entity) = target.0 else {
            continue;
        };

        // Get target's health and apply damage
        commands.trigger(AttackEvent::new(entity, target_entity, stats.damage));
    }
}

/// Debug: Draw lines from one player unit to its target.
fn _debug_draw_targets(
    mut gizmos: Gizmos,
    q_player_units: Query<(&GlobalTransform, &Target), With<PlayerUnit>>,
    q_targets: Query<&GlobalTransform>,
) {
    let Some((transform, target)) = q_player_units.iter().last() else {
        return;
    };

    let start = transform.translation();

    // Green circle on player unit
    gizmos.circle(start, 32.0, Color::srgb(0.0, 1.0, 0.0));

    let Some(target_entity) = target.0 else {
        return;
    };

    let Ok(target_transform) = q_targets.get(target_entity) else {
        return;
    };

    let end = target_transform.translation();

    // Red circle on target
    gizmos.circle(end, 32.0, Color::srgb(1.0, 0.0, 0.0));

    // Line connecting them
    gizmos.line(start, end, Color::srgb(0.0, 1.0, 0.0));
}
