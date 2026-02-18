use strum::{AsRefStr, EnumCount, EnumIter};

use crate::prelude::*;
pub(crate) fn plugin(_app: &mut bevy::app::App) {}
/// Unit kind for targeting
#[derive(
    serde::Deserialize,
    Component,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Reflect,
    AsRefStr,
    EnumIter,
    EnumCount,
)]
pub enum UnitKind {
    Shield,
    Spear,
    Archer,
    Cavalry,
}
