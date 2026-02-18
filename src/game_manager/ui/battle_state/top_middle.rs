use crate::prelude::*;

use super::root::{BattleRootNode, BattleUiSets};

#[derive(Component)]
pub struct PlayerUnitBarMarker;

#[derive(Component)]
pub struct EnemyUnitBarMarker;

#[derive(Component)]
pub struct PlayerUnitCountMarker;

#[derive(Component)]
pub struct EnemyUnitCountMarker;

// #[derive(Component)]
// pub struct MoraleBarMarker;

/// Tracks the initial unit counts at the start of battle
#[derive(Resource, Default)]
pub struct InitialUnitCounts {
    pub player: usize,
    pub enemy: usize,
}

pub(crate) fn plugin(app: &mut bevy::app::App) {
    app.init_resource::<InitialUnitCounts>()
        .add_systems(
            OnEnter(GameState::Battle),
            spawn_top_middle_ui.in_set(BattleUiSets::SpawnChildren),
        )
        .add_systems(
            Update,
            (record_initial_counts, update_battle_bars).run_if(in_state(GameState::Battle)),
        );
}

fn spawn_top_middle_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    q_root: Query<Entity, With<BattleRootNode>>,
    palette: Res<ColorPalette>,
) {
    let Ok(root_entity) = q_root.single() else {
        warn!("BattleRootNode not found, cannot spawn top_middle UI");
        return;
    };

    let font = asset_server.load("fonts/Quicksand-Regular.ttf");

    commands.entity(root_entity).with_children(|parent| {
        parent
            .spawn(Node {
                width: percent(100),
                height: percent(100),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            })
            .with_children(|parent| {
                // Main container with top margin
                parent
                    .spawn(Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        margin: UiRect::top(px(16)),
                        ..default()
                    })
                    .with_children(|parent| {
                        // Single panel container
                        parent
                            .spawn((
                                Node {
                                    flex_direction: FlexDirection::Column,
                                    padding: UiRect::axes(px(12), px(8)),
                                    border: UiRect::all(px(1)),
                                    border_radius: BorderRadius::all(px(12)),
                                    ..default()
                                },
                                BackgroundColor(palette.tan_lightest.with_alpha(0.5)),
                                BorderColor::all(palette.blue_lighter.with_alpha(0.5)),
                            ))
                            .with_children(|parent| {
                                // Labels row
                                parent
                                    .spawn(Node {
                                        flex_direction: FlexDirection::Row,
                                        justify_content: JustifyContent::SpaceBetween,
                                        width: percent(100),
                                        margin: UiRect::bottom(px(8)),
                                        ..default()
                                    })
                                    .with_children(|parent| {
                                        // ALLIES label with dot
                                        parent
                                            .spawn(Node {
                                                flex_direction: FlexDirection::Row,
                                                align_items: AlignItems::Center,
                                                column_gap: px(8),
                                                ..default()
                                            })
                                            .with_children(|parent| {
                                                // Blue dot
                                                parent.spawn((
                                                    Node {
                                                        width: px(8),
                                                        height: px(8),
                                                        border_radius: BorderRadius::MAX,
                                                        ..default()
                                                    },
                                                    BackgroundColor(palette.blue_medium),
                                                ));

                                                // ALLIES text
                                                parent.spawn((
                                                    Text::new("WILL"),
                                                    TextFont {
                                                        font: font.clone(),
                                                        font_size: 11.0,
                                                        ..default()
                                                    },
                                                    TextColor(palette.brown_dark.with_alpha(0.85)),
                                                ));
                                            });

                                        // ENEMIES label with dot
                                        parent
                                            .spawn(Node {
                                                flex_direction: FlexDirection::Row,
                                                align_items: AlignItems::Center,
                                                column_gap: px(8),
                                                ..default()
                                            })
                                            .with_children(|parent| {
                                                // ENEMIES text
                                                parent.spawn((
                                                    Text::new("URGE"),
                                                    TextFont {
                                                        font: font.clone(),
                                                        font_size: 11.0,
                                                        ..default()
                                                    },
                                                    TextColor(palette.brown_dark.with_alpha(0.85)),
                                                ));

                                                // Red dot
                                                parent.spawn((
                                                    Node {
                                                        width: px(8),
                                                        height: px(8),
                                                        border_radius: BorderRadius::MAX,
                                                        ..default()
                                                    },
                                                    BackgroundColor(palette.brown_reddish),
                                                ));
                                            });
                                    });

                                // Battle bars container
                                parent
                                    .spawn(Node {
                                        flex_direction: FlexDirection::Row,
                                        align_items: AlignItems::Center,
                                        column_gap: px(8),
                                        ..default()
                                    })
                                    .with_children(|parent| {
                                        // Player bar section
                                        parent
                                            .spawn((
                                                Node {
                                                    flex_direction: FlexDirection::Row,
                                                    justify_content: JustifyContent::FlexEnd,
                                                    width: px(176),
                                                    height: px(24),
                                                    border: UiRect::all(px(1)),
                                                    border_radius: BorderRadius::all(px(6)),
                                                    overflow: Overflow::clip(),
                                                    ..default()
                                                },
                                                BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.15)),
                                                BorderColor::all(Color::srgba(1.0, 1.0, 1.0, 0.20)),
                                            ))
                                            .with_children(|parent| {
                                                parent.spawn((
                                                    Node {
                                                        height: percent(100),
                                                        width: percent(100),
                                                        border_radius: BorderRadius::all(px(6)),
                                                        ..default()
                                                    },
                                                    BackgroundColor(palette.blue_medium),
                                                    PlayerUnitBarMarker,
                                                ));
                                            });

                                        // Center emblem
                                        parent
                                            .spawn((
                                                Node {
                                                    width: px(32),
                                                    height: px(32),
                                                    justify_content: JustifyContent::Center,
                                                    align_items: AlignItems::Center,
                                                    border: UiRect::all(px(2)),
                                                    border_radius: BorderRadius::MAX,
                                                    ..default()
                                                },
                                                BackgroundColor(palette.blue_dark.with_alpha(0.5)),
                                                BorderColor::all(Color::srgba(1.0, 1.0, 1.0, 0.20)),
                                            ))
                                            .with_children(|parent| {
                                                parent.spawn((
                                                    Node {
                                                        width: px(12),
                                                        height: px(12),
                                                        border_radius: BorderRadius::MAX,
                                                        ..default()
                                                    },
                                                    BackgroundColor(palette.blue_medium.with_alpha(0.6)),
                                                ));
                                            });

                                        // Enemy bar section
                                        parent
                                            .spawn((
                                                Node {
                                                    flex_direction: FlexDirection::Row,
                                                    justify_content: JustifyContent::FlexStart,
                                                    width: px(176),
                                                    height: px(24),
                                                    border: UiRect::all(px(1)),
                                                    border_radius: BorderRadius::all(px(6)),
                                                    overflow: Overflow::clip(),
                                                    ..default()
                                                },
                                                BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.15)),
                                                BorderColor::all(Color::srgba(1.0, 1.0, 1.0, 0.20)),
                                            ))
                                            .with_children(|parent| {
                                                parent.spawn((
                                                    Node {
                                                        height: percent(100),
                                                        width: percent(100),
                                                        border_radius: BorderRadius::all(px(6)),
                                                        ..default()
                                                    },
                                                    BackgroundColor(palette.brown_reddish),
                                                    EnemyUnitBarMarker,
                                                ));
                                            });
                                    });

                                // Unit counts row
                                parent
                                    .spawn(Node {
                                        flex_direction: FlexDirection::Row,
                                        justify_content: JustifyContent::SpaceBetween,
                                        width: percent(100),
                                        margin: UiRect::top(px(8)),
                                        padding: UiRect::horizontal(px(4)),
                                        ..default()
                                    })
                                    .with_children(|parent| {
                                        // Player count
                                        parent.spawn((
                                            Text::new("0"),
                                            TextFont {
                                                font: font.clone(),
                                                font_size: 17.0,
                                                ..default()
                                            },
                                            TextColor(palette.brown_dark),
                                            PlayerUnitCountMarker,
                                        ));

                                        // VS text
                                        parent.spawn((
                                            Text::new("VS"),
                                            TextFont {
                                                font: font.clone(),
                                                font_size: 13.0,
                                                ..default()
                                            },
                                            TextColor(palette.brown_dark.with_alpha(0.7)),
                                        ));

                                        // Enemy count
                                        parent.spawn((
                                            Text::new("0"),
                                            TextFont {
                                                font: font.clone(),
                                                font_size: 17.0,
                                                ..default()
                                            },
                                            TextColor(palette.brown_dark),
                                            EnemyUnitCountMarker,
                                        ));
                                    });

                                // // Morale label
                                // parent
                                //     .spawn(Node {
                                //         flex_direction: FlexDirection::Row,
                                //         justify_content: JustifyContent::Center,
                                //         width: percent(100),
                                //         margin: UiRect::axes(px(0), px(8)).with_bottom(px(4)),
                                //         ..default()
                                //     })
                                //     .with_children(|parent| {
                                //         parent.spawn((
                                //             Text::new("SAINTY"),
                                //             TextFont {
                                //                 font: font.clone(),
                                //                 font_size: 11.0,
                                //                 ..default()
                                //             },
                                //             TextColor(Color::srgb(0.541, 0.541, 0.667)), // #8a8aaa
                                //         ));
                                //     });

                                // // Morale bar container
                                // parent
                                //     .spawn((
                                //         Node {
                                //             position_type: PositionType::Relative,
                                //             width: percent(100),
                                //             height: px(12),
                                //             border: UiRect::all(px(1)),
                                //             border_radius: BorderRadius::all(px(6)),
                                //             overflow: Overflow::clip(),
                                //             ..default()
                                //         },
                                //         BackgroundColor(Color::srgb(0.039, 0.039, 0.078)), // #0a0a14
                                //         BorderColor::all(Color::srgb(0.165, 0.165, 0.290)), // #2a2a4a
                                //     ))
                                //     .with_children(|parent| {
                                //         // Fill bar (starts at 50% for neutral)
                                //         parent.spawn((
                                //             Node {
                                //                 height: percent(100),
                                //                 width: percent(50),
                                //                 ..default()
                                //             },
                                //             BackgroundColor(Color::srgb(0.416, 0.416, 0.667)), // #6a6aaa
                                //             MoraleBarMarker,
                                //         ));
                                //     });
                            });
                    });
            });
    });
}

/// Record initial unit counts once at the start
fn record_initial_counts(
    mut initial_counts: ResMut<InitialUnitCounts>,
    q_player_units: Query<(), With<PlayerUnit>>,
    q_enemy_units: Query<(), With<EnemyUnit>>,
) {
    // Only record once when counts are zero
    if initial_counts.player == 0 && initial_counts.enemy == 0 {
        let player_count = q_player_units.iter().count();
        let enemy_count = q_enemy_units.iter().count();

        if player_count > 0 || enemy_count > 0 {
            initial_counts.player = player_count;
            initial_counts.enemy = enemy_count;
        }
    }
}

/// Update the battle bars based on current unit counts
fn update_battle_bars(
    initial_counts: Res<InitialUnitCounts>,
    q_player_units: Query<(), With<PlayerUnit>>,
    q_enemy_units: Query<(), With<EnemyUnit>>,
    mut q_player_bar: Query<&mut Node, (With<PlayerUnitBarMarker>, Without<EnemyUnitBarMarker>)>,
    mut q_enemy_bar: Query<&mut Node, (With<EnemyUnitBarMarker>, Without<PlayerUnitBarMarker>)>,
    mut q_player_count: Query<
        &mut Text,
        (With<PlayerUnitCountMarker>, Without<EnemyUnitCountMarker>),
    >,
    mut q_enemy_count: Query<
        &mut Text,
        (With<EnemyUnitCountMarker>, Without<PlayerUnitCountMarker>),
    >,
) {
    // Skip if initial counts not recorded yet
    if initial_counts.player == 0 && initial_counts.enemy == 0 {
        return;
    }

    let current_player = q_player_units.iter().count();
    let current_enemy = q_enemy_units.iter().count();

    // Calculate percentages (clamped to 100% max to handle unit growth)
    let player_percent = if initial_counts.player > 0 {
        ((current_player as f32 / initial_counts.player as f32) * 100.0).min(100.0)
    } else {
        0.0
    };

    let enemy_percent = if initial_counts.enemy > 0 {
        ((current_enemy as f32 / initial_counts.enemy as f32) * 100.0).min(100.0)
    } else {
        0.0
    };

    // Update player bar width
    if let Ok(mut node) = q_player_bar.single_mut() {
        node.width = Val::Percent(player_percent);
    }

    // Update enemy bar width
    if let Ok(mut node) = q_enemy_bar.single_mut() {
        node.width = Val::Percent(enemy_percent);
    }

    // Update player count text
    if let Ok(mut text) = q_player_count.single_mut() {
        **text = current_player.to_string();
    }

    // Update enemy count text
    if let Ok(mut text) = q_enemy_count.single_mut() {
        **text = current_enemy.to_string();
    }
}

// /// Update the morale bar based on CombatFlux
// fn update_morale_bar(
//     combat_flux: Res<crate::game_manager::army::CombatFlux>,
//     mut q_morale_bar: Query<(&mut Node, &mut BackgroundColor), With<MoraleBarMarker>>,
// ) {
//     let Ok((mut node, mut bg_color)) = q_morale_bar.single_mut() else {
//         return;
//     };

//     let morale = combat_flux.morale();

//     // Convert morale from [-100, 100] range to [0, 100] percentage
//     // -100 = 0%, 0 = 50%, +100 = 100%
//     let fill_percent = ((morale + 100.0) / 200.0) * 100.0;

//     // Set color based on which side has advantage
//     let color = if morale < -5.0 {
//         Color::srgb(0.937, 0.267, 0.267) // Red for enemy advantage (#ef4444)
//     } else if morale > 5.0 {
//         Color::srgb(0.231, 0.510, 0.965) // Blue for player advantage (#3b82f6)
//     } else {
//         Color::srgb(0.416, 0.416, 0.667) // Neutral purple (#6a6aaa)
//     };

//     node.width = Val::Percent(fill_percent);
//     *bg_color = BackgroundColor(color);
// }
