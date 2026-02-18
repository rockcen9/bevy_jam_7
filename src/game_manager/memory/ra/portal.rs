use rock_materials::FirePortalMaterial;

use crate::prelude::*;

use crate::game_manager::memory::{EnemyMemory, PlayerMemory};

use super::ra::RA;

const PORTAL_SIZE: f32 = 800.0;
const PORTAL_RANGE: f32 = 400.0;
const PORTAL_DURATION: f32 = 3.0;
const PORTAL_DAMAGE: f32 = 2.0;
const PORTAL_DAMAGE_INTERVAL: f32 = 0.5;

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(Update, spawn_portal_on_ra_appear);
    app.add_systems(Update, tick_portal);
}

#[derive(Component)]
struct RaPortal {
    is_player_memory: bool,
    lifetime: Timer,
    damage_tick: Timer,
}

fn spawn_portal_on_ra_appear(
    mut commands: Commands,
    new_ra: Query<(&Transform, Has<PlayerMemory>, Has<EnemyMemory>), Added<RA>>,
    enemy_units: Query<&GlobalTransform, With<EnemyUnit>>,
    player_units: Query<&GlobalTransform, With<PlayerUnit>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut portal_materials: ResMut<Assets<FirePortalMaterial>>,
) {
    for (ra_transform, is_player_memory, is_enemy_memory) in &new_ra {
        let ra_pos = ra_transform.translation.truncate();

        let closest_pos = if is_player_memory {
            enemy_units
                .iter()
                .map(|t| t.translation().truncate())
                .min_by(|a, b| {
                    a.distance(ra_pos)
                        .partial_cmp(&b.distance(ra_pos))
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
        } else if is_enemy_memory {
            player_units
                .iter()
                .map(|t| t.translation().truncate())
                .min_by(|a, b| {
                    a.distance(ra_pos)
                        .partial_cmp(&b.distance(ra_pos))
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
        } else {
            continue;
        };

        let Some(pos) = closest_pos else {
            continue;
        };

        let material = portal_materials.add(FirePortalMaterial::default());
        let mut damage_tick = Timer::from_seconds(PORTAL_DAMAGE_INTERVAL, TimerMode::Repeating);
        damage_tick.set_elapsed(std::time::Duration::from_secs_f32(PORTAL_DAMAGE_INTERVAL));

        commands.spawn((
            Name::new("RaPortal"),
            RaPortal {
                is_player_memory,
                lifetime: Timer::from_seconds(PORTAL_DURATION, TimerMode::Once),
                damage_tick,
            },
            Mesh2d(meshes.add(Rectangle::new(PORTAL_SIZE, PORTAL_SIZE))),
            MeshMaterial2d(material),
            Transform::from_xyz(pos.x, pos.y, -100.),
            // SpriteLayer::PortalVFX,
        ));
    }
}

fn tick_portal(
    time: Res<Time>,
    mut commands: Commands,
    mut portals: Query<(Entity, &Transform, &mut RaPortal)>,
    enemy_units: Query<(Entity, &GlobalTransform), With<EnemyUnit>>,
    player_units: Query<(Entity, &GlobalTransform), With<PlayerUnit>>,
) {
    for (portal_entity, portal_transform, mut portal) in &mut portals {
        portal.lifetime.tick(time.delta());
        portal.damage_tick.tick(time.delta());

        if portal.damage_tick.just_finished() {
            let portal_pos = portal_transform.translation.truncate();

            let targets: Vec<Entity> = if portal.is_player_memory {
                enemy_units
                    .iter()
                    .filter(|(_, gt)| {
                        gt.translation().truncate().distance(portal_pos) <= PORTAL_RANGE
                    })
                    .map(|(e, _)| e)
                    .collect()
            } else {
                player_units
                    .iter()
                    .filter(|(_, gt)| {
                        gt.translation().truncate().distance(portal_pos) <= PORTAL_RANGE
                    })
                    .map(|(e, _)| e)
                    .collect()
            };

            for target in targets {
                commands.trigger(AttackEvent::new(portal_entity, target, PORTAL_DAMAGE));
            }
        }

        if portal.lifetime.just_finished() {
            commands.entity(portal_entity).despawn();
        }
    }
}
