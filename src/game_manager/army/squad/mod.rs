use crate::prelude::*;

mod squad;
pub(crate) use squad::*;

mod spawn;

mod move_squad;
pub(crate) use move_squad::SelectSquad;

mod player_squad;
pub(crate) use player_squad::*;

mod enemy_squad;
pub(crate) use enemy_squad::*;

pub(crate) fn plugin(app: &mut App) {
    squad::plugin(app);
    spawn::plugin(app);
    move_squad::plugin(app);
    player_squad::plugin(app);
    enemy_squad::plugin(app);
}
