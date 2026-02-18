use super::root::{PrepareRootNode, PrepareUiSets};
use crate::prelude::*;
use bevy::sprite::Anchor;
use pyri_tooltip::prelude::*;

/// Marker for the text node inside the night tooltip panel
#[derive(Component, Default)]
struct NightTooltipTextMarker;

/// Marker component for battle dots with their index
#[derive(Component, Clone)]
pub struct BattleDotMarker(pub usize);

/// Marker component for night text
#[derive(Component, Default)]
pub struct NightTextMarker;

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(GameState::Preparing),
        (
            spawn_top_middle_ui.in_set(PrepareUiSets::SpawnChildren),
            spawn_night_tooltip.after(PrepareUiSets::SpawnChildren),
        ),
    )
    .add_systems(
        Update,
        (update_battle_dots, update_night_text, update_night_tooltip)
            .run_if(in_state(GameState::Preparing)),
    );
}

fn spawn_top_middle_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    root_query: Query<Entity, With<PrepareRootNode>>,
) {
    let Ok(root_entity) = root_query.single() else {
        return;
    };

    let font = asset_server.load("fonts/Quicksand-Regular.ttf");

    commands.entity(root_entity).with_children(|parent| {
        parent
            .spawn((
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(20.0), // HIG: 20pt recommended margin
                    left: Val::Px(0.0),
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                Name::new("Top Middle UI"),
            ))
            .with_children(|parent| {
                parent
                    .spawn((
                        Node {
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            column_gap: Val::Px(16.0), // HIG: 16pt spacing
                            padding: UiRect::axes(Val::Px(20.0), Val::Px(16.0)), // HIG: 16pt vertical padding
                            border: UiRect::all(Val::Px(1.0)),
                            border_radius: BorderRadius::all(Val::Px(20.0)), // HIG: 20pt for cards
                            ..default()
                        },
                        BackgroundColor(Color::srgba_u8(0xf1, 0xdf, 0xc1, 0x7f)), // tan_lightest 50%
                        BorderColor::all(Color::srgba_u8(0xdb, 0xb5, 0x7a, 0x7f)), // tan_medium 50%
                    ))
                    .with_children(|parent| {
                        // Night Label
                        parent
                            .spawn((
                                Node {
                                    flex_direction: FlexDirection::Row,
                                    align_items: AlignItems::Center,
                                    column_gap: Val::Px(8.0), // HIG: 8pt spacing
                                    ..default()
                                },
                                NightTextMarker,
                                Tooltip::fixed(Anchor::TOP_CENTER, ""),
                            ))
                            .with_children(|parent| {
                                parent.spawn((
                                    Text::new("NIGHT"),
                                    TextFont {
                                        font: font.clone(),
                                        font_size: 11.0, // HIG: 11pt caption (minimum readable)
                                        ..default()
                                    },
                                    TextColor(Color::srgba_u8(0x5e, 0x4f, 0x5d, 0xaa)), // brown_dark 67%
                                ));

                                parent.spawn((
                                    Text::new("1"),
                                    TextFont {
                                        font: font.clone(),
                                        font_size: 22.0, // HIG: 22pt Title 2
                                        ..default()
                                    },
                                    TextColor(Color::srgba_u8(0x5e, 0x4f, 0x5d, 0xff)), // brown_dark
                                ));
                            });

                        // Separator
                        parent.spawn((
                            Node {
                                width: Val::Px(2.0),
                                height: Val::Px(24.0),
                                ..default()
                            },
                            BackgroundColor(Color::srgba_u8(0xdb, 0xb5, 0x7a, 0x7f)), // tan_medium 50%
                        ));

                        // Battle Dots
                        parent
                            .spawn(Node {
                                flex_direction: FlexDirection::Row,
                                column_gap: Val::Px(8.0), // HIG: 8pt spacing
                                ..default()
                            })
                            .with_children(|parent| {
                                // Spawn all 3 battle dots
                                for i in 0..3 {
                                    parent.spawn((
                                        Node {
                                            width: Val::Px(16.0), // HIG: 16pt (8pt grid)
                                            height: Val::Px(16.0),
                                            border: UiRect::all(Val::Px(2.0)),
                                            border_radius: BorderRadius::MAX,
                                            ..default()
                                        },
                                        BackgroundColor(Color::srgba_u8(0xe4, 0xbb, 0xf7, 0x7f)), // purple_lighter 50%
                                        BorderColor::all(Color::srgba_u8(0xbb, 0x72, 0x9f, 0x7f)), // pink_medium 50%
                                        BattleDotMarker(i),
                                    ));
                                }
                            });
                    });
            });
    });
}

fn update_battle_dots(
    campaign: Option<Res<GameProgress>>,
    mut dots: Query<(&BattleDotMarker, &mut BackgroundColor)>,
    new_markers: Query<(), Added<BattleDotMarker>>,
) {
    let Some(campaign) = campaign else {
        return;
    };

    let is_new = !new_markers.is_empty();
    if !campaign.is_changed() && !is_new {
        return;
    }

    for (marker, mut bg) in &mut dots {
        if let Some(status) = campaign.history.get(marker.0) {
            bg.0 = status.color();
        }
    }
}

fn update_night_text(
    campaign: Option<Res<GameProgress>>,
    parent_query: Query<&Children, With<NightTextMarker>>,
    mut text_query: Query<&mut Text>,
    new_markers: Query<(), Added<NightTextMarker>>,
) {
    let Some(campaign) = campaign else {
        return;
    };

    let is_new = !new_markers.is_empty();
    if !campaign.is_changed() && !is_new {
        return;
    }

    // Night is current_round + 1 (1-indexed for display)
    let night = campaign.current_round;

    // Find the night number text (second child)
    for children in &parent_query {
        if let Some(&child) = children.get(1) {
            if let Ok(mut text) = text_query.get_mut(child) {
                **text = night.to_string();
            }
        }
    }
}

fn spawn_night_tooltip(
    mut commands: Commands,
    palette: Res<ColorPalette>,
    night_q: Query<Entity, With<NightTextMarker>>,
) {
    let Ok(night_entity) = night_q.single() else {
        return;
    };

    let panel = commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(12.0)),
                border: UiRect::all(Val::Px(1.0)),
                border_radius: BorderRadius::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(palette.blue_darkest.with_alpha(0.95)),
            BorderColor::all(palette.purple_lighter.with_alpha(0.40)),
            Visibility::Hidden,
            GlobalZIndex(1),
            Pickable::IGNORE,
        ))
        .with_children(|p| {
            p.spawn((
                Text::new(""),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(palette.tan_lightest),
                NightTooltipTextMarker,
            ));
        })
        .id();

    commands.entity(night_entity).insert(
        Tooltip::fixed(Anchor::TOP_CENTER, panel).with_placement(TooltipPlacement {
            offset_y: Val::Px(-12.0),
            ..Anchor::TOP_CENTER.into()
        }),
    );
}

fn update_night_tooltip(
    battle_score: Option<Res<BattleScore>>,
    new_markers: Query<(), Added<NightTooltipTextMarker>>,
    mut text_q: Query<&mut Text, With<NightTooltipTextMarker>>,
) {
    let Some(battle_score) = battle_score else {
        return;
    };

    let is_new = !new_markers.is_empty();
    if !battle_score.is_changed() && !is_new {
        return;
    }

    let score_amount = battle_score.score_amount.unwrap_or(0) * 3;
    let tooltip_text = format!("We endured {} long nights together.", score_amount);

    for mut text in &mut text_q {
        **text = tooltip_text.clone();
    }
}
