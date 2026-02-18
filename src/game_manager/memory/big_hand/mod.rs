mod big_hand;
mod vortex;

pub use big_hand::BigHand;

pub(crate) fn plugin(app: &mut bevy::app::App) {
    big_hand::plugin(app);
    vortex::plugin(app);
}
