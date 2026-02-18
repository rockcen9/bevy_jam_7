use crate::{game_manager::camera::MainCamera, prelude::*};

use super::root::{BattleRootNode, BattleUiSets};

#[derive(Component)]
struct CombatMessageContainerMarker;

#[derive(Component)]
struct CombatMessageTextMarker;

#[derive(Component)]
struct CombatMessageFadeOut {
    timer: Timer,
}

pub(crate) fn plugin(app: &mut bevy::app::App) {
    app.add_systems(
        OnEnter(GameState::Battle),
        spawn_bottom_right_ui.in_set(BattleUiSets::SpawnChildren),
    )
    .add_systems(
        Update,
        (display_combat_message, fade_out_combat_message).run_if(in_state(GameState::Battle)),
    );
}

fn spawn_bottom_right_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    q_root: Query<Entity, With<BattleRootNode>>,
) {
    let Ok(root_entity) = q_root.single() else {
        warn!("BattleRootNode not found, cannot spawn bottom_right UI");
        return;
    };

    let font = asset_server.load("fonts/Quicksand-Regular.ttf");

    commands.entity(root_entity).with_children(|parent| {
        // Combat message container (initially hidden)
        parent
            .spawn((
                Node {
                    position_type: PositionType::Absolute,
                    bottom: px(96),
                    right: px(20),
                    display: Display::None, // Initially hidden
                    padding: UiRect::axes(px(32), px(24)),
                    border: UiRect::all(px(4)),
                    border_radius: BorderRadius::all(px(12)),
                    min_width: px(256),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgba(0.102, 0.102, 0.180, 0.90)), // #1a1a2eE6
                BorderColor::all(Color::srgb(0.290, 0.290, 0.416)),       // #4a4a6a
                CombatMessageContainerMarker,
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new("..."),
                    TextFont {
                        font: font.clone(),
                        font_size: 34.0,
                        ..default()
                    },
                    TextColor(Color::srgb(1.0, 0.843, 0.0)), // #ffd700 gold
                    CombatMessageTextMarker,
                ));
            });
    });
}

fn display_combat_message(
    mut ev_combat: MessageReader<CombatMessage>,
    mut q_container: Query<(Entity, &mut Node), With<CombatMessageContainerMarker>>,
    mut q_text: Query<&mut Text, With<CombatMessageTextMarker>>,
    mut commands: Commands,
    q_fadeout: Query<Entity, With<CombatMessageFadeOut>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    q_window: Query<&Window>,
) {
    for msg in ev_combat.read() {
        // Only show messages that affect the player
        // Player is source = good for player, Enemy is source = bad for player
        let is_player_affected = msg.source == Faction::Player || msg.source == Faction::Enemy;
        if !is_player_affected {
            continue;
        }

        let (text, _color) = if msg.source == Faction::Player {
            // Player did something good
            let event_name = msg.trigger.to_positive();
            (format!("{:?}!", event_name), "#4ade80") // green
        } else {
            // Enemy did something (bad for player)
            let event_name = msg.trigger.to_negative();
            (format!("{:?}!", event_name), "#f87171") // red
        };

        // Update the text
        if let Ok(mut text_component) = q_text.single_mut() {
            **text_component = text.clone();
        }

        // Remove any existing fadeout timer
        for fadeout_entity in &q_fadeout {
            commands
                .entity(fadeout_entity)
                .remove::<CombatMessageFadeOut>();
        }

        // Show container and add fadeout timer
        if let Ok((entity, mut node)) = q_container.single_mut() {
            node.display = Display::Flex;
            commands.entity(entity).insert(CombatMessageFadeOut {
                timer: Timer::from_seconds(3.0, TimerMode::Once),
            });
        }

        // Trigger sparks at bottom-right corner (where message appears)
        if let (Ok((camera, camera_transform)), Ok(window)) = (q_camera.single(), q_window.single())
        {
            // Bottom-right screen position with some padding
            // 200 is at panel, 250 is at top of panel
            let screen_pos = Vec2::new(window.width() - 150.0, window.height() - 250.0);
            if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, screen_pos) {
                commands.trigger(VfxEvent::sparks(world_pos));
            }
        }
    }
}

fn fade_out_combat_message(
    time: Res<Time>,
    mut q_fadeout: Query<(Entity, &mut CombatMessageFadeOut, &mut Node)>,
    mut commands: Commands,
) {
    for (entity, mut fadeout, mut node) in &mut q_fadeout {
        fadeout.timer.tick(time.delta());

        if fadeout.timer.just_finished() {
            node.display = Display::None;
            commands.entity(entity).remove::<CombatMessageFadeOut>();
        }
    }
}
