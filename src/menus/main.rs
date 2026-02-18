//! The main menu (seen on the title screen).

use bevy::prelude::*;
use bevy::ui::{BackgroundGradient, ColorStop, LinearGradient, ui_transform::UiTransform};
use bevy_tweening::{lens::UiTransformTranslationPxLens, *};
use std::time::Duration;

use crate::prelude::*;
use crate::{menus::Menu, screens::Screen, theme::widget};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Main), spawn_main_menu);
    app.add_systems(
        Update,
        handle_main_menu_button_hover.run_if(in_state(Menu::Main)),
    );
}

fn spawn_main_menu(
    mut commands: Commands,
    palette: Res<ColorPalette>,
    progress: Res<GameProgress>,
    asset_server: Res<AssetServer>,
) {
    let play_label = format!("Night {}", progress.current_round);
    let play_label = if progress.current_round > 1 {
        format!("{} Rewind", play_label)
    } else {
        play_label
    };
    commands.spawn((
        widget::ui_root("Main Menu"),
        BackgroundGradient::from(LinearGradient {
            angle: LinearGradient::TO_BOTTOM,
            stops: vec![
                // Slightly lighter brown_dark at top (#867b85 = brown_dark lightened 25%)
                ColorStop::auto(Color::srgb_u8(0x86, 0x7b, 0x85)),
                // Pure brown_dark at bottom
                ColorStop::auto(palette.brown_dark),
            ],
            ..default()
        }),
        GlobalZIndex(2),
        DespawnOnExit(Menu::Main),
        #[cfg(not(target_family = "wasm"))]
        children![
            widget::button(play_label, enter_loading_screen, &palette, &asset_server),
            widget::button("Settings", open_settings_menu, &palette, &asset_server),
            widget::button("Credits", open_credits_menu, &palette, &asset_server),
            widget::button("Exit", exit_app, &palette, &asset_server),
        ],
        #[cfg(target_family = "wasm")]
        children![
            widget::button(play_label, enter_loading_screen, &palette, &asset_server),
            widget::button("Settings", open_settings_menu, &palette, &asset_server),
            widget::button("Credits", open_credits_menu, &palette, &asset_server),
        ],
    ));
}

fn enter_loading_screen(_on: On<Pointer<Click>>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Loading);
}

fn open_settings_menu(_: On<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Settings);
}

fn open_credits_menu(_: On<Pointer<Click>>) {
    #[cfg(feature = "backend")]
    {
        let url = "https://rockcen.itch.io/just-let-me-sleep";
        if let Err(e) = webbrowser::open(url) {
            error!("Couldn't open browser: {}", e);
        }
    }
}

#[cfg(not(target_family = "wasm"))]
fn exit_app(_: On<Pointer<Click>>, mut app_exit: MessageWriter<AppExit>) {
    app_exit.write(AppExit::Success);
}

fn handle_main_menu_button_hover(
    mut commands: Commands,
    button_query: Query<(Entity, &Interaction, &UiTransform), (Changed<Interaction>, With<Button>)>,
) {
    const LIFT_DISTANCE: f32 = -6.0; // Negative Y means up in UI coordinates
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
