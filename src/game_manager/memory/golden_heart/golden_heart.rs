use crate::prelude::*;

use crate::game_manager::memory::{EnemyMemory, PlayerMemory};
use bevy_rand::{global::GlobalRng, prelude::ChaCha8Rng};
use rock_materials::ChromaticAberrationV2Material;
use rand::Rng;

#[derive(Component)]
struct PendingGoldenHeartMesh;

#[derive(Component, Default, Prefab)]
#[require(
    SpriteLayer::Memory,
    RootStationMemory,
    DespawnOnExit::<GameState>(GameState::Battle)
)]
pub struct GoldenHeart;

#[derive(Component, Default)]
#[require(MemoryBuff)]
pub struct GoldenHeartBuff;

/// Phase 1: watches squad hit count for 5 seconds after spawn.
#[derive(Component, Reflect)]
pub struct GoldenHeartObserving {
    pub squad: Entity,
    pub baseline: u32,
    pub timer: Timer,
}

/// Phase 2: wave is about to trigger.
#[derive(Component)]
pub struct GoldenHeartActive {
    pub timer: Timer,
}

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            spawn_golden_heart_on_buff_timer,
            golden_heart_observe_system,
            golden_heart_active_system,
            setup_golden_heart_mesh,
            update_golden_heart_fill,
        )
            .run_if(in_state(GameState::Battle)),
    );
}

/// Every 10-15 seconds (random), spawn a GoldenHeart at a random squad member location for each buffed squad
fn spawn_golden_heart_on_buff_timer(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: Local<Option<Timer>>,
    q_buffed_squads: Query<
        (Entity, &RootStationSquad, &Faction, &SquadTakeHitCount),
        With<GoldenHeartBuff>,
    >,
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

    for (squad_entity, root_station, faction, take_hit_count) in &q_buffed_squads {
        debug!(
            "[GoldenHeart] Timer fired - squad {:?} has {} members, faction {:?}",
            squad_entity,
            root_station.len(),
            faction
        );

        if root_station.is_empty() {
            debug!(
                "[GoldenHeart] Squad {:?} has no members, skipping",
                squad_entity
            );
            continue;
        }

        let idx = rng.random_range(0..root_station.len());
        let unit_entity = root_station[idx];

        let Ok(unit_transform) = q_unit_transform.get(unit_entity) else {
            debug!(
                "[GoldenHeart] Could not get GlobalTransform for unit {:?}, skipping",
                unit_entity
            );
            continue;
        };

        let position = unit_transform.translation();
        let baseline = take_hit_count.0;

        let golden_heart_entity = match faction {
            Faction::Player => {
                let e = commands
                    .spawn((
                        GoldenHeart,
                        PlayerMemory,
                        Memory::with_seconds(100.0),
                        Transform::from_translation(position),
                        RootStation::default(),
                    ))
                    .id();
                let model = commands
                    .spawn((
                        Name::new("GoldenHeartModel"),
                        Model,
                        ChildOf(e),
                        BelongTo(e),
                    ))
                    .id();
                commands.spawn((
                    Name::new("GoldenHeartMesh"),
                    PendingGoldenHeartMesh,
                    ChildOf(model),
                    BelongTo(e),
                ));
                info!(
                    "Spawned GoldenHeart with PlayerMemory at {:?} (GoldenHeartBuff timer), baseline hits: {}",
                    position, baseline
                );
                e
            }
            Faction::Enemy => {
                let e = commands
                    .spawn((
                        GoldenHeart,
                        EnemyMemory,
                        Memory::with_seconds(100.0),
                        Transform::from_translation(position),
                        RootStation::default(),
                    ))
                    .id();
                let model = commands
                    .spawn((
                        Name::new("GoldenHeartModel"),
                        Model,
                        ChildOf(e),
                        BelongTo(e),
                    ))
                    .id();
                commands.spawn((
                    Name::new("GoldenHeartMesh"),
                    PendingGoldenHeartMesh,
                    ChildOf(model),
                    BelongTo(e),
                ));
                info!(
                    "Spawned GoldenHeart with EnemyMemory at {:?} (GoldenHeartBuff timer), baseline hits: {}",
                    position, baseline
                );
                e
            }
        };

        commands
            .entity(golden_heart_entity)
            .insert(GoldenHeartObserving {
                squad: squad_entity,
                baseline,
                timer: Timer::from_seconds(5.0, TimerMode::Once),
            });

        commands
            .entity(squad_entity)
            .insert(BelongToMemory(golden_heart_entity));
    }
}

/// Waits for the GoldenHeart texture to load, then inserts Mesh2d + ChromaticAberrationV2Material.
fn setup_golden_heart_mesh(
    mut commands: Commands,
    q_pending: Query<Entity, With<PendingGoldenHeartMesh>>,
    images: Res<Assets<Image>>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ChromaticAberrationV2Material>>,
) {
    if q_pending.is_empty() {
        return;
    }
    let texture: Handle<Image> = asset_server.load("procreate/GoldenHeart.png");
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
            .remove::<PendingGoldenHeartMesh>()
            .insert((
                Mesh2d(meshes.add(Rectangle::new(size.x, size.y))),
                MeshMaterial2d(mat),
                SpriteLayer::Memory,
            ));
    }
}

/// Updates the fill percentage on the GoldenHeart mesh based on current hit count (1 hit = 1%).
fn update_golden_heart_fill(
    q_observing: Query<(Entity, &GoldenHeartObserving)>,
    q_mesh: Query<(&BelongTo, &MeshMaterial2d<ChromaticAberrationV2Material>)>,
    q_hit_count: Query<&SquadTakeHitCount>,
    mut materials: ResMut<Assets<ChromaticAberrationV2Material>>,
) {
    for (golden_heart_entity, obs) in q_observing.iter() {
        let current_hits = q_hit_count
            .get(obs.squad)
            .map(|c| c.0)
            .unwrap_or(obs.baseline);
        let hits = current_hits.saturating_sub(obs.baseline);
        let fill_percentage = (hits as f32 * 0.01).min(1.0);

        // Find mesh that belongs to this GoldenHeart and update its fill
        for (belong_to, mesh_material) in q_mesh.iter() {
            if belong_to.0 == golden_heart_entity {
                if let Some(mat) = materials.get_mut(mesh_material.id()) {
                    mat.fill = fill_percentage;
                }
            }
        }
    }
}

/// Ticks observation timer. Fires early if bar is full (100 hits taken), otherwise waits for timer.
fn golden_heart_observe_system(
    mut commands: Commands,
    time: Res<Time>,
    mut q_observing: Query<(Entity, &mut GoldenHeartObserving)>,
    q_hit_count: Query<&SquadTakeHitCount>,
) {
    for (entity, mut obs) in &mut q_observing {
        obs.timer.tick(time.delta());

        let hits = q_hit_count
            .get(obs.squad)
            .map(|c| c.0.saturating_sub(obs.baseline))
            .unwrap_or(0);

        // Fire early if bar is full (100 hits = 100% fill)
        let is_full = hits >= 100;
        let timer_finished = obs.timer.just_finished();

        if !is_full && !timer_finished {
            continue;
        }

        let fill = (hits as f32 * 0.01).min(1.0);
        let reason = if is_full && !timer_finished {
            "bar full (early fire)"
        } else {
            "timer finished"
        };

        info!(
            "[GoldenHeart] Observation done ({}) - {} hits taken, fill: {:.0}%",
            reason,
            hits,
            fill * 100.0
        );

        commands
            .entity(entity)
            .remove::<GoldenHeartObserving>()
            .insert(GoldenHeartActive {
                timer: Timer::from_seconds(1.0, TimerMode::Once),
            });
    }
}

/// Ticks active timer. On finish, despawns GoldenHeart.
fn golden_heart_active_system(
    mut commands: Commands,
    time: Res<Time>,
    mut q_active: Query<(Entity, &mut GoldenHeartActive)>,
) {
    for (entity, mut active) in &mut q_active {
        if !active.timer.tick(time.delta()).just_finished() {
            continue;
        }

        info!("[GoldenHeart] Active phase ended, despawning");
        commands.entity(entity).insert(ShrinkDespawn);
    }
}
