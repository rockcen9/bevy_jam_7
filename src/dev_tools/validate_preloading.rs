//! Validates that all assets are preloaded before the game starts.

use bevy::prelude::*;
#[cfg(feature = "backend")]
use bevy_seedling::sample::SamplePlayer;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(validate_mesh);
    app.add_observer(validate_scene);
    #[cfg(feature = "backend")]
    app.add_observer(validate_audio);
}

fn validate_mesh(add: On<Add, Mesh3d>, q_mesh: Query<&Mesh3d>, assets: Res<AssetServer>) {
    let handle = &q_mesh.get(add.entity).unwrap().0;
    validate_asset(handle, &assets, "Mesh");
}

fn validate_scene(add: On<Add, SceneRoot>, q_scene: Query<&SceneRoot>, assets: Res<AssetServer>) {
    let handle = &q_scene.get(add.entity).unwrap().0;
    validate_asset(handle, &assets, "Scene");
}

fn validate_asset<T: Asset>(handle: &Handle<T>, assets: &AssetServer, type_name: &str) {
    let Some(path) = handle.path() else {
        return;
    };
    if !assets.is_loaded_with_dependencies(handle) {
        warn!("{type_name} at path \"{path}\" was not preloaded and will load during gameplay.",);
    }
}

#[cfg(feature = "backend")]
fn validate_audio(
    add: On<Add, SamplePlayer>,
    q_audio: Query<&SamplePlayer>,
    assets: Res<AssetServer>,
) {
    let handle = &q_audio.get(add.entity).unwrap().sample;
    validate_asset(handle, &assets, "Audio");
}
