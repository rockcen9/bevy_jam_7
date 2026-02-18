mod health;
pub(crate) use health::*;
mod pawn;
pub(crate) use pawn::*;
pub(crate) fn plugin(app: &mut bevy::app::App) {
    health::plugin(app);
}
