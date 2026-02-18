use crate::prelude::*;
use bevy_tweening::{lens::UiTransformScaleLens, *};
use std::time::Duration;

use super::root::{PrepareRootNode, PrepareUiSets};
use crate::game_manager::battle::GameProgress;
use crate::game_manager::shop::{PlayerGold, PlayerGoldNotEnoughMessage};

#[derive(Component, Default)]
pub struct BigEyeButtonMarker;

#[derive(Component, Default)]
pub struct RaButtonMarker;

#[derive(Component, Default)]
pub struct BigHandButtonMarker;

#[derive(Component, Default)]
pub struct GoldenHeartButtonMarker;

pub(crate) fn plugin(app: &mut bevy::app::App) {
    app.add_systems(
        OnEnter(GameState::Preparing),
        spawn_bottom_left_ui.in_set(PrepareUiSets::SpawnChildren),
    )
    .add_systems(
        Update,
        (handle_button_interaction, handle_button_hover).run_if(in_state(GameState::Preparing)),
    );
}

fn spawn_bottom_left_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    root_query: Query<Entity, With<PrepareRootNode>>,
    game_progress: Res<GameProgress>,
    palette: Res<ColorPalette>,
) {
    // Only show memory panel from round 2 onwards
    if game_progress.current_round < 2 && std::env::var("ENABLE_MEMORY").is_err() {
        return;
    }

    let Ok(root_entity) = root_query.single() else {
        warn!("PrepareRootNode not found for bottom left UI");
        return;
    };

    let bigeye_img = asset_server.load("procreate/BigEyeIcon.png");
    let goldenheart_img = asset_server.load("procreate/GoldenHeartIcon.png");

    commands.entity(root_entity).with_children(|parent| {
        parent
            .spawn((
                Node {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(20.0),
                    left: Val::Px(20.0),
                    ..default()
                },
                Name::new("Bottom Left UI"),
            ))
            .with_children(|parent| {
                parent
                    .spawn(Node {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(16.0),
                        padding: UiRect::all(Val::Px(16.0)),
                        border: UiRect::all(Val::Px(1.0)),
                        border_radius: BorderRadius::all(Val::Px(20.0)),
                        ..default()
                    })
                    .insert(BackgroundColor(palette.blue_dark.with_alpha(0.5)))
                    .insert(BorderColor::all(palette.purple_lighter.with_alpha(0.40)))
                    .with_children(|parent| {
                        // BigEye button
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
                                BigEyeButtonMarker,
                            ))
                            .with_children(|parent| {
                                parent.spawn((
                                    ImageNode {
                                        image: bigeye_img.clone(),
                                        ..default()
                                    },
                                    Node {
                                        width: Val::Percent(120.0),
                                        height: Val::Auto,
                                        ..default()
                                    },
                                ));
                            });

                        // GoldenHeart button
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
                                GoldenHeartButtonMarker,
                            ))
                            .with_children(|parent| {
                                parent.spawn((
                                    ImageNode {
                                        image: goldenheart_img.clone(),
                                        ..default()
                                    },
                                    Node {
                                        width: Val::Percent(90.0),
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
    _commands: Commands,
    _goldenheart_q: Query<
        (Entity, &Interaction),
        (Changed<Interaction>, With<GoldenHeartButtonMarker>),
    >,
    _player_gold: Res<PlayerGold>,
    _not_enough_gold_msg: MessageWriter<PlayerGoldNotEnoughMessage>,
) {
    // GoldenHeart button interaction is now handled in golden_heart/ghost.rs
}

fn handle_button_hover(
    mut commands: Commands,
    palette: Res<ColorPalette>,
    mut button_q: Query<
        (Entity, &Interaction, &UiTransform, &mut BackgroundColor),
        (
            Changed<Interaction>,
            Or<(With<BigEyeButtonMarker>, With<GoldenHeartButtonMarker>)>,
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

// fn spawn_click_wiggle_animation(commands: &mut Commands, entity: Entity) {
//     // Create a wiggle animation: 0 -> -30deg -> +30deg -> 0
//     // Total duration: 200ms
//     let degrees_30 = 30_f32.to_radians();

//     let tween1 = Tween::new(
//         EaseFunction::QuadraticOut,
//         Duration::from_millis(50),
//         UiTransformRotationLens {
//             start: Rot2::radians(0.0),
//             end: Rot2::radians(-degrees_30),
//         },
//     );

//     let tween2 = Tween::new(
//         EaseFunction::QuadraticInOut,
//         Duration::from_millis(100),
//         UiTransformRotationLens {
//             start: Rot2::radians(-degrees_30),
//             end: Rot2::radians(degrees_30),
//         },
//     );

//     let tween3 = Tween::new(
//         EaseFunction::QuadraticIn,
//         Duration::from_millis(50),
//         UiTransformRotationLens {
//             start: Rot2::radians(degrees_30),
//             end: Rot2::radians(0.0),
//         },
//     );

//     // Chain the tweens into a sequence
//     let sequence = tween1.then(tween2).then(tween3);

//     // Spawn separate animation entity to avoid being overwritten by scale animation
//     commands.spawn((
//         TweenAnim::new(sequence),
//         AnimTarget::component::<UiTransform>(entity),
//     ));
// }
