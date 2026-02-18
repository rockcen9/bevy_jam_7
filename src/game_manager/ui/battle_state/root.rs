
use crate::prelude::*;

#[derive(Component)]
pub struct BattleRootNode;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum BattleUiSets {
    SpawnRoot,
    SpawnChildren,
}

pub(crate) fn plugin(app: &mut App) {
    app.configure_sets(
        OnEnter(GameState::Battle),
        (BattleUiSets::SpawnRoot, BattleUiSets::SpawnChildren).chain(),
    )
    .add_systems(
        OnEnter(GameState::Battle),
        spawn_battle_root.in_set(BattleUiSets::SpawnRoot),
    );
}

/// Spawn the root UI container for the battle state
fn spawn_battle_root(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            ..default()
        },
        BattleRootNode,
        DespawnOnEnter(GameState::Preparing),
        Pickable::IGNORE,
    ));
}
