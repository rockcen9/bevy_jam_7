use bevy_tweening::TweeningPlugin;

pub(crate) fn plugin(app: &mut bevy::app::App) {
    app.add_plugins(TweeningPlugin);
}
