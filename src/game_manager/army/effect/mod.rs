
mod buff;
pub(crate) use buff::*;

mod debuff;
pub(crate) use debuff::*;

pub(crate) fn plugin(app: &mut bevy::app::App) {
    buff::plugin(app);
    debuff::plugin(app);
}

