mod put;
mod shake_despawn;
mod shake_left_right;
mod shrink_despawn;
mod squash;
// mod wobble;
pub(crate) use put::*;
pub(crate) use shake_despawn::ShakeDespawn;
pub(crate) use shake_left_right::ShakeLeftRight;
pub(crate) use shrink_despawn::ShrinkDespawn;

pub(crate) fn plugin(app: &mut bevy::app::App) {
    put::plugin(app);
    shake_despawn::plugin(app);
    shake_left_right::plugin(app);
    shrink_despawn::plugin(app);
    // wobble::plugin(app);
    squash::plugin(app);
}
