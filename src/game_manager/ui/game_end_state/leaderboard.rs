#[cfg(not(feature = "backend"))]
pub(crate) fn plugin(_app: &mut bevy::app::App) {}
#[cfg(feature = "backend")]
pub(crate) use backend::plugin;

#[cfg(feature = "backend")]
mod backend {

    use crate::prelude::*;
    use std::cmp::Ordering;

    #[cfg(feature = "backend")]
    use bevy_jornet::{JornetEvent, Leaderboard};

    use crate::{
        game_manager::ui::game_end_state::root::{BattleEndRootNode, BattleEndUiSets},
        palette::ColorPalette,
    };

    pub(crate) fn plugin(app: &mut bevy::app::App) {
        app.add_systems(OnEnter(GameState::Preparing), initialize_leaderboard_player);

        app.add_systems(
            Update,
            (
                refresh_top_ten_entries,
                refresh_player_score_display,
                on_refresh_leaderboard_on_score_sent,
                refresh_copy_player_name_to_score,
            ),
        );

        app.add_systems(
            OnEnter(GameState::Leaderboard),
            (
                submit_player_score,
                spawn_leaderboard_ui.in_set(BattleEndUiSets::SpawnChildren),
            ),
        );
    }

    #[derive(Component, Default)]
    #[require(DespawnOnExit::<GameState>(GameState::Leaderboard))]
    pub struct LeaderboardContainer;

    #[derive(Component)]
    pub struct LeaderboardEntry {
        pub index: usize,
        pub is_name: bool,
    }

    #[derive(Component)]
    pub struct LeaderboardRankText {
        pub index: usize,
    }

    #[derive(Component, Default)]
    pub struct LeaderboardLoadingText;

    #[derive(Component, Default)]
    pub struct LeaderboardRowsContainer;

    #[derive(Component, Default)]
    pub struct YourScoreSection;

    #[derive(Component, Default)]
    pub struct YourRankText;

    #[derive(Component, Default)]
    pub struct YourNameText;

    #[derive(Component, Default)]
    pub struct YourScoreText;

    fn initialize_leaderboard_player(_commands: Commands, mut leaderboard: ResMut<Leaderboard>) {
        leaderboard.create_player(None);
        leaderboard.refresh_leaderboard();
    }

    fn refresh_copy_player_name_to_score(
        _commands: Commands,
        leaderboard: ResMut<Leaderboard>,
        mut score: ResMut<BattleScore>,
    ) {
        if !leaderboard.is_changed() {
            return;
        }

        let player = leaderboard.get_player();
        if let Some(player) = player {
            score.player_name = Some(player.name.clone());
        }
        score.score_amount = Some(leaderboard.get_leaderboard().len() as u32);
    }

    fn submit_player_score(leaderboard: Res<Leaderboard>, score: Res<BattleScore>) {
        leaderboard.send_score(score.score as f32);
    }

    fn spawn_leaderboard_ui(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        palette: Res<ColorPalette>,
        root_query: Query<Entity, With<BattleEndRootNode>>,
    ) {
        let Ok(root_entity) = root_query.single() else {
            return;
        };

        let font = asset_server.load("fonts/Quicksand-Regular.ttf");

        // Design tokens mapped from ColorPalette
        let gold = palette.tan_light; // #e7d388 warm gold — rank #1 & scores
        let silver = palette.blue_lightest; // #bbd1ee cool silver — rank #2
        let bronze = palette.brown_light; // #ce976b warm bronze — rank #3
        let text_primary = palette.tan_lightest; // #f1dfc1 — primary readable text
        let text_dim = palette.purple_lighter.with_alpha(0.55); // #e4bbf7 dimmed — labels
        let separator = palette.pink_medium.with_alpha(0.20); // #bb729f faint — dividers

        commands.entity(root_entity).with_children(|parent| {
            // Main card
            parent
                .spawn((
                    Node {
                        width: Val::Px(560.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Stretch,
                        border: UiRect::all(Val::Px(1.0)),
                        border_radius: BorderRadius::all(Val::Px(24.0)),
                        ..default()
                    },
                    BackgroundColor(palette.brown_dark.with_alpha(0.97)),
                    BorderColor::all(palette.pink_medium.with_alpha(0.45)),
                    LeaderboardContainer,
                ))
                .with_children(|card| {
                    // === HEADER ===
                    card.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            row_gap: Val::Px(4.0),
                            padding: UiRect {
                                top: Val::Px(24.0),
                                bottom: Val::Px(20.0),
                                left: Val::Px(24.0),
                                right: Val::Px(24.0),
                            },
                            ..default()
                        },
                        BackgroundColor(palette.pink_dark.with_alpha(0.25)),
                    ))
                    .with_children(|header| {
                        header.spawn((
                            Text::new("DREAM JOURNAL"),
                            TextFont {
                                font: font.clone(),
                                font_size: 28.0,
                                ..default()
                            },
                            TextColor(gold),
                        ));
                        header.spawn((
                            Text::new("Clearest Thoughts"),
                            TextFont {
                                font: font.clone(),
                                font_size: 13.0,
                                ..default()
                            },
                            TextColor(text_dim),
                        ));
                    });

                    // Gold header separator
                    card.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(1.0),
                            ..default()
                        },
                        BackgroundColor(palette.tan_light.with_alpha(0.50)),
                    ));

                    // === COLUMN HEADERS ===
                    card.spawn((Node {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        padding: UiRect {
                            top: Val::Px(8.0),
                            bottom: Val::Px(8.0),
                            left: Val::Px(24.0),
                            right: Val::Px(24.0),
                        },
                        ..default()
                    },))
                        .with_children(|headers| {
                            headers
                                .spawn((Node {
                                    width: Val::Px(48.0),
                                    ..default()
                                },))
                                .with_child((
                                    Text::new("#"),
                                    TextFont {
                                        font: font.clone(),
                                        font_size: 11.0,
                                        ..default()
                                    },
                                    TextColor(text_dim),
                                ));
                            headers
                                .spawn((Node {
                                    flex_grow: 1.0,
                                    ..default()
                                },))
                                .with_child((
                                    Text::new("DREAMER"),
                                    TextFont {
                                        font: font.clone(),
                                        font_size: 11.0,
                                        ..default()
                                    },
                                    TextColor(text_dim),
                                ));
                            headers.spawn((
                                Text::new("CLARITY"),
                                TextFont {
                                    font: font.clone(),
                                    font_size: 11.0,
                                    ..default()
                                },
                                TextColor(text_dim),
                            ));
                        });

                    // Separator under headers
                    card.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(1.0),
                            ..default()
                        },
                        BackgroundColor(separator),
                    ));

                    // === LOADING TEXT ===
                    card.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            justify_content: JustifyContent::Center,
                            padding: UiRect::vertical(Val::Px(24.0)),
                            ..default()
                        },
                        LeaderboardLoadingText,
                    ))
                    .with_child((
                        Text::new("Fetching scores..."),
                        TextFont {
                            font: font.clone(),
                            font_size: 15.0,
                            ..default()
                        },
                        TextColor(text_dim),
                    ));

                    // === ROWS CONTAINER ===
                    card.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            display: Display::None,
                            ..default()
                        },
                        LeaderboardRowsContainer,
                    ))
                    .with_children(|rows| {
                        for i in 0..10 {
                            let (row_bg, rank_color) = match i {
                                0 => (palette.tan_light.with_alpha(0.12), gold),
                                1 => (palette.blue_lightest.with_alpha(0.10), silver),
                                2 => (palette.brown_light.with_alpha(0.12), bronze),
                                n if n % 2 == 0 => {
                                    (palette.purple_lightest.with_alpha(0.04), text_dim)
                                }
                                _ => (palette.brown_dark.with_alpha(0.0), text_dim),
                            };

                            rows.spawn((
                                Node {
                                    width: Val::Percent(100.0),
                                    flex_direction: FlexDirection::Row,
                                    align_items: AlignItems::Center,
                                    padding: UiRect {
                                        top: Val::Px(11.0),
                                        bottom: Val::Px(11.0),
                                        left: Val::Px(24.0),
                                        right: Val::Px(24.0),
                                    },
                                    ..default()
                                },
                                BackgroundColor(row_bg),
                            ))
                            .with_children(|row| {
                                // Rank column
                                row.spawn((Node {
                                    width: Val::Px(48.0),
                                    ..default()
                                },))
                                    .with_child((
                                        Text::new(format!("{}", i + 1)),
                                        TextFont {
                                            font: font.clone(),
                                            font_size: 17.0,
                                            ..default()
                                        },
                                        TextColor(rank_color),
                                        LeaderboardRankText { index: i },
                                    ));

                                // Name column (grows)
                                row.spawn((Node {
                                    flex_grow: 1.0,
                                    ..default()
                                },))
                                    .with_child((
                                        Text::new(""),
                                        TextFont {
                                            font: font.clone(),
                                            font_size: 17.0,
                                            ..default()
                                        },
                                        TextColor(text_primary),
                                        LeaderboardEntry {
                                            index: i,
                                            is_name: true,
                                        },
                                    ));

                                // Score column
                                row.spawn((
                                    Text::new(""),
                                    TextFont {
                                        font: font.clone(),
                                        font_size: 17.0,
                                        ..default()
                                    },
                                    TextColor(gold),
                                    LeaderboardEntry {
                                        index: i,
                                        is_name: false,
                                    },
                                ));
                            });
                        }
                    });

                    // === YOUR SCORE SECTION ===
                    card.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            display: Display::None,
                            ..default()
                        },
                        YourScoreSection,
                    ))
                    .with_children(|section| {
                        // Separator
                        section.spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Px(1.0),
                                ..default()
                            },
                            BackgroundColor(separator),
                        ));

                        // "YOUR SCORE" label
                        section
                            .spawn((Node {
                                padding: UiRect {
                                    top: Val::Px(10.0),
                                    bottom: Val::Px(4.0),
                                    left: Val::Px(24.0),
                                    right: Val::Px(24.0),
                                },
                                ..default()
                            },))
                            .with_child((
                                Text::new("YOU ARE HERE"),
                                TextFont {
                                    font: font.clone(),
                                    font_size: 11.0,
                                    ..default()
                                },
                                TextColor(palette.blue_light.with_alpha(0.85)),
                            ));

                        // Your score row
                        section
                            .spawn((
                                Node {
                                    width: Val::Percent(100.0),
                                    flex_direction: FlexDirection::Row,
                                    align_items: AlignItems::Center,
                                    padding: UiRect {
                                        top: Val::Px(11.0),
                                        bottom: Val::Px(16.0),
                                        left: Val::Px(24.0),
                                        right: Val::Px(24.0),
                                    },
                                    ..default()
                                },
                                BackgroundColor(palette.blue_medium.with_alpha(0.20)),
                            ))
                            .with_children(|row| {
                                // Rank col
                                row.spawn((Node {
                                    width: Val::Px(48.0),
                                    ..default()
                                },))
                                    .with_child((
                                        Text::new("-"),
                                        TextFont {
                                            font: font.clone(),
                                            font_size: 17.0,
                                            ..default()
                                        },
                                        TextColor(palette.blue_light),
                                        YourRankText,
                                    ));

                                // Name col (grows)
                                row.spawn((Node {
                                    flex_grow: 1.0,
                                    ..default()
                                },))
                                    .with_child((
                                        Text::new("You"),
                                        TextFont {
                                            font: font.clone(),
                                            font_size: 17.0,
                                            ..default()
                                        },
                                        TextColor(text_primary),
                                        YourNameText,
                                    ));

                                // Score col
                                row.spawn((
                                    Text::new("0"),
                                    TextFont {
                                        font: font.clone(),
                                        font_size: 17.0,
                                        ..default()
                                    },
                                    TextColor(gold),
                                    YourScoreText,
                                ));
                            });
                    });
                });
        });
    }

    fn refresh_top_ten_entries(
        leaderboard: Res<Leaderboard>,
        mut entries: Query<(&LeaderboardEntry, &mut Text), Without<LeaderboardRankText>>,
        mut rank_texts: Query<(&LeaderboardRankText, &mut Text), Without<LeaderboardEntry>>,
        mut loading: Query<
            &mut Node,
            (
                With<LeaderboardLoadingText>,
                Without<LeaderboardRowsContainer>,
            ),
        >,
        mut rows: Query<
            &mut Node,
            (
                With<LeaderboardRowsContainer>,
                Without<LeaderboardLoadingText>,
            ),
        >,
    ) {
        if !leaderboard.is_changed() {
            return;
        }

        let mut scores = leaderboard.get_leaderboard();
        scores
            .sort_unstable_by(|s1, s2| s2.score.partial_cmp(&s1.score).unwrap_or(Ordering::Equal));
        scores.truncate(10);

        let has_scores = !scores.is_empty();

        for mut node in loading.iter_mut() {
            node.display = if has_scores {
                Display::None
            } else {
                Display::Flex
            };
        }
        for mut node in rows.iter_mut() {
            node.display = if has_scores {
                Display::Flex
            } else {
                Display::None
            };
        }

        for (entry, mut text) in entries.iter_mut() {
            if entry.index >= scores.len() {
                text.0 = String::new();
                continue;
            }
            if entry.is_name {
                text.0 = scores[entry.index].player.clone();
            } else {
                text.0 = format!("{:.0}", scores[entry.index].score);
            }
        }

        for (rank_text, mut text) in rank_texts.iter_mut() {
            if rank_text.index >= scores.len() {
                text.0 = String::new();
            } else {
                text.0 = format!("{}", rank_text.index + 1);
            }
        }
    }

    fn refresh_player_score_display(
        leaderboard: Res<Leaderboard>,
        score: Res<BattleScore>,
        mut your_section: Query<&mut Node, With<YourScoreSection>>,
        mut your_rank: Query<
            &mut Text,
            (
                With<YourRankText>,
                Without<YourNameText>,
                Without<YourScoreText>,
            ),
        >,
        mut your_name: Query<
            &mut Text,
            (
                With<YourNameText>,
                Without<YourRankText>,
                Without<YourScoreText>,
            ),
        >,
        mut your_score: Query<
            &mut Text,
            (
                With<YourScoreText>,
                Without<YourRankText>,
                Without<YourNameText>,
            ),
        >,
    ) {
        if !leaderboard.is_changed() {
            return;
        }

        let mut scores = leaderboard.get_leaderboard();
        if scores.is_empty() {
            return;
        }

        scores
            .sort_unstable_by(|s1, s2| s2.score.partial_cmp(&s1.score).unwrap_or(Ordering::Equal));

        let player_name = score.player_name.as_deref().unwrap_or("");

        // Find the player's rank in the full leaderboard
        let player_rank = scores
            .iter()
            .position(|s| s.player == player_name)
            .map(|i| i + 1);

        // Show the section
        for mut node in your_section.iter_mut() {
            node.display = Display::Flex;
        }

        if let Some(rank) = player_rank {
            for mut text in your_rank.iter_mut() {
                text.0 = format!("{}.", rank);
            }
        } else {
            for mut text in your_rank.iter_mut() {
                text.0 = "-".to_string();
            }
        }

        for mut text in your_name.iter_mut() {
            if player_name.is_empty() {
                text.0 = "You".to_string();
            } else {
                text.0 = player_name.to_string();
            }
        }

        for mut text in your_score.iter_mut() {
            text.0 = format!("{:.0}", score.score);
        }
    }

    fn on_refresh_leaderboard_on_score_sent(
        mut events: MessageReader<JornetEvent>,
        leaderboard: Res<Leaderboard>,
    ) {
        for event in events.read() {
            if *event == JornetEvent::SendScoreSuccess {
                leaderboard.refresh_leaderboard();
            }
        }
    }
}
