//! The pause menu.

use crate::theme::widget;
use crate::*;
use crate::{menus::Menu, screens::Screen};
use bevy::{input::common_conditions::input_just_pressed, prelude::*, ui::ui_transform::UiTransform};
use bevy_tweening::{lens::UiTransformTranslationPxLens, *};
use std::time::Duration;
pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Pause), spawn_pause_menu);
    app.add_systems(
        Update,
        (
            go_back.run_if(input_just_pressed(KeyCode::Escape)),
            handle_pause_menu_button_hover,
        )
            .run_if(in_state(Menu::Pause)),
    );
}

fn spawn_pause_menu(
    mut commands: Commands,
    mut time: ResMut<Time<Virtual>>,
    palette: Res<ColorPalette>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        widget::ui_root("Pause Menu"),
        GlobalZIndex(2),
        DespawnOnExit(Menu::Pause),
        children![
            widget::header("Game paused", &palette, &asset_server),
            widget::button("Continue", close_menu, &palette, &asset_server),
            widget::button("Settings", open_settings_menu, &palette, &asset_server),
            widget::button("Quit to title", quit_to_title, &palette, &asset_server),
        ],
    ));

    time.pause();
}

fn open_settings_menu(_on: On<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Settings);
}

fn close_menu(
    _on: On<Pointer<Click>>,
    mut next_menu: ResMut<NextState<Menu>>,
    mut time: ResMut<Time<Virtual>>,
) {
    next_menu.set(Menu::None);

    time.unpause();
}

fn quit_to_title(_on: On<Pointer<Click>>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}

fn go_back(mut next_menu: ResMut<NextState<Menu>>, mut time: ResMut<Time<Virtual>>) {
    next_menu.set(Menu::None);
    time.unpause();
}

fn handle_pause_menu_button_hover(
    mut commands: Commands,
    button_query: Query<
        (Entity, &Interaction, &UiTransform),
        (Changed<Interaction>, With<Button>),
    >,
) {
    const LIFT_DISTANCE: f32 = -6.0;
    const ANIMATION_DURATION_MS: u64 = 150;

    for (entity, interaction, _transform) in &button_query {
        match *interaction {
            Interaction::Hovered => {
                let tween = Tween::new(
                    EaseFunction::QuadraticOut,
                    Duration::from_millis(ANIMATION_DURATION_MS),
                    UiTransformTranslationPxLens {
                        start: Vec2::ZERO,
                        end: Vec2::new(0.0, LIFT_DISTANCE),
                    },
                );
                commands.spawn((
                    TweenAnim::new(tween),
                    AnimTarget::component::<UiTransform>(entity),
                ));
            }
            Interaction::None | Interaction::Pressed => {
                let tween = Tween::new(
                    EaseFunction::QuadraticIn,
                    Duration::from_millis(ANIMATION_DURATION_MS),
                    UiTransformTranslationPxLens {
                        start: Vec2::new(0.0, LIFT_DISTANCE),
                        end: Vec2::ZERO,
                    },
                );
                commands.spawn((
                    TweenAnim::new(tween),
                    AnimTarget::component::<UiTransform>(entity),
                ));
            }
        }
    }
}
