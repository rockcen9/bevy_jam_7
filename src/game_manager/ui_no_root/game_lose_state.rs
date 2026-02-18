use crate::prelude::*;

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Lose), spawn_game_lose_ui);
}

fn spawn_game_lose_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    palette: Res<ColorPalette>,
) {
    let font = asset_server.load("fonts/Quicksand-Regular.ttf");
    commands
        .spawn((
            Node {
                width: percent(100.0),
                height: percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.75)),
            DespawnOnExit(GameState::Lose),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: px(440.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        row_gap: px(16.0),
                        padding: UiRect::all(px(32.0)),
                        border: UiRect::all(px(2.0)),
                        border_radius: BorderRadius::all(px(24.0)),
                        ..default()
                    },
                    BackgroundColor(palette.brown_dark),
                    BorderColor::all(palette.pink_medium),
                ))
                .with_children(|parent| {
                    // Eyebrow
                    parent.spawn((
                        Text::new("✦  NIGHTMARE  ✦"),
                        TextFont { font: font.clone(), font_size: 13.0, ..default() },
                        TextColor(palette.pink_medium),
                        TextLayout::new_with_justify(Justify::Center),
                    ));

                    // Divider
                    parent.spawn((
                        Node {
                            width: percent(100.0),
                            height: px(1.0),
                            ..default()
                        },
                        BackgroundColor(palette.brown_medium),
                    ));

                    // Main message
                    parent.spawn((
                        Text::new(
                            "You jolt awake in the middle\nof the night, eyes wide\nand drenched in sweat.",
                        ),
                        TextFont { font: font.clone(), font_size: 22.0, ..default() },
                        TextColor(palette.purple_lighter),
                        TextLayout::new_with_justify(Justify::Center),
                    ));

                    // Divider
                    parent.spawn((
                        Node {
                            width: percent(100.0),
                            height: px(1.0),
                            ..default()
                        },
                        BackgroundColor(palette.brown_medium),
                    ));

                    // Footer
                    parent.spawn((
                        Text::new("The nightmare continues..."),
                        TextFont { font: font.clone(), font_size: 17.0, ..default() },
                        TextColor(palette.pink_dark),
                        TextLayout::new_with_justify(Justify::Center),
                    ));
                });
        });
}
