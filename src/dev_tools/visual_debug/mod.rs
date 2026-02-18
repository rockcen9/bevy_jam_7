
// use iyes_perf_ui::{
//     PerfUiPlugin,
//     prelude::{PerfUiAllEntries, PerfUiRoot},
// };
use crate::prelude::*;
pub fn visual_debug_plugin(_app: &mut App) {
    #[cfg(feature = "dev")]
    {
        // Toggle the debug overlay for UI.
        // app.add_systems(
        //     Update,
        //     toggle_debug_ui.run_if(input_just_pressed(TOGGLE_KEY)),
        // );
    }
    // FPS
    // app.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
    //     .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
    //     .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
    //     .add_plugins(bevy::render::diagnostic::RenderDiagnosticsPlugin)
    //     .add_plugins(PerfUiPlugin);
    // app.add_systems(Update, toggle_fps.before(iyes_perf_ui::PerfUiSet::Setup));
}

// #[cfg(feature = "dev")]
// use bevy::prelude::UiDebugOptions;
// #[cfg(feature = "dev")]
// const TOGGLE_KEY: KeyCode = KeyCode::Backquote;

// #[cfg(feature = "dev")]
// fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
//     options.toggle();
// }

// fn toggle_fps(
//     mut commands: Commands,
//     q_root: Query<Entity, With<PerfUiRoot>>,
//     kbd: Res<ButtonInput<KeyCode>>,
// ) {
//     if kbd.just_pressed(KeyCode::F5) {
//         if let Ok(e) = q_root.single() {
//             // despawn the existing Perf UI
//             commands.entity(e).despawn();
//         } else {
//             commands.spawn(PerfUiAllEntries::default());
//         }
//     }
// }
