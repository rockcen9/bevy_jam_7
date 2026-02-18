use bevy_image_font::{
    ImageFont, ImageFontText, LetterSpacing, atlas_sprites::ImageFontSpriteText,
};
use bevy_tweening::{Tween, TweenAnim, lens::TransformPositionLens, *};

use crate::{asset_tracking::LoadResource, prelude::*, screens::Screen};

#[derive(Component)]
#[require(DespawnOnExit::<GameState>(GameState::Battle))]
struct FloatDamageText;

pub(crate) fn plugin(app: &mut App) {
    app.load_resource::<FontAssets>();
    app.add_systems(
        Update,
        (spawn_float_damage, despawn_float_damage).run_if(in_state(Screen::Gameplay)),
    );
}
#[allow(dead_code)]
#[derive(Resource, Asset, Clone, TypePath)]
struct FontAssets {
    #[dependency]
    font: Handle<ImageFont>,
}

impl FromWorld for FontAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            font: assets.load("image_font/example_variable_width_font.image_font.ron"),
        }
    }
}

fn spawn_float_damage(
    mut ev_damage: MessageReader<TakeDamageMessage>,
    q_transform: Query<(&GlobalTransform, Option<&EnemyUnit>)>,
    mut commands: Commands,
    assets: Res<AssetServer>,
    palette: Res<ColorPalette>,
) {
    for ev in ev_damage.read() {
        let Ok((global_transform, enemy_unit)) = q_transform.get(ev.target) else {
            continue;
        };

        let pos = global_transform.translation();
        let start = Vec3::new(pos.x, pos.y + 32.0, 10.0);
        let end = Vec3::new(pos.x, pos.y + 92.0, 10.0);

        let tween = Tween::new(
            EaseFunction::QuadraticOut,
            std::time::Duration::from_secs_f32(0.5),
            TransformPositionLens { start, end },
        );

        let color = if enemy_unit.is_some() {
            Color::WHITE
        } else {
            palette.brown_medium_red
        };

        commands.spawn((
            FloatDamageText,
            ImageFontSpriteText::default()
                .color(color)
                .letter_spacing(LetterSpacing::Pixel(2)),
            ImageFontText::default()
                .text(format!("{}", ev.damage as i32))
                .font(assets.load("image_font/example_variable_width_font.image_font.ron"))
                .font_height(if ev.damage >= 10.0 { 48.0 } else { 24.0 }),
            Transform::from_translation(start),
            TweenAnim::new(tween),
        ));
    }
}

fn despawn_float_damage(
    mut anim_completed: MessageReader<AnimCompletedEvent>,
    q_float: Query<Entity, With<FloatDamageText>>,
    mut commands: Commands,
) {
    for event in anim_completed.read() {
        if q_float.get(event.anim_entity).is_ok() {
            commands.entity(event.anim_entity).despawn();
        }
    }
}
