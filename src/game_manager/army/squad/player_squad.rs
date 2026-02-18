use crate::prelude::*;

pub(crate) fn plugin(_app: &mut App) {}

#[derive(Component, Default, Reflect)]
#[require(Transform, Visibility, SpriteLayer::Pawn, Faction::Player)]
pub struct PlayerSquad;
#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct SquadOriginPosition(pub Vec2);
/// Spawns a PlayerSquad entity at the given world position.
/// Units will be spawned automatically by the `spawn_units_for_new_squads` system.
pub fn spawn_player_squad(
    commands: &mut Commands,
    prefab_name: &str,
    position: Vec2,
    unit_count: usize,
) -> Entity {
    commands
        .spawn((
            PlayerSquad,
            Squad::new(prefab_name.to_string(), unit_count),
            RootStationSquad::default(),
            Transform::from_xyz(position.x, position.y, 0.0),
            Name::new(format!("{}_Squad", prefab_name)),
        ))
        .id()
}
