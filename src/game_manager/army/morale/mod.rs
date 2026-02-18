mod combat_flux;
pub(crate) use combat_flux::*;

mod rapid_kills;

mod unit_reduce;

pub(crate) fn plugin(app: &mut bevy::app::App) {
    combat_flux::plugin(app);
    rapid_kills::plugin(app);
    unit_reduce::plugin(app);
}
