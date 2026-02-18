//! Helper functions for creating common widgets.

use std::borrow::Cow;

use bevy::{
    ecs::{spawn::SpawnWith, system::IntoObserverSystem},
    prelude::*,
    ui::{ui_transform::UiTransform, Val::*},
};

use crate::{theme::prelude::InteractionPalette, *};

/// A root UI node that fills the window and centers its content.
pub(crate) fn ui_root(name: impl Into<Cow<'static, str>>) -> impl Bundle {
    (
        Name::new(name),
        Node {
            position_type: PositionType::Absolute,
            width: Percent(100.0),
            height: Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Px(16.0),
            ..default()
        },
        // Don't block picking events for other UI roots.
        Pickable::IGNORE,
    )
}

/// A simple header label. Bigger than [`label`].
pub(crate) fn header(
    text: impl Into<String>,
    palette: &ColorPalette,
    asset_server: &AssetServer,
) -> impl Bundle {
    (
        Name::new("Header"),
        Text(text.into()),
        TextFont {
            font: asset_server.load("fonts/Quicksand-Regular.ttf"),
            font_size: 34.0,
            ..default()
        },
        TextColor(palette.get(UiColorName::HeaderText)),
    )
}

/// A simple text label.
pub(crate) fn label(
    text: impl Into<String>,
    palette: &ColorPalette,
    asset_server: &AssetServer,
) -> impl Bundle {
    label_base(text, 22.0, palette, asset_server)
}

pub(crate) fn label_small(
    text: impl Into<String>,
    palette: &ColorPalette,
    asset_server: &AssetServer,
) -> impl Bundle {
    label_base(text, 13.0, palette, asset_server)
}

/// A simple text label.
fn label_base(
    text: impl Into<String>,
    font_size: f32,
    palette: &ColorPalette,
    asset_server: &AssetServer,
) -> impl Bundle {
    (
        Name::new("Label"),
        Text(text.into()),
        TextFont {
            font: asset_server.load("fonts/Quicksand-Regular.ttf"),
            font_size,
            ..default()
        },
        TextColor(palette.get(UiColorName::LabelText)),
    )
}

/// A large rounded button with text and an action defined as an [`Observer`].
pub(crate) fn button<E, B, M, I>(
    text: impl Into<String>,
    action: I,
    palette: &ColorPalette,
    asset_server: &AssetServer,
) -> impl Bundle
where
    E: EntityEvent,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    button_base(
        text,
        action,
        Node {
            width: px(380),
            height: px(56),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            border_radius: BorderRadius::MAX,
            ..default()
        },
        24.0,
        palette,
        asset_server,
    )
}

/// A small square button with text and an action defined as an [`Observer`].
pub(crate) fn button_small<E, B, M, I>(
    text: impl Into<String>,
    action: I,
    palette: &ColorPalette,
    asset_server: &AssetServer,
) -> impl Bundle
where
    E: EntityEvent,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    button_base(
        text,
        action,
        Node {
            width: Px(44.0),
            height: Px(44.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            border_radius: BorderRadius::all(Px(8.0)),
            ..default()
        },
        20.0,
        palette,
        asset_server,
    )
}

/// A simple button with text and an action defined as an [`Observer`]. The button's layout is provided by `button_bundle`.
fn button_base<E, B, M, I>(
    text: impl Into<String>,
    action: I,
    button_bundle: impl Bundle,
    font_size: f32,
    palette: &ColorPalette,
    asset_server: &AssetServer,
) -> impl Bundle
where
    E: EntityEvent,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    let text = text.into();
    let action = IntoObserverSystem::into_system(action);
    let button_bg = palette.get(UiColorName::ButtonBackground);
    let button_hovered = palette.get(UiColorName::ButtonHoveredBackground);
    let button_pressed = palette.get(UiColorName::ButtonPressedBackground);
    let button_text = palette.get(UiColorName::ButtonText);
    let font = asset_server.load("fonts/Quicksand-Regular.ttf");

    (
        Name::new("Button"),
        Node::default(),
        Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
            parent
                .spawn((
                    Name::new("Button Inner"),
                    Button,
                    BackgroundColor(button_bg),
                    UiTransform::default(),
                    InteractionPalette {
                        none: button_bg,
                        hovered: button_hovered,
                        pressed: button_pressed,
                    },
                    children![(
                        Name::new("Button Text"),
                        Text(text.clone()),
                        TextFont {
                            font: font.clone(),
                            font_size,
                            ..default()
                        },
                        TextColor(button_text),
                        // Don't bubble picking events from the text up to the button.
                        Pickable::IGNORE,
                    )],
                ))
                .insert(button_bundle)
                .observe(action);
        })),
    )
}

pub(crate) fn plus_minus_bar<E, B, M, I1, I2>(
    label_marker: impl Component,
    lower: I1,
    raise: I2,
    palette: &ColorPalette,
    asset_server: &AssetServer,
) -> impl Bundle
where
    E: EntityEvent,
    B: Bundle,
    I1: IntoObserverSystem<E, B, M>,
    I2: IntoObserverSystem<E, B, M>,
{
    (
        Node {
            justify_self: JustifySelf::Start,
            ..default()
        },
        children![
            button_small("-", lower, palette, asset_server),
            button_small("+", raise, palette, asset_server),
            (
                Node {
                    padding: UiRect::horizontal(Px(10.0)),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                children![(label("", palette, asset_server), label_marker)],
            ),
        ],
    )
}
