use crate::prelude::*;

use super::root::{BattleRootNode, BattleUiSets};

#[derive(Component)]
struct SpeedLabelTextMarker;

#[derive(Component)]
struct SpeedBtnPauseMarker;

#[derive(Component)]
struct SpeedBtn1xMarker;

#[derive(Component)]
struct SpeedBtn2xMarker;

#[derive(Component)]
struct SpeedBtn4xMarker;

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(GameState::Battle),
        spawn_top_right_ui.in_set(BattleUiSets::SpawnChildren),
    );
    app.add_systems(OnExit(GameState::Battle), reset_speed_to_default);
    app.add_systems(
        Update,
        (
            handle_speed_buttons,
            update_speed_label,
            update_speed_button_styles,
        )
            .run_if(in_state(GameState::Battle)),
    );
}

fn reset_speed_to_default(mut time: ResMut<Time<Virtual>>) {
    time.set_relative_speed(1.0);
}

fn spawn_top_right_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    q_root: Query<Entity, With<BattleRootNode>>,
    palette: Res<ColorPalette>,
) {
    let Ok(root_entity) = q_root.single() else {
        warn!("BattleRootNode not found, cannot spawn top_right UI");
        return;
    };

    let font = asset_server.load("fonts/Quicksand-Regular.ttf");

    commands.entity(root_entity).with_children(|parent| {
        // Absolute positioned container at top-right
        parent
            .spawn(Node {
                position_type: PositionType::Absolute,
                top: px(20),
                right: px(20),
                ..default()
            })
            .with_children(|parent| {
                // Speed control panel
                parent
                    .spawn((
                        Node {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            width: px(64),
                            border: UiRect::all(px(1)),
                            border_radius: BorderRadius::all(px(12)),
                            ..default()
                        },
                        BackgroundColor(palette.tan_lightest.with_alpha(0.6)),
                        BorderColor::all(palette.tan_medium.with_alpha(0.6)),
                    ))
                    .with_children(|parent| {
                        // Speed header
                        parent
                            .spawn((
                                Node {
                                    width: percent(100),
                                    justify_content: JustifyContent::Center,
                                    padding: UiRect::vertical(px(8)),
                                    border_radius: BorderRadius::px(12.0, 12.0, 0.0, 0.0), // top-left, top-right, bottom-right, bottom-left
                                    ..default()
                                },
                                BackgroundColor(palette.tan_light.with_alpha(0.4)),
                            ))
                            .with_children(|parent| {
                                parent.spawn((
                                    Text::new("1x"),
                                    TextFont {
                                        font: font.clone(),
                                        font_size: 13.0,
                                        ..default()
                                    },
                                    TextColor(palette.brown_dark),
                                    SpeedLabelTextMarker,
                                ));
                            });

                        // Speed buttons container
                        parent
                            .spawn(Node {
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                padding: UiRect::vertical(px(8)),
                                row_gap: px(4),
                                ..default()
                            })
                            .with_children(|parent| {
                                // Pause button
                                parent
                                    .spawn((
                                        Node {
                                            width: px(48),
                                            height: px(44),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            border_radius: BorderRadius::all(px(10)),
                                            ..default()
                                        },
                                        Button,
                                        BackgroundColor(palette.tan_medium.with_alpha(0.35)),
                                        SpeedBtnPauseMarker,
                                    ))
                                    .with_children(|parent| {
                                        parent.spawn((
                                            Text::new("||"),
                                            TextFont {
                                                font: font.clone(),
                                                font_size: 13.0,
                                                ..default()
                                            },
                                            TextColor(palette.brown_dark),
                                        ));
                                    });

                                // 1x button
                                parent
                                    .spawn((
                                        Node {
                                            width: px(48),
                                            height: px(44),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            border_radius: BorderRadius::all(px(10)),
                                            ..default()
                                        },
                                        Button,
                                        BackgroundColor(palette.green_medium.with_alpha(0.7)),
                                        SpeedBtn1xMarker,
                                    ))
                                    .with_children(|parent| {
                                        parent.spawn((
                                            Text::new("1x"),
                                            TextFont {
                                                font: font.clone(),
                                                font_size: 13.0,
                                                ..default()
                                            },
                                            TextColor(palette.brown_dark),
                                        ));
                                    });

                                // 2x button
                                parent
                                    .spawn((
                                        Node {
                                            width: px(48),
                                            height: px(44),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            border_radius: BorderRadius::all(px(10)),
                                            ..default()
                                        },
                                        Button,
                                        BackgroundColor(palette.tan_medium.with_alpha(0.35)),
                                        SpeedBtn2xMarker,
                                    ))
                                    .with_children(|parent| {
                                        parent.spawn((
                                            Text::new("2x"),
                                            TextFont {
                                                font: font.clone(),
                                                font_size: 13.0,
                                                ..default()
                                            },
                                            TextColor(palette.brown_dark),
                                        ));
                                    });

                                // 4x button
                                parent
                                    .spawn((
                                        Node {
                                            width: px(48),
                                            height: px(44),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            border_radius: BorderRadius::all(px(10)),
                                            ..default()
                                        },
                                        Button,
                                        BackgroundColor(palette.tan_medium.with_alpha(0.35)),
                                        SpeedBtn4xMarker,
                                    ))
                                    .with_children(|parent| {
                                        parent.spawn((
                                            Text::new("4x"),
                                            TextFont {
                                                font: font.clone(),
                                                font_size: 13.0,
                                                ..default()
                                            },
                                            TextColor(palette.brown_dark),
                                        ));
                                    });
                            });
                    });
            });
    });
}

fn handle_speed_buttons(
    mut time: ResMut<Time<Virtual>>,
    pause_query: Query<&Interaction, (Changed<Interaction>, With<SpeedBtnPauseMarker>)>,
    speed_1x_query: Query<&Interaction, (Changed<Interaction>, With<SpeedBtn1xMarker>)>,
    speed_2x_query: Query<&Interaction, (Changed<Interaction>, With<SpeedBtn2xMarker>)>,
    speed_4x_query: Query<&Interaction, (Changed<Interaction>, With<SpeedBtn4xMarker>)>,
) {
    for interaction in pause_query.iter() {
        if *interaction == Interaction::Pressed {
            time.set_relative_speed(0.0);
        }
    }
    for interaction in speed_1x_query.iter() {
        if *interaction == Interaction::Pressed {
            time.set_relative_speed(1.0);
        }
    }
    for interaction in speed_2x_query.iter() {
        if *interaction == Interaction::Pressed {
            time.set_relative_speed(2.0);
        }
    }
    for interaction in speed_4x_query.iter() {
        if *interaction == Interaction::Pressed {
            time.set_relative_speed(4.0);
        }
    }
}

fn update_speed_label(
    time: Res<Time<Virtual>>,
    mut query: Query<&mut Text, With<SpeedLabelTextMarker>>,
) {
    let speed = time.relative_speed();
    let label = if speed == 0.0 {
        "||".to_string()
    } else {
        format!("{}x", speed as u32)
    };

    for mut text in query.iter_mut() {
        **text = label.clone();
    }
}

fn update_speed_button_styles(
    time: Res<Time<Virtual>>,
    mut pause_query: Query<
        &mut BackgroundColor,
        (
            With<SpeedBtnPauseMarker>,
            Without<SpeedBtn1xMarker>,
            Without<SpeedBtn2xMarker>,
            Without<SpeedBtn4xMarker>,
        ),
    >,
    mut speed_1x_query: Query<
        &mut BackgroundColor,
        (
            With<SpeedBtn1xMarker>,
            Without<SpeedBtnPauseMarker>,
            Without<SpeedBtn2xMarker>,
            Without<SpeedBtn4xMarker>,
        ),
    >,
    mut speed_2x_query: Query<
        &mut BackgroundColor,
        (
            With<SpeedBtn2xMarker>,
            Without<SpeedBtnPauseMarker>,
            Without<SpeedBtn1xMarker>,
            Without<SpeedBtn4xMarker>,
        ),
    >,
    mut speed_4x_query: Query<
        &mut BackgroundColor,
        (
            With<SpeedBtn4xMarker>,
            Without<SpeedBtnPauseMarker>,
            Without<SpeedBtn1xMarker>,
            Without<SpeedBtn2xMarker>,
        ),
    >,
) {
    let speed = time.relative_speed();
    let active_color = Color::srgba_u8(0x78, 0x9a, 0x73, 0xff); // green_medium
    let inactive_color = Color::srgba_u8(0xdb, 0xb5, 0x7a, 0x59); // tan_medium 35%

    for mut bg in pause_query.iter_mut() {
        *bg = if speed == 0.0 {
            active_color.into()
        } else {
            inactive_color.into()
        };
    }
    for mut bg in speed_1x_query.iter_mut() {
        *bg = if speed == 1.0 {
            active_color.into()
        } else {
            inactive_color.into()
        };
    }
    for mut bg in speed_2x_query.iter_mut() {
        *bg = if speed == 2.0 {
            active_color.into()
        } else {
            inactive_color.into()
        };
    }
    for mut bg in speed_4x_query.iter_mut() {
        *bg = if speed == 4.0 {
            active_color.into()
        } else {
            inactive_color.into()
        };
    }
}
