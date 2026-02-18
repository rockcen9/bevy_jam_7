use crate::prelude::*;

pub(crate) fn plugin(_app: &mut App) {
    //todo
    // app.add_systems(Update, deepen_color_system);
}

#[derive(Component, Default, Reflect)]
#[require(Unit, Faction::Enemy, OriginalColor(Color::WHITE))]
pub struct EnemyUnit;

// fn deepen_color_system(
//     q_enemy: Query<Entity, With<EnemyUnit>>,
//     mut q_sprite: Query<(&mut Sprite, &BelongTo), (With<MainMesh>, Added<Sprite>)>,
// ) {
//     for (mut sprite, belong_to) in &mut q_sprite {
//         if q_enemy.get(belong_to.0).is_err() {
//             continue;
//         }
//         let mut hsla = Hsla::from(sprite.color);
//         hsla.lightness -= 0.2;
//         hsla.saturation += 0.2;
//         sprite.color = Color::from(hsla);
//     }
// }
