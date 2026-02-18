use std::collections::HashSet;

use crate::prelude::*;
use rock_materials::WaveDistortionMaterial;

use super::golden_heart::GoldenHeartActive;

const MESH_SIZE: f32 = 2000.0;
const WAVE_SPEED: f32 = 0.15;
const WAVE_THICKNESS_UV: f32 = 0.1;
const WAVE_DAMAGE: f32 = 10.0;

#[derive(Component)]
struct PendingWaveDistortion {
    position: Vec3,
    delay: Timer,
    /// true = spawned by a player-side heart, so wave targets enemies
    targets_enemies: bool,
}

#[derive(Component)]
struct WaveDistortionVfx {
    lifetime: Timer,
    start_time: f32,
    wave_position: Vec2,
    targets_enemies: bool,
    already_hit: HashSet<Entity>,
}

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            detect_golden_heart,
            tick_pending_wave,
            wave_damage,
            tick_wave_lifetime,
        ),
    );
}

fn detect_golden_heart(
    mut commands: Commands,
    q_active: Query<(&Transform, Has<PlayerMemory>), Added<GoldenHeartActive>>,
) {
    for (transform, has_player_memory) in &q_active {
        let mut pos = transform.translation;
        pos.z = 10.0;
        commands.spawn(PendingWaveDistortion {
            position: pos,
            delay: Timer::from_seconds(0.5, TimerMode::Once),
            targets_enemies: has_player_memory,
        });
        info!(
            "[GoldenHeart] Wave triggered! Position: {:?}, targets_enemies: {}",
            pos, has_player_memory
        );
        commands.trigger(SFXEvent::space("wave", transform.translation.truncate()));
    }
}

fn tick_pending_wave(
    mut commands: Commands,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<WaveDistortionMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut q_pending: Query<(Entity, &mut PendingWaveDistortion)>,
) {
    for (entity, mut pending) in &mut q_pending {
        pending.delay.tick(time.delta());
        if !pending.delay.just_finished() {
            continue;
        }

        let translucent = images.add(Image::new_fill(
            bevy::render::render_resource::Extent3d {
                width: 1,
                height: 1,
                depth_or_array_layers: 1,
            },
            bevy::render::render_resource::TextureDimension::D2,
            &[255, 255, 255, 0],
            bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb,
            bevy::asset::RenderAssetUsages::RENDER_WORLD,
        ));

        let start_time = time.elapsed_secs();
        let material = materials.add(WaveDistortionMaterial {
            texture: translucent,
            wave_center: Vec2::new(0.5, 0.5),
            wave_params: Vec3::new(10.0, 0.8, WAVE_THICKNESS_UV),
            alpha: 1.0,
            start_time,
        });

        commands.spawn((
            Name::new("GoldenHeart WaveDistortion VFX"),
            WaveDistortionVfx {
                lifetime: Timer::from_seconds(5.0, TimerMode::Once),
                start_time,
                wave_position: pending.position.xy(),
                targets_enemies: pending.targets_enemies,
                already_hit: HashSet::new(),
            },
            Mesh2d(meshes.add(Rectangle::new(MESH_SIZE, MESH_SIZE))),
            MeshMaterial2d(material),
            Transform::from_translation(pending.position),
            SpriteLayer::VFX,
        ));

        commands.entity(entity).despawn();
    }
}

fn wave_damage(
    mut commands: Commands,
    time: Res<Time>,
    mut q_waves: Query<(Entity, &mut WaveDistortionVfx)>,
    q_units: Query<(Entity, &GlobalTransform), With<Unit>>,
    q_enemy: Query<(), With<EnemyUnit>>,
    q_player: Query<(), With<PlayerUnit>>,
) {
    let elapsed = time.elapsed_secs();

    for (wave_entity, mut vfx) in &mut q_waves {
        let local_time = (elapsed - vfx.start_time) * WAVE_SPEED;

        // Wave is fully invisible after this UV distance (matches shader smoothstep fade)
        const FADE_END_UV: f32 = 0.45;
        if local_time >= FADE_END_UV {
            continue;
        }

        // UV distance scales by full MESH_SIZE (UV 0→1 = 2000 world px)
        let ring_radius = local_time * MESH_SIZE;
        // Use a tight band so damage only fires at the visible ring center
        let half_band = 16.0;

        for (unit_entity, unit_gtransform) in &q_units {
            if vfx.already_hit.contains(&unit_entity) {
                continue;
            }

            let is_opponent = if vfx.targets_enemies {
                q_enemy.contains(unit_entity)
            } else {
                q_player.contains(unit_entity)
            };

            if !is_opponent {
                continue;
            }

            let dist = vfx
                .wave_position
                .distance(unit_gtransform.translation().xy());
            if dist >= ring_radius - half_band && dist <= ring_radius + half_band {
                commands.trigger(AttackEvent::new(wave_entity, unit_entity, WAVE_DAMAGE));
                vfx.already_hit.insert(unit_entity);
            }
        }
    }
}

fn tick_wave_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut q_vfx: Query<(Entity, &mut WaveDistortionVfx)>,
) {
    for (entity, mut vfx) in &mut q_vfx {
        vfx.lifetime.tick(time.delta());
        if vfx.lifetime.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}
