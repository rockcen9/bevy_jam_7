use crate::game_manager::audio::SFXEvent;
use crate::game_manager::balance::MemoryStatsCache;
use crate::game_manager::camera::MainCamera;
use crate::game_manager::shop::{PlayerGold, PlayerGoldNotEnoughMessage};
use crate::game_manager::ui::prepare_state::bottom_left::GoldenHeartButtonMarker;
use crate::prelude::*;
use bevy_tweening::{lens::UiTransformRotationLens, *};
use rock_materials::ChromaticAberrationMaterial;
use std::time::Duration;

#[derive(Component)]
pub(super) struct GoldenHeartGhost;

/// Waiting for texture to load before mesh/material can be applied
#[derive(Component)]
struct PendingGhostMaterial {
    texture_path: &'static str,
}

pub(super) fn plugin(app: &mut bevy::app::App) {
    app.add_systems(
        Update,
        (
            handle_golden_heart_button_press,
            setup_ghost_material,
            update_golden_heart_ghost_position,
            update_ghost_alpha_on_hover,
            despawn_golden_heart_ghost_on_release,
            spawn_golden_heart_sprite_on_buff,
        )
            .run_if(in_state(GameState::Preparing)),
    )
    .add_systems(OnEnter(GameState::Battle), despawn_golden_heart_indicators);
}

fn handle_golden_heart_button_press(
    mut commands: Commands,
    q_button: Query<(Entity, &Interaction), (Changed<Interaction>, With<GoldenHeartButtonMarker>)>,
    q_ghost: Query<Entity, With<GoldenHeartGhost>>,
    memory_stats: Res<MemoryStatsCache>,
    mut player_gold: ResMut<PlayerGold>,
    mut not_enough_gold_msg: MessageWriter<PlayerGoldNotEnoughMessage>,
) {
    for (button_entity, interaction) in q_button.iter() {
        if *interaction == Interaction::Pressed && q_ghost.is_empty() {
            // Get the GoldenHeart cost from stats cache
            let Some(memory_row) = memory_stats.stats.get("GoldenHeart") else {
                warn!("[GoldenHeart] No stats found for GoldenHeart");
                return;
            };

            // Check if player has enough gold
            if (player_gold.amount as i32) < memory_row.price {
                warn!(
                    "[GoldenHeart] Not enough gold. Cost: {}, Available: {}",
                    memory_row.price, player_gold.amount
                );
                commands.trigger(SFXEvent::ui("invalid"));
                spawn_click_wiggle_animation(&mut commands, button_entity);
                not_enough_gold_msg.write(PlayerGoldNotEnoughMessage);
                return;
            }

            // Deduct the cost
            player_gold.amount = (player_gold.amount as i32 - memory_row.price) as u32;

            info!(
                "[GoldenHeart] Button pressed — spawning drag ghost. Cost: {}. Remaining gold: {}",
                memory_row.price, player_gold.amount
            );
            commands.spawn((
                GoldenHeartGhost,
                PendingGhostMaterial {
                    texture_path: "procreate/GoldenHeart.png",
                },
                Transform::from_xyz(0.0, 0.0, 100.0),
            ));
        }
    }
}

fn setup_ghost_material(
    mut commands: Commands,
    q_ghost: Query<(Entity, &PendingGhostMaterial), With<GoldenHeartGhost>>,
    images: Res<Assets<Image>>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut chromatic_materials: ResMut<Assets<ChromaticAberrationMaterial>>,
) {
    for (entity, pending) in q_ghost.iter() {
        let texture: Handle<Image> = asset_server.load(pending.texture_path);
        let Some(image) = images.get(&texture) else {
            continue; // not loaded yet, retry next frame
        };

        let size = image.size_f32();
        let material = chromatic_materials.add(ChromaticAberrationMaterial {
            texture,
            amount: 0.05,
            alpha: 0.3,
        });

        commands
            .entity(entity)
            .remove::<PendingGhostMaterial>()
            .insert((
                Mesh2d(meshes.add(Rectangle::new(size.x, size.y))),
                MeshMaterial2d(material),
                SpriteLayer::Memory,
            ));
    }
}

fn update_golden_heart_ghost_position(
    window_q: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut q_ghost: Query<&mut Transform, With<GoldenHeartGhost>>,
) {
    if q_ghost.is_empty() {
        return;
    }
    let Ok(window) = window_q.single() else {
        return;
    };
    let Ok((camera, camera_transform)) = q_camera.single() else {
        return;
    };
    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };
    let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) else {
        return;
    };
    for mut transform in &mut q_ghost {
        transform.translation.x = world_pos.x;
        transform.translation.y = world_pos.y;
    }
}

fn update_ghost_alpha_on_hover(
    window_q: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    q_ghost: Query<&MeshMaterial2d<ChromaticAberrationMaterial>, With<GoldenHeartGhost>>,
    q_units: Query<&GlobalTransform, With<PlayerUnit>>,
    mut chromatic_materials: ResMut<Assets<ChromaticAberrationMaterial>>,
) {
    let Ok(material_handle) = q_ghost.single() else {
        return;
    };

    let Ok(window) = window_q.single() else {
        return;
    };
    let Ok((camera, camera_transform)) = q_camera.single() else {
        return;
    };
    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };
    let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) else {
        return;
    };

    // Check if hovering over any player unit
    let mut is_hovering = false;
    for transform in &q_units {
        let unit_pos = transform.translation().truncate();
        if unit_pos.distance(world_pos) <= 32.0 {
            is_hovering = true;
            break;
        }
    }

    // Update material alpha
    if let Some(material) = chromatic_materials.get_mut(material_handle.0.id()) {
        material.alpha = if is_hovering { 1.0 } else { 0.3 };
    }
}

fn despawn_golden_heart_ghost_on_release(
    mouse: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    q_ghost: Query<Entity, With<GoldenHeartGhost>>,
    window_q: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    q_units: Query<(&GlobalTransform, &BelongToSquad), With<PlayerUnit>>,
    q_squads: Query<&super::golden_heart::GoldenHeartBuff>,
    memory_stats: Res<MemoryStatsCache>,
    mut player_gold: ResMut<PlayerGold>,
) {
    if !mouse.just_released(MouseButton::Left) || q_ghost.is_empty() {
        return;
    }

    let Ok(ghost_entity) = q_ghost.single() else {
        return;
    };
    info!("[GoldenHeart] Mouse released — despawning drag ghost");

    let Ok(window) = window_q.single() else {
        return;
    };
    let Ok((camera, camera_transform)) = q_camera.single() else {
        return;
    };
    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };
    let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) else {
        return;
    };

    let mut buff_applied = false;
    for (transform, belong_to_squad) in &q_units {
        let unit_pos = transform.translation().truncate();
        if unit_pos.distance(world_pos) <= 32.0 {
            // Check if squad already has GoldenHeartBuff
            if q_squads.get(belong_to_squad.0).is_ok() {
                info!(
                    "[GoldenHeart] Squad {:?} already has GoldenHeartBuff — treating as failed",
                    belong_to_squad.0
                );
                commands.entity(belong_to_squad.0).insert(ShakeLeftRight);
                break; // Treat as failed, will trigger refund below
            }

            info!(
                "[GoldenHeart] Applied GoldenHeartBuff to squad {:?}",
                belong_to_squad.0
            );
            commands
                .entity(belong_to_squad.0)
                .insert(super::golden_heart::GoldenHeartBuff);
            buff_applied = true;

            commands.entity(ghost_entity).insert(ShrinkDespawn);

            commands.trigger(SFXEvent::ui("imbuse"));
            break;
        }
    }

    if !buff_applied {
        info!("[GoldenHeart] No unit found — refunding cost and playing invalid SFX");

        // Refund the cost
        if let Some(memory_row) = memory_stats.stats.get("GoldenHeart") {
            player_gold.amount = (player_gold.amount as i32 + memory_row.price) as u32;
            info!(
                "[GoldenHeart] Refunded {}. Current gold: {}",
                memory_row.price, player_gold.amount
            );
        }

        commands.entity(ghost_entity).insert(ShakeDespawn);
        commands.trigger(SFXEvent::ui("invalid"));
    }
}

/// Component to mark the GoldenHeart sprite indicator on squad entity
#[derive(Component)]
struct GoldenHeartSquadIndicator;

/// Spawns a GoldenHeart sprite indicator when a squad receives GoldenHeartBuff
fn spawn_golden_heart_sprite_on_buff(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    q_buffed_squads: Query<Entity, Added<super::golden_heart::GoldenHeartBuff>>,
) {
    for squad_entity in &q_buffed_squads {
        info!(
            "[GoldenHeart] Squad {:?} received GoldenHeartBuff — spawning sprite indicator",
            squad_entity
        );

        let texture = asset_server.load("procreate/GoldenHeart.png");

        commands.entity(squad_entity).with_children(|parent| {
            parent.spawn((
                Name::new("GoldenHeartIndicator"),
                GoldenHeartSquadIndicator,
                Sprite {
                    image: texture,
                    color: Color::srgba(1.0, 1.0, 1.0, 0.8),
                    ..default()
                },
                Transform::from_xyz(0.0, 40.0, 1.0).with_scale(Vec3::splat(0.5)), // Scale to 50% size
                SpriteLayer::Memory,
            ));
        });
    }
}

/// Despawns all GoldenHeart sprite indicators when entering Battle state
fn despawn_golden_heart_indicators(
    mut commands: Commands,
    q_indicators: Query<Entity, With<GoldenHeartSquadIndicator>>,
) {
    for entity in &q_indicators {
        info!("[GoldenHeart] Despawning sprite indicator {:?}", entity);
        commands.entity(entity).despawn();
    }
}

fn spawn_click_wiggle_animation(commands: &mut Commands, entity: Entity) {
    // Create a wiggle animation: 0 -> -30deg -> +30deg -> 0
    // Total duration: 200ms
    let degrees_30 = 30_f32.to_radians();

    let tween1 = Tween::new(
        EaseFunction::QuadraticOut,
        Duration::from_millis(50),
        UiTransformRotationLens {
            start: Rot2::radians(0.0),
            end: Rot2::radians(-degrees_30),
        },
    );

    let tween2 = Tween::new(
        EaseFunction::QuadraticInOut,
        Duration::from_millis(100),
        UiTransformRotationLens {
            start: Rot2::radians(-degrees_30),
            end: Rot2::radians(degrees_30),
        },
    );

    let tween3 = Tween::new(
        EaseFunction::QuadraticIn,
        Duration::from_millis(50),
        UiTransformRotationLens {
            start: Rot2::radians(degrees_30),
            end: Rot2::radians(0.0),
        },
    );

    // Chain the tweens into a sequence
    let sequence = tween1.then(tween2).then(tween3);

    // Spawn separate animation entity to avoid being overwritten by scale animation
    commands.spawn((
        TweenAnim::new(sequence),
        AnimTarget::component::<UiTransform>(entity),
    ));
}
