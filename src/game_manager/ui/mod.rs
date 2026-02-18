use bevy::dev_tools::picking_debug::DebugPickingPlugin;

mod battle_state;

pub(crate) mod prepare_state;

mod persistent;

mod game_end_state;

pub(crate) fn plugin(app: &mut bevy::app::App) {
    app.add_plugins(DebugPickingPlugin);
    battle_state::plugin(app);
    prepare_state::plugin(app);

    persistent::plugin(app);
    game_end_state::plugin(app);
}
