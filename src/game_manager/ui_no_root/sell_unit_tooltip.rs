use crate::game_manager::balance::{UnitRow, UnitStatsCache};
use crate::game_manager::ui::prepare_state::bottom_middle::{
    ArcherButtonMarker, ShieldButtonMarker, SpearButtonMarker,
};
use crate::game_manager::ui::prepare_state::root::PrepareUiSets;
use crate::prelude::*;
use bevy::sprite::Anchor;
use pyri_tooltip::prelude::*;

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(GameState::Preparing),
        add_tooltips_to_unit_buttons.after(PrepareUiSets::SpawnChildren),
    );
}

fn build_tooltip(
    commands: &mut Commands,
    name: &str,
    desc: &str,
    row: &UnitRow,
    palette: &ColorPalette,
    asset_server: &AssetServer,
) -> Entity {
    let bg = palette.blue_darkest.with_alpha(0.95);
    let border_col = palette.purple_lighter.with_alpha(0.40);
    let divider_col = Color::srgba(1.0, 1.0, 1.0, 0.25);
    let header_col = Color::WHITE;
    let label_col = palette.blue_medium.with_alpha(0.85);
    let desc_col = palette.tan_lightest;
    let value_col = palette.tan_lightest;
    let cost_col = palette.tan_light;
    let font = asset_server.load("fonts/Quicksand-Regular.ttf");

    // Stats: 3 pairs per row, 2 rows (HP/ATK/DEF, Speed/Move/Range)
    let stat_pairs: &[(&str, String)] = &[
        ("HP", format!("{}", row.hp)),
        ("ATK", format!("{}", row.atk)),
        ("DEF", format!("{}", row.def)),
        ("SPD", format!("{}", row.atk_speed)),
        ("MOV", format!("{}", row.move_speed)),
        ("RNG", format!("{}", row.range)),
    ];

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
            // Header: unit name + description
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
                    Text::new(format!("{}", row.cost)),
                    TextFont {
                        font: font.clone(),
                        font_size: 15.0,
                        ..default()
                    },
                    TextColor(cost_col),
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

            // Stats grid: 3 pairs per row (6 cols: label | val | label | val | label | val)
            p.spawn(Node {
                display: Display::Grid,
                grid_template_columns: vec![
                    GridTrack::auto(),
                    GridTrack::flex(1.0),
                    GridTrack::auto(),
                    GridTrack::flex(1.0),
                    GridTrack::auto(),
                    GridTrack::flex(1.0),
                ],
                column_gap: Val::Px(6.0),
                row_gap: Val::Px(4.0),
                ..default()
            })
            .with_children(|g| {
                for (label, value) in stat_pairs {
                    g.spawn((
                        Text::new(*label),
                        TextFont {
                            font: font.clone(),
                            font_size: 11.0,
                            ..default()
                        },
                        TextColor(label_col),
                    ));
                    g.spawn((
                        Text::new(value.clone()),
                        TextFont {
                            font: font.clone(),
                            font_size: 11.0,
                            ..default()
                        },
                        TextColor(value_col),
                    ));
                }
            });
        })
        .id()
}

/// Add tooltips to the unit buttons showing CSV data
fn add_tooltips_to_unit_buttons(
    mut commands: Commands,
    cache: Res<UnitStatsCache>,
    palette: Res<ColorPalette>,
    asset_server: Res<AssetServer>,
    shield_q: Query<Entity, With<ShieldButtonMarker>>,
    archer_q: Query<Entity, With<ArcherButtonMarker>>,
    spear_q: Query<Entity, With<SpearButtonMarker>>,
) {
    let buttons: [(&str, Option<Entity>); 3] = [
        ("Shield", shield_q.single().ok()),
        ("Archer", archer_q.single().ok()),
        ("Spear", spear_q.single().ok()),
    ];

    for (unit_id, maybe_entity) in buttons {
        let Some(entity) = maybe_entity else {
            continue;
        };
        let Some(row) = cache.stats.get(unit_id) else {
            warn!("No stats found for unit '{}'", unit_id);
            continue;
        };

        let tooltip_entity =
            build_tooltip(&mut commands, &row.game_unit_name, &row.desc, row, &palette, &asset_server);
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
