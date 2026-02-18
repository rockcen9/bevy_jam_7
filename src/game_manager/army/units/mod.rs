mod archer;
pub(crate) use archer::*;

mod shield;
pub(crate) use shield::*;

mod spear;
pub(crate) use spear::*;

mod cavalry;
pub(crate) use cavalry::*;

mod shadow;
pub(crate) use shadow::*;

mod unit;
pub(crate) use unit::*;
pub(crate) fn plugin(app: &mut bevy::app::App) {
    archer::plugin(app);
    shield::plugin(app);
    spear::plugin(app);
    cavalry::plugin(app);
    shadow::plugin(app);

    unit::plugin(app);
}
