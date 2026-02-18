
use crate::prelude::*;

#[derive(Component)]
pub struct BattleEndRootNode;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum BattleEndUiSets {
    SpawnRoot,
    SpawnChildren,
}

pub(crate) fn plugin(app: &mut App) {
    app.configure_sets(
        OnEnter(GameState::WinAndNextDay),
        (BattleEndUiSets::SpawnRoot, BattleEndUiSets::SpawnChildren).chain(),
    )
    .add_systems(
        OnEnter(GameState::WinAndNextDay),
        spawn_battle_end_root.in_set(BattleEndUiSets::SpawnRoot),
    );
}

/// Spawn the root UI container for the battle end state
fn spawn_battle_end_root(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BattleEndRootNode,
        DespawnOnEnter(GameState::Preparing),
        Pickable::IGNORE,
    ));
}
