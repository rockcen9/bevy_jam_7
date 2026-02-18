use bevy_prefab::PrefabPlugin;

pub(crate) fn plugin(app: &mut bevy::app::App) {
    app.add_plugins(PrefabPlugin);
}
