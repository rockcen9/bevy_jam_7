use crate::prelude::*;

pub fn print_log_plugin(_app: &mut App) {
    // #[cfg(feature = "dev")]
    // {
    //     use bevy::dev_tools::states::log_transitions;
    //     _app.add_systems(Update, log_transitions::<GameState>);
    // }

    // if DEV_TRACE_ENEMY_ATTACK_TIMING {
    //     app.add_observer(dev_enemy_attack_added);
    //     app.add_systems(Update, debug_current_animation);
    // }
    // _app.add_systems(Update, _print_trace_log);
}

pub fn _print_trace_log(mut scale: ResMut<UiScale>) {
    scale.0 = 0.5;
}
