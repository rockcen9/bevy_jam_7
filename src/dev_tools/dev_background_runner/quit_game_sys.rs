use crate::prelude::*;

// Handles ESC key press to exit the game during development
pub fn _quit_on_esc(keyboard: Res<ButtonInput<KeyCode>>, mut exit: MessageWriter<AppExit>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        info!("ESC pressed - exiting game");
        exit.write(AppExit::Success);
    }
}
