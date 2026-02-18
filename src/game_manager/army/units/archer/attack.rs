use crate::prelude::*;

// ============================================================================
// Constants
// ============================================================================

/// Maximum height of the arrow's parabolic arc in pixels.
const ARROW_MAX_HEIGHT: f32 = 80.0;

/// Duration of arrow flight in seconds.
const ARROW_FLIGHT_DURATION: f32 = 0.6;

// ============================================================================
// Components
// ============================================================================

/// Marker component for arrow projectiles.
/// Uses Actor system which auto-creates Model → MainMesh hierarchy.
/// The Actor's Transform handles ground position (linear X/Y movement).
/// The Model's Transform.y is used for visual arc height.
#[derive(Component, Default, Prefab, Reflect)]
#[require(SpriteActor, SpriteLayer::VFX, DespawnOnExit::<GameState>(GameState::Battle))]
pub struct Arrow;

/// Flight data for arrow projectiles.
#[derive(Component, Reflect)]
pub struct ArrowFlight {
    /// Starting position (2D ground position).
    pub start: Vec2,
    /// Target position (2D ground position).
    pub end: Vec2,
    /// Flight progress timer.
    pub timer: Timer,
    /// Maximum arc height in pixels.
    pub max_height: f32,
    /// Damage to deal on impact.
    pub damage: f32,
    /// Target entity to damage.
    pub target: Entity,
    /// The archer unit that fired this arrow.
    pub shooter: Entity,
}

// ============================================================================
// Plugin
// ============================================================================

pub(crate) fn plugin(app: &mut bevy::app::App) {
    app.register_type::<Arrow>();
    app.register_type::<ArrowFlight>();

    app.add_systems(
        Update,
        (
            archer_attack_system.in_set(AttackSet::Attack),
            arrow_movement_system.after(AttackSet::Attack),
            arrow_impact_system.after(arrow_movement_system),
        )
            .run_if(in_state(GameState::Battle)),
    );
}

// ============================================================================
// Systems
// ============================================================================

/// Handles archer units firing arrows when attack timer finishes.
fn archer_attack_system(
    time: Res<Time>,
    mut q_archers: Query<
        (
            Entity,
            &UnitState,
            &Target,
            &UnitStats,
            &mut AttackTimer,
            &GlobalTransform,
        ),
        With<Ranged>,
    >,
    q_targets: Query<&GlobalTransform>,
    mut commands: Commands,
) {
    for (archer_entity, state, target, stats, mut attack_timer, archer_transform) in &mut q_archers
    {
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

        // Get target position
        let Ok(target_transform) = q_targets.get(target_entity) else {
            continue;
        };

        let start_pos = archer_transform.translation().truncate();
        let end_pos = target_transform.translation().truncate();

        // Spawn the arrow projectile
        spawn_arrow(
            &mut commands,
            start_pos,
            end_pos,
            stats.damage,
            target_entity,
            archer_entity,
        );
        commands.trigger(SFXEvent::space("arrow", start_pos).with_random_pitch(0.9, 1.1));
    }
}

/// Spawns an arrow projectile.
///
/// Structure (via Actor system):
/// - Arrow (Actor): Ground position, moves linearly on X/Y
///   - Model: Visual offset, Y adjusted for arc height
///     - MainMesh: Actual sprite (loaded from aseprite)
fn spawn_arrow(
    commands: &mut Commands,
    start: Vec2,
    end: Vec2,
    damage: f32,
    target: Entity,
    shooter: Entity,
) {
    // Calculate initial rotation to face target
    let direction = (end - start).normalize_or_zero();
    let angle = direction.y.atan2(direction.x);

    commands.spawn((
        Arrow,
        ArrowFlight {
            start,
            end,
            timer: Timer::from_seconds(ARROW_FLIGHT_DURATION, TimerMode::Once),
            max_height: ARROW_MAX_HEIGHT,
            damage,
            target,
            shooter,
        },
        Transform::from_translation(Vec3::new(start.x, start.y, 0.0))
            .with_rotation(Quat::from_rotation_z(angle)),
        Name::new("Arrow"),
    ));
}

/// Updates arrow positions: linear movement for Actor, parabolic height for Model child.
fn arrow_movement_system(
    time: Res<Time>,
    mut q_arrows: Query<(Entity, &mut ArrowFlight, &mut Transform), With<Arrow>>,
    q_model_belongs: Query<(Entity, &BelongTo), With<Model>>,
    mut q_model_transform: Query<&mut Transform, (With<Model>, Without<Arrow>)>,
) {
    for (arrow_entity, mut flight, mut transform) in &mut q_arrows {
        // Tick the flight timer
        flight.timer.tick(time.delta());

        // Calculate progress (0.0 to 1.0)
        let t = flight.timer.fraction();

        // Update Actor position (linear interpolation on X/Y)
        let current_pos = flight.start.lerp(flight.end, t);
        transform.translation.x = current_pos.x;
        transform.translation.y = current_pos.y;

        // Calculate parabolic height: h = 4 * max_height * t * (1 - t)
        // This creates a smooth arc peaking at t = 0.5
        let height = 4.0 * flight.max_height * t * (1.0 - t);

        // Calculate arc slope for rotation tilt
        // Derivative of height: dh/dt = 4 * max_height * (1 - 2t)
        let slope = 4.0 * flight.max_height * (1.0 - 2.0 * t);

        // Direction vector for base rotation
        let direction = (flight.end - flight.start).normalize_or_zero();
        let base_angle = direction.y.atan2(direction.x);

        // Calculate tilt angle based on arc slope
        let horizontal_dist = flight.start.distance(flight.end);
        let tilt = if horizontal_dist > 0.0 {
            (slope / horizontal_dist).atan()
        } else {
            0.0
        };

        // Combined rotation: base direction + arc tilt
        let final_angle = base_angle + tilt;
        transform.rotation = Quat::from_rotation_z(final_angle);

        // Find and update Model's Y position for visual height
        for (model_entity, belong_to) in &q_model_belongs {
            if belong_to.0 == arrow_entity {
                if let Ok(mut model_transform) = q_model_transform.get_mut(model_entity) {
                    // Update Y for visual height (Z stays unchanged for layering)
                    model_transform.translation.y = height;
                }
                break;
            }
        }
    }
}

/// Handles arrow impact: deals damage and despawns arrow.
fn arrow_impact_system(
    q_arrows: Query<(Entity, &ArrowFlight), With<Arrow>>,
    mut commands: Commands,
) {
    for (entity, flight) in &q_arrows {
        // Check if arrow has reached its target
        if !flight.timer.is_finished() {
            continue;
        }

        commands.trigger(AttackEvent::new(
            flight.shooter,
            flight.target,
            flight.damage,
        ));

        // Despawn the arrow and its children
        commands.entity(entity).despawn();
    }
}
