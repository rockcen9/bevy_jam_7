mod units;
pub(crate) use units::*;

mod squad;
pub(crate) use squad::*;

mod movement;
pub(crate) use movement::*;

mod attack;
pub(crate) use attack::*;

mod enemy;
pub(crate) use enemy::*;

mod player;
pub(crate) use player::*;

// mod morale;
// pub(crate) use morale::*;

mod color;

mod effect;
pub(crate) use effect::*;

pub(crate) fn plugin(app: &mut bevy::app::App) {
    units::plugin(app);
    squad::plugin(app);
    movement::plugin(app);
    attack::plugin(app);
    enemy::plugin(app);
    // morale::plugin(app);
    player::plugin(app);
    color::plugin(app);
    effect::plugin(app);
}
