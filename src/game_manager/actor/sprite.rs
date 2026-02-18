// use bevy_aseprite_ultra::prelude::{Animation, AseAnimation};
// use bevy_prefab::PrefabId;

use crate::prelude::*;

#[derive(Component, Default)]
#[require(RequiredAseprite, Actor)]
pub struct SpriteActor;

#[derive(Component)]
#[require(Visibility)]
pub struct MainMesh;
#[derive(Component, Default)]
pub struct RequiredAseprite;

pub(crate) fn plugin(app: &mut bevy::app::App) {
    app.add_observer(setup_mesh);
    app.add_systems(Update, setup_sprite);
}

pub fn setup_sprite(
    q_actor: Query<Entity, With<RequiredAseprite>>,
    q_belong_to: Query<(Entity, &BelongTo), With<MainMesh>>,
    q_prefab: Query<&PrefabId>,
    q_enemy: Query<(), With<EnemyUnit>>,
    mut commands: Commands,
    server: Res<AssetServer>,
) {
    for (main_mesh, belong_to) in q_belong_to.iter() {
        let Ok(actor) = q_actor.get(belong_to.0) else {
            continue;
        };
        let Ok(prefab_id) = q_prefab.get(actor) else {
            continue;
        };

        let postfix = if q_enemy.get(actor).is_ok() {
            "Enemy"
        } else {
            ""
        };
        let path = format!("procreate/{}{}.png", prefab_id.id, postfix);
        commands.entity(main_mesh).insert((
            Pickable::default(),
            Sprite::from_image(server.load(path)),
            SpriteLayer::Pawn,
        ));

        commands.entity(actor).remove::<RequiredAseprite>();
    }
}
