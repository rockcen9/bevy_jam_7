use std::time::Duration;

use bevy_spatial::{
    AutomaticUpdate, SpatialAccess, SpatialStructure, TransformMode, kdtree::KDTree2,
};

use crate::prelude::*;
// ============================================================================
// Plugin
// ============================================================================

pub(crate) fn plugin(app: &mut bevy::app::App) {
    // Add velocity tracking systems
    app.add_systems(
        Update,
        (init_velocity_tracking, update_velocity_from_movement)
            .chain()
            .in_set(MovementSet::VelocityTracking),
    );
    // Add spatial indexing plugins for both unit types
    // Using KDTree2 for 2D spatial queries (ignores Z axis)
    // GlobalTransform mode because units are children of squads
    app.add_plugins(
        AutomaticUpdate::<PlayerUnit>::new()
            .with_spatial_ds(SpatialStructure::KDTree2)
            .with_transform(TransformMode::GlobalTransform)
            .with_frequency(Duration::from_millis(50)),
    );
    app.add_plugins(
        AutomaticUpdate::<EnemyUnit>::new()
            .with_spatial_ds(SpatialStructure::KDTree2)
            .with_transform(TransformMode::GlobalTransform)
            .with_frequency(Duration::from_millis(50)),
    );

    // Configure system ordering: VelocityTracking -> TargetFinding -> Movement -> Separation
    // VelocityTracking must run first so animations can read velocity in the same frame
    app.configure_sets(
        Update,
        (
            MovementSet::VelocityTracking,
            MovementSet::TargetFinding,
            MovementSet::Movement,
            MovementSet::Separation,
        )
            .chain()
            .run_if(in_state(GameState::Battle)),
    );

    app.add_systems(
        Update,
        (
            target_finding_system.in_set(MovementSet::TargetFinding),
            movement_and_state_system.in_set(MovementSet::Movement),
            separation_system.in_set(MovementSet::Separation),
        ),
    );
}
// ============================================================================
// Components
// ============================================================================

/// Velocity component tracks the current movement velocity of a unit.
/// Calculated from position changes between frames and used by animation systems.
#[derive(Component, Default, Reflect, Debug)]
pub struct Velocity(pub Vec2);

/// Internal component to track previous position for velocity calculation.
#[derive(Component)]
struct PreviousPosition(Vec2);

#[derive(Component, Default, PartialEq, Clone, Copy, Debug, Reflect)]
pub struct Melee;

#[derive(Component, Default, PartialEq, Clone, Copy, Debug, Reflect)]
pub struct Ranged;
/// Unit state machine for movement and combat behavior.
#[derive(Component, Default, PartialEq, Clone, Copy, Debug, Reflect)]
pub enum UnitState {
    #[default]
    Idle,
    Moving,
    Attacking,
    Dead,
}

/// Stats that control unit movement and attack behavior.
#[derive(Component, Debug, Clone, Reflect)]
pub struct UnitStats {
    /// Movement speed in pixels per second.
    pub speed: f32,
    /// Distance at which the unit can attack its target.
    pub attack_range: f32,
    /// Damage dealt per attack.
    pub damage: f32,
    /// Seconds between attacks.
    pub attack_speed: f32,
    /// Defense value that reduces incoming damage.
    pub defense: f32,
    /// The kind of this unit.
    pub unity_kind: UnitKind,
    /// The unit kind this unit counters, if any.
    pub counter: Option<UnitKind>,
}

impl UnitStats {
    pub fn melee(unity_type: UnitKind) -> Self {
        Self {
            speed: 100.0,
            attack_range: 64.0,
            damage: 10.0,
            attack_speed: 1.0,
            defense: 0.0,
            unity_kind: unity_type,
            counter: None,
        }
    }
    pub fn ranged(unity_type: UnitKind) -> Self {
        Self {
            speed: 100.0,
            attack_range: 1024.0,
            damage: 5.0,
            attack_speed: 1.0,
            defense: 0.0,
            unity_kind: unity_type,
            counter: None,
        }
    }
}

/// Soft collision component for unit separation.
#[derive(Component, Debug, Clone, Reflect)]
pub struct UnitCollider {
    /// Collision radius in pixels.
    pub radius: f32,
    /// How strongly this unit pushes others apart (0.0 - 1.0).
    pub push_strength: f32,
}

impl Default for UnitCollider {
    fn default() -> Self {
        Self {
            radius: 32.0, // Half of 64px unit size
            push_strength: 0.5,
        }
    }
}

/// The current target this unit is chasing/attacking.
#[derive(Component, Default, Debug, Reflect)]
pub struct Target(pub Option<Entity>);

// ============================================================================
// Spatial Index Type Aliases
// ============================================================================

/// Spatial index for player units (enemies query this to find players).
type PlayerSpatialTree = KDTree2<PlayerUnit>;

/// Spatial index for enemy units (players query this to find enemies).
type EnemySpatialTree = KDTree2<EnemyUnit>;

// ============================================================================
// System Sets for Ordering
// ============================================================================

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum MovementSet {
    /// Tracks velocity from position changes - runs first
    VelocityTracking,
    TargetFinding,
    Movement,
    Separation,
}

// ============================================================================
// Systems
// ============================================================================

/// Number of nearest candidates to consider for target selection.
const K_NEAREST_CANDIDATES: usize = 3;

/// Distance threshold for melee proximity override.
/// If an enemy is closer than this, the unit will immediately switch targets.
/// This prevents "tunnel vision" where units ignore enemies blocking their path.
const MELEE_THRESHOLD: f32 = 80.0;

/// Finds targets for units using spatial indexing (k-d tree).
/// Uses k-nearest neighbor search with anti-convergence heuristic.
///
/// Implements "Melee Proximity Override": if an enemy is within MELEE_THRESHOLD,
/// the unit immediately switches to that target, unless current target is already
/// in melee range (debounce to prevent flickering).
fn target_finding_system(
    mut q_player_units: Query<
        (Entity, &GlobalTransform, &mut Target),
        (With<PlayerUnit>, Without<EnemyUnit>),
    >,
    mut q_enemy_units: Query<
        (Entity, &GlobalTransform, &mut Target),
        (With<EnemyUnit>, Without<PlayerUnit>),
    >,
    player_tree: Res<PlayerSpatialTree>,
    enemy_tree: Res<EnemySpatialTree>,
    q_unit_state: Query<&UnitState>,
    q_global_transform: Query<&GlobalTransform>,
) {
    // Player units find enemies using the enemy spatial tree
    for (entity, transform, mut target) in &mut q_player_units {
        let my_pos = transform.translation().truncate();

        // Debounce: if current target is already in melee range, keep it
        if is_current_target_in_melee_range(my_pos, &target, &q_unit_state, &q_global_transform) {
            continue;
        }

        // Melee Proximity Override: check if closest alive enemy is within melee threshold
        // Use k=5 to skip over any dead units (corpses) that might be closer
        if let Some(new_target) = check_melee_override(
            my_pos,
            enemy_tree.k_nearest_neighbour(my_pos, 5),
            &q_unit_state,
        ) {
            target.0 = Some(new_target);
            continue;
        }

        // Standard logic: only find new target if current is invalid
        if !needs_new_target(&target, &q_unit_state) {
            continue;
        }

        target.0 = find_target_from_candidates(
            entity,
            my_pos,
            enemy_tree.k_nearest_neighbour(my_pos, K_NEAREST_CANDIDATES),
            &q_unit_state,
        );
    }

    // Enemy units find players using the player spatial tree
    for (entity, transform, mut target) in &mut q_enemy_units {
        let my_pos = transform.translation().truncate();

        // Debounce: if current target is already in melee range, keep it
        if is_current_target_in_melee_range(my_pos, &target, &q_unit_state, &q_global_transform) {
            continue;
        }

        // Melee Proximity Override: check if closest alive player is within melee threshold
        // Use k=5 to skip over any dead units (corpses) that might be closer
        if let Some(new_target) = check_melee_override(
            my_pos,
            player_tree.k_nearest_neighbour(my_pos, 5),
            &q_unit_state,
        ) {
            target.0 = Some(new_target);
            continue;
        }

        // Standard logic: only find new target if current is invalid
        if !needs_new_target(&target, &q_unit_state) {
            continue;
        }

        target.0 = find_target_from_candidates(
            entity,
            my_pos,
            player_tree.k_nearest_neighbour(my_pos, K_NEAREST_CANDIDATES),
            &q_unit_state,
        );
    }
}

/// Checks if the current target is alive and within melee range.
/// Used for debouncing to prevent target flickering.
fn is_current_target_in_melee_range(
    my_pos: Vec2,
    target: &Target,
    q_unit_state: &Query<&UnitState>,
    q_global_transform: &Query<&GlobalTransform>,
) -> bool {
    let Some(target_entity) = target.0 else {
        return false;
    };

    // Check if target is alive
    if let Ok(state) = q_unit_state.get(target_entity) {
        if *state == UnitState::Dead {
            return false;
        }
    } else {
        return false;
    }

    // Check if target is within melee range
    let Ok(target_transform) = q_global_transform.get(target_entity) else {
        return false;
    };

    let target_pos = target_transform.translation().truncate();
    let distance_sq = my_pos.distance_squared(target_pos);
    let threshold_sq = MELEE_THRESHOLD * MELEE_THRESHOLD;

    distance_sq < threshold_sq
}

/// Checks if the closest alive enemy is within melee threshold.
/// Returns Some(entity) if override should happen, None otherwise.
/// Iterates through candidates to skip dead units (e.g., corpses blocking the way).
fn check_melee_override(
    my_pos: Vec2,
    nearest: Vec<(Vec2, Option<Entity>)>,
    q_unit_state: &Query<&UnitState>,
) -> Option<Entity> {
    let threshold_sq = MELEE_THRESHOLD * MELEE_THRESHOLD;

    for (pos, maybe_entity) in nearest {
        let Some(entity) = maybe_entity else {
            continue;
        };

        // Check if entity is alive
        if let Ok(state) = q_unit_state.get(entity) {
            if *state == UnitState::Dead {
                continue; // Skip dead units, check next candidate
            }
        } else {
            continue;
        }

        // 2D distance only (X and Y) as per project rules
        let distance_sq = my_pos.distance_squared(pos);

        if distance_sq < threshold_sq {
            return Some(entity);
        } else {
            // k_nearest is sorted by distance, so if the first alive unit is too far,
            // all subsequent ones will be even farther
            return None;
        }
    }
    None
}

/// Checks if a unit needs a new target (no target, or target is dead/despawned).
fn needs_new_target(target: &Target, q_unit_state: &Query<&UnitState>) -> bool {
    match target.0 {
        None => true,
        Some(target_entity) => {
            // Check if target still exists and is not dead
            match q_unit_state.get(target_entity) {
                Ok(state) => *state == UnitState::Dead,
                Err(_) => true, // Entity despawned
            }
        }
    }
}

/// Selects a target from k-nearest candidates with anti-convergence heuristic.
///
/// Strategy:
/// 1. Filter out dead/despawned entities
/// 2. Use entity index as a deterministic "jitter" to spread target selection
///    - This prevents all backline units from locking onto the exact same front-line enemy
/// 3. Still prioritize closer targets, but with slight variance
fn find_target_from_candidates(
    self_entity: Entity,
    my_pos: Vec2,
    candidates: Vec<(Vec2, Option<Entity>)>,
    q_unit_state: &Query<&UnitState>,
) -> Option<Entity> {
    // Filter to only valid (alive, existing) candidates
    let valid_candidates: Vec<(Entity, Vec2, f32)> = candidates
        .into_iter()
        .filter_map(|(pos, maybe_entity)| {
            let entity = maybe_entity?;

            // Skip dead entities
            if let Ok(state) = q_unit_state.get(entity) {
                if *state == UnitState::Dead {
                    return None;
                }
            } else {
                // Entity doesn't have UnitState (despawned or invalid)
                return None;
            }

            // 2D distance only (X and Y) as per project rules
            let dist_sq = my_pos.distance_squared(pos);
            Some((entity, pos, dist_sq))
        })
        .collect();

    if valid_candidates.is_empty() {
        return None;
    }

    // If only one candidate, return it
    if valid_candidates.len() == 1 {
        return Some(valid_candidates[0].0);
    }

    // Anti-convergence heuristic:
    // Use the querying entity's index to deterministically pick from top candidates.
    // This spreads targets across the army without pure randomness.
    let self_index = self_entity.index_u32() as usize;

    // Find the closest distance squared for reference
    let min_dist_sq = valid_candidates
        .iter()
        .map(|(_, _, d_sq)| *d_sq)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(0.0);

    // Filter to candidates within 20% of the closest (or within 64px, whichever is larger)
    // This creates a "tier" of roughly equivalent targets
    // Note: comparing squared distances - adjust threshold accordingly
    let min_dist = min_dist_sq.sqrt();
    let distance_threshold_sq = ((min_dist * 1.2).max(min_dist + 64.0)).powi(2);

    let top_tier: Vec<_> = valid_candidates
        .iter()
        .filter(|(_, _, dist_sq)| *dist_sq <= distance_threshold_sq)
        .collect();

    if top_tier.is_empty() {
        // Fallback to closest
        return valid_candidates
            .iter()
            .min_by(|a, b| a.2.partial_cmp(&b.2).unwrap())
            .map(|(e, _, _)| *e);
    }

    // Pick from top tier based on entity index (deterministic spread)
    let pick_index = self_index % top_tier.len();
    Some(top_tier[pick_index].0)
}

/// Radius within which nearby allies exert a lateral separation steering force.
/// Wider than the hard collision radius to proactively bend movement paths.
const ALLY_SEPARATION_RADIUS: f32 = 90.0;

/// Weight of the ally separation force relative to the seek force.
/// Higher values cause units to flow around each other more aggressively.
const ALLY_SEPARATION_WEIGHT: f32 = 0.7;

/// Computes a NOC-style separation steering force away from nearby allies.
/// Returns an average of inverse-distance-weighted flee vectors.
/// This is the standard "separation" behavior from Reynolds / Nature of Code.
fn compute_ally_separation(entity: Entity, pos: Vec2, allies: &[(Entity, Vec2)]) -> Vec2 {
    let mut force = Vec2::ZERO;
    let mut count = 0u32;

    let radius_sq = ALLY_SEPARATION_RADIUS * ALLY_SEPARATION_RADIUS;
    let min_dist_sq = 0.001 * 0.001;

    for (other_entity, other_pos) in allies {
        if *other_entity == entity {
            continue;
        }
        let diff = pos - *other_pos;
        let dist_sq = diff.length_squared();

        if dist_sq < radius_sq && dist_sq > min_dist_sq {
            // Only calculate sqrt when we know we're in range
            let dist = dist_sq.sqrt();
            // Weight by inverse distance: closer allies push harder
            force += diff.normalize_or_zero() / dist;
            count += 1;
        }
    }

    if count > 0 {
        force / count as f32
    } else {
        Vec2::ZERO
    }
}

/// Handles unit movement towards target and state transitions.
/// Uses NOC steering: blends seek force with ally separation to produce
/// fluid enveloping movement around the frontline.
fn movement_and_state_system(
    time: Res<Time>,
    mut q_units: Query<(
        Entity,
        &mut Transform,
        &GlobalTransform,
        &mut UnitState,
        &Target,
        &UnitStats,
        &UnitCollider,
    )>,
    q_targets: Query<(&GlobalTransform, &UnitCollider)>,
) {
    // Snapshot all ally positions (read-only pass) for separation calculation.
    // Collected once before the mutable loop to avoid borrow conflicts.
    let ally_snapshot: Vec<(Entity, Vec2)> = q_units
        .iter()
        .filter(|(_, _, _, state, _, _, _)| **state != UnitState::Dead)
        .map(|(e, _, gt, _, _, _, _)| (e, gt.translation().truncate()))
        .collect();

    for (entity, mut transform, global_transform, mut state, target, stats, collider) in
        &mut q_units
    {
        // Skip dead units
        if *state == UnitState::Dead {
            continue;
        }

        let Some(target_entity) = target.0 else {
            // No target - go idle
            if *state != UnitState::Idle {
                *state = UnitState::Idle;
            }
            continue;
        };

        let Ok((target_transform, target_collider)) = q_targets.get(target_entity) else {
            // Target despawned - will get new target next frame
            continue;
        };

        // Use GlobalTransform for world-space position (units are children of squads)
        let my_pos = global_transform.translation().truncate();
        let target_pos = target_transform.translation().truncate();

        // 2D distance only (X and Y) as per project rules
        let dist_to_target_sq = my_pos.distance_squared(target_pos);

        // Calculate stop threshold:
        // We stop when we're within attack range, but also account for collision radii
        // to prevent units from getting stuck just outside attack range.
        // Add a small buffer (2.0px) to create a "dead zone" between movement and separation.
        // Without this buffer, movement stops at 64.0 and separation pushes at <64.0,
        // causing position oscillation (jitter) at the boundary.
        let combined_radii = collider.radius + target_collider.radius;
        let stop_buffer = 2.0;
        let stop_threshold = stats.attack_range.max(combined_radii + stop_buffer);

        // Hysteresis: If already attacking, allow being pushed back slightly without
        // immediately switching back to Moving. This prevents oscillation between
        // Movement (pulling in) and Separation (pushing out) systems.
        let hysteresis_buffer = if *state == UnitState::Attacking {
            5.0
        } else {
            0.0
        };
        let threshold_with_hysteresis = stop_threshold + hysteresis_buffer;
        let is_in_range =
            dist_to_target_sq <= (threshold_with_hysteresis * threshold_with_hysteresis);

        if is_in_range {
            // Close enough to attack
            if *state != UnitState::Attacking {
                *state = UnitState::Attacking;
            }
            // Face the target even while attacking
            face_target(&mut transform, my_pos, target_pos);
        } else {
            // Need to move closer
            if *state != UnitState::Moving {
                *state = UnitState::Moving;
            }

            // --- NOC Steering: seek + ally separation ---
            // Desired velocity: full speed straight toward target
            let seek = (target_pos - my_pos).normalize_or_zero() * stats.speed;

            // Separation force: steer away from nearby allies.
            // This bends the seek path so units flow around each other
            // instead of tunneling into a single column.
            let sep = compute_ally_separation(entity, my_pos, &ally_snapshot) * stats.speed;

            // Blend: seek drives toward enemy, separation steers laterally around allies.
            // clamp_length_max ensures we never exceed max speed.
            let desired = seek + sep * ALLY_SEPARATION_WEIGHT;
            let movement = desired.clamp_length_max(stats.speed) * time.delta_secs();

            transform.translation.x += movement.x;
            transform.translation.y += movement.y;

            // Face the target (not movement direction) so units always look at the enemy
            face_target(&mut transform, my_pos, target_pos);
        }
    }
}

fn face_target(transform: &mut Transform, my_pos: Vec2, target_pos: Vec2) {
    let direction = target_pos - my_pos;

    if direction.x.abs() > 1.0 {
        let current_scale_x_abs = transform.scale.x.abs();

        if direction.x < 0.0 {
            transform.scale.x = -current_scale_x_abs;
        } else {
            transform.scale.x = current_scale_x_abs;
        }
    }
}

/// Maximum push distance per frame to prevent explosive jitter.
const MAX_PUSH_PER_FRAME: f32 = 1.0;

/// Soft collision separation system with anti-jitter damping.
/// Pushes overlapping units apart to prevent stacking.
/// Uses O(N^2) pairwise comparison - acceptable for small unit counts.
fn separation_system(
    time: Res<Time>,
    mut q_units: Query<(
        Entity,
        &mut Transform,
        &GlobalTransform,
        &UnitCollider,
        &UnitState,
    )>,
) {
    // Collect all unit data for comparison (position, radius, push_strength, state)
    let mut units: Vec<(Entity, Vec2, f32, f32, UnitState)> = Vec::new();
    for (entity, _transform, global_transform, collider, state) in q_units.iter() {
        // Skip dead units for collision
        if *state == UnitState::Dead {
            continue;
        }
        // Use GlobalTransform for world-space position (units are children of squads)
        units.push((
            entity,
            global_transform.translation().truncate(),
            collider.radius,
            collider.push_strength,
            *state,
        ));
    }

    // Calculate push vectors for each entity
    let mut push_map: std::collections::HashMap<Entity, Vec2> = std::collections::HashMap::new();

    for i in 0..units.len() {
        for j in (i + 1)..units.len() {
            let (entity_a, pos_a, radius_a, strength_a, state_a) = units[i];
            let (entity_b, pos_b, radius_b, strength_b, state_b) = units[j];

            let combined_radius = radius_a + radius_b;
            let diff = pos_a - pos_b;
            // 2D distance only (X and Y) as per project rules
            let dist_sq = diff.length_squared();
            let combined_radius_sq = combined_radius * combined_radius;
            let min_dist_sq = 0.001 * 0.001;

            // Check for overlap
            if dist_sq < combined_radius_sq && dist_sq > min_dist_sq {
                // Only calculate sqrt when we know there's overlap
                let dist = dist_sq.sqrt();

                // Calculate overlap depth
                let overlap = combined_radius - dist;

                // Normalize direction (from B to A)
                let push_dir = diff / dist;

                // State weighting: attacking units are harder to push (more "mass").
                // This keeps front-line units stable while back-line units slide around.
                let weight_a = if state_a == UnitState::Attacking {
                    0.2
                } else {
                    1.0
                };
                let weight_b = if state_b == UnitState::Attacking {
                    0.2
                } else {
                    1.0
                };

                // Average push strength of both units
                let avg_strength = (strength_a + strength_b) * 0.5;

                // Calculate push force
                let push_force = push_dir * overlap * avg_strength;

                // Unit A gets pushed in positive direction, B in negative
                // Apply state weighting to reduce push on attacking units
                *push_map.entry(entity_a).or_insert(Vec2::ZERO) += push_force * weight_a;
                *push_map.entry(entity_b).or_insert(Vec2::ZERO) -= push_force * weight_b;
            }
        }
    }

    // Apply push vectors to transforms with max force clamping
    // Respect paused time: if game is paused, don't push units
    let delta = time.delta_secs();
    if delta > 0.0 {
        for (entity, mut transform, _global_transform, _collider, state) in &mut q_units {
            if *state == UnitState::Dead {
                continue;
            }

            if let Some(push) = push_map.get(&entity) {
                // Clamp push magnitude to prevent explosive movement from accumulated forces.
                // Even if 100 units push you, you only move MAX_PUSH_PER_FRAME pixels.
                let push_len_sq = push.length_squared();
                let max_push_sq = MAX_PUSH_PER_FRAME * MAX_PUSH_PER_FRAME;

                let final_push = if push_len_sq > max_push_sq {
                    push.normalize() * MAX_PUSH_PER_FRAME
                } else {
                    *push
                };

                transform.translation.x += final_push.x;
                transform.translation.y += final_push.y;
            }
        }
    }
}

// ============================================================================
// Velocity Tracking Systems
// ============================================================================

/// Deadzone threshold for velocity calculation (in pixels).
/// Movements smaller than this are considered zero to prevent jittering.
/// This prevents animation flickering when units are stuck, pushing each other,
/// or experiencing floating-point rounding errors.
// const VELOCITY_DEADZONE: f32 = 1.0;

/// Automatically add Velocity and PreviousPosition components to units that don't have them.
fn init_velocity_tracking(
    q_units: Query<
        (Entity, &GlobalTransform),
        (Or<(With<PlayerUnit>, With<EnemyUnit>)>, Without<Velocity>),
    >,
    mut commands: Commands,
) {
    for (entity, global_transform) in q_units.iter() {
        let pos = global_transform.translation().truncate();
        commands
            .entity(entity)
            .insert((Velocity(Vec2::ZERO), PreviousPosition(pos)));
    }
}

fn update_velocity_from_movement(
    time: Res<Time>,
    mut q_units: Query<
        (&GlobalTransform, &mut Velocity, &mut PreviousPosition),
        Or<(With<PlayerUnit>, With<EnemyUnit>)>,
    >,
) {
    let delta_time = time.delta_secs();
    if delta_time <= 0.0 {
        return;
    }

    for (global_transform, mut velocity, mut prev_pos) in q_units.iter_mut() {
        let current_pos = global_transform.translation().truncate();
        let delta_pos = current_pos - prev_pos.0;

        let actual_velocity = delta_pos / delta_time;

        if actual_velocity.length_squared() < 100.0 {
            velocity.0 = Vec2::ZERO;
        } else {
            velocity.0 = actual_velocity;
        }

        prev_pos.0 = current_pos;
    }
}
