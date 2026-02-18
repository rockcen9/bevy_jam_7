use bevy_rand::{global::GlobalRng, prelude::ChaCha8Rng};
use extol_sprite_layer::LayerIndex;
use rock_particles::{ComboRingColor, ComboRingEvent};
use rand::Rng;

use crate::prelude::*;
pub(crate) fn plugin(app: &mut bevy::app::App) {
    app.add_systems(Update, (corpse_system.in_set(AttackSet::Corpse),));
}

#[derive(Component, Default)]
pub struct Corpse;
fn corpse_system(
    mut q_enemy_units: Query<(&Health, &GlobalTransform), (With<EnemyUnit>, Without<PlayerUnit>)>,
    mut q_player_units: Query<(&Health, &GlobalTransform), (With<PlayerUnit>, Without<EnemyUnit>)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut rng: Single<&mut ChaCha8Rng, With<GlobalRng>>,
) {
    for (health, transform) in &mut q_enemy_units {
        if health.get_current() == 100.0 {
            continue;
        }
        if !health.is_alive() {
            let corpse_num = rng.random_range(1..=4);
            let corpse_path = format!("procreate/B{}.png", corpse_num);
            let random_rotation = rng.random_range(0.0..std::f32::consts::TAU);
            commands.spawn((
                Name::new("Corpse"),
                Corpse,
                Transform::from_translation(
                    transform
                        .translation()
                        .truncate()
                        .extend(SpriteLayer::Corpse.as_z_coordinate()),
                )
                .with_rotation(Quat::from_rotation_z(random_rotation)),
                Sprite::from_image(asset_server.load(&corpse_path)),
            ));
            commands.trigger(ComboRingEvent::new(
                transform.translation().truncate(),
                ComboRingColor::Purple,
            ));
            // commands.trigger(HanabiEvent::new(
            //     "poison",
            //     transform.translation().truncate(),
            // ));
        }
    }

    for (health, transform) in &mut q_player_units {
        if health.get_current() == 100.0 {
            continue;
        }
        if !health.is_alive() {
            let corpse_num = rng.random_range(1..=4);
            let corpse_path = format!("procreate/C{}.png", corpse_num);
            let random_rotation = rng.random_range(0.0..std::f32::consts::TAU);
            commands.spawn((
                Name::new("Corpse"),
                Corpse,
                Transform::from_translation(
                    transform
                        .translation()
                        .truncate()
                        .extend(SpriteLayer::Corpse.as_z_coordinate()),
                )
                .with_rotation(Quat::from_rotation_z(random_rotation)),
                Sprite::from_image(asset_server.load(&corpse_path)),
            ));
            commands.trigger(ComboRingEvent::new(
                transform.translation().truncate(),
                ComboRingColor::Pink,
            ));
            // commands.trigger(HanabiEvent::new(
            //     "poison",
            //     transform.translation().truncate(),
            // ));
        }
    }
}
