use crate::prelude::*;

mod game_win_state;

mod game_lose_state;

mod sell_unit_tooltip;
mod sell_memory_tooltip;
mod world_unit_tooltipi;
pub fn plugin(app: &mut App) {
    game_win_state::plugin(app);
    game_lose_state::plugin(app);
    sell_unit_tooltip::plugin(app);
    sell_memory_tooltip::plugin(app);
    world_unit_tooltipi::plugin(app);
}
