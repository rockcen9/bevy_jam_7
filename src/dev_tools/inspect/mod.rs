use crate::{dev_tools::inspect::sys_filter::debug_component, prelude::*};

mod resource_sys;
pub use resource_sys::*;
mod sys_filter;
pub fn filter_component_plugin(app: &mut App) {
    // app.add_plugins(EguiPlugin::default());

    // app.add_plugins(
    //     ResourceInspectorPlugin::<GameState>::default()
    //         .run_if(command_key_toggle_active(false, KeyCode::Digit2)),
    // );
    // app.add_plugins(
    //     ResourceInspectorPlugin::<Days>::default()
    //         .run_if(command_key_toggle_active(false, KeyCode::Digit4)),
    // );
    // app.add_plugins(
    //     ResourceInspectorPlugin::<EnemyDataConfig>::default()
    //         .run_if(command_key_toggle_active(false, KeyCode::Digit4)),
    // );
    // app.add_plugins(
    //     ResourceInspectorPlugin::<DifficultyCurveConfig>::default()
    //         .run_if(command_key_toggle_active(false, KeyCode::Digit4)),
    // );
    // app.add_plugins(
    //     ResourceInspectorPlugin::<Energy>::default()
    //         .run_if(command_key_toggle_active(false, KeyCode::Digit4)),
    // );
    // app.add_plugins(
    //     StateInspectorPlugin::<Screen>::default()
    //         .run_if(command_key_toggle_active(false, KeyCode::Digit4)),
    // );
    // app.add_plugins(
    //     StateInspectorPlugin::<Menu>::default()
    //         .run_if(command_key_toggle_active(false, KeyCode::Digit4)),
    // );
    // app.add_plugins(
    //     StateInspectorPlugin::<GameState>::default()
    //         .run_if(command_key_toggle_active(false, KeyCode::Digit4)),
    // );

    debug_resource_plugin(app);
    debug_component(app);
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
