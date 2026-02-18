use bevy::prelude::*;

#[cfg(not(feature = "web"))]
pub(crate) fn plugin(_app: &mut App) {}

#[cfg(feature = "web")]
pub(crate) fn plugin(app: &mut App) {
    use bevy_fix_cursor_unlock_web::FixPointerUnlockPlugin;
    app.add_plugins(FixPointerUnlockPlugin);
}
