use crate::prelude::*;

use super::BattleStatusType;

pub(crate) fn plugin(app: &mut App) {
    app.insert_resource(GameProgress::new());
    app.register_type::<GameProgress>();
}

#[derive(Resource, Debug, Default, Reflect)]
pub struct GameProgress {
    pub history: Vec<BattleStatusType>,
    pub current_round: usize,
    pub total_wins: u32,
    pub total_losses: u32,
}

impl GameProgress {
    pub fn new() -> Self {
        Self {
            history: vec![BattleStatusType::Pending; 15],
            current_round: 1,
            total_wins: 0,
            total_losses: 0,
        }
    }

    pub fn is_game_over(&self) -> bool {
        self.total_losses >= 3 || self.total_wins >= 10 || self.current_round >= 15
    }

    pub fn record_battle(&mut self, is_victory: bool) {
        if self.is_game_over() {
            warn!("Game is already over!");
            return;
        }

        let status = if is_victory {
            self.total_wins += 1;
            BattleStatusType::Victory
        } else {
            self.total_losses += 1;
            BattleStatusType::Defeat
        };

        if self.current_round > 0 && self.current_round <= self.history.len() {
            self.history[self.current_round - 1] = status;
            if is_victory {
                self.current_round += 1;
            }
        }

        if self.total_wins >= 10 {
            info!("Campaign Victory!");
        } else if self.total_losses >= 3 {
            info!("Campaign Defeat!");
        }
    }
}
