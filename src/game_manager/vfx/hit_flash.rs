use crate::prelude::*;

#[derive(Event)]
pub struct HitFlashVfxEvent {
    pub target: Entity,
}

#[derive(Component)]
pub struct DamageFlash {
    timer: Timer,
    original_color: Color,
}

pub(crate) fn plugin(app: &mut bevy::app::App) {
    app.add_observer(on_hit_flash_event);
    app.add_systems(Update, tick_damage_flash);
}

fn on_hit_flash_event(
    event: On<HitFlashVfxEvent>,
    mut q_sprite: Query<(&mut Sprite, &BelongTo), With<MainMesh>>,
    q_original_color: Query<&OriginalColor>,
    q_player: Query<(), With<PlayerUnit>>,
    q_enemy: Query<(), With<EnemyUnit>>,
    mut commands: Commands,
    _palette: Res<ColorPalette>,
) {
    let target = event.target;

    let flash_color = if q_player.get(target).is_ok() {
        // PlayerUnit flashes pink
        Color::linear_rgb(10.0, 2.0, 8.0)
    } else if q_enemy.get(target).is_ok() {
        // EnemyUnit flashes white
        Color::linear_rgb(10.0, 10.0, 10.0)
    } else {
        Color::linear_rgb(10.0, 10.0, 10.0)
    };

    for (mut sprite, belong_to) in q_sprite.iter_mut() {
        if belong_to.0 != target {
            continue;
        }

        let original_color = q_original_color
            .get(belong_to.0)
            .map(|c| c.0)
            .unwrap_or(Color::WHITE);

        sprite.color = flash_color;

        commands.entity(belong_to.0).insert(DamageFlash {
            timer: Timer::from_seconds(0.05, TimerMode::Once),
            original_color,
        });
    }
}

fn tick_damage_flash(
    time: Res<Time>,
    mut q_flash: Query<(Entity, &mut DamageFlash)>,
    mut q_sprite: Query<(&mut Sprite, &BelongTo), With<MainMesh>>,
    mut commands: Commands,
) {
    for (entity, mut flash) in q_flash.iter_mut() {
        flash.timer.tick(time.delta());

        if flash.timer.is_finished() {
            for (mut sprite, belong_to) in q_sprite.iter_mut() {
                if belong_to.0 == entity {
                    sprite.color = flash.original_color;
                }
            }
            commands.entity(entity).remove::<DamageFlash>();
        }
    }
}
