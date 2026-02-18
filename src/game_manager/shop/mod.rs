use crate::prelude::*;

mod player_gold;
pub(crate) use player_gold::*;

pub(crate) fn plugin(app: &mut App) {
    player_gold::plugin(app);
}
