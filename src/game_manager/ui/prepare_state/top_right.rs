use crate::prelude::*;
use crate::game_manager::shop::PlayerGoldNotEnoughMessage;
use bevy_tweening::{lens::UiTransformTranslationPxLens, *};
use std::time::Duration;

use super::root::{PrepareRootNode, PrepareUiSets};

#[derive(Component)]
pub struct PlayerGoldTextMarker;

#[derive(Component)]
pub struct PlayerGoldPanelMarker;

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(GameState::Preparing),
        spawn_top_right_ui.in_set(PrepareUiSets::SpawnChildren),
    )
    .add_systems(
        Update,
        (
            update_gold_display,
            handle_gold_not_enough,
        ).run_if(in_state(GameState::Preparing)),
    );
}

/// Spawn the top right UI (gold display panel)
fn spawn_top_right_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    palette: Res<ColorPalette>,
    root_query: Query<Entity, With<PrepareRootNode>>,
) {
    let Ok(root_entity) = root_query.single() else {
        warn!("PrepareRootNode not found for top right UI");
        return;
    };

    let font = asset_server.load("fonts/Quicksand-Regular.ttf");

    commands.entity(root_entity).with_children(|parent| {
        // Top-right container
        parent
            .spawn(Node {
                position_type: PositionType::Absolute,
                top: Val::Px(20.0), // HIG: 20pt recommended margin
                right: Val::Px(20.0), // HIG: 20pt recommended margin
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(16.0), // HIG: 16pt spacing
                ..default()
            })
            .with_children(|parent| {
                // Mental display panel
                parent
                    .spawn((
                        Node {
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            column_gap: Val::Px(16.0), // HIG: 16pt spacing
                            padding: UiRect {
                                left: Val::Px(20.0),
                                right: Val::Px(20.0),
                                top: Val::Px(16.0), // HIG: 16pt padding
                                bottom: Val::Px(16.0),
                            },
                            border: UiRect::all(Val::Px(1.0)),
                            border_radius: BorderRadius::all(Val::Px(20.0)), // HIG: 20pt for cards
                            ..default()
                        },
                        BackgroundColor(palette.tan_lightest.with_alpha(0.60)),
                        BorderColor::all(palette.tan_medium.with_alpha(0.6)),
                        UiTransform::default(),
                        PlayerGoldPanelMarker,
                    ))
                    .with_children(|parent| {
                        // Mental icon circle
                        parent
                            .spawn((
                                Node {
                                    width: Val::Px(40.0), // 40pt is good (5x8pt)
                                    height: Val::Px(40.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    border: UiRect::all(Val::Px(2.0)),
                                    border_radius: BorderRadius::all(Val::Px(20.0)),
                                    ..default()
                                },
                                BackgroundColor(palette.pink_medium.with_alpha(0.6)),
                                BorderColor::all(palette.pink_dark.with_alpha(0.6)),
                            ))
                            .with_children(|parent| {
                                parent.spawn((
                                    Text::new("◎"),
                                    TextFont {
                                        font: font.clone(),
                                        font_size: 22.0, // HIG: 22pt Title 2
                                        ..default()
                                    },
                                    TextColor(palette.purple_lightest),
                                ));
                            });

                        // Gold amount section
                        parent
                            .spawn(Node {
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Start,
                                row_gap: Val::Px(4.0), // HIG: 4pt micro spacing for tight elements
                                ..default()
                            })
                            .with_children(|parent| {
                                // "Mental" label
                                parent.spawn((
                                    Text::new("Mental"),
                                    TextFont {
                                        font: font.clone(),
                                        font_size: 11.0, // HIG: 11pt caption (already correct!)
                                        ..default()
                                    },
                                    TextColor(palette.brown_dark),
                                ));

                                // Mental amount text
                                parent.spawn((
                                    Text::new("0"),
                                    TextFont {
                                        font: font.clone(),
                                        font_size: 34.0, // HIG: 34pt Large Title
                                        ..default()
                                    },
                                    TextColor(palette.brown_dark),
                                    PlayerGoldTextMarker,
                                ));
                            });
                    });
            });
    });
}

/// System to update the gold display in the UI
fn update_gold_display(
    player_gold: Option<Res<PlayerGold>>,
    mut text_query: Query<&mut Text, With<PlayerGoldTextMarker>>,
    new_markers: Query<(), Added<PlayerGoldTextMarker>>,
) {
    let Some(gold) = player_gold else {
        return;
    };

    // Update when gold changes or when new text markers are added
    if !gold.is_changed() && new_markers.is_empty() {
        return;
    }

    // Update the gold text
    for mut text in &mut text_query {
        **text = gold.amount.to_string();
    }
}

/// System to handle gold not enough message and trigger shake animation
fn handle_gold_not_enough(
    mut commands: Commands,
    mut not_enough_msg: MessageReader<PlayerGoldNotEnoughMessage>,
    panel_query: Query<Entity, With<PlayerGoldPanelMarker>>,
) {
    // Check if we received the message
    if not_enough_msg.read().next().is_none() {
        return;
    }

    // Get the gold panel entity
    let Ok(panel_entity) = panel_query.single() else {
        return;
    };

    // Create left-right shake animation
    // 0 -> -10px -> +10px -> 0
    // Total duration: 200ms
    let shake_distance = 10.0;

    let tween1 = Tween::new(
        EaseFunction::QuadraticOut,
        Duration::from_millis(50),
        UiTransformTranslationPxLens {
            start: Vec2::ZERO,
            end: Vec2::new(-shake_distance, 0.0),
        },
    );

    let tween2 = Tween::new(
        EaseFunction::QuadraticInOut,
        Duration::from_millis(100),
        UiTransformTranslationPxLens {
            start: Vec2::new(-shake_distance, 0.0),
            end: Vec2::new(shake_distance, 0.0),
        },
    );

    let tween3 = Tween::new(
        EaseFunction::QuadraticIn,
        Duration::from_millis(50),
        UiTransformTranslationPxLens {
            start: Vec2::new(shake_distance, 0.0),
            end: Vec2::ZERO,
        },
    );

    let sequence = tween1.then(tween2).then(tween3);

    commands.entity(panel_entity).insert(TweenAnim::new(sequence));
}
