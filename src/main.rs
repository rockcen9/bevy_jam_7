// Disable console on Windows for non-dev builds.
#![cfg_attr(feature = "release", windows_subsystem = "windows")]

mod animation;
mod asset_processing;
mod asset_tracking;
mod config;
mod dbg;
#[cfg(feature = "dev")]
mod dev_tools;
mod menus;
mod prelude;
mod screens;
mod theme;
pub use config::*;
mod palette;
mod sprite_layer;
mod third_party;
mod ui_camera;
use crate::prelude::*;
// mod dbg;

mod game_manager;

mod test_scene;
use bevy::ecs::error::error;
use bevy::log::LogPlugin;
use bevy::log::tracing_subscriber::field::MakeExt;

use bevy::asset::AssetMetaCheck;

#[cfg(all(feature = "native", feature = "web"))]
compile_error!(
    "Exactly one of the `native` or the `web` feature must be active at the same time. Instead, both are currently enabled."
);
#[cfg(not(any(feature = "native", feature = "web")))]
compile_error!(
    "Exactly one of the `native` or the `web` feature must be active at the same time. Instead, both are currently disabled."
);
#[cfg(all(feature = "dev", feature = "release"))]
compile_error!(
    "Exactly one of the `dev` or the `release` feature must be active at the same time. Instead, both are currently enabled."
);
#[cfg(not(any(feature = "dev", feature = "release")))]
compile_error!(
    "Exactly one of the `dev` or the `release` feature must be active at the same time. Instead, both are currently disabled."
);

fn main() -> AppExit {
    #[cfg(feature = "dev_native")]
    {
        if let Ok(_) = dotenvy::dotenv() {
            println!("Feature dev_mode enabled, .env loaded.");
        }
    }
    let mut app = App::new();
    // Don't panic on Bevy system errors, just log them.
    app.set_error_handler(error);

    // Add Bevy plugins.
    app.add_plugins((DefaultPlugins
        .set(AssetPlugin {
            // Wasm builds will check for meta files (that don't exist) if this isn't set.
            // This causes errors and even panics on web build on itch.
            // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
            meta_check: AssetMetaCheck::Never,
            ..default()
        })
        .set(WindowPlugin {
            primary_window: Window {
                visible: false,
                title: "Just Let Me Sleep".to_string(),
                fit_canvas_to_parent: true,
                resolution: bevy::window::WindowResolution::new(
                    GAME_WIDTH as u32,
                    GAME_HEIGHT as u32,
                ),
                #[cfg(feature = "web")]
                prevent_default_event_handling: true,
                ..default()
            }
            .into(),
            ..default()
        })
        .set(LogPlugin {
            filter: format!(
                concat!(
                    "{default},",
                    "symphonia_bundle_mp3::demuxer=warn,",
                    "symphonia_format_caf::demuxer=warn,",
                    "symphonia_format_isompf4::demuxer=warn,",
                    "symphonia_format_mkv::demuxer=warn,",
                    "symphonia_format_ogg::demuxer=warn,",
                    "symphonia_format_riff::demuxer=warn,",
                    "symphonia_format_wav::demuxer=warn,",
                    "bevy_ecs_ldtk::assets::ldtk_project=warn,",
                    "calloop::loop_logic=error,",
                    "bevy_game::game_manager::army::move_squad=warn,",
                    "bevy_game::game_manager::ui_no_root::for_sell_ui=warn,",
                    "bevy_game::game_manager::battle::next_level=warn,",
                    "bevy_game::game_manager::battle::end=info,",
                    "bevy_game::game_manager::scene::switch_level=info,",
                    "bevy_game::game_manager::scene::boundary=warn,",
                    "bevy_game::game_manager::score=info,",
                    "pyri_tooltip=warn,",
                    "bevy_game::game_manager::audio::music=warn,",
                    "haniba_vfx=warn,",
                    "bevy_game::game_manager::memory::ra=warn,",
                    "bevy_game::game_manager::army::squad::spawn=warn,",
                    "bevy_game::game_manager::battle::end=warn,",
                    "bevy_game::game_manager::memory::big_eye=warn,",
                    "bevy_egui::render=error",
                ),
                default = bevy::log::DEFAULT_FILTER
            ),
            fmt_layer: |_| {
                Some(Box::new(
                    bevy::log::tracing_subscriber::fmt::Layer::default()
                        .without_time()
                        .map_fmt_fields(MakeExt::debug_alt)
                        .with_writer(std::io::stderr),
                ))
            },
            ..default()
        }),));

    bevy_seedling_ext::plugin(&mut app);
    bevy_jornet_ext::plugin(&mut app);
    bevy_prefab_ext::plugin(&mut app);
    bevy_sprite_layer_ext::plugin(&mut app);
    bevy_ecs_ldtk_ext::plugin(&mut app);
    bevy_ui_anchor_ext::plugin(&mut app);
    pyri_tooltip_ext::plugin(&mut app);
    pan_camera_ext::plugin(&mut app);
    bevy_rand_ext::plugin(&mut app);
    bevy_tweening_ext::plugin(&mut app);
    bevy_image_font_ext::plugin(&mut app);
    bevy_framepace_ext::plugin(&mut app);
    fps_tool_ext::plugin(&mut app);
    fixes_cursor_unlock_ext::plugin(&mut app);

    // Order new `AppSet` variants by adding them here:
    app.configure_sets(
        Update,
        (
            PostPhysicsAppSystems::TickTimers,
            PostPhysicsAppSystems::ChangeUi,
            PostPhysicsAppSystems::PlaySounds,
            PostPhysicsAppSystems::PlayAnimations,
            PostPhysicsAppSystems::Update,
        )
            .chain(),
    );

    app.add_systems(Update, make_window_visible);

    // Add other plugins.
    app.add_plugins((
        asset_processing::plugin,
        asset_tracking::plugin,
        #[cfg(feature = "dev")]
        dev_tools::plugin,
        screens::plugin,
        menus::plugin,
        ui_camera::plugin,
        palette::plugin,
    ));
    app.add_plugins(game_manager_plugin);

    #[cfg(feature = "dev_native")]
    app.add_plugins(test_scene::plugin);

    app.run()
}
fn make_window_visible(mut window: Query<&mut Window>, mut frames: Local<u32>) {
    *frames += 1;

    if *frames == 5 {
        if let Ok(mut window) = window.single_mut() {
            window.visible = true;
        }
    }
}
