
use crate::prelude::*;

/// Marker component for the persistent UI root node
#[derive(Component)]
pub struct PersistentUiRoot;

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_persistent_ui_root);
}

pub(super) fn spawn_persistent_ui_root(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            ..default()
        },
        PersistentUiRoot,
        Name::new("Persistent UI Root"),
        Pickable::IGNORE,
    ));
}
