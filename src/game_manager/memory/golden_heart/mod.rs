mod golden_heart;
pub use golden_heart::GoldenHeart;

mod wave;

mod ghost;

pub(crate) fn plugin(app: &mut bevy::app::App) {
    golden_heart::plugin(app);
    wave::plugin(app);
    ghost::plugin(app);
}
