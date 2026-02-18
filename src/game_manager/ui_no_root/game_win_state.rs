use bevy::prelude::*;

use crate::prelude::*;

#[derive(Component)]
pub struct WinMessageMarker;

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::WinAndNextDay), spawn_game_win_ui);
    app.add_systems(
        Update,
        update_win_message.run_if(in_state(GameState::WinAndNextDay)),
    );
}

fn spawn_game_win_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    palette: Res<ColorPalette>,
) {
    let font = asset_server.load("fonts/Quicksand-Regular.ttf");
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.75)),
            DespawnOnExit(GameState::WinAndNextDay),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Px(440.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        row_gap: Val::Px(16.0),
                        padding: UiRect::all(Val::Px(32.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        border_radius: BorderRadius::all(Val::Px(24.0)),
                        ..default()
                    },
                    BackgroundColor(palette.blue_dark),
                    BorderColor::all(palette.tan_medium),
                ))
                .with_children(|parent| {
                    // Eyebrow
                    parent.spawn((
                        Text::new("✦  DAY COMPLETE  ✦"),
                        TextFont {
                            font: font.clone(),
                            font_size: 13.0,
                            ..default()
                        },
                        TextColor(palette.tan_medium),
                        TextLayout::new_with_justify(Justify::Center),
                    ));

                    // Divider
                    parent.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(1.0),
                            ..default()
                        },
                        BackgroundColor(palette.blue_medium),
                    ));

                    // Main message
                    parent.spawn((
                        Text::new("Tonight, you finally get\na good night's sleep."),
                        TextFont {
                            font: font.clone(),
                            font_size: 28.0,
                            ..default()
                        },
                        TextColor(palette.tan_lightest),
                        TextLayout::new_with_justify(Justify::Center),
                    ));

                    // Divider
                    parent.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(1.0),
                            ..default()
                        },
                        BackgroundColor(palette.blue_medium),
                    ));

                    // Footer
                    parent.spawn((
                        Text::new("Sweet dreams..."),
                        TextFont {
                            font: font.clone(),
                            font_size: 17.0,
                            ..default()
                        },
                        TextColor(palette.blue_lighter),
                        TextLayout::new_with_justify(Justify::Center),
                    ));
                });
        });
}

fn update_win_message(
    score: Res<BattleScore>,
    mut q_message: Query<&mut Text, With<WinMessageMarker>>,
) {
    let player_name = score.player_name.as_deref().unwrap_or("Player");
    let message = format!("Congratulations, {}! You win!", player_name);
    for mut text in q_message.iter_mut() {
        if **text != message {
            **text = message.clone();
        }
    }
}
