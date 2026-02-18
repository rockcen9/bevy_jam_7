//! Development tools for the game. This plugin is only enabled in dev builds.

use bevy::{dev_tools::states::log_transitions, prelude::*};

pub(crate) mod log_components;
mod validate_preloading;

mod print_log;
use print_log::*;

mod visual_debug;
use visual_debug::*;

pub mod dev_background_runner;

// mod cheat_buttons;
// use cheat_buttons::*;

mod change_detection;
use change_detection::*;

mod bevy_inspector_egui;
// mod statistic;

// mod display_resource;
// use display_resource::*;

mod inspect;
pub use inspect::*;

// mod scenario_test;
use crate::{
    dev_tools::dev_background_runner::dev_runner_plugin, menus::Menu,
    screens::loading::LoadingScreen,
};
// pub use scenario_test::*;

pub(super) fn plugin(app: &mut App) {
    bevy_inspector_egui::plugin(app);
    // debug_panel_plugin(app);
    print_log_plugin(app);
    visual_debug_plugin(app);
    dev_runner_plugin(app);
    change_detection_plugin(app);
    // display_resource_plugin(app);
    filter_component_plugin(app);
    // scenario_test_plugin(app);
    // Log `Screen` state transitions.
    // app.add_plugins(DebugPickingPlugin);
    app.add_systems(
        Update,
        (log_transitions::<Menu>, log_transitions::<LoadingScreen>).chain(),
    );

    app.add_plugins((validate_preloading::plugin, log_components::plugin));
    // app.add_systems(Update, _print_hover_on_click);
}
pub fn command_key_toggle_active(
    default: bool,
    key: KeyCode,
) -> impl FnMut(Res<ButtonInput<KeyCode>>) -> bool + Clone {
    let mut active = default;
    move |inputs: Res<ButtonInput<KeyCode>>| {
        if inputs.pressed(KeyCode::SuperLeft) && inputs.just_pressed(key) {
            active = !active;
        }
        active
    }
}
use bevy::picking::hover::HoverMap;

pub fn _print_hover_on_click(
    hover_map: Option<Res<HoverMap>>,
    names: Query<&Name>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    let Some(hover_map) = hover_map else {
        return;
    };

    println!("--- HoverMap State on Click ---");
    for (_pointer_id, pointer_map) in hover_map.iter() {
        for (entity, hit) in pointer_map.iter() {
            let name = names.get(*entity).map(|n| n.as_str()).unwrap_or("Unknown");
            println!("Entity hit by raycast: {:?} [{}]", entity, name);
            println!("   HitData: {:?}", hit);
        }
    }
    println!("--------------------------------------");
}
