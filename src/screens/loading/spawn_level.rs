//! The loading screen that appears when the game is starting, but still spawning the level.

use bevy_ecs_ldtk::assets::LdtkProject;

use crate::prelude::*;

use super::LoadingScreen;
use crate::screens::Screen;
use crate::theme::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(LoadingScreen::Level), spawn_level_loading_screen);
    app.add_systems(
        Update,
        advance_to_gameplay_screen.run_if(in_state(LoadingScreen::Level)),
    );
}

fn spawn_level_loading_screen(mut commands: Commands, palette: Res<ColorPalette>, asset_server: Res<AssetServer>) {
    commands.spawn((
        widget::ui_root("Loading Screen"),
        BackgroundColor(palette.get(UiColorName::ScreenBackground)),
        DespawnOnExit(LoadingScreen::Level),
        children![widget::label("Spawning Level...", &palette, &asset_server)],
    ));
}

// todo
fn advance_to_gameplay_screen(
    mut next_screen: ResMut<NextState<Screen>>,
    asset_server: Res<AssetServer>,
    game_assets: Res<SceneAssets>,
    ldtk_projects: Res<Assets<LdtkProject>>,
    balance: Res<UnitBalanceAssets>,
) {
    if !asset_server.is_loaded_with_dependencies(&game_assets.level) {
        return;
    }

    if !asset_server.is_loaded_with_dependencies(&balance.units) {
        return;
    }
    if ldtk_projects.get(&game_assets.level).is_none() {
        return;
    }

    next_screen.set(Screen::Gameplay);
}
