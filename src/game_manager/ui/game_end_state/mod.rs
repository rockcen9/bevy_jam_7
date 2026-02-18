use crate::prelude::*;

mod leaderboard;
mod root;
pub fn plugin(app: &mut App) {
    root::plugin(app);
    leaderboard::plugin(app);
}
