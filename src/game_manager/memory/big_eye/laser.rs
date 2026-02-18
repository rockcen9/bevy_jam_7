use bevy_rand::{global::GlobalRng, prelude::ChaCha8Rng};
use rock_materials::LaserBeamMaterial;
use rand::Rng;

use crate::prelude::*;

use super::{BigEyeActive, EnemyMemory, PlayerMemory};

const BEAM_HEIGHT: f32 = 200.0;
const BEAM_DURATION: f32 = 3.0;
const BEAM_DAMAGE_PER_TICK: f32 = 5.0;
const BEAM_DAMAGE_INTERVAL: f32 = 0.5;

// How far the beam extends past the last opponent
const BEAM_OVERSHOOT: f32 = 800.0;

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (spawn_laser_on_big_eye_appear, tick_laser_and_fire).run_if(in_state(GameState::Battle)),
    );
}

#[derive(Component)]
struct BigEyeLaser {
    origin: Vec2,
    direction: Vec2,
    beam_length: f32,
    is_player_memory: bool,
    lifetime: Timer,
    damage_tick: Timer,
}

fn spawn_laser_on_big_eye_appear(
    mut commands: Commands,
    new_big_eyes: Query<(&Transform, Has<PlayerMemory>, Has<EnemyMemory>), Added<BigEyeActive>>,
    player_units: Query<(Entity, &GlobalTransform), With<PlayerUnit>>,
    enemy_units: Query<(Entity, &GlobalTransform), With<EnemyUnit>>,
    mut rng: Single<&mut ChaCha8Rng, With<GlobalRng>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut laser_materials: ResMut<Assets<LaserBeamMaterial>>,
) {
    for (big_eye_transform, is_player_memory, is_enemy_memory) in &new_big_eyes {
        let origin = big_eye_transform.translation.truncate();

        let opponents: Vec<(Entity, Vec2)> = if is_player_memory {
            enemy_units
                .iter()
                .map(|(e, t)| (e, t.translation().truncate()))
                .collect()
        } else if is_enemy_memory {
            player_units
                .iter()
                .map(|(e, t)| (e, t.translation().truncate()))
                .collect()
        } else {
            continue;
        };

        if opponents.is_empty() {
            continue;
        }

        // Pick a random opponent to aim at; the beam will damage everyone it passes through
        let idx = rng.random_range(0..opponents.len());
        let (_, aim_pos) = opponents[idx];

        let diff = aim_pos - origin;
        let distance = diff.length();
        if distance < 1.0 {
            continue;
        }

        let direction = diff / distance;
        let beam_length = distance + BEAM_OVERSHOOT;

        // Position the mesh so its left edge sits at the BigEye origin
        let midpoint = origin + direction * (beam_length * 0.5);
        let angle = direction.y.atan2(direction.x);

        let material = laser_materials.add(LaserBeamMaterial {
            resolution: Vec2::new(beam_length, BEAM_HEIGHT),
            alpha: 1.0,
            _padding: 0.0,
        });

        commands.trigger(CameraShakeEvent);
        commands.trigger(SFXEvent::space("beam", origin));
        commands.spawn((
            BigEyeLaser {
                origin,
                direction,
                beam_length,
                is_player_memory,
                lifetime: Timer::from_seconds(BEAM_DURATION, TimerMode::Once),
                damage_tick: {
                    let mut t = Timer::from_seconds(BEAM_DAMAGE_INTERVAL, TimerMode::Repeating);
                    t.set_elapsed(std::time::Duration::from_secs_f32(BEAM_DAMAGE_INTERVAL));
                    t
                },
            },
            Mesh2d(meshes.add(Rectangle::new(beam_length, BEAM_HEIGHT))),
            MeshMaterial2d(material),
            Transform::from_xyz(midpoint.x, midpoint.y, 0.0)
                .with_rotation(Quat::from_rotation_z(angle)),
            SpriteLayer::VFX,
            DespawnOnExit(GameState::Battle),
        ));
    }
}

fn tick_laser_and_fire(
    mut commands: Commands,
    time: Res<Time>,
    mut lasers: Query<(Entity, &mut BigEyeLaser)>,
    player_units: Query<(Entity, &GlobalTransform), With<PlayerUnit>>,
    enemy_units: Query<(Entity, &GlobalTransform), With<EnemyUnit>>,
) {
    for (laser_entity, mut laser) in &mut lasers {
        laser.lifetime.tick(time.delta());
        laser.damage_tick.tick(time.delta());

        if laser.damage_tick.just_finished() {
            let targets: Box<dyn Iterator<Item = (Entity, Vec2)>> = if laser.is_player_memory {
                Box::new(
                    enemy_units
                        .iter()
                        .map(|(e, t)| (e, t.translation().truncate())),
                )
            } else {
                Box::new(
                    player_units
                        .iter()
                        .map(|(e, t)| (e, t.translation().truncate())),
                )
            };

            for (target_entity, target_pos) in targets {
                let to_target = target_pos - laser.origin;
                let along = to_target.dot(laser.direction);
                if along < 0.0 || along > laser.beam_length {
                    continue;
                }
                let perp = (to_target - laser.direction * along).length();
                if perp < BEAM_HEIGHT * 0.5 {
                    commands.trigger(AttackEvent::new(
                        laser_entity,
                        target_entity,
                        BEAM_DAMAGE_PER_TICK,
                    ));
                }
            }
        }

        if laser.lifetime.just_finished() {
            commands.entity(laser_entity).despawn();
        }
    }
}
