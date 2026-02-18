use crate::game_manager::balance::{MemoryRow, MemoryStatsCache};
use crate::game_manager::ui::prepare_state::bottom_left::{
    BigEyeButtonMarker, BigHandButtonMarker, GoldenHeartButtonMarker, RaButtonMarker,
};
use crate::game_manager::ui::prepare_state::root::PrepareUiSets;
use crate::prelude::*;
use bevy::sprite::Anchor;
use pyri_tooltip::prelude::*;

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(GameState::Preparing),
        add_tooltips_to_memory_buttons.after(PrepareUiSets::SpawnChildren),
    );
}

fn build_tooltip(
    commands: &mut Commands,
    name: &str,
    desc: &str,
    row: &MemoryRow,
    palette: &ColorPalette,
    asset_server: &AssetServer,
) -> Entity {
    let bg = palette.blue_darkest.with_alpha(0.95);
    let border_col = palette.purple_lighter.with_alpha(0.40);
    let divider_col = Color::srgba(1.0, 1.0, 1.0, 0.25);
    let header_col = Color::WHITE;
    let label_col = palette.blue_medium.with_alpha(0.85);
    let subtitle_col = palette.blue_lightest;
    let desc_col = palette.tan_lightest;
    let cost_col = palette.tan_light;
    let font = asset_server.load("fonts/Quicksand-Regular.ttf");

    commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(12.0)),
                row_gap: Val::Px(8.0),
                width: Val::Px(280.0),
                border: UiRect::all(Val::Px(1.0)),
                border_radius: BorderRadius::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(bg),
            BorderColor::all(border_col),
            Visibility::Hidden,
            GlobalZIndex(1),
            Pickable::IGNORE,
        ))
        .with_children(|p| {
            // Header: memory name + subtitle + description
            p.spawn(Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(2.0),
                ..default()
            })
            .with_children(|h| {
                h.spawn((
                    Text::new(name),
                    TextFont {
                        font: font.clone(),
                        font_size: 17.0,
                        ..default()
                    },
                    TextColor(header_col),
                ));
                h.spawn((
                    Text::new("Connect to emotion"),
                    TextFont {
                        font: font.clone(),
                        font_size: 11.0,
                        ..default()
                    },
                    TextColor(subtitle_col),
                ));
                h.spawn((
                    Text::new(format!("\"{}\"", desc)),
                    TextFont {
                        font: font.clone(),
                        font_size: 12.0,
                        ..default()
                    },
                    TextColor(desc_col),
                    TextLayout::new_with_linebreak(LineBreak::WordBoundary),
                ));
            });

            // Divider
            p.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(1.0),
                    ..default()
                },
                BackgroundColor(divider_col),
            ));

            // Cost row
            p.spawn(Node {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Baseline,
                column_gap: Val::Px(5.0),
                ..default()
            })
            .with_children(|c| {
                c.spawn((
                    Text::new("Cost"),
                    TextFont {
                        font: font.clone(),
                        font_size: 13.0,
                        ..default()
                    },
                    TextColor(label_col),
                ));
                c.spawn((
                    Text::new(format!("{}", row.price)),
                    TextFont {
                        font: font.clone(),
                        font_size: 15.0,
                        ..default()
                    },
                    TextColor(cost_col),
                ));
            });
        })
        .id()
}

/// Add tooltips to the memory buttons showing CSV data
fn add_tooltips_to_memory_buttons(
    mut commands: Commands,
    cache: Res<MemoryStatsCache>,
    palette: Res<ColorPalette>,
    asset_server: Res<AssetServer>,
    bigeye_q: Query<Entity, With<BigEyeButtonMarker>>,
    ra_q: Query<Entity, With<RaButtonMarker>>,
    bighand_q: Query<Entity, With<BigHandButtonMarker>>,
    goldenheart_q: Query<Entity, With<GoldenHeartButtonMarker>>,
) {
    let buttons: [(&str, Option<Entity>); 4] = [
        ("BigEye", bigeye_q.single().ok()),
        ("RA", ra_q.single().ok()),
        ("BigHand", bighand_q.single().ok()),
        ("GoldenHeart", goldenheart_q.single().ok()),
    ];

    for (memory_id, maybe_entity) in buttons {
        let Some(entity) = maybe_entity else {
            continue;
        };
        let Some(row) = cache.stats.get(memory_id) else {
            warn!("No stats found for memory '{}'", memory_id);
            continue;
        };

        let tooltip_entity =
            build_tooltip(&mut commands, &row.name, &row.description, row, &palette, &asset_server);
        commands.entity(entity).insert(
            Tooltip::fixed(Anchor::TOP_CENTER, tooltip_entity).with_placement(
                TooltipPlacement {
                    offset_y: Val::Px(-12.0),
                    ..Anchor::TOP_CENTER.into()
                },
            ),
        );
    }
}
