//! The settings screen accessible from the title screen.
//! We can add all manner of settings and accessibility options here.
//! For 3D, we'd also place the camera sensitivity and FOV here.

use bevy::window::PresentMode;
use bevy::{input::common_conditions::input_just_pressed, prelude::*, ui::{ui_transform::UiTransform, Val::*}};
use bevy_framepace::{FramepaceSettings, Limiter};
#[cfg(feature = "backend")]
use bevy_seedling::prelude::*;
use bevy_tweening::{lens::UiTransformTranslationPxLens, *};
use std::time::Duration;

use crate::prelude::*;
use crate::{
    Pause,
    menus::Menu,
    screens::Screen,
    theme::prelude::*,
};
#[cfg(feature = "backend")]
use crate::game_manager::audio::{DEFAULT_MAIN_VOLUME, perceptual::PerceptualVolumeConverter};

pub(super) fn plugin(app: &mut App) {
    #[cfg(feature = "backend")]
    app.init_resource::<VolumeSliderSettings>();
    app.init_resource::<VsyncSetting>();
    app.init_resource::<FpsLimiterSettings>();
    app.add_systems(OnEnter(Menu::Settings), spawn_settings_menu);
    app.add_systems(
        Update,
        go_back.run_if(in_state(Menu::Settings).and(input_just_pressed(KeyCode::Escape))),
    );
    #[cfg(feature = "backend")]
    app.add_systems(
        Update,
        (
            update_global_volume.run_if(resource_exists_and_changed::<VolumeSliderSettings>),
            update_volume_label,
        )
            .run_if(in_state(Menu::Settings)),
    );
    app.add_systems(
        Update,
        (
            update_vsync.run_if(resource_exists_and_changed::<VsyncSetting>),
            update_vsync_label,
            update_fps_limiter.run_if(resource_exists_and_changed::<FpsLimiterSettings>),
            update_fps_limiter_enabled_label,
            update_fps_limiter_target_label,
            handle_settings_menu_button_hover,
        )
            .run_if(in_state(Menu::Settings)),
    );
}

fn spawn_settings_menu(
    mut commands: Commands,
    paused: Res<State<Pause>>,
    palette: Res<ColorPalette>,
    asset_server: Res<AssetServer>,
) {
    let mut entity_commands = commands.spawn((
        widget::ui_root("Settings Screen"),
        DespawnOnExit(Menu::Settings),
        GlobalZIndex(2),
        children![
            widget::header("Settings", &palette, &asset_server),
            (
                Name::new("Settings Grid"),
                Node {
                    display: Display::Grid,
                    row_gap: Px(10.0),
                    column_gap: Px(30.0),
                    grid_template_columns: RepeatedGridTrack::px(2, 400.0),
                    ..default()
                },
                children![
                    // Audio
                    (
                        widget::label("Audio Volume", &palette, &asset_server),
                        Node {
                            justify_self: JustifySelf::End,
                            ..default()
                        }
                    ),
                    widget::plus_minus_bar(GlobalVolumeLabel, lower_volume, raise_volume, &palette, &asset_server),
                    // Camera Sensitivity
                    (
                        widget::label("Camera Sensitivity", &palette, &asset_server),
                        Node {
                            justify_self: JustifySelf::End,
                            ..default()
                        }
                    ),
                    // Camera FOV
                    (
                        widget::label("Camera FOV", &palette, &asset_server),
                        Node {
                            justify_self: JustifySelf::End,
                            ..default()
                        }
                    ),
                    // VSync
                    (
                        widget::label("VSync", &palette, &asset_server),
                        Node {
                            justify_self: JustifySelf::End,
                            ..default()
                        }
                    ),
                    widget::plus_minus_bar(VsyncLabel, disable_vsync, enable_vsync, &palette, &asset_server),
                    // FPS Limiter (Enable/Disable)
                    (
                        widget::label("FPS Limiter", &palette, &asset_server),
                        Node {
                            justify_self: JustifySelf::End,
                            ..default()
                        }
                    ),
                    widget::plus_minus_bar(
                        FpsLimiterEnabledLabel,
                        disable_fps_limiter,
                        enable_fps_limiter,
                        &palette,
                        &asset_server
                    ),
                    // FPS Target
                    (
                        widget::label("FPS Target", &palette, &asset_server),
                        Node {
                            justify_self: JustifySelf::End,
                            ..default()
                        }
                    ),
                    widget::plus_minus_bar(
                        FpsLimiterTargetLabel,
                        lower_fps_target,
                        raise_fps_target,
                        &palette,
                        &asset_server
                    ),
                ],
            ),
            widget::button("Back", go_back_on_click, &palette, &asset_server),
        ],
    ));
    if paused.get() == &Pause(false) {
        entity_commands.insert(BackgroundColor(palette.get(UiColorName::ScreenBackground)));
    }
}

#[cfg(feature = "backend")]
#[derive(Resource, Reflect, Debug)]
struct VolumeSliderSettings(usize);

#[cfg(feature = "backend")]
impl VolumeSliderSettings {
    fn increment(&mut self) {
        self.0 = Self::MAX_TICK_COUNT.min(self.0 + 1);
    }

    fn decrement(&mut self) {
        self.0 = self.0.saturating_sub(1);
    }

    fn fraction(&self) -> f32 {
        self.0 as f32 / Self::MAX_TICK_COUNT as f32
    }

    /// How many ticks the volume slider supports
    const MAX_TICK_COUNT: usize = 20;
}

#[cfg(feature = "backend")]
impl Default for VolumeSliderSettings {
    fn default() -> Self {
        Self(
            (PerceptualVolumeConverter::default().to_perceptual(DEFAULT_MAIN_VOLUME)
                * Self::MAX_TICK_COUNT as f32)
                .round() as usize,
        )
    }
}

#[cfg(feature = "backend")]
fn update_global_volume(
    mut master: Single<&mut VolumeNode, With<MainBus>>,
    volume_step: Res<VolumeSliderSettings>,
) {
    master.volume = PerceptualVolumeConverter::default().to_volume(volume_step.fraction());
}

#[cfg(feature = "backend")]
fn lower_volume(_on: On<Pointer<Click>>, mut volume_step: ResMut<VolumeSliderSettings>) {
    volume_step.decrement();
}
#[cfg(not(feature = "backend"))]
fn lower_volume(_on: On<Pointer<Click>>) {}

#[cfg(feature = "backend")]
fn raise_volume(_on: On<Pointer<Click>>, mut volume_step: ResMut<VolumeSliderSettings>) {
    volume_step.increment();
}
#[cfg(not(feature = "backend"))]
fn raise_volume(_on: On<Pointer<Click>>) {}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct GlobalVolumeLabel;

#[cfg(feature = "backend")]
fn update_volume_label(
    mut label: Single<&mut Text, With<GlobalVolumeLabel>>,
    slider: Res<VolumeSliderSettings>,
) {
    let ticks = slider.0;
    let filled = "█".repeat(ticks);
    let empty = " ".repeat(VolumeSliderSettings::MAX_TICK_COUNT - ticks);
    let text = filled + &empty + "|";
    label.0 = text;
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct CameraSensitivityLabel;

#[derive(Resource, Reflect, Debug)]
struct VsyncSetting(bool);

impl Default for VsyncSetting {
    fn default() -> Self {
        Self(true)
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct VsyncLabel;

fn enable_vsync(_on: On<Pointer<Click>>, mut setting: ResMut<VsyncSetting>) {
    setting.0 = true;
}

fn disable_vsync(_on: On<Pointer<Click>>, mut setting: ResMut<VsyncSetting>) {
    setting.0 = false;
}

fn update_vsync(mut window: Single<&mut Window>, setting: Res<VsyncSetting>) {
    window.present_mode = if setting.0 {
        PresentMode::AutoVsync
    } else {
        PresentMode::AutoNoVsync
    };
}

fn update_vsync_label(mut label: Single<&mut Text, With<VsyncLabel>>, setting: Res<VsyncSetting>) {
    label.0 = if setting.0 { "On".into() } else { "Off".into() };
}

#[derive(Resource, Reflect, Debug)]
struct FpsLimiterSettings {
    enabled: bool,
    target_fps: u32,
}

impl Default for FpsLimiterSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            target_fps: 60,
        }
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct FpsLimiterEnabledLabel;

#[derive(Component, Reflect)]
#[reflect(Component)]
struct FpsLimiterTargetLabel;

fn enable_fps_limiter(
    _on: On<Pointer<Click>>,
    mut settings: ResMut<FpsLimiterSettings>,
    mut framepace: ResMut<FramepaceSettings>,
) {
    settings.enabled = true;
    framepace.limiter = Limiter::from_framerate(settings.target_fps as f64);
}

fn disable_fps_limiter(
    _on: On<Pointer<Click>>,
    mut settings: ResMut<FpsLimiterSettings>,
    mut framepace: ResMut<FramepaceSettings>,
) {
    settings.enabled = false;
    framepace.limiter = Limiter::Off;
}

fn lower_fps_target(_on: On<Pointer<Click>>, mut settings: ResMut<FpsLimiterSettings>) {
    let min_fps = 30;
    let step = 5;
    settings.target_fps = settings.target_fps.saturating_sub(step).max(min_fps);
}

fn raise_fps_target(_on: On<Pointer<Click>>, mut settings: ResMut<FpsLimiterSettings>) {
    let max_fps = 360;
    let step = 5;
    settings.target_fps = (settings.target_fps + step).min(max_fps);
}

fn update_fps_limiter(mut framepace: ResMut<FramepaceSettings>, settings: Res<FpsLimiterSettings>) {
    framepace.limiter = if settings.enabled {
        Limiter::from_framerate(settings.target_fps as f64)
    } else {
        Limiter::Off
    };
}

fn update_fps_limiter_enabled_label(
    mut label: Single<&mut Text, With<FpsLimiterEnabledLabel>>,
    settings: Res<FpsLimiterSettings>,
) {
    label.0 = if settings.enabled {
        "On".into()
    } else {
        "Off".into()
    };
}

fn update_fps_limiter_target_label(
    mut label: Single<&mut Text, With<FpsLimiterTargetLabel>>,
    settings: Res<FpsLimiterSettings>,
) {
    label.0 = format!("{}", settings.target_fps);
}

fn go_back_on_click(
    _on: On<Pointer<Click>>,
    screen: Res<State<Screen>>,
    mut next_menu: ResMut<NextState<Menu>>,
) {
    next_menu.set(if screen.get() == &Screen::Title {
        Menu::Main
    } else {
        Menu::Pause
    });
}

fn go_back(screen: Res<State<Screen>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(if screen.get() == &Screen::Title {
        Menu::Main
    } else {
        Menu::Pause
    });
}

fn handle_settings_menu_button_hover(
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
