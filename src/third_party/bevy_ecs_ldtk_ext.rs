use crate::prelude::*;
use crate::screens::{Screen, loading::LoadingScreen};
use bevy_ecs_ldtk::{
    EntityInstance, IntGridRendering, LdtkEntity, LdtkPlugin, LdtkProjectHandle, LdtkSettings,
    LdtkWorldBundle, LevelEvent, LevelIid, LevelSelection, LevelSpawnBehavior,
    assets::{LdtkProject, LevelMetadataAccessor},
    prelude::LdtkFields,
};
pub(crate) fn plugin(app: &mut bevy::app::App) {
    app.add_plugins(LdtkPlugin)
        .insert_resource(LevelSelection::default());
    app.insert_resource(LdtkSettings {
        level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
            load_level_neighbors: false,
        },
        level_background: bevy_ecs_ldtk::LevelBackground::Nonexistent,
        int_grid_rendering: IntGridRendering::Invisible,
        ..Default::default()
    });
    app.add_systems(OnEnter(LoadingScreen::Level), setup_level);
    app.add_systems(OnEnter(Screen::Title), despawn_ldtk_world);
    // app.add_systems(Update, list_all_levels);
    app.add_systems(Update, camera_follow_level_on_load);
}
fn setup_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ldtk_handle = asset_server.load("chaos_dream.ldtk").into();

    commands.spawn(LdtkWorldBundle {
        ldtk_handle,
        ..Default::default()
    });
}

fn despawn_ldtk_world(
    mut commands: Commands,
    q_world: Query<Entity, With<LdtkProjectHandle>>,
) {
    for entity in q_world.iter() {
        commands.entity(entity).despawn();
    }
}
#[derive(Default, Bundle, LdtkEntity)]
pub struct LDTKBundle {
    #[with(name_from_field)]
    name: Name,
    require_spawn: RequireSpawnPrefab,
}
pub fn name_from_field(entity_instance: &EntityInstance) -> Name {
    Name::new(
        entity_instance
            .get_string_field("name")
            .expect("expected entity to have non-nullable name string field")
            .clone(),
    )
}
fn camera_follow_level_on_load(
    mut level_messages: MessageReader<LevelEvent>,
    level_selection: Res<LevelSelection>,
    ldtk_projects: Query<&LdtkProjectHandle>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    levels: Query<(&LevelIid, &GlobalTransform)>,
    mut camera: Query<&mut Transform, With<MainCamera>>,
) {
    for message in level_messages.read() {
        let LevelEvent::Transformed(transformed_iid) = message else {
            continue;
        };

        debug!(
            "LevelEvent::Transformed received for IID: {:?}",
            transformed_iid
        );

        let Ok(ldtk_handle) = ldtk_projects.single() else {
            warn!(
                "camera_follow_level_on_load: Expected exactly one LdtkProject entity, found {}",
                ldtk_projects.iter().count()
            );
            return;
        };

        let Some(ldtk_project) = ldtk_project_assets.get(ldtk_handle) else {
            warn!("camera_follow_level_on_load: LdtkProject asset not found for handle");
            return;
        };

        // Find the selected level's raw data to get its size
        let Some(selected_level) = ldtk_project.find_raw_level_by_level_selection(&level_selection)
        else {
            warn!(
                "camera_follow_level_on_load: Could not find raw level for selection: {:?}",
                *level_selection
            );
            return;
        };

        // Only move camera if the transformed level matches the current selection
        if transformed_iid.get() != &selected_level.iid {
            debug!(
                "camera_follow_level_on_load: Transformed level IID {:?} does not match selected level IID {}",
                transformed_iid, selected_level.iid
            );
            continue;
        }

        debug!("camera_follow_level_on_load: Main Camera moved to level center");

        // Find the spawned level entity matching the transformed level
        for (level_iid, level_transform) in levels.iter() {
            if level_iid == transformed_iid {
                // Calculate level center
                let level_pos = level_transform.translation();
                let center_x = level_pos.x + selected_level.px_wid as f32 / 2.0;
                let center_y = level_pos.y + selected_level.px_hei as f32 / 2.0;

                // Move camera to level center
                if let Ok(mut camera_transform) = camera.single_mut() {
                    camera_transform.translation.x = center_x;
                    camera_transform.translation.y = center_y;
                } else {
                    warn!(
                        "camera_follow_level_on_load: MainCamera not found or multiple cameras found"
                    );
                }
                break;
            }
        }
    }
}

// fn list_all_levels(
//     keys: Res<ButtonInput<KeyCode>>,
//     mut level_selection: ResMut<LevelSelection>,
//     ldtk_projects: Query<&LdtkProjectHandle>,
//     ldtk_project_assets: Res<Assets<LdtkProject>>,
//     levels: Query<(&LevelIid, &GlobalTransform)>,
// ) -> Result {
//     if keys.just_pressed(KeyCode::Space) {
//         // Setup level example
//         *level_selection = LevelSelection::Identifier("PreBattle".to_string());

//         let ldtk_project = ldtk_project_assets
//             .get(ldtk_projects.single()?)
//             .expect("ldtk project should be loaded before player is spawned");

//         for (level_iid, _level_transform) in levels.iter() {
//             let level = ldtk_project
//                 .get_raw_level_by_iid(level_iid.get())
//                 .expect("level should exist in only project");

//             dbg!(level.identifier.clone());
//         }
//     }
//     Ok(())
// }
