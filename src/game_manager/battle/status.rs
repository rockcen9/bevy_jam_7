use bevy::color::palettes::basic::GRAY;

use crate::prelude::*;

pub(crate) fn plugin(app: &mut App) {
    app.insert_resource(BattleStatusType::default());
    app.register_type::<BattleStatusType>();

    app.init_resource::<BattleStatus>();
    app.register_type::<BattleStatus>();

    app.add_message::<UnitReductionMessage>();

    app.add_systems(OnEnter(GameState::Battle), reset_battle_status);
    app.add_systems(
        Update,
        track_unit_reduction.run_if(in_state(GameState::Battle)),
    );
    app.add_systems(
        Update,
        track_death_locations
            .in_set(AttackSet::DeathRecord)
            .run_if(in_state(GameState::Battle)),
    );
}

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
pub enum BattleStatusType {
    #[default]
    Pending,
    Victory,
    Defeat,
}

impl BattleStatusType {
    pub fn color(&self) -> Color {
        match self {
            BattleStatusType::Pending => GRAY.into(),
            BattleStatusType::Victory => Color::srgba_u8(0x78, 0x9a, 0x73, 0xff), // green_medium from palette
            BattleStatusType::Defeat => Color::srgba_u8(0x9c, 0x58, 0x64, 0xff), // brown_reddish from palette
        }
    }
}

#[derive(Resource, Debug, Default, Reflect)]
pub struct BattleStatus {
    pub max_total_units: u32,
    pub current_total_units: u32,

    pub max_enemy_units: u32,
    pub current_enemy_units: u32,

    pub max_player_units: u32,
    pub current_player_units: u32,

    pub last_any_unit_death_position: Option<Vec2>,
    pub last_player_death_position: Option<Vec2>,
    pub last_enemy_death_position: Option<Vec2>,
}

impl BattleStatus {
    pub fn initialize(&mut self, player_count: u32, enemy_count: u32) {
        let total = player_count + enemy_count;

        self.max_total_units = total;
        self.current_total_units = total;

        self.max_player_units = player_count;
        self.current_player_units = player_count;

        self.max_enemy_units = enemy_count;
        self.current_enemy_units = enemy_count;
    }

    pub fn update_player_units(&mut self, count: u32) {
        self.current_player_units = count;
        self.update_total();
    }

    pub fn update_enemy_units(&mut self, count: u32) {
        self.current_enemy_units = count;
        self.update_total();
    }

    fn update_total(&mut self) {
        self.current_total_units = self.current_player_units + self.current_enemy_units;
    }
}

#[allow(dead_code)]
#[derive(Message, Debug, Clone)]
pub struct UnitReductionMessage {
    pub faction: Faction,
    pub percentage_lost: u32, // 10, 20, 30, ... 100
    pub remaining_units: u32,
    pub max_units: u32,
}

fn reset_battle_status(
    mut battle_status: ResMut<BattleStatus>,
    player_units: Query<(), With<PlayerUnit>>,
    enemy_units: Query<(), With<EnemyUnit>>,
) {
    let player_count = player_units.iter().count() as u32;
    let enemy_count = enemy_units.iter().count() as u32;

    battle_status.initialize(player_count, enemy_count);

    info!(
        "Battle status reset: {} player units, {} enemy units, {} total",
        player_count,
        enemy_count,
        player_count + enemy_count
    );
}

fn track_unit_reduction(
    mut battle_status: ResMut<BattleStatus>,
    player_units: Query<(), With<PlayerUnit>>,
    enemy_units: Query<(), With<EnemyUnit>>,
    mut message_writer: MessageWriter<UnitReductionMessage>,
) {
    let current_player_count = player_units.iter().count() as u32;
    let current_enemy_count = enemy_units.iter().count() as u32;

    if current_player_count != battle_status.current_player_units {
        let old_percentage = calculate_loss_percentage(
            battle_status.current_player_units,
            battle_status.max_player_units,
        );
        let new_percentage =
            calculate_loss_percentage(current_player_count, battle_status.max_player_units);

        if new_percentage / 10 > old_percentage / 10 {
            let threshold = (new_percentage / 10) * 10;
            message_writer.write(UnitReductionMessage {
                faction: Faction::Player,
                percentage_lost: threshold,
                remaining_units: current_player_count,
                max_units: battle_status.max_player_units,
            });
            info!(
                "Player units reduced by {}%: {}/{} remaining",
                threshold, current_player_count, battle_status.max_player_units
            );
        }

        battle_status.update_player_units(current_player_count);
    }

    if current_enemy_count != battle_status.current_enemy_units {
        let old_percentage = calculate_loss_percentage(
            battle_status.current_enemy_units,
            battle_status.max_enemy_units,
        );
        let new_percentage =
            calculate_loss_percentage(current_enemy_count, battle_status.max_enemy_units);

        if new_percentage / 10 > old_percentage / 10 {
            let threshold = (new_percentage / 10) * 10;
            message_writer.write(UnitReductionMessage {
                faction: Faction::Enemy,
                percentage_lost: threshold,
                remaining_units: current_enemy_count,
                max_units: battle_status.max_enemy_units,
            });
            info!(
                "Enemy units reduced by {}%: {}/{} remaining",
                threshold, current_enemy_count, battle_status.max_enemy_units
            );
        }

        battle_status.update_enemy_units(current_enemy_count);
    }
}

fn calculate_loss_percentage(current: u32, max: u32) -> u32 {
    if max == 0 {
        return 0;
    }
    let remaining_percentage = (current * 100) / max;
    100 - remaining_percentage
}

fn track_death_locations(
    mut battle_status: ResMut<BattleStatus>,
    mut death_messages: MessageReader<UnitDeathMessage>,
) {
    for msg in death_messages.read() {
        battle_status.last_any_unit_death_position = Some(msg.position);

        if msg.is_enemy {
            battle_status.last_enemy_death_position = Some(msg.position);
            debug!("Enemy unit died at position: {:?}", msg.position);
        } else {
            battle_status.last_player_death_position = Some(msg.position);
            debug!("Player unit died at position: {:?}", msg.position);
        }
    }
}
