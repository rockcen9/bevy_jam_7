//! A loading screen during which game assets are loaded.
//! This reduces stuttering, especially for audio on Wasm.

use bevy::prelude::*;

use crate::prelude::*;
use crate::{asset_tracking::ResourceHandles, theme::prelude::*};

use super::LoadingScreen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(LoadingScreen::Assets),
        spawn_or_skip_asset_loading_screen,
    );

    //todo
    app.add_systems(
        Update,
        (
            update_loading_assets_label,
            enter_load_level_screen.run_if(all_assets_loaded.and(in_state(LoadingScreen::Assets))),
        ),
    );
}
fn enter_load_level_screen(mut next_screen: ResMut<NextState<LoadingScreen>>) {
    next_screen.set(LoadingScreen::Level);
}

fn spawn_or_skip_asset_loading_screen(
    mut commands: Commands,
    resource_handles: Res<ResourceHandles>,
    mut next_screen: ResMut<NextState<LoadingScreen>>,
    palette: Res<ColorPalette>,
    asset_server: Res<AssetServer>,
) {
    if resource_handles.is_all_done() {
        next_screen.set(LoadingScreen::Level);
        return;
    }
    commands.spawn((
        widget::ui_root("Loading Screen"),
        BackgroundColor(palette.get(UiColorName::ScreenBackground)),
        DespawnOnExit(LoadingScreen::Assets),
        children![(
            widget::label("Loading Assets", &palette, &asset_server),
            LoadingAssetsLabel
        )],
    ));
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct LoadingAssetsLabel;

fn update_loading_assets_label(
    mut query: Query<&mut Text, With<LoadingAssetsLabel>>,
    resource_handles: Res<ResourceHandles>,
) {
    for mut text in query.iter_mut() {
        text.0 = format!(
            "Loading Assets: {} / {}",
            resource_handles.finished_count(),
            resource_handles.total_count()
        );
    }
}
fn all_assets_loaded(resource_handles: Res<ResourceHandles>) -> bool {
    resource_handles.is_all_done()
}
