use crate::prelude::*;

#[derive(Component)]
pub struct PrepareRootNode;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PrepareUiSets {
    SpawnRoot,
    SpawnChildren,
}

pub(crate) fn plugin(app: &mut App) {
    app.configure_sets(
        OnEnter(GameState::Preparing),
        (PrepareUiSets::SpawnRoot, PrepareUiSets::SpawnChildren).chain(),
    )
    .add_systems(
        OnEnter(GameState::Preparing),
        spawn_prepare_root.in_set(PrepareUiSets::SpawnRoot),
    );
}

/// Spawn the root UI container for the prepare state
fn spawn_prepare_root(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            PrepareRootNode,
            Pickable::IGNORE,
            DespawnOnExit(GameState::Preparing),
        ))
        .with_children(|_parent| {
            // UI elements will be spawned as children of this root node
        });
}
