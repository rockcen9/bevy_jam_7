use crate::prelude::*;

pub fn plugin(app: &mut App) {
    app.init_resource::<PlayerGold>();
    app.add_message::<PlayerGoldNotEnoughMessage>();
}

#[derive(Resource, Debug)]
pub struct PlayerGold {
    pub amount: u32,
}

impl Default for PlayerGold {
    fn default() -> Self {
        Self { amount: 80 }
    }
}

/// Event triggered when player attempts to purchase something but doesn't have enough gold
#[derive(Message, Debug, Clone)]
pub struct PlayerGoldNotEnoughMessage;
