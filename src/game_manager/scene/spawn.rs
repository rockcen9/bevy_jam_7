
use crate::prelude::*;

#[derive(Component, Default)]
pub struct RequireSpawnPrefab;

pub(crate) fn plugin(_app: &mut bevy::app::App) {
    // app.add_systems(Update, spawn_prefab_replacement);
}

// pub fn spawn_prefab_replacement(
//     mut level_messages: MessageReader<LevelEvent>,
//     q_required: Query<(
//         Entity,
//         &RequireSpawnPrefab,
//         &Name,
//         &GlobalTransform,
//         &ChildOf,
//     )>,
//     q_levels: Query<&LevelIid>,
//     q_parents: Query<&ChildOf>,
//     mut commands: Commands,
// ) {
//     for message in level_messages.read() {
//         let LevelEvent::Transformed(transformed_iid) = message else {
//             continue;
//         };
//         for (entity, _require, name, global, child_of) in q_required.iter() {
//             // Traverse up hierarchy to find the level (Level -> Layer -> Entity)
//             let level_iid = find_ancestor_level(child_of.parent(), &q_levels, &q_parents);
//             let Some(level_iid) = level_iid else {
//                 continue;
//             };
//             if level_iid != transformed_iid {
//                 continue;
//             }
//             commands
//                 .spawn_prefab(name.to_string())
//                 .insert(Transform::from_translation(global.translation()));
//             commands.entity(entity).despawn();
//         }
//     }
// }

// fn find_ancestor_level<'a>(
//     mut current: Entity,
//     q_levels: &'a Query<&LevelIid>,
//     q_parents: &Query<&ChildOf>,
// ) -> Option<&'a LevelIid> {
//     // Walk up the hierarchy until we find an entity with LevelIid
//     loop {
//         if let Ok(level_iid) = q_levels.get(current) {
//             return Some(level_iid);
//         }
//         if let Ok(child_of) = q_parents.get(current) {
//             current = child_of.parent();
//         } else {
//             return None;
//         }
//     }
// }
