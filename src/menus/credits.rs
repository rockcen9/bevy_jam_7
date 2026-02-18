//! A credits menu.

use crate::prelude::*;
use crate::{Pause, menus::Menu, theme::prelude::*};
use bevy::{
    ecs::spawn::SpawnIter,
    input::common_conditions::input_just_pressed,
    prelude::*,
    ui::{Val::*, ui_transform::UiTransform},
};
use bevy_tweening::{lens::UiTransformTranslationPxLens, *};
use std::time::Duration;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Credits), spawn_credits_menu);
    app.add_systems(
        Update,
        (
            go_back.run_if(input_just_pressed(KeyCode::Escape)),
            handle_credits_menu_button_hover,
        )
            .run_if(in_state(Menu::Credits)),
    );
}

fn spawn_credits_menu(
    mut commands: Commands,
    paused: Res<State<Pause>>,
    palette: Res<ColorPalette>,
    asset_server: Res<AssetServer>,
) {
    let mut entity_commands = commands.spawn((
        widget::ui_root("Credits Screen"),
        DespawnOnExit(Menu::Credits),
        GlobalZIndex(2),
        children![
            widget::header("Created by", &palette, &asset_server),
            created_by(&palette, &asset_server),
            widget::header("Assets", &palette, &asset_server),
            assets(&palette, &asset_server),
            widget::button(
                "View on itch.io",
                open_credits_link,
                &palette,
                &asset_server
            ),
            widget::button("Back", go_back_on_click, &palette, &asset_server),
        ],
    ));
    if paused.get() == &Pause(false) {
        entity_commands.insert(BackgroundColor(palette.get(UiColorName::ScreenBackground)));
    }
}

fn created_by(palette: &ColorPalette, asset_server: &AssetServer) -> impl Bundle {
    grid(
        vec![
            ["Joe Shmoe", "Implemented alligator wrestling AI"],
            ["Jane Doe", "Made the music for the alien invasion"],
        ],
        palette,
        asset_server,
    )
}

fn assets(palette: &ColorPalette, asset_server: &AssetServer) -> impl Bundle {
    grid(
        vec![
            [
                "Bevy logo",
                "All rights reserved by the Bevy Foundation, permission granted for splash screen use when unmodified",
            ],
            ["Button SFX", "CC0 by Jaszunio15"],
            ["Music", "CC BY 3.0 by Kevin MacLeod"],
            ["Ambient music and Footstep SFX", "CC0 by NOX SOUND"],
            [
                "Throw SFX",
                "FilmCow Royalty Free SFX Library License Agreement by Jason Steele",
            ],
            [
                "Fox model",
                "CC0 1.0 Universal by PixelMannen (model), CC BY 4.0 International by tomkranis (Rigging & Animation), CC BY 4.0 International by AsoboStudio and scurest (Conversion to glTF)",
            ],
            [
                "Player model",
                "You can use it commercially without the need to credit me by Drillimpact",
            ],
            ["Vocals", "CC BY 4.0 by Dillon Becker"],
            ["Night Sky HDRI 001", "CC0 by ambientCG"],
            [
                "Rest of the assets",
                "CC BY-NC-SA 3.0 by The Dark Mod Team, converted to Bevy-friendly assets by Jan Hohenheim",
            ],
        ],
        palette,
        asset_server,
    )
}

fn grid(
    content: Vec<[&'static str; 2]>,
    palette: &ColorPalette,
    asset_server: &AssetServer,
) -> impl Bundle {
    let label_color = palette.get(UiColorName::LabelText);
    let font = asset_server.load("fonts/Quicksand-Regular.ttf");
    (
        Name::new("Grid"),
        Node {
            display: Display::Grid,
            row_gap: Px(10.0),
            column_gap: Px(30.0),
            grid_template_columns: RepeatedGridTrack::px(2, 400.0),
            ..default()
        },
        Children::spawn(SpawnIter(content.into_iter().flatten().enumerate().map(
            move |(i, text)| {
                (
                    Name::new("Label"),
                    Text(text.to_string()),
                    TextFont {
                        font: font.clone(),
                        font_size: 12.0,
                        ..default()
                    },
                    TextColor(label_color),
                    Node {
                        justify_self: if i % 2 == 0 {
                            JustifySelf::End
                        } else {
                            JustifySelf::Start
                        },
                        ..default()
                    },
                )
            },
        ))),
    )
}

fn go_back_on_click(_: On<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Main);
}

fn go_back(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Main);
}

fn open_credits_link(_: On<Pointer<Click>>) {
    #[cfg(feature = "backend")]
    {
        let url = "https://rockcen.itch.io/just-let-me-sleep";
        if let Err(e) = webbrowser::open(url) {
            error!("Couldn't open browser: {}", e);
        }
    }
}

fn handle_credits_menu_button_hover(
    mut commands: Commands,
    button_query: Query<(Entity, &Interaction, &UiTransform), (Changed<Interaction>, With<Button>)>,
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
