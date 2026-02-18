
mod units_csv;
pub(crate) use units_csv::*;

mod memory_csv;
pub(crate) use memory_csv::*;

mod apply_unit;

pub(crate) fn plugin(app: &mut bevy::app::App) {
    units_csv::plugin(app);
    memory_csv::plugin(app);
    apply_unit::plugin(app);
}
