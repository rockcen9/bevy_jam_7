use extol_sprite_layer::LayerIndex;

use crate::prelude::*;
// Sprite z-layer constants for proper rendering order

#[derive(Debug, Clone, Component, Hash, PartialEq, Eq, Reflect)]
pub enum SpriteLayer {
    Memory,
    VFX,
    SelectSquad,
    Item,

    Pawn,
    Dark,
    Building,
    Grid,
    Shadow,
    Corpse,
    PortalVFX,
}

impl LayerIndex for SpriteLayer {
    fn as_z_coordinate(&self) -> f32 {
        match *self {
            Self::VFX => 900.0,
            Self::SelectSquad => 850.0,
            Self::Item => 800.,
            Self::Memory => 600.,
            Self::Pawn => 500.0,
            Self::Dark => 200.0,
            Self::Building => 100.0,
            Self::Grid => 75.0,
            Self::Shadow => -45.0,
            Self::Corpse => -55.0,
            Self::PortalVFX => 50.0,
        }
    }
}
