use bevy::picking::prelude::*;
use bevy::ui::Val::*;
use bevy_ui_anchor::prelude::*;

use crate::prelude::*;

pub(crate) fn plugin(app: &mut bevy::app::App) {
    app.add_systems(
        Update,
        (
            add_hover_observers_to_units,
            update_panel_text,
            update_buffs_text,
        ),
    );
}

/// Marker for the health panel UI
#[derive(Component)]
struct UnitHealthPanel;

/// Links the health panel to the unit it displays
#[derive(Component)]
struct PanelForUnit(Entity);

/// Marker for the health text element
#[derive(Component, Default)]
struct HealthTextMarker;

/// Marker for the unit name text element
#[derive(Component, Default)]
struct UnitNameMarker;

/// Marker for the damage text element
#[derive(Component, Default)]
struct DamageTextMarker;

/// Marker for the defense text element
#[derive(Component, Default)]
struct DefenseTextMarker;

/// Marker for the speed text element
#[derive(Component, Default)]
struct SpeedTextMarker;

/// Marker for the buffs text element
#[derive(Component, Default)]
struct BuffsTextMarker;

fn add_hover_observers_to_units(
    q_main_mesh: Query<(Entity, &BelongTo), Added<MainMesh>>,
    q_unit: Query<&Unit>,
    mut commands: Commands,
) {
    for (main_mesh_entity, belong_to) in &q_main_mesh {
        // Only add hover to units
        let Ok(_unit) = q_unit.get(belong_to.0) else {
            continue;
        };
        commands
            .entity(main_mesh_entity)
            .insert(Pickable::default())
            .observe(on_hover_start)
            .observe(on_hover_end);
    }
}

fn on_hover_start(
    trigger: On<Pointer<Over>>,
    q_main_mesh: Query<&BelongTo, With<MainMesh>>,
    q_health: Query<&Health>,
    q_existing_panel: Query<&PanelForUnit, With<UnitHealthPanel>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    palette: Res<ColorPalette>,
) {
    if mouse_input.pressed(MouseButton::Left) {
        return;
    }
    let Ok(belong_to) = q_main_mesh.get(trigger.entity) else {
        return;
    };

    let unit_entity = belong_to.0;

    // Check if panel already exists for this unit
    for panel_for in &q_existing_panel {
        if panel_for.0 == unit_entity {
            return;
        }
    }

    // Only spawn if unit has health
    if q_health.get(unit_entity).is_err() {
        return;
    }

    // Spawn the health panel anchored to the unit
    let font = asset_server.load("fonts/Quicksand-Regular.ttf");

    commands
        .spawn((
            UnitHealthPanel,
            PanelForUnit(unit_entity),
            Node {
                flex_direction: FlexDirection::Column,
                padding: UiRect::axes(Px(12.0), Px(8.0)), // HIG: 8pt grid (horizontal 12pt, vertical 8pt)
                border: UiRect::all(Px(1.0)),
                border_radius: BorderRadius::all(Px(12.0)), // HIG: 12pt for small UI panels
                row_gap: Px(4.0), // HIG: 4pt micro spacing (was 2pt, bumped up slightly)
                ..default()
            },
            BackgroundColor(palette.blue_darkest.with_alpha(0.92)),
            BorderColor::all(palette.blue_medium.with_alpha(0.55)),
            Name::new("Unit Health Panel"),
            AnchorUiNode::to_entity(unit_entity),
            AnchorUiConfig {
                anchorpoint: AnchorPoint::bottommid(),
                offset: Some(Vec3::new(0.0, 40.0, 0.0)), // 40pt = 5x8pt (good)
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            // Unit name text
            parent.spawn((
                Text::new("Unit"),
                TextFont {
                    font: font.clone(),
                    font_size: 13.0, // HIG: 13pt Footnote (already compliant!)
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.8, 0.0)), // #ffcc00
                UnitNameMarker,
            ));

            // Health text
            parent.spawn((
                Text::new("HP: 0/0"),
                TextFont {
                    font: font.clone(),
                    font_size: 11.0, // HIG: 11pt Caption (already compliant!)
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.42, 0.42)), // #ff6b6b
                HealthTextMarker,
            ));

            // Stats row (damage and defense)
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Px(8.0), // HIG: 8pt spacing (already good!)
                    ..default()
                })
                .with_children(|stats_parent| {
                    // Damage text
                    stats_parent.spawn((
                        Text::new("ATK: 0"),
                        TextFont {
                            font: font.clone(),
                            font_size: 11.0, // HIG: 11pt Caption (minimum readable, was 10pt)
                            ..default()
                        },
                        TextColor(Color::srgb(1.0, 0.6, 0.4)), // #ff9966
                        DamageTextMarker,
                    ));

                    // Defense text
                    stats_parent.spawn((
                        Text::new("DEF: 0"),
                        TextFont {
                            font: font.clone(),
                            font_size: 11.0, // HIG: 11pt Caption (minimum readable, was 10pt)
                            ..default()
                        },
                        TextColor(Color::srgb(0.4, 0.8, 1.0)), // #66ccff
                        DefenseTextMarker,
                    ));
                });

            // Speed text
            parent.spawn((
                Text::new("SPD: 0"),
                TextFont {
                    font: font.clone(),
                    font_size: 11.0, // HIG: 11pt Caption (minimum readable, was 10pt)
                    ..default()
                },
                TextColor(Color::srgb(0.6, 1.0, 0.6)), // #99ff99
                SpeedTextMarker,
            ));

            // Buffs text
            parent.spawn((
                Text::new("-"),
                TextFont {
                    font,
                    font_size: 11.0, // HIG: 11pt Caption (minimum readable, was 10pt)
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.6, 1.0)), // #cc99ff
                BuffsTextMarker,
            ));
        });
}

/// Update panel text when panels are created
fn update_panel_text(
    q_new_panels: Query<&PanelForUnit, Added<UnitHealthPanel>>,
    q_unit_data: Query<(&Health, &UnitGameName, &UnitStats)>,
    mut q_health_text: Query<&mut Text, With<HealthTextMarker>>,
    mut q_name_text: Query<&mut Text, (With<UnitNameMarker>, Without<HealthTextMarker>)>,
    mut q_damage_text: Query<
        &mut Text,
        (
            With<DamageTextMarker>,
            Without<HealthTextMarker>,
            Without<UnitNameMarker>,
        ),
    >,
    mut q_defense_text: Query<
        &mut Text,
        (
            With<DefenseTextMarker>,
            Without<HealthTextMarker>,
            Without<UnitNameMarker>,
            Without<DamageTextMarker>,
        ),
    >,
    mut q_speed_text: Query<
        &mut Text,
        (
            With<SpeedTextMarker>,
            Without<HealthTextMarker>,
            Without<UnitNameMarker>,
            Without<DamageTextMarker>,
            Without<DefenseTextMarker>,
        ),
    >,
) {
    for panel_for in &q_new_panels {
        let Ok((health, name, stats)) = q_unit_data.get(panel_for.0) else {
            continue;
        };

        // Find all text entities in panel's children
        let panel_entity = q_new_panels.iter().find(|p| p.0 == panel_for.0);
        if panel_entity.is_none() {
            continue;
        }

        // We need to traverse the hierarchy to find text entities
        // This is simpler - just update all matching text entities
        for mut text in &mut q_health_text {
            **text = format!("HP: {:.0}/{:.0}", health.get_current(), health.get_max());
        }

        for mut text in &mut q_name_text {
            **text = name.0.to_string();
        }

        for mut text in &mut q_damage_text {
            **text = format!("ATK: {:.0}", stats.damage);
        }

        for mut text in &mut q_defense_text {
            **text = format!("DEF: {:.0}", stats.defense);
        }

        for mut text in &mut q_speed_text {
            **text = format!("SPD: {:.0}", stats.speed);
        }
    }
}

/// Update buffs text every frame (buffs can change dynamically)
fn update_buffs_text(
    q_panels: Query<&PanelForUnit, With<UnitHealthPanel>>,
    q_buffs: Query<&ActiveBuffs>,
    mut q_buffs_text: Query<&mut Text, With<BuffsTextMarker>>,
) {
    for panel_for in &q_panels {
        // Get the buffs for this unit
        let buff_text = if let Ok(buffs) = q_buffs.get(panel_for.0) {
            format_buffs(&buffs.list)
        } else {
            String::new()
        };

        // Update all buffs text markers
        // In practice, each panel will only have one buffs text child
        for mut text in &mut q_buffs_text {
            **text = buff_text.clone();
        }
    }
}

fn format_buffs(buffs: &[BuffEffect]) -> String {
    if buffs.is_empty() {
        return String::new();
    }

    let buff_strs: Vec<String> = buffs
        .iter()
        .map(|buff| match buff {
            BuffEffect::Poison(data) => {
                // Poison deals stacks damage per tick, then decrements
                // Total damage = n + (n-1) + (n-2) + ... + 1 = n*(n+1)/2
                let total_damage = data.stacks * (data.stacks + 1) / 2;
                format!(
                    "Poison x{} ({} total dmg, {}\u{2192}1/sec)",
                    data.stacks, total_damage, data.stacks
                )
            }
            BuffEffect::Block(data) => {
                let block_chance = data.current_stacks as f32 * 5.0;
                format!(
                    "Block {}/{} ({:.0}% chance)",
                    data.current_stacks, data.max_stacks, block_chance
                )
            }
            BuffEffect::Stun(data) => {
                let stun_chance = data.current_stacks as f32 * 5.0;
                format!(
                    "Stun {}/{} ({:.0}% chance)",
                    data.current_stacks, data.max_stacks, stun_chance
                )
            }
            BuffEffect::AttackSpeed(data) => {
                let speed_bonus = data.stacks as f32 * 5.0;
                format!(
                    "AtkSpd {}/{} ({:.0}% faster)",
                    data.stacks, data.max_stacks, speed_bonus
                )
            }
            BuffEffect::Invincible => "Invincible".to_string(),
        })
        .collect();

    format!("Buffs: {}", buff_strs.join(" | "))
}

fn on_hover_end(
    trigger: On<Pointer<Out>>,
    q_main_mesh: Query<&BelongTo, With<MainMesh>>,
    q_panel: Query<(Entity, &PanelForUnit), With<UnitHealthPanel>>,
    mut commands: Commands,
) {
    let Ok(belong_to) = q_main_mesh.get(trigger.entity) else {
        return;
    };

    let unit_entity = belong_to.0;

    // Find and despawn the panel for this unit
    for (panel_entity, panel_for) in &q_panel {
        if panel_for.0 == unit_entity {
            commands.entity(panel_entity).despawn();
        }
    }
}
