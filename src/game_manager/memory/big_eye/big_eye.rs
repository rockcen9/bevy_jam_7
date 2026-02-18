use super::super::{EnemyMemory, Memory, PlayerMemory};
use crate::prelude::*;
use bevy_rand::{global::GlobalRng, prelude::ChaCha8Rng};
use rock_materials::ChromaticAberrationV2Material;
use rand::Rng;

#[derive(Component)]
struct PendingBigEyeMesh;

#[derive(Component, Default, Prefab)]
#[require(
    SpriteLayer::Memory,
    RootStationMemory,
    DespawnOnExit::<GameState>(GameState::Battle)
)]
pub struct BigEye;

#[derive(Component, Default)]
#[require(MemoryBuff)]
pub struct BigEyeBuff;

/// Phase 1: watches squad attack count for 5 seconds after spawn.
#[derive(Component, Reflect)]
pub struct BigEyeObserving {
    pub squad: Entity,
    pub baseline: u32,
    pub timer: Timer,
}

/// Phase 2: damage bonus is active for 3 seconds.
#[derive(Component)]
pub struct BigEyeActive {
    pub squad: Entity,
    pub timer: Timer,
}

/// Placed on the squad entity. 1 attack = 1% outgoing damage bonus, capped at 100%.
#[derive(Component, Default, Reflect)]
pub struct BigEyeDamageBonus(pub f32);

pub(super) fn plugin(app: &mut bevy::app::App) {
    app.add_systems(
        Update,
        (
            spawn_big_eye_on_buff_timer,
            big_eye_observe_system,
            big_eye_active_system,
            setup_big_eye_mesh,
            update_big_eye_fill,
        )
            .run_if(in_state(GameState::Battle)),
    );
}

/// Every 10-15 seconds (random), spawn a BigEye at a random squad member location for each buffed squad
fn spawn_big_eye_on_buff_timer(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: Local<Option<Timer>>,
    q_buffed_squads: Query<(Entity, &RootStationSquad, &Faction, &SquadHitCount), With<BigEyeBuff>>,
    q_unit_transform: Query<&GlobalTransform>,
    mut rng: Single<&mut ChaCha8Rng, With<GlobalRng>>,
) {
    let timer = timer.get_or_insert_with(|| {
        let duration = rng.random_range(10.0..=15.0);
        Timer::from_seconds(duration, TimerMode::Once)
    });

    if !timer.tick(time.delta()).just_finished() {
        return;
    }

    // Reset timer with new random duration for next spawn
    let duration = rng.random_range(10.0..=15.0);
    *timer = Timer::from_seconds(duration, TimerMode::Once);

    for (squad_entity, root_station, faction, hit_count) in &q_buffed_squads {
        debug!(
            "[BigEye] Timer fired - squad {:?} has {} members, faction {:?}",
            squad_entity,
            root_station.len(),
            faction
        );

        if root_station.is_empty() {
            debug!("[BigEye] Squad {:?} has no members, skipping", squad_entity);
            continue;
        }

        let idx = rng.random_range(0..root_station.len());
        let unit_entity = root_station[idx];

        let Ok(unit_transform) = q_unit_transform.get(unit_entity) else {
            debug!(
                "[BigEye] Could not get GlobalTransform for unit {:?}, skipping",
                unit_entity
            );
            continue;
        };

        let position = unit_transform.translation();
        let baseline = hit_count.0;

        let big_eye_entity = match faction {
            Faction::Player => {
                let e = commands
                    .spawn((
                        BigEye,
                        PlayerMemory,
                        Memory::with_seconds(100.0),
                        Transform::from_translation(position),
                        RootStation::default(),
                    ))
                    .id();
                let model = commands
                    .spawn((Name::new("BigEyeModel"), Model, ChildOf(e), BelongTo(e)))
                    .id();
                commands.spawn((
                    Name::new("BigEyeMesh"),
                    PendingBigEyeMesh,
                    ChildOf(model),
                    BelongTo(e),
                ));
                info!(
                    "Spawned BigEye with PlayerMemory at {:?} (BigEyeBuff timer), baseline hits: {}",
                    position, baseline
                );
                e
            }
            Faction::Enemy => {
                let e = commands
                    .spawn((
                        BigEye,
                        EnemyMemory,
                        Memory::with_seconds(100.0),
                        Transform::from_translation(position),
                    ))
                    .id();
                info!(
                    "Spawned BigEye with EnemyMemory at {:?} (BigEyeBuff timer), baseline hits: {}",
                    position, baseline
                );
                e
            }
        };

        commands.entity(big_eye_entity).insert(BigEyeObserving {
            squad: squad_entity,
            baseline,
            timer: Timer::from_seconds(5.0, TimerMode::Once),
        });

        commands
            .entity(squad_entity)
            .insert(BelongToMemory(big_eye_entity));
    }
}

/// Updates the fill percentage on the BigEye mesh based on current attack count (1 hit = 1%).
fn update_big_eye_fill(
    q_observing: Query<(Entity, &BigEyeObserving)>,
    q_mesh: Query<(&BelongTo, &MeshMaterial2d<ChromaticAberrationV2Material>)>,
    q_hit_count: Query<&SquadHitCount>,
    mut materials: ResMut<Assets<ChromaticAberrationV2Material>>,
) {
    for (big_eye_entity, obs) in q_observing.iter() {
        let current_hits = q_hit_count
            .get(obs.squad)
            .map(|c| c.0)
            .unwrap_or(obs.baseline);
        let attacks = current_hits.saturating_sub(obs.baseline);
        let fill_percentage = (attacks as f32 * 0.01).min(1.0);

        // Find mesh that belongs to this BigEye and update its fill
        for (belong_to, mesh_material) in q_mesh.iter() {
            if belong_to.0 == big_eye_entity {
                if let Some(mat) = materials.get_mut(mesh_material.id()) {
                    mat.fill = fill_percentage;
                }
            }
        }
    }
}

/// Ticks observation timer. Fires early if bar reaches 100% (100 attacks), otherwise waits for timer.
fn big_eye_observe_system(
    mut commands: Commands,
    time: Res<Time>,
    mut q_observing: Query<(Entity, &mut BigEyeObserving)>,
    q_hit_count: Query<&SquadHitCount>,
) {
    for (entity, mut obs) in &mut q_observing {
        obs.timer.tick(time.delta());

        let attacks = q_hit_count
            .get(obs.squad)
            .map(|c| c.0.saturating_sub(obs.baseline))
            .unwrap_or(0);

        // Fire early if bar is full (100 attacks = 100% fill)
        let is_full = attacks >= 100;
        let timer_finished = obs.timer.just_finished();

        if !is_full && !timer_finished {
            continue;
        }

        let bonus = (attacks as f32 * 0.01).min(1.0);
        let reason = if is_full && !timer_finished {
            "bar full (early fire)"
        } else {
            "timer finished"
        };

        info!(
            "[BigEye] Observation done ({}) - {} attacks recorded, damage bonus: {:.0}%",
            reason,
            attacks,
            bonus * 100.0
        );

        commands.entity(obs.squad).insert(BigEyeDamageBonus(bonus));
        commands
            .entity(entity)
            .remove::<BigEyeObserving>()
            .insert(BigEyeActive {
                squad: obs.squad,
                timer: Timer::from_seconds(3.0, TimerMode::Once),
            });
    }
}

/// Waits for the BigEye texture to load, then inserts Mesh2d + ChromaticAberrationV2Material.
fn setup_big_eye_mesh(
    mut commands: Commands,
    q_pending: Query<Entity, With<PendingBigEyeMesh>>,
    images: Res<Assets<Image>>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ChromaticAberrationV2Material>>,
) {
    if q_pending.is_empty() {
        return;
    }
    let texture: Handle<Image> = asset_server.load("procreate/BigEye.png");
    let Some(image) = images.get(&texture) else {
        return;
    };
    let size = image.size_f32();
    for entity in q_pending.iter() {
        let mat = materials.add(ChromaticAberrationV2Material {
            texture: texture.clone(),
            fill: 0.0,
            fill_color: LinearRgba::new(1.0, 0.0, 0.0, 0.6), // red with 60% alpha
            ..default()
        });
        commands
            .entity(entity)
            .remove::<PendingBigEyeMesh>()
            .insert((
                Mesh2d(meshes.add(Rectangle::new(size.x, size.y))),
                MeshMaterial2d(mat),
                SpriteLayer::Memory,
            ));
    }
}

/// Ticks active timer. On finish, removes damage bonus and despawns BigEye.
fn big_eye_active_system(
    mut commands: Commands,
    time: Res<Time>,
    mut q_active: Query<(Entity, &mut BigEyeActive)>,
) {
    for (entity, mut active) in &mut q_active {
        if !active.timer.tick(time.delta()).just_finished() {
            continue;
        }

        commands.entity(active.squad).remove::<BigEyeDamageBonus>();
        info!(
            "[BigEye] Active phase ended, damage bonus removed from squad {:?}",
            active.squad
        );

        commands.entity(entity).insert(ShrinkDespawn);
    }
}
