const UNIT_SIZE_1080: f32 = 128.0;
use bevy::platform::collections::HashMap;

use crate::prelude::*;
// ============================================================================
// Plugin
// ============================================================================

pub(crate) fn plugin(app: &mut bevy::app::App) {
    // Init spatial grid resource
    app.init_resource::<UnitSpatialGrid>();

    app.add_systems(
        Update,
        (|mut q_unit: Query<(Entity, &mut Transform, &GlobalTransform), With<Unit>>,
          mut commands: Commands| {
            for (entity, mut transform, global_transform) in q_unit.iter_mut() {
                transform.translation = global_transform.translation();
                commands.entity(entity).remove::<ChildOf>();
            }
        })
        .run_if(in_state(GameState::Battle)),
    );
    // Add velocity tracking systems
    app.add_systems(
        Update,
        (init_velocity_tracking, update_velocity_from_movement)
            .chain()
            .in_set(MovementSet::VelocityTracking),
    );

    // Configure system ordering:
    // VelocityTracking -> SpatialGridUpdate -> TargetFinding -> Movement -> Separation
    app.configure_sets(
        Update,
        (
            MovementSet::VelocityTracking,
            MovementSet::SpatialGridUpdate,
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
            update_spatial_grid.in_set(MovementSet::SpatialGridUpdate),
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
pub enum UnitAction {
    #[default]
    Idle,
    Moving,
    Attacking,
}

/// Stats that control unit movement and attack behavior.
#[derive(Component, Debug, Clone, Reflect)]
pub struct CombatAttributes {
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

impl CombatAttributes {
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
            radius: UNIT_SIZE_1080 / 2.0,
            push_strength: 0.5,
        }
    }
}

/// The current target this unit is chasing/attacking.
#[derive(Component, Default, Debug, Reflect)]
pub struct Target(pub Option<Entity>);

// ============================================================================
// Uniform Spatial Hash Grid
// ============================================================================

/// Cell size for the spatial hash grid.
/// Must be >= the largest query radius used (ALLY_SEPARATION_RADIUS = 90,
/// max collision diameter = 64). 128 gives a comfortable margin so a single
/// 9-cell neighbourhood covers all relevant neighbours.
const GRID_CELL_SIZE: f32 = 128.0;

fn world_to_grid(pos: Vec2) -> (i32, i32) {
    (
        (pos.x / GRID_CELL_SIZE).floor() as i32,
        (pos.y / GRID_CELL_SIZE).floor() as i32,
    )
}

/// Per-unit data stored in the spatial grid.
#[derive(Clone, Copy)]
pub(crate) struct GridUnit {
    pub entity: Entity,
    pub pos: Vec2,
    pub radius: f32,
    pub push_strength: f32,
    pub faction: Faction,
}

/// Uniform spatial hash grid for O(1) neighbour lookups.
/// Rebuilt every frame in O(N).
#[derive(Resource, Default)]
pub(crate) struct UnitSpatialGrid {
    pub cells: HashMap<(i32, i32), Vec<GridUnit>>,
}

/// Maximum ring radius for expanding search (covers 1920x1080 diagonal ≈ 2203px).
const MAX_RING_RADIUS: i32 = 32;

impl UnitSpatialGrid {
    /// Iterate cells on the perimeter of a square ring at distance `r` from `center`.
    /// Ring 0 = just the center cell (1 cell).
    /// Ring r = the border cells of the (2r+1)×(2r+1) square (8r cells for r>0).
    fn for_each_in_ring(&self, center: (i32, i32), r: i32, mut f: impl FnMut(&GridUnit)) {
        if r == 0 {
            if let Some(neighbors) = self.cells.get(&center) {
                for n in neighbors {
                    f(n);
                }
            }
            return;
        }
        // Top and bottom rows of the ring
        for dx in -r..=r {
            for &dy in &[-r, r] {
                let cell = (center.0 + dx, center.1 + dy);
                if let Some(neighbors) = self.cells.get(&cell) {
                    for n in neighbors {
                        f(n);
                    }
                }
            }
        }
        // Left and right columns (excluding corners already covered)
        for dy in (-r + 1)..r {
            for &dx in &[-r, r] {
                let cell = (center.0 + dx, center.1 + dy);
                if let Some(neighbors) = self.cells.get(&cell) {
                    for n in neighbors {
                        f(n);
                    }
                }
            }
        }
    }

    /// Find the nearest alive enemy using expanding ring search.
    /// Starts from the unit's cell and expands outward one ring at a time.
    /// Stops as soon as a ring produces at least one candidate, with one
    /// extra ring to catch enemies at the boundary.
    // Grid contains only alive units (update_spatial_grid skips Dead), so no state check needed.
    pub(crate) fn find_nearest_enemy(
        &self,
        my_entity: Entity,
        my_pos: Vec2,
        my_faction: Faction,
        max_radius: f32,
    ) -> Option<Entity> {
        let my_cell = world_to_grid(my_pos);
        let max_ring = (max_radius / GRID_CELL_SIZE).ceil() as i32;
        let max_ring = max_ring.min(MAX_RING_RADIUS);
        let radius_sq = max_radius * max_radius;

        let mut best: Option<(Entity, f32)> = None;
        let mut found_ring: Option<i32> = None;

        for r in 0..=max_ring {
            // If we found something 2 rings ago, the best is already confirmed
            // (one extra ring ensures we don't miss a closer unit at the boundary)
            if let Some(fr) = found_ring {
                if r > fr + 1 {
                    break;
                }
            }

            self.for_each_in_ring(my_cell, r, |neighbor| {
                if neighbor.entity == my_entity {
                    return;
                }
                if neighbor.faction == my_faction {
                    return;
                }

                let dist_sq = my_pos.distance_squared(neighbor.pos);
                if dist_sq > radius_sq {
                    return;
                }

                if best.is_none() || dist_sq < best.unwrap().1 {
                    best = Some((neighbor.entity, dist_sq));
                }
            });

            if best.is_some() && found_ring.is_none() {
                found_ring = Some(r);
            }
        }

        best.map(|(e, _)| e)
    }
}

const NEIGHBOR_OFFSETS: [(i32, i32); 9] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (0, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

/// Rebuilds the spatial hash grid every frame. O(N).
/// Dead units are despawned, so every entity in the query is alive.
fn update_spatial_grid(
    mut grid: ResMut<UnitSpatialGrid>,
    q_units: Query<(Entity, &GlobalTransform, &UnitCollider, &Faction)>,
) {
    // Clear each Vec's contents but retain its allocated capacity (zero heap allocations after warm-up).
    for vec in grid.cells.values_mut() {
        vec.clear();
    }

    for (entity, global_transform, collider, faction) in q_units.iter() {
        let pos = global_transform.translation().truncate();
        let cell_coord = world_to_grid(pos);

        grid.cells.entry(cell_coord).or_default().push(GridUnit {
            entity,
            pos,
            radius: collider.radius,
            push_strength: collider.push_strength,
            faction: *faction,
        });
    }
}

// ============================================================================
// System Sets for Ordering
// ============================================================================

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum MovementSet {
    /// Tracks velocity from position changes - runs first
    VelocityTracking,
    /// Rebuilds the spatial hash grid
    SpatialGridUpdate,
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

/// How many frames to spread target searches across.
/// Units without a target are bucketed by entity index so at most 1/N do a full
/// grid search per frame, capping the per-frame cost at large unit counts.
const TARGET_SLICE_COUNT: u32 = 10;

/// Finds targets for units using the spatial hash grid.
///
/// Execution order per unit:
/// 1. Debounce  – current target still in melee range → keep it, skip all work.
/// 2. Valid target check – still has a living target → skip all work.
/// 3. Time slicing – only 1/TARGET_SLICE_COUNT of targetless units search per frame.
/// 4. Melee Proximity Override – snap to any enemy within MELEE_THRESHOLD.
/// 5. Full grid search – expanding ring until an enemy is found.
fn target_finding_system(
    grid: Res<UnitSpatialGrid>,
    mut frame: Local<u32>,
    mut q_units: Query<(Entity, &Transform, &mut Target, &Faction, &CombatAttributes)>,
    q_pawn: Query<&Pawn>,
    q_transform: Query<&Transform>,
) {
    let current_frame = *frame;
    *frame = frame.wrapping_add(1);

    for (entity, transform, mut target, faction, _stats) in &mut q_units {
        let my_pos = transform.translation.truncate();

        // 1. Debounce: current target is alive and already in melee range → keep it
        if is_current_target_in_melee_range(my_pos, &target, &q_transform) {
            continue;
        }

        // 2. Still has a valid living target → nothing to do
        if !needs_new_target(&target, &q_pawn) {
            continue;
        }

        // 3. Time slicing: spread targetless units across TARGET_SLICE_COUNT frames
        //    to avoid thousands of full grid searches in a single frame.
        if entity.index_u32() % TARGET_SLICE_COUNT != current_frame % TARGET_SLICE_COUNT {
            continue;
        }

        // 4. Melee Proximity Override: snap to any enemy already inside melee range
        if let Some(new_target) = grid.find_nearest_enemy(entity, my_pos, *faction, MELEE_THRESHOLD)
        {
            target.0 = Some(new_target);
            continue;
        }

        // 5. Full grid search
        target.0 = find_target_from_grid(entity, my_pos, *faction, &grid);
    }
}

/// Checks if the current target is within melee range.
/// Dead units are despawned, so a missing entity means the target is gone.
fn is_current_target_in_melee_range(
    my_pos: Vec2,
    target: &Target,
    q_transform: &Query<&Transform>,
) -> bool {
    let Some(target_entity) = target.0 else {
        return false;
    };

    let Ok(target_transform) = q_transform.get(target_entity) else {
        return false;
    };

    let target_pos = target_transform.translation.truncate();
    let distance_sq = my_pos.distance_squared(target_pos);
    let threshold_sq = MELEE_THRESHOLD * MELEE_THRESHOLD;

    distance_sq < threshold_sq
}

fn needs_new_target(target: &Target, q_unit: &Query<&Pawn>) -> bool {
    match target.0 {
        None => true,
        Some(target_entity) => !q_unit.contains(target_entity),
    }
}

/// Selects a target from the spatial grid using expanding ring search
/// with anti-convergence heuristic.
///
/// Strategy:
/// 1. Expand outward ring by ring until we find enemies
/// 2. Collect candidates from the found ring + 1 extra ring (boundary)
/// 3. Use entity index as a deterministic "jitter" to spread target selection
// Grid contains only alive units (update_spatial_grid skips Dead), so no state check needed.
fn find_target_from_grid(
    self_entity: Entity,
    my_pos: Vec2,
    my_faction: Faction,
    grid: &UnitSpatialGrid,
) -> Option<Entity> {
    let my_cell = world_to_grid(my_pos);

    let mut candidates: Vec<(Entity, f32)> = Vec::new();
    let mut found_ring: Option<i32> = None;

    for r in 0..=MAX_RING_RADIUS {
        // Stop after one extra ring past the first ring with results
        if let Some(fr) = found_ring {
            if r > fr + 1 {
                break;
            }
        }

        grid.for_each_in_ring(my_cell, r, |neighbor| {
            if neighbor.entity == self_entity {
                return;
            }
            if neighbor.faction == my_faction {
                return;
            }

            let dist_sq = my_pos.distance_squared(neighbor.pos);
            candidates.push((neighbor.entity, dist_sq));
        });

        if !candidates.is_empty() && found_ring.is_none() {
            found_ring = Some(r);
        }
    }

    if candidates.is_empty() {
        return None;
    }

    if candidates.len() == 1 {
        return Some(candidates[0].0);
    }

    // Sort by distance and take top K
    candidates.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    candidates.truncate(K_NEAREST_CANDIDATES);

    // Anti-convergence heuristic:
    // Use the querying entity's index to deterministically pick from top candidates.
    let self_index = self_entity.index_u32() as usize;

    let min_dist_sq = candidates[0].1;
    let min_dist = min_dist_sq.sqrt();
    let distance_threshold_sq = ((min_dist * 1.2).max(min_dist + 64.0)).powi(2);

    let top_tier: Vec<_> = candidates
        .iter()
        .filter(|(_, dist_sq)| *dist_sq <= distance_threshold_sq)
        .collect();

    if top_tier.is_empty() {
        return Some(candidates[0].0);
    }

    let pick_index = self_index % top_tier.len();
    Some(top_tier[pick_index].0)
}

/// Radius within which nearby allies exert a lateral separation steering force.
/// Wider than the hard collision radius to proactively bend movement paths.
const ALLY_SEPARATION_RADIUS: f32 = 90.0;

/// Weight of the ally separation force relative to the seek force.
/// Higher values cause units to flow around each other more aggressively.
const ALLY_SEPARATION_WEIGHT: f32 = 0.7;

/// Computes a NOC-style separation steering force away from nearby allies
/// using the spatial hash grid for O(1) neighbour lookup instead of O(N) full scan.
fn compute_ally_separation(entity: Entity, pos: Vec2, grid: &UnitSpatialGrid) -> Vec2 {
    let mut force = Vec2::ZERO;
    let mut count = 0u32;

    let radius_sq = ALLY_SEPARATION_RADIUS * ALLY_SEPARATION_RADIUS;
    let min_dist_sq = 0.001 * 0.001;

    let my_cell = world_to_grid(pos);

    for (dx, dy) in NEIGHBOR_OFFSETS.iter() {
        let check_cell = (my_cell.0 + dx, my_cell.1 + dy);

        let Some(neighbors) = grid.cells.get(&check_cell) else {
            continue;
        };

        for neighbor in neighbors {
            if neighbor.entity == entity {
                continue;
            }
            let diff = pos - neighbor.pos;
            let dist_sq = diff.length_squared();

            if dist_sq < radius_sq && dist_sq > min_dist_sq {
                // diff / dist_sq == (diff / dist) / dist == normalize / dist
                // Avoids sqrt() entirely while preserving inverse-distance weighting.
                force += diff / dist_sq;
                count += 1;
            }
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
    grid: Res<UnitSpatialGrid>,
    mut q_units: Query<(
        Entity,
        &mut Transform,
        &GlobalTransform,
        &mut UnitAction,
        &Target,
        &CombatAttributes,
        &UnitCollider,
    )>,
    q_targets: Query<(&GlobalTransform, &UnitCollider)>,
) {
    let delta_secs = time.delta_secs();
    q_units.par_iter_mut().for_each(
        |(entity, mut transform, global_transform, mut state, target, stats, collider)| {
            let Some(target_entity) = target.0 else {
                // No target - go idle
                state.set_if_neq(UnitAction::Idle);
                return;
            };

            let Ok((target_transform, target_collider)) = q_targets.get(target_entity) else {
                // Target despawned - will get new target next frame
                return;
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
            let hysteresis_buffer = if *state == UnitAction::Attacking {
                5.0
            } else {
                0.0
            };
            let threshold_with_hysteresis = stop_threshold + hysteresis_buffer;
            let is_in_range =
                dist_to_target_sq <= (threshold_with_hysteresis * threshold_with_hysteresis);

            if is_in_range {
                // Close enough to attack
                state.set_if_neq(UnitAction::Attacking);
                // Face the target even while attacking
                face_target(&mut transform, my_pos, target_pos);
            } else {
                // Need to move closer
                state.set_if_neq(UnitAction::Moving);

                // --- NOC Steering: seek + ally separation ---
                // Desired velocity: full speed straight toward target
                let seek = (target_pos - my_pos).normalize_or_zero() * stats.speed;

                // Separation force: steer away from nearby allies (spatial grid lookup).
                let sep = compute_ally_separation(entity, my_pos, &grid) * stats.speed;

                // Blend: seek drives toward enemy, separation steers laterally around allies.
                // clamp_length_max ensures we never exceed max speed.
                let desired = seek + sep * ALLY_SEPARATION_WEIGHT;
                let movement = desired.clamp_length_max(stats.speed) * delta_secs;

                transform.translation.x += movement.x;
                transform.translation.y += movement.y;

                // Face the target (not movement direction) so units always look at the enemy
                face_target(&mut transform, my_pos, target_pos);
            }
        },
    );
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
const MAX_PUSH_PER_FRAME: f32 = 0.35;

/// Soft collision separation system using spatial hash grid.
/// Only checks the 9 neighbouring cells instead of all units → O(N) instead of O(N²).
fn separation_system(
    time: Res<Time>,
    grid: Res<UnitSpatialGrid>,
    mut q_units: Query<(
        Entity,
        &mut Transform,
        &GlobalTransform,
        &UnitCollider,
        &UnitAction,
    )>,
) {
    let delta = time.delta_secs();
    if delta <= 0.0 {
        return;
    }

    q_units.par_iter_mut().for_each(
        |(entity, mut transform, global_transform, collider, state)| {
            let my_pos = global_transform.translation().truncate();
            let my_cell = world_to_grid(my_pos);
            let mut total_push = Vec2::ZERO;

            let min_dist_sq = 0.001 * 0.001;

            // Only check the 9 neighbouring cells
            for (dx, dy) in NEIGHBOR_OFFSETS.iter() {
                let check_cell = (my_cell.0 + dx, my_cell.1 + dy);

                let Some(neighbors) = grid.cells.get(&check_cell) else {
                    continue;
                };

                for neighbor in neighbors {
                    if neighbor.entity == entity {
                        continue;
                    }

                    let combined_radius = collider.radius + neighbor.radius;
                    let diff = my_pos - neighbor.pos;
                    let dist_sq = diff.length_squared();
                    let combined_radius_sq = combined_radius * combined_radius;

                    if dist_sq < combined_radius_sq && dist_sq > min_dist_sq {
                        let dist = dist_sq.sqrt();
                        let overlap = combined_radius - dist;
                        let push_dir = diff / dist;

                        // State weighting: attacking units are harder to push
                        let my_weight = if *state == UnitAction::Attacking {
                            0.2
                        } else {
                            1.0
                        };

                        let avg_strength = (collider.push_strength + neighbor.push_strength) * 0.5;
                        total_push += push_dir * overlap * avg_strength * my_weight;
                    }
                }
            }

            if total_push != Vec2::ZERO {
                let push_len_sq = total_push.length_squared();
                let max_push_sq = MAX_PUSH_PER_FRAME * MAX_PUSH_PER_FRAME;

                let final_push = if push_len_sq > max_push_sq {
                    total_push.normalize() * MAX_PUSH_PER_FRAME
                } else {
                    total_push
                };

                transform.translation.x += final_push.x;
                transform.translation.y += final_push.y;
            }
        },
    );
}

// ============================================================================
// Velocity Tracking Systems
// ============================================================================

/// Automatically add Velocity and PreviousPosition components to units that don't have them.
fn init_velocity_tracking(
    q_units: Query<
        (Entity, &GlobalTransform),
        (
            Or<(With<PlayerFaction>, With<EnemyFaction>)>,
            Without<Velocity>,
        ),
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
        Or<(With<PlayerFaction>, With<EnemyFaction>)>,
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
