use crate::prelude::*;

// mod ra;
// pub use ra::{RA, RALifetime};
pub(crate) mod big_eye;
pub(crate) use big_eye::*;
// pub(crate) mod big_hand;
// pub(crate) use big_hand::*;
mod golden_heart;
pub use golden_heart::GoldenHeart;
mod squad_hit_count;
pub(crate) use squad_hit_count::*;
mod squad_take_hit_count;
pub(crate) use squad_take_hit_count::*;

pub(crate) fn plugin(app: &mut App) {
    // ra::plugin(app);
    big_eye::plugin(app);
    // big_hand::plugin(app);
    golden_heart::plugin(app);
    squad_hit_count::plugin(app);
    squad_take_hit_count::plugin(app);
    app.add_systems(Update, despawn_expired_memory);
}

#[derive(Component, Default)]
pub struct MemoryBuff;

#[derive(Component)]
pub struct Memory {
    lifetime: Timer,
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            lifetime: Timer::from_seconds(5.0, TimerMode::Once),
        }
    }
}

impl Memory {
    pub fn with_seconds(seconds: f32) -> Self {
        Self {
            lifetime: Timer::from_seconds(seconds, TimerMode::Once),
        }
    }
}

fn despawn_expired_memory(
    mut commands: Commands,
    time: Res<Time>,
    mut q_memory: Query<(Entity, &mut Memory)>,
) {
    for (entity, mut memory) in &mut q_memory {
        memory.lifetime.tick(time.delta());
        if memory.lifetime.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}

#[derive(Component, Default, Reflect)]
#[require(Memory, Faction::Player, OriginalColor(Color::WHITE))]
pub struct PlayerMemory;

#[derive(Component, Default, Reflect)]
#[require(
    Memory,
    Faction::Player,
    OriginalColor(Color::linear_rgb(1., 0.3, 0.3))
)]
pub struct EnemyMemory;

#[derive(Component, Reflect, Debug)]
#[relationship(relationship_target = RootStationMemory)]
pub struct BelongToMemory(pub Entity);

#[derive(Component, Deref, Default, Reflect)]
#[relationship_target(relationship = BelongToMemory)]
pub struct RootStationMemory(Vec<Entity>);
