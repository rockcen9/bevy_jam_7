use bevy_ecs_ldtk::LevelSelection;

use crate::prelude::*;

const UNIT_SIZE: f32 = 64.0;
const UNIT_SPACING: f32 = 10.0;
const GRID_WIDTH: usize = 10;

pub fn scenario_test_plugin(app: &mut App) {
    app.insert_resource(BattleScenarioConfig::scenario_a());
    app.add_systems(Update, skip_to_battle_if_scenario_active);
    app.add_systems(
        OnEnter(GameState::Battle),
        (switch_to_prebattle, spawn_scenario_armies)
            .chain()
            .run_if(|config: Res<BattleScenarioConfig>| config.active),
    );
}

#[derive(Resource, Default)]
pub struct BattleScenarioConfig {
    pub active: bool,
    pub player_army: Vec<(String, usize)>, // (Prefab Name, Count)
    pub enemy_army: Vec<(String, usize)>,  // (Prefab Name, Count)
}

impl BattleScenarioConfig {
    pub fn scenario_a() -> Self {
        Self {
            active: ACTIVE_SCENARIO,
            player_army: vec![("Spear".to_string(), 50)],
            enemy_army: vec![("Shield".to_string(), 50)],
        }
    }
}

fn skip_to_battle_if_scenario_active(
    config: Res<BattleScenarioConfig>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if config.active && *current_state == GameState::NewGame {
        next_state.set(GameState::Battle);
    }
}

fn switch_to_prebattle(mut level_selection: ResMut<LevelSelection>) {
    *level_selection = LevelSelection::Identifier("PreBattle".to_string());
}

fn spawn_scenario_armies(mut commands: Commands, config: Res<BattleScenarioConfig>) {
    // Spawn player army on the right side
    let player_x_offset = 300.0;
    for (prefab_name, count) in &config.player_army {
        spawn_army_formation(
            &mut commands,
            prefab_name,
            *count,
            player_x_offset,
            true, // is player
        );
    }

    // Spawn enemy army on the left side
    let enemy_x_offset = -300.0;
    for (prefab_name, count) in &config.enemy_army {
        spawn_army_formation(
            &mut commands,
            prefab_name,
            *count,
            enemy_x_offset,
            false, // is enemy
        );
    }
}

fn spawn_army_formation(
    commands: &mut Commands,
    prefab_name: &str,
    total_units: usize,
    x_offset: f32,
    is_player: bool,
) {
    let step = UNIT_SIZE + UNIT_SPACING;
    let rows = (total_units as f32 / GRID_WIDTH as f32).ceil() as usize;
    let offset_x = -((GRID_WIDTH as f32 - 1.0) * step) / 2.0;
    let offset_y = -((rows as f32 - 1.0) * step) / 2.0;

    let squad_name = if is_player {
        format!("Player_{}_Squad", prefab_name)
    } else {
        format!("Enemy_{}_Squad", prefab_name)
    };

    // Spawn squad entity with marker + data components
    // unit_count: 0 because units are spawned manually below
    let squad_entity = if is_player {
        commands
            .spawn((
                PlayerSquad,
                Squad {
                    child_prefab_name: prefab_name.to_string(),
                    unit_count: 0,
                },
                Name::new(squad_name),
                Transform::from_xyz(x_offset, 0.0, 0.0),
            ))
            .id()
    } else {
        commands
            .spawn((
                EnemySquad,
                Squad {
                    child_prefab_name: prefab_name.to_string(),
                    unit_count: 0,
                },
                Name::new(squad_name),
                Transform::from_xyz(x_offset, 0.0, 0.0),
            ))
            .id()
    };

    // Spawn units and collect their entities
    let mut unit_entities = Vec::with_capacity(total_units);
    for i in 0..total_units {
        let col = i % GRID_WIDTH;
        let row = i / GRID_WIDTH;

        let x = offset_x + (col as f32 * step);
        let y = offset_y + (row as f32 * step);

        let entity = if is_player {
            commands
                .spawn_prefab(prefab_name)
                .insert((
                    Transform::from_xyz(x, y, 0.0),
                    PlayerUnit,
                    Name::new(format!("Unit_{}", i)),
                ))
                .id()
        } else {
            commands
                .spawn_prefab(prefab_name)
                .insert((
                    Transform::from_xyz(x, y, 0.0),
                    EnemyUnit,
                    Name::new(format!("Unit_{}", i)),
                ))
                .id()
        };

        unit_entities.push(entity);
    }

    // Add units as children of the squad
    commands.entity(squad_entity).add_children(&unit_entities);
}
