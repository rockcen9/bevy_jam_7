use crate::prelude::*;

// pub const GAME_WIDTH: f32 = 1920.;
// pub const GAME_HEIGHT: f32 = 1080.;
//
// pub const GAME_WIDTH: f32 = 1280.16;
// pub const GAME_HEIGHT: f32 = 721.92;
//
pub const GAME_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const GAME_WIDTH: f32 = 1920.;
pub const GAME_HEIGHT: f32 = 1080.;
pub const GAME_ASPECT_RATIO: f32 = GAME_WIDTH / GAME_HEIGHT;
/// Configuration for iron bar skill behavior
#[derive(Debug, Clone)]
pub struct IronBarConfig {
    /// Iron block fall speed in blocks per second
    pub falling_speed: f32,
    /// Energy cost to activate iron bar skill
    pub energy_cost: u8,
    /// Iron block lifetime in seconds before auto-cleanup
    pub life_time: f32,
}

impl Default for IronBarConfig {
    fn default() -> Self {
        Self {
            falling_speed: 8.0,
            energy_cost: 36,
            life_time: 10.0,
        }
    }
}

/// Configuration for spike skill behavior
#[derive(Debug, Clone)]
pub struct SpikeConfig {
    /// Spike block fall speed in blocks per second
    pub falling_speed: f32,
    /// Energy cost to activate spike skill
    pub energy_cost: u8,
}

impl Default for SpikeConfig {
    fn default() -> Self {
        Self {
            falling_speed: 8.0,
            energy_cost: 36,
        }
    }
}

/// Configuration for rock skill behavior
#[derive(Debug, Clone)]
pub struct RockConfig {
    /// Rock block fall speed in blocks per second
    pub falling_speed: f32,
    /// Energy cost to activate rock skill
    pub energy_cost: u8,
}

impl Default for RockConfig {
    fn default() -> Self {
        Self {
            falling_speed: 8.0,
            energy_cost: 10,
        }
    }
}

/// Configuration for enemy auto-attack behavior
#[derive(Debug, Clone)]
pub struct EnemyAttackConfig {
    /// Energy threshold needed to trigger auto-attack
    pub energy_threshold: u8,
    /// Cooldown between attacks in seconds
    pub attack_cooldown: f32,
    /// How often to check for attack opportunities in seconds
    pub check_interval: f32,
}

impl Default for EnemyAttackConfig {
    fn default() -> Self {
        Self {
            energy_threshold: 10,
            attack_cooldown: 5.0,
            check_interval: 1.0,
        }
    }
}

/// Configuration for all skill behaviors
#[derive(Debug, Clone, Default)]
pub struct SkillConfig {
    pub iron_bar: IronBarConfig,
    pub spike: SpikeConfig,
    pub rock: RockConfig,
}

/// Central game configuration resource for all game systems
#[derive(Resource, Debug, Clone)]
pub struct GameConfig {
    pub skill: SkillConfig,
    pub enemy_attack: EnemyAttackConfig,
    /// Controls visibility of Enemy's NextPosition (default: false)
    pub enemy_next_position_visibility: bool,
    /// Number of players in the game (1 for single player, 2 for two player)
    pub player_count: usize,
    /// Current level to load (1, 2, or 3)
    pub current_level: u8,
    /// Extra health bonus applied to all enemies (default: 0)
    pub enemy_extra_health: u32,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            skill: SkillConfig::default(),
            enemy_attack: EnemyAttackConfig::default(),
            enemy_next_position_visibility: false,
            player_count: 2,
            current_level: 1,
            enemy_extra_health: 0,
        }
    }
}
