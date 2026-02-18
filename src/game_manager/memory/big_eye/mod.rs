use crate::prelude::*;

mod big_eye;
pub(crate) use big_eye::*;
mod ghost;
mod laser;

pub(crate) fn plugin(app: &mut bevy::app::App) {
    laser::plugin(app);
    ghost::plugin(app);
    big_eye::plugin(app);
}
