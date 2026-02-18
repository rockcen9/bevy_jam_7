use crate::{game_manager::BattleSystems, prelude::*};

pub(crate) fn plugin(app: &mut bevy::app::App) {
    app.init_resource::<CombatFlux>();
    app.add_message::<CombatMessage>();
    app.add_systems(
        Update,
        calculate_combat_flux
            .in_set(BattleSystems::CalculateCombatFlux)
            .run_if(in_state(GameState::Battle)),
    );
}

#[derive(Resource, Reflect)]
pub struct CombatFlux {
    morale: f32,

    player_count: u32,
    enemy_count: u32,

    player_max_hp: f32,
    enemy_max_hp: f32,

    timer: Timer,
}

impl Default for CombatFlux {
    fn default() -> Self {
        Self {
            morale: 0.0,
            player_count: 0,
            enemy_count: 0,
            player_max_hp: 0.0,
            enemy_max_hp: 0.0,
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        }
    }
}

impl CombatFlux {
    pub fn add_morale(&mut self, delta: f32) {
        self.morale = (self.morale + delta).clamp(-100.0, 100.0);
    }

    pub fn _set_morale(&mut self, value: f32) {
        self.morale = value.clamp(-100.0, 100.0);
    }

    pub fn morale(&self) -> f32 {
        self.morale
    }

    pub fn player_count(&self) -> u32 {
        self.player_count
    }

    pub fn enemy_count(&self) -> u32 {
        self.enemy_count
    }
}

fn calculate_combat_flux(
    mut combat_flux: ResMut<CombatFlux>,
    time: Res<Time>,
    q_player_units: Query<&Health, (With<SpriteActor>, Without<EnemyUnit>)>,
    q_enemy_units: Query<&Health, (With<SpriteActor>, With<EnemyUnit>)>,
) {
    combat_flux.timer.tick(time.delta());

    if !combat_flux.timer.just_finished() {
        return;
    }

    // Calculate total HP and count for each side
    let mut player_current_hp = 0.0;
    let mut player_max_hp = 0.0;
    let mut player_count = 0u32;
    for health in &q_player_units {
        player_current_hp += health.get_current();
        player_max_hp += health.get_max();
        player_count += 1;
    }

    let mut enemy_current_hp = 0.0;
    let mut enemy_max_hp = 0.0;
    let mut enemy_count = 0u32;
    for health in &q_enemy_units {
        enemy_current_hp += health.get_current();
        enemy_max_hp += health.get_max();
        enemy_count += 1;
    }

    combat_flux.player_count = player_count;
    combat_flux.enemy_count = enemy_count;
    combat_flux.player_max_hp = player_max_hp;
    combat_flux.enemy_max_hp = enemy_max_hp;

    // Calculate morale based on HP ratio difference
    if player_max_hp > 0.0 && enemy_max_hp > 0.0 {
        let player_ratio = player_current_hp / player_max_hp;
        let enemy_ratio = enemy_current_hp / enemy_max_hp;
        let morale_delta = (player_ratio - enemy_ratio) * 10.0;

        debug!(
            "Combat Flux - Player: {}/{} ({:.1}%), Enemy: {}/{} ({:.1}%), Delta: {:.2}, Old Morale: {:.2}",
            player_current_hp,
            player_max_hp,
            player_ratio * 100.0,
            enemy_current_hp,
            enemy_max_hp,
            enemy_ratio * 100.0,
            morale_delta,
            combat_flux.morale
        );

        combat_flux.add_morale(morale_delta);

        debug!("New Morale: {:.2}", combat_flux.morale);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PositiveMoraleEvent {
    Massacre,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NegativeMoraleEvent {
    Wipeout,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleTrigger {
    RapidKills,
}
// impl BattleTrigger {
//     pub fn to_positive(&self) -> PositiveMoraleEvent {
//         match self {
//             BattleTrigger::RapidKills => PositiveMoraleEvent::Massacre,
//         }
//     }

//     pub fn to_negative(&self) -> NegativeMoraleEvent {
//         match self {
//             BattleTrigger::RapidKills => NegativeMoraleEvent::Wipeout,
//         }
//     }
// }

#[derive(Message)]
pub struct CombatMessage {
    pub _trigger: BattleTrigger,
    pub _source: Faction,
    pub _value: f32,
}
