use bevy_aseprite_ultra::prelude::{Animation, AnimationEvents, AnimationRepeat, AseAnimation};

use crate::prelude::*;

pub(crate) fn plugin(app: &mut bevy::app::App) {
    app.add_observer(setup_aseprite);
    app.add_systems(Update, despawn_on_animation_finish);
}

fn despawn_on_animation_finish(
    mut events: MessageReader<AnimationEvents>,
    mut commands: Commands,
    q_vfx: Query<&AseVfxEntity>,
) {
    for event in events.read() {
        match event {
            AnimationEvents::Finished(entity) => {
                if q_vfx.contains(*entity) {
                    commands.entity(*entity).despawn();
                }
            }
            AnimationEvents::LoopCycleFinished(_) => {}
        }
    }
}

/// Marker component for VFX entities that should despawn when animation finishes
#[derive(Component)]
pub struct AseVfxEntity;

#[derive(Event)]
pub struct AseVfxEvent {
    pub id: String,
    pub position: Vec2,
    pub scale: f32,
}

impl AseVfxEvent {
    pub fn new(id: impl Into<String>, position: Vec2) -> Self {
        Self {
            id: id.into(),
            position,
            scale: 1.0,
        }
    }
    pub fn with_scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }
}

pub fn setup_aseprite(trigger: On<AseVfxEvent>, server: Res<AssetServer>, mut commands: Commands) {
    let tag = trigger.event().id.clone();
    let pos = trigger.event().position;
    let scale = trigger.event().scale;
    commands.spawn((
        AseVfxEntity,
        AseAnimation {
            animation: Animation::tag(&tag).with_repeat(AnimationRepeat::Count(0)),
            aseprite: server.load("aseprite/TinySwordVFX.aseprite"),
        },
        Transform::from_xyz(pos.x, pos.y, 0.0).with_scale(Vec3::splat(scale)),
        Sprite::default(),
        SpriteLayer::VFX,
    ));
}
