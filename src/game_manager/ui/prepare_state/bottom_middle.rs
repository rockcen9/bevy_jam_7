use crate::game_manager::balance::UnitStatsCache;
use crate::game_manager::scene::InBoundary;
use crate::game_manager::shop::{PlayerGold, PlayerGoldNotEnoughMessage};
use crate::game_manager::{DEFAULT_SQUAD_SIZE, spawn_player_squad};
use crate::prelude::*;
use bevy_tweening::{
    lens::{UiTransformRotationLens, UiTransformScaleLens},
    *,
};
use std::time::Duration;

use super::root::{PrepareRootNode, PrepareUiSets};

#[derive(Component, Default)]
pub(crate) struct ShieldButtonMarker;

#[derive(Component, Default)]
pub(crate) struct ArcherButtonMarker;

#[derive(Component, Default)]
pub(crate) struct SpearButtonMarker;

/// Marker component for squads that are being dragged and should follow cursor
#[derive(Component)]
pub struct FollowingCursor;

pub(crate) fn plugin(app: &mut bevy::app::App) {
    app.add_systems(
        OnEnter(GameState::Preparing),
        spawn_bottom_middle_ui.in_set(PrepareUiSets::SpawnChildren),
    )
    .add_systems(
        Update,
        (
            handle_button_interaction,
            update_following_squads,
            handle_mouse_release,
            handle_button_hover,
        )
            .run_if(in_state(GameState::Preparing)),
    );
}

fn spawn_bottom_middle_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    root_query: Query<Entity, With<PrepareRootNode>>,
    palette: Res<ColorPalette>,
) {
    let Ok(root_entity) = root_query.single() else {
        warn!("PrepareRootNode not found for bottom middle UI");
        return;
    };

    // Load unit images
    let shield_img = asset_server.load("procreate/Shield.png");
    let archer_img = asset_server.load("procreate/Archer.png");
    let spear_img = asset_server.load("procreate/Spear.png");

    commands.entity(root_entity).with_children(|parent| {
        // Bottom-middle container
        parent
            .spawn((
                Node {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(20.0), // HIG: 20pt recommended margin
                    left: Val::Px(0.0),
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                Name::new("Bottom Middle UI"),
            ))
            .with_children(|parent| {
                // Button container
                parent
                    .spawn(Node {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(16.0), // HIG: 8pt grid (16pt between buttons)
                        padding: UiRect::all(Val::Px(16.0)), // HIG: 16pt standard padding
                        border: UiRect::all(Val::Px(1.0)),
                        border_radius: BorderRadius::all(Val::Px(20.0)), // HIG: 20pt for cards
                        ..default()
                    })
                    .insert(BackgroundColor(palette.blue_dark.with_alpha(0.5)))
                    .insert(BorderColor::all(palette.purple_lighter.with_alpha(0.40)))
                    .with_children(|parent| {
                        // Spear button
                        parent
                            .spawn((
                                Button,
                                Node {
                                    width: Val::Px(88.0),
                                    height: Val::Px(88.0),
                                    padding: UiRect::all(Val::Px(16.0)),
                                    margin: UiRect::vertical(Val::Px(4.0)),
                                    border_radius: BorderRadius::all(Val::Px(12.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BackgroundColor(Color::NONE),
                                UiTransform::default(),
                                SpearButtonMarker,
                            ))
                            .with_children(|parent| {
                                parent.spawn((
                                    ImageNode {
                                        image: spear_img.clone(),
                                        ..default()
                                    },
                                    Node {
                                        width: Val::Percent(55.0),
                                        height: Val::Auto,
                                        ..default()
                                    },
                                ));
                            });

                        // Shield button
                        parent
                            .spawn((
                                Button,
                                Node {
                                    width: Val::Px(88.0),
                                    height: Val::Px(88.0),
                                    padding: UiRect::all(Val::Px(16.0)),
                                    margin: UiRect::vertical(Val::Px(4.0)),
                                    border_radius: BorderRadius::all(Val::Px(12.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BackgroundColor(Color::NONE),
                                UiTransform::default(),
                                ShieldButtonMarker,
                            ))
                            .with_children(|parent| {
                                parent.spawn((
                                    ImageNode {
                                        image: shield_img.clone(),
                                        ..default()
                                    },
                                    Node {
                                        width: Val::Percent(70.0),
                                        height: Val::Auto,
                                        ..default()
                                    },
                                ));
                            });

                        // Archer button
                        parent
                            .spawn((
                                Button,
                                Node {
                                    width: Val::Px(88.0),
                                    height: Val::Px(88.0),
                                    padding: UiRect::all(Val::Px(16.0)),
                                    margin: UiRect::vertical(Val::Px(4.0)),
                                    border_radius: BorderRadius::all(Val::Px(12.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BackgroundColor(Color::NONE),
                                UiTransform::default(),
                                ArcherButtonMarker,
                            ))
                            .with_children(|parent| {
                                parent.spawn((
                                    ImageNode {
                                        image: archer_img.clone(),
                                        ..default()
                                    },
                                    Node {
                                        width: Val::Percent(80.0),
                                        height: Val::Auto,
                                        ..default()
                                    },
                                ));
                            });
                    });
            });
    });
}

fn handle_button_interaction(
    mut commands: Commands,
    shield_q: Query<(Entity, &Interaction), (Changed<Interaction>, With<ShieldButtonMarker>)>,
    archer_q: Query<(Entity, &Interaction), (Changed<Interaction>, With<ArcherButtonMarker>)>,
    spear_q: Query<(Entity, &Interaction), (Changed<Interaction>, With<SpearButtonMarker>)>,
    unit_stats: Res<UnitStatsCache>,
    mut player_gold: ResMut<PlayerGold>,
    window_q: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut not_enough_gold_msg: MessageWriter<PlayerGoldNotEnoughMessage>,
) {
    let Ok(window) = window_q.single() else {
        return;
    };
    let Ok((camera, camera_transform)) = camera_q.single() else {
        return;
    };

    for (entity, interaction) in shield_q.iter() {
        if *interaction == Interaction::Pressed {
            try_spawn_unit(
                &mut commands,
                UnitKind::Shield,
                entity,
                &unit_stats,
                &mut player_gold,
                window,
                camera,
                camera_transform,
                &mut not_enough_gold_msg,
            );
        }
    }

    for (entity, interaction) in archer_q.iter() {
        if *interaction == Interaction::Pressed {
            try_spawn_unit(
                &mut commands,
                UnitKind::Archer,
                entity,
                &unit_stats,
                &mut player_gold,
                window,
                camera,
                camera_transform,
                &mut not_enough_gold_msg,
            );
        }
    }

    for (entity, interaction) in spear_q.iter() {
        if *interaction == Interaction::Pressed {
            try_spawn_unit(
                &mut commands,
                UnitKind::Spear,
                entity,
                &unit_stats,
                &mut player_gold,
                window,
                camera,
                camera_transform,
                &mut not_enough_gold_msg,
            );
        }
    }
}

/// Try to spawn a unit if player can afford it
fn try_spawn_unit(
    commands: &mut Commands,
    unit_type: UnitKind,
    button_entity: Entity,
    unit_stats: &UnitStatsCache,
    player_gold: &mut PlayerGold,
    window: &Window,
    camera: &Camera,
    camera_transform: &GlobalTransform,
    not_enough_gold_msg: &mut MessageWriter<PlayerGoldNotEnoughMessage>,
) {
    // Get the unit cost from the stats cache
    let Some(unit_row) = unit_stats.stats.get(unit_type.as_ref()) else {
        warn!("No stats found for unit type: {:?}", unit_type);
        return;
    };

    // Check if player has enough gold
    if (player_gold.amount as i32) < unit_row.cost {
        warn!(
            "Not enough gold to spawn {:?}. Cost: {}, Available: {}",
            unit_type, unit_row.cost, player_gold.amount
        );
        commands.trigger(SFXEvent::ui("invalid"));
        spawn_click_wiggle_animation(commands, button_entity);
        not_enough_gold_msg.write(PlayerGoldNotEnoughMessage);
        return;
    }

    // Get cursor world position
    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };
    let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) else {
        return;
    };

    // Deduct the cost
    player_gold.amount = (player_gold.amount as i32 - unit_row.cost) as u32;

    info!(
        "Spawning {:?} squad at world position: {:?}. Cost: {}. Remaining gold: {}",
        unit_type, world_pos, unit_row.cost, player_gold.amount
    );

    let squad_id = spawn_player_squad(commands, unit_type.as_ref(), world_pos, DEFAULT_SQUAD_SIZE);
    commands.entity(squad_id).insert(FollowingCursor);
}

/// System to update squads that are following the cursor
fn update_following_squads(
    window_q: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut following_q: Query<&mut Transform, With<FollowingCursor>>,
) {
    let Ok(window) = window_q.single() else {
        return;
    };
    let Ok((camera, camera_transform)) = camera_q.single() else {
        return;
    };

    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };
    let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) else {
        return;
    };

    // Update all following squads to cursor position
    for mut transform in &mut following_q {
        transform.translation.x = world_pos.x;
        transform.translation.y = world_pos.y;
    }
}

/// System to stop following when mouse is released and trigger deploy effects
fn handle_mouse_release(
    mouse: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    in_boundary: Res<InBoundary>,
    following_q: Query<(Entity, &Transform, &Squad), With<FollowingCursor>>,
    unit_stats: Res<UnitStatsCache>,
    mut player_gold: ResMut<PlayerGold>,
) {
    if mouse.just_released(MouseButton::Left) {
        for (entity, transform, squad) in &following_q {
            if !in_boundary.0 {
                // Out of boundary: refund gold and despawn
                if let Some(unit_row) = unit_stats.stats.get(&squad.child_prefab_name) {
                    player_gold.amount = player_gold.amount.saturating_add(unit_row.cost as u32);
                    info!(
                        "Squad out of boundary. Refunded {} gold. Total: {}",
                        unit_row.cost, player_gold.amount
                    );
                }
                commands.entity(entity).insert(ShakeDespawn);
                commands.trigger(SFXEvent::ui("invalid"));
                continue;
            }

            // Get the final world position where squad is deployed
            let world_pos = Vec2::new(transform.translation.x, transform.translation.y);

            // Trigger deploy effects at final position
            commands.trigger(CameraShakeEvent);
            commands.trigger(VfxEvent::dust(world_pos));

            // Stop following and play deploy animation
            commands.entity(entity).remove::<FollowingCursor>().insert((
                RequiredAnimation::Put,
                RunWithNoModel,
                SquadOriginPosition(world_pos),
            ));
        }
    }
}

fn handle_button_hover(
    mut commands: Commands,
    palette: Res<ColorPalette>,
    mut button_q: Query<
        (Entity, &Interaction, &UiTransform, &mut BackgroundColor),
        (
            Changed<Interaction>,
            Or<(
                With<ShieldButtonMarker>,
                With<ArcherButtonMarker>,
                With<SpearButtonMarker>,
            )>,
        ),
    >,
) {
    for (entity, interaction, transform, mut bg) in button_q.iter_mut() {
        match *interaction {
            Interaction::Hovered => {
                bg.0 = palette.blue_darkest.with_alpha(0.5);
                let tween = Tween::new(
                    EaseFunction::QuadraticIn,
                    Duration::from_millis(200),
                    UiTransformScaleLens {
                        start: transform.scale,
                        end: Vec2::splat(1.1),
                    },
                );
                commands.spawn((
                    TweenAnim::new(tween),
                    AnimTarget::component::<UiTransform>(entity),
                ));
            }
            Interaction::None => {
                bg.0 = Color::NONE;
                let tween = Tween::new(
                    EaseFunction::QuadraticIn,
                    Duration::from_millis(200),
                    UiTransformScaleLens {
                        start: transform.scale,
                        end: Vec2::ONE,
                    },
                );
                commands.spawn((
                    TweenAnim::new(tween),
                    AnimTarget::component::<UiTransform>(entity),
                ));
            }
            _ => {}
        }
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
