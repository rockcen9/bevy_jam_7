use bevy_rand::{global::GlobalRng, prelude::ChaCha8Rng};
use rand::Rng;

use crate::{game_manager::ui::prepare_state::bottom_middle::FollowingCursor, prelude::*};

#[derive(Component, Default)]
pub struct RequireShadowSprite;

#[derive(Component)]
#[require(Name::new("Shadow"), Visibility)]
pub struct ShadowSprite;

pub fn plugin(app: &mut bevy::app::App) {
    app.add_systems(Update, setup_shadow_sprite);
    app.add_systems(
        Update,
        update_shadow_visibility.run_if(in_state(GameState::Preparing)),
    );
}

fn setup_shadow_sprite(
    q_actor: Query<Entity, With<RequireShadowSprite>>,
    q_belong_to: Query<(Entity, &BelongTo), With<MainMesh>>,
    mut rng: Single<&mut ChaCha8Rng, With<GlobalRng>>,
    mut commands: Commands,
    server: Res<AssetServer>,
) {
    for (main_mesh, belong_to) in q_belong_to.iter() {
        let Ok(_actor) = q_actor.get(belong_to.0) else {
            continue;
        };

        let shadow_num = rng.random_range(1..=4);
        let path = format!("procreate/Shadow{}.png", shadow_num);
        commands.spawn((
            Name::new("Shadow"),
            Transform::default().with_translation(Vec3::new(0., -48., -100.)),
            Sprite::from_image(server.load(path)),
            ShadowSprite,
            SpriteLayer::Shadow,
            ChildOf(main_mesh),
        ));
        commands.entity(belong_to.0).remove::<RequireShadowSprite>();
    }
}

fn update_shadow_visibility(
    q_shadow: Query<(Entity, &ChildOf), With<ShadowSprite>>,
    q_main_mesh: Query<&BelongTo, With<MainMesh>>,
    q_actor: Query<&BelongToSquad>,
    q_squad: Query<(Has<SelectSquad>, Has<FollowingCursor>)>,
    mut commands: Commands,
) {
    for (shadow, child_of) in q_shadow.iter() {
        let Ok(belong_to) = q_main_mesh.get(child_of.0) else {
            continue;
        };
        let Ok(belong_to_squad) = q_actor.get(belong_to.0) else {
            continue;
        };
        let Ok((has_select_squad, has_follow_cursor)) = q_squad.get(belong_to_squad.0) else {
            continue;
        };

        if has_select_squad || has_follow_cursor {
            commands.entity(shadow).insert(Visibility::Hidden);
        } else {
            commands.entity(shadow).insert(Visibility::Visible);
        }
    }
}
