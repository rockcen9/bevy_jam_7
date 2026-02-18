use crate::prelude::*;

#[derive(Message)]
pub struct VFXMessage {
    pub target: Entity,
    pub vfx_type: VFXType,
}
pub enum VFXType {
    #[allow(dead_code)]
    Damage,
}

#[derive(Component)]
pub struct DamageFlash {
    timer: Timer,
    original_color: Color,
}

pub(crate) fn plugin(app: &mut bevy::app::App) {
    app.add_message::<VFXMessage>();
    app.add_systems(Update, (handle_vfx_message, tick_damage_flash));
}

fn handle_vfx_message(
    mut messages: MessageReader<VFXMessage>,
    mut q_sprite: Query<(&mut Sprite, &BelongTo), With<MainMesh>>,
    mut commands: Commands,
) {
    for message in messages.read() {
        if !matches!(message.vfx_type, VFXType::Damage) {
            continue;
        }

        for (mut sprite, belong_to) in q_sprite.iter_mut() {
            if belong_to.0 != message.target {
                continue;
            }

            let original_color = Color::WHITE;
            // Use bright HDR white to create flash effect
            sprite.color = Color::linear_rgb(10.0, 10.0, 10.0);

            commands.entity(belong_to.0).insert(DamageFlash {
                timer: Timer::from_seconds(0.2, TimerMode::Once),
                original_color,
            });
        }
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
