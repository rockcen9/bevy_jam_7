use bevy::picking::prelude::*;
use bevy_ecs_ldtk::app::LdtkEntityAppExt;

use crate::prelude::*;

pub(crate) fn plugin(app: &mut App) {
    app.register_ldtk_entity::<LDTKBundle>("SelectSquad");

    app.add_systems(
        Update,
        show_select_squad.run_if(in_state(GameState::Preparing)),
    );
    app.add_systems(OnExit(GameState::Preparing), hide_select_squad);
    app.add_observer(on_drag_start);
    app.add_observer(on_drag);
    app.add_observer(on_drag_end);
}

#[derive(Component, Default, Prefab, Reflect)]
#[require(SpriteLayer::SelectSquad)]
pub struct SelectSquad;

#[derive(Component, Reflect)]
pub struct SquadOriginalPosition(pub Vec3);

fn show_select_squad(mut q_select_squad: Query<&mut Visibility, (With<SelectSquad>, With<Squad>)>) {
    for mut visibility in &mut q_select_squad {
        *visibility = Visibility::Visible;
    }
}

fn hide_select_squad(mut q_select_squad: Query<&mut Visibility, With<SelectSquad>>) {
    for mut visibility in &mut q_select_squad {
        *visibility = Visibility::Hidden;
    }
}

// ============================================================================
// Drag & Drop Observers
// ============================================================================

fn on_drag_start(
    drag_start: On<Pointer<DragStart>>,
    q_main_mesh: Query<&BelongTo, With<MainMesh>>,
    q_unit_belong_to: Query<&BelongToSquad, (Without<MainMesh>, With<PlayerUnit>)>,
    q_squad_transform: Query<&Transform>,
    mut commands: Commands,
) {
    debug!("Drag started on MainMesh {:?}", drag_start.entity);
    // MainMesh -> Unit
    let Ok(belong_to_unit) = q_main_mesh.get(drag_start.entity) else {
        return;
    };
    let unit_entity = belong_to_unit.0;

    // Unit -> SelectSquad
    debug!("Drag started on Unit {:?}", unit_entity);
    let Ok(belong_to_squad) = q_unit_belong_to.get(unit_entity) else {
        return;
    };
    let squad_entity = belong_to_squad.0;

    let original_pos = q_squad_transform
        .get(squad_entity)
        .map(|t| t.translation)
        .unwrap_or(Vec3::ZERO);

    debug!("Drag started on SelectSquad {:?}", squad_entity);
    commands
        .entity(squad_entity)
        .insert(SelectSquad)
        .insert(SquadOriginalPosition(original_pos))
        .insert(RequiredAnimation::Pick);
}

fn on_drag(
    drag: On<Pointer<Drag>>,

    mut q_select_squad: Query<&mut Transform, With<SelectSquad>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    let Ok(mut transform) = q_select_squad.single_mut() else {
        return;
    };

    let Ok((camera, camera_transform)) = camera_query.single() else {
        return;
    };

    let viewport_pos = drag.pointer_location.position;
    let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, viewport_pos) else {
        return;
    };

    // Update SelectSquad position - PlayerSquad is synced by sync_player_squad_transform
    transform.translation.x = world_pos.x;
    transform.translation.y = world_pos.y;
}

fn on_drag_end(
    _drag_end: On<Pointer<DragEnd>>,
    mut q_select_squad: Query<(Entity, &mut Transform, &SquadOriginalPosition), With<SelectSquad>>,
    in_boundary: Res<InBoundary>,
    mut commands: Commands,
) {
    let Ok((entity, mut transform, original_pos)) = q_select_squad.single_mut() else {
        return;
    };

    if !in_boundary.0 {
        transform.translation = original_pos.0;
        commands
            .entity(entity)
            .remove::<(SelectSquad, SquadOriginalPosition)>();
        return;
    }

    commands.trigger(CameraShakeEvent);
    commands.trigger(VfxEvent::dust(transform.translation.truncate()));
    commands
        .entity(entity)
        .insert((RequiredAnimation::Put, RunWithNoModel))
        .remove::<SquadOriginalPosition>();
    commands.entity(entity).remove::<SelectSquad>();
}
