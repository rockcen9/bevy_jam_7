use crate::prelude::*;

const UNIT_SIZE: f32 = 64.0;
const UNIT_SPACING: f32 = 10.0;
const GRID_WIDTH: usize = 10;

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(Update, (spawn_units_for_new_squads, initialize_squad_count));
}

/// System that watches for new Squad components and spawns their units
fn spawn_units_for_new_squads(
    mut commands: Commands,
    new_squads: Query<(Entity, &Squad, &Faction), Added<Squad>>,
) {
    for (squad_entity, squad, faction) in new_squads.iter() {
        if squad.max_unit_count == 0 {
            continue;
        }

        let unit_entities = spawn_units_in_formation(
            &mut commands,
            squad.max_unit_count,
            &squad.child_prefab_name,
            *faction,
            squad_entity,
        );
        commands.entity(squad_entity).add_children(&unit_entities);

        info!(
            "Spawned {} units for {:?} {} squad {:?}",
            squad.max_unit_count, faction, squad.child_prefab_name, squad_entity
        );
    }
}

/// Spawns units in a grid formation as child entities
fn spawn_units_in_formation(
    commands: &mut Commands,
    total_units: usize,
    prefab_name: &str,
    faction: Faction,
    squad_entity: Entity,
) -> Vec<Entity> {
    let step = UNIT_SIZE + UNIT_SPACING;
    let rows = (total_units as f32 / GRID_WIDTH as f32).ceil() as usize;
    let offset_x = -((GRID_WIDTH as f32 - 1.0) * step) / 2.0;
    let offset_y = -((rows as f32 - 1.0) * step) / 2.0;

    let mut entities = Vec::with_capacity(total_units);

    for index in 0..total_units {
        let col = index % GRID_WIDTH;
        let row = index / GRID_WIDTH;

        let x = offset_x + (col as f32 * step);
        let y = offset_y + (row as f32 * step);

        let mut entity_commands = commands.spawn_prefab(prefab_name);
        entity_commands.insert((
            BelongToSquad(squad_entity),
            Transform::from_xyz(x, y, 0.0),
            Name::new(format!("{}_{}", prefab_name, index)),
        ));

        match faction {
            Faction::Player => entity_commands.insert(PlayerUnit),
            Faction::Enemy => entity_commands.insert(EnemyUnit),
        };

        entities.push(entity_commands.id());
    }

    entities
}

/// Initializes current_unit_count to match max_unit_count for newly created squads
fn initialize_squad_count(mut q_new_squads: Query<&mut Squad, Added<Squad>>) {
    for mut squad in &mut q_new_squads {
        squad.current_unit_count = squad.max_unit_count;
    }
}
