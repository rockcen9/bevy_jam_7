use crate::{game_manager::BattleSystems, prelude::*};

use super::{BattleTrigger, CombatFlux, CombatMessage, Faction};

pub(crate) fn plugin(app: &mut bevy::app::App) {
    app.init_resource::<RapidKillsHeat>();
    app.add_systems(
        Update,
        accumulate_kill_heat
            .in_set(AttackSet::DeathRecord)
            .run_if(in_state(GameState::Battle)),
    );
    app.add_systems(
        Update,
        (decay_kill_heat, check_rapid_kill_trigger)
            .chain()
            .in_set(BattleSystems::CalculateCombatFlux)
            .run_if(in_state(GameState::Battle)),
    );
}

// ============================================================================
// Configuration Constants
// ============================================================================

/// Base constant for the hybrid threshold (minimum kills needed)
const BASE_THRESHOLD: f32 = 5.0;

/// Scaling factor (1.5% of army size added to threshold)
const SCALING_FACTOR: f32 = 0.01;

/// Decay rate as percentage of threshold per second (30%)
const DECAY_RATE_PERCENT: f32 = 0.30;

// ============================================================================
// Resource
// ============================================================================

/// Tracks the "heat" of rapid kills for each faction.
/// Heat accumulates when units are killed and decays over time.
#[derive(Resource, Reflect, Default)]
pub struct RapidKillsHeat {
    /// Heat accumulated by player kills (enemy deaths)
    player_heat: f32,
    /// Heat accumulated by enemy kills (player deaths)
    enemy_heat: f32,
}

impl RapidKillsHeat {
    /// Calculate the threshold based on army size using the hybrid formula:
    /// Threshold = Base_Constant + (Army_Size × Scaling_Factor)
    fn calculate_threshold(army_size: u32) -> f32 {
        BASE_THRESHOLD + (army_size as f32 * SCALING_FACTOR)
    }

    /// Calculate decay amount per delta time based on threshold
    fn calculate_decay(threshold: f32, delta_seconds: f32) -> f32 {
        threshold * DECAY_RATE_PERCENT * delta_seconds
    }

    pub fn _player_heat(&self) -> f32 {
        self.player_heat
    }

    pub fn _enemy_heat(&self) -> f32 {
        self.enemy_heat
    }
}

// ============================================================================
// Systems
// ============================================================================

/// Accumulates heat when units die
fn accumulate_kill_heat(
    mut rapid_kills: ResMut<RapidKillsHeat>,
    mut ev_death: MessageReader<UnitDeathMessage>,
) {
    for ev in ev_death.read() {
        if ev.is_enemy {
            // Enemy died -> player killed them -> increase player heat
            rapid_kills.player_heat += 1.0;
            debug!(
                "[RAPID_KILLS] Enemy died -> player_heat: {:.1}",
                rapid_kills.player_heat
            );
        } else {
            // Player unit died -> enemy killed them -> increase enemy heat
            rapid_kills.enemy_heat += 1.0;
            debug!(
                "[RAPID_KILLS] Player died -> enemy_heat: {:.1}",
                rapid_kills.enemy_heat
            );
        }
    }
}

/// Decays heat over time based on the threshold
fn decay_kill_heat(
    mut rapid_kills: ResMut<RapidKillsHeat>,
    combat_flux: Res<CombatFlux>,
    time: Res<Time>,
) {
    let delta = time.delta_secs();

    // Calculate thresholds based on current army sizes
    let player_threshold = RapidKillsHeat::calculate_threshold(combat_flux.player_count());
    let enemy_threshold = RapidKillsHeat::calculate_threshold(combat_flux.enemy_count());

    // Calculate decay amounts
    let player_decay = RapidKillsHeat::calculate_decay(player_threshold, delta);
    let enemy_decay = RapidKillsHeat::calculate_decay(enemy_threshold, delta);

    let old_player_heat = rapid_kills.player_heat;
    let old_enemy_heat = rapid_kills.enemy_heat;

    // Apply decay (don't go below 0)
    rapid_kills.player_heat = (rapid_kills.player_heat - player_decay).max(0.0);
    rapid_kills.enemy_heat = (rapid_kills.enemy_heat - enemy_decay).max(0.0);

    // Log significant decay events
    if old_player_heat > 1.0 && rapid_kills.player_heat < 0.1 {
        debug!(
            "[RAPID_KILLS] Player heat decayed to zero (was {:.1}, decay_rate={:.2}/s)",
            old_player_heat,
            player_decay / delta
        );
    }
    if old_enemy_heat > 1.0 && rapid_kills.enemy_heat < 0.1 {
        debug!(
            "[RAPID_KILLS] Enemy heat decayed to zero (was {:.1}, decay_rate={:.2}/s)",
            old_enemy_heat,
            enemy_decay / delta
        );
    }
}

/// Checks if heat exceeds threshold and triggers RapidKills event
fn check_rapid_kill_trigger(
    mut rapid_kills: ResMut<RapidKillsHeat>,
    combat_flux: Res<CombatFlux>,
    mut ev_combat: MessageWriter<CombatMessage>,
) {
    // Check player's rapid kills (based on enemy army size)
    let enemy_threshold = RapidKillsHeat::calculate_threshold(combat_flux.enemy_count());
    if rapid_kills.player_heat > 0.0 {
        debug!(
            "[RAPID_KILLS] Player heat check: {:.1} / {:.1} (enemy_count={})",
            rapid_kills.player_heat,
            enemy_threshold,
            combat_flux.enemy_count()
        );
    }
    if rapid_kills.player_heat >= enemy_threshold {
        info!(
            "[RAPID_KILLS] >>> PLAYER TRIGGER! heat={:.1} >= threshold={:.1}",
            rapid_kills.player_heat, enemy_threshold
        );
        ev_combat.write(CombatMessage {
            _trigger: BattleTrigger::RapidKills,
            _source: Faction::Player,
            _value: rapid_kills.player_heat,
        });
        // Reset heat after triggering
        rapid_kills.player_heat = 0.0;
    }

    // Check enemy's rapid kills (based on player army size)
    let player_threshold = RapidKillsHeat::calculate_threshold(combat_flux.player_count());
    if rapid_kills.enemy_heat > 0.0 {
        debug!(
            "[RAPID_KILLS] Enemy heat check: {:.1} / {:.1} (player_count={})",
            rapid_kills.enemy_heat,
            player_threshold,
            combat_flux.player_count()
        );
    }
    if rapid_kills.enemy_heat >= player_threshold {
        info!(
            "[RAPID_KILLS] >>> ENEMY TRIGGER! heat={:.1} >= threshold={:.1}",
            rapid_kills.enemy_heat, player_threshold
        );
        ev_combat.write(CombatMessage {
            _trigger: BattleTrigger::RapidKills,
            _source: Faction::Enemy,
            _value: rapid_kills.enemy_heat,
        });
        // Reset heat after triggering
        rapid_kills.enemy_heat = 0.0;
    }
}
