use crate::{dbg::DebugConfig, prelude::*};

mod dev_spawn_salad;
// use dev_spawn_salad::*;

mod quit_game_sys;

// enum DevMode {
//     Normal,
//     Sidecar,
//     SteamDeck,
// }
// const DEV_MOVE_WINDOWS_FOR_SIDECAR: bool = false;
// const DEV_MAKE_WINDOW_BIGGER: bool = false;

pub fn dev_runner_plugin(_app: &mut App) {
    // app.add_systems(Update, dev_restart_game);
    if cfg!(debug_assertions) {
        _app.add_systems(Startup, move_window_after_startup);
    }

    _app.add_systems(Update, make_window_bigger);

    // ESC to quit game during development
    // _app.add_systems(Update, quit_on_esc);

    // _app.add_observer(spawn_salad);
    // app.add_systems(Update, dev_to_store);

    // if DEV_SKIP_TO_LOADING_SCREEN {
    //     app.add_systems(OnEnter(Screen::Splash), skip_to_loading_screen);
    // }

    // if config_dev::DEV_HALF_PLAYER_HEALTH {
    //     app.add_systems(OnEnter(GameState::PlayerPlanning), set_half_player_health);
    // }

    // if config_dev::DEV_HALF_PLAYER_SAINT {
    //     app.add_systems(OnEnter(GameState::PlayerPlanning), set_half_player_saint);
    // }

    // if config_dev::DEV_HALF_PLAYER_CURSED {
    //     app.add_systems(OnEnter(GameState::PlayerPlanning), set_half_player_cursed);
    // }

    // if config_dev::DEV_HALF_PLAYER_STAMINA {
    //     app.add_systems(OnEnter(GameState::PlayerPlanning), set_half_player_stamina);
    // }
    // _app.add_systems(OnEnter(Screen::MainMenu), conditional_auto_to_in_game);
}

pub fn move_window_after_startup(mut windows: Query<&mut Window>, debug_config: Res<DebugConfig>) {
    if debug_config.side_car_windows {
        for mut window in windows.iter_mut() {
            //ipad?
            // window.position = WindowPosition::At(IVec2::new(-2560, 468));
            //
            window.position = WindowPosition::At(IVec2::new(-3456, 2956));
        }
    }
}
pub fn _print_window_position(window_query: Query<&Window>, keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::KeyP) {
        if let Ok(window) = window_query.single() {
            dbg!("Window position: {:?}", window.position);
        } else {
            println!("Could not get window");
        }
    }
}

// pub fn skip_to_loading_screen(mut next_screen_state: ResMut<NextState<Screen>>) {
//     next_screen_state.set(Screen::Loading);
// }

///2459
/// 1387
///
pub fn make_window_bigger(mut window: Single<&mut Window>, debug_config: Res<DebugConfig>) {
    if !debug_config.make_window_bigger {
        return;
    }
    if window.width() >= 2459. {
        return;
    }
    if debug_config.make_window_bigger {
        window.position = WindowPosition::At(IVec2::new(-3456, 2956));
        //
        window.resolution.set(1728., 974.);
    } else {
        window.position = WindowPosition::At(IVec2::new(0, 0));
        window.resolution.set(2459., 1387.);
    }
}
