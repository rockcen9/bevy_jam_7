use crate::prelude::*;

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(Update, apply_original_color);
}

fn apply_original_color(
    q_actor: Query<&OriginalColor>,
    mut q_main_mesh: Query<(&mut Sprite, &BelongTo), (With<MainMesh>, Added<Sprite>)>,
) {
    for (mut sprite, belong_to) in q_main_mesh.iter_mut() {
        if let Ok(original_color) = q_actor.get(belong_to.0) {
            sprite.color = original_color.0;
        }
    }
}
