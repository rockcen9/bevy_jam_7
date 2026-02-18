use bevy_ecs_ldtk::{app::LdtkEntityAppExt, prelude::LdtkFields, *};

use crate::prelude::*;

pub fn plugin(app: &mut App) {
    app.register_ldtk_entity::<EnemySquadBundle>("EnemySquad")
        .add_systems(Update, reset_enemy_squad_scale);
}

fn reset_enemy_squad_scale(mut query: Query<&mut Transform, Added<EnemySquad>>) {
    for mut transform in &mut query {
        transform.scale = Vec3::ONE;
    }
}

#[derive(Component, Default, Reflect)]
#[require(Transform, Visibility, SpriteLayer::Pawn, Faction::Enemy)]
pub struct EnemySquad;

#[derive(Default, Bundle, LdtkEntity)]
pub struct EnemySquadBundle {
    #[with(name_from_field)]
    name: Name,
    #[with(squad_from_field)]
    squad: Squad,
    enemy_squad: EnemySquad,
}

pub fn squad_from_field(entity_instance: &EntityInstance) -> Squad {
    Squad::new(
        entity_instance
            .get_string_field("unit_prefab")
            .expect("expected entity to have non-nullable name string field")
            .clone(),
        (entity_instance
            .get_int_field("unit_count")
            .expect("expected entity to have non-nullable unit_count int field")
            .clone()) as usize,
    )
}
