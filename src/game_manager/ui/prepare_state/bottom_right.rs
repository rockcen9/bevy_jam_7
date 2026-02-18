use crate::prelude::*;
use bevy_tweening::{
    lens::{UiTransformRotationLens, UiTransformScaleLens},
    *,
};
use std::time::Duration;

use super::root::{PrepareRootNode, PrepareUiSets};

#[derive(Component)]
pub struct FightButtonMarker;

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(GameState::Preparing),
        spawn_bottom_right_ui.in_set(PrepareUiSets::SpawnChildren),
    )
    .add_systems(
        Update,
        (handle_fight_button, handle_fight_button_hover).run_if(in_state(GameState::Preparing)),
    );
}

/// Spawn the bottom right UI (fight button)
fn spawn_bottom_right_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    root_query: Query<Entity, With<PrepareRootNode>>,
    palette: Res<ColorPalette>,
) {
    let Ok(root_entity) = root_query.single() else {
        warn!("PrepareRootNode not found for bottom right UI");
        return;
    };

    let font = asset_server.load("fonts/Quicksand-Regular.ttf");

    commands.entity(root_entity).with_children(|parent| {
        // Bottom-right container
        parent
            .spawn(Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(20.0), // HIG: 20pt recommended margin
                right: Val::Px(20.0),  // HIG: 20pt recommended margin
                ..default()
            })
            .with_children(|parent| {
                // Fight button
                parent
                    .spawn((
                        Button,
                        Node {
                            padding: UiRect {
                                left: Val::Px(32.0), // 32pt is good (4x8pt)
                                right: Val::Px(32.0),
                                top: Val::Px(16.0), // HIG: 16pt padding
                                bottom: Val::Px(16.0),
                            },
                            border: UiRect::all(Val::Px(1.0)),
                            border_radius: BorderRadius::all(Val::Px(12.0)), // HIG: 10-14pt for buttons
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(palette.pink_dark),
                        BorderColor::all(palette.pink_medium.with_alpha(0.8)),
                        FightButtonMarker,
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            Text::new("Engage!"),
                            TextFont {
                                font: font.clone(),
                                font_size: 28.0, // HIG: 28pt Title 1 (already correct!)
                                ..default()
                            },
                            TextColor(palette.get(UiColorName::FightButtonText)),
                        ));
                    });
            });
    });
}

fn handle_fight_button(
    button_q: Query<(Entity, &Interaction), (Changed<Interaction>, With<FightButtonMarker>)>,
    player_units: Query<(), With<PlayerUnit>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
) {
    for (entity, interaction) in button_q.iter() {
        if *interaction == Interaction::Pressed {
            if player_units.is_empty() {
                spawn_click_wiggle_animation(&mut commands, entity);
                commands.trigger(SFXEvent::ui("invalid"));
                continue;
            }
            game_state.set(GameState::Battle);
            commands.trigger(SFXEvent::ui("put"));
        }
    }
}

fn handle_fight_button_hover(
    mut commands: Commands,
    button_q: Query<
        (Entity, &Interaction, &UiTransform),
        (Changed<Interaction>, With<FightButtonMarker>),
    >,
) {
    for (entity, interaction, transform) in button_q.iter() {
        match *interaction {
            Interaction::Hovered => {
                let tween = Tween::new(
                    EaseFunction::QuadraticIn,
                    Duration::from_millis(200),
                    UiTransformScaleLens {
                        start: transform.scale,
                        end: Vec2::splat(1.1),
                    },
                );
                // Spawn separate animation entity to avoid overwriting rotation
                commands.spawn((
                    TweenAnim::new(tween),
                    AnimTarget::component::<UiTransform>(entity),
                ));
            }
            Interaction::None => {
                let tween = Tween::new(
                    EaseFunction::QuadraticIn,
                    Duration::from_millis(200),
                    UiTransformScaleLens {
                        start: transform.scale,
                        end: Vec2::ONE,
                    },
                );
                // Spawn separate animation entity to avoid overwriting rotation
                commands.spawn((
                    TweenAnim::new(tween),
                    AnimTarget::component::<UiTransform>(entity),
                ));
            }
            _ => {}
        }
    }
}

fn spawn_click_wiggle_animation(commands: &mut Commands, entity: Entity) {
    // Create a wiggle animation: 0 -> -30deg -> +30deg -> 0
    // Total duration: 200ms
    let degrees_30 = 30_f32.to_radians();

    let tween1 = Tween::new(
        EaseFunction::QuadraticOut,
        Duration::from_millis(50),
        UiTransformRotationLens {
            start: Rot2::radians(0.0),
            end: Rot2::radians(-degrees_30),
        },
    );

    let tween2 = Tween::new(
        EaseFunction::QuadraticInOut,
        Duration::from_millis(100),
        UiTransformRotationLens {
            start: Rot2::radians(-degrees_30),
            end: Rot2::radians(degrees_30),
        },
    );

    let tween3 = Tween::new(
        EaseFunction::QuadraticIn,
        Duration::from_millis(50),
        UiTransformRotationLens {
            start: Rot2::radians(degrees_30),
            end: Rot2::radians(0.0),
        },
    );

    // Chain the tweens into a sequence
    let sequence = tween1.then(tween2).then(tween3);

    // Spawn separate animation entity to avoid being overwritten by scale animation
    commands.spawn((
        TweenAnim::new(sequence),
        AnimTarget::component::<UiTransform>(entity),
    ));
}
