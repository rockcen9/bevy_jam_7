use crate::prelude::*;

pub(crate) fn plugin(app: &mut bevy::app::App) {
    app.add_systems(Update, tick_poison_debuff);
}

/// Ticks poison debuff timer. Each tick deals damage_per_tick * stacks, then decrements stacks.
/// When stacks reach 0, the poison stops.
fn tick_poison_debuff(time: Res<Time>, mut query: Query<(&mut ActiveDeBuffs, &mut Health)>) {
    for (mut debuffs, mut health) in &mut query {
        for debuff in &mut debuffs.list {
            if let DebuffEffect::Poison(data) = debuff {
                if data.stacks == 0 {
                    continue;
                }
                data.timer.tick(time.delta());
                if data.timer.just_finished() {
                    let damage = data.damage_per_tick * data.stacks as f32;
                    health.take_damage(damage);
                    data.stacks -= 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    /// Helper to manually tick poison timer and apply damage (for testing)
    fn manual_poison_tick(world: &mut World, entity: Entity, tick_duration: Duration) {
        // First, get the damage to apply (if any) by checking debuffs
        let mut damage_to_apply = None;

        if let Some(mut debuffs) = world.entity_mut(entity).get_mut::<ActiveDeBuffs>() {
            for debuff in &mut debuffs.list {
                if let DebuffEffect::Poison(data) = debuff {
                    if data.stacks == 0 {
                        continue;
                    }
                    data.timer.tick(tick_duration);
                    if data.timer.just_finished() {
                        damage_to_apply = Some(data.damage_per_tick * data.stacks as f32);
                        data.stacks -= 1;
                    }
                }
            }
        }

        // Now apply the damage if needed
        if let Some(damage) = damage_to_apply {
            if let Some(mut health) = world.entity_mut(entity).get_mut::<Health>() {
                health.take_damage(damage);
            }
        }
    }

    fn setup_poison_test() -> (World, Entity) {
        let mut world = World::new();

        // Spawn an entity with health and poison debuff
        let entity = world
            .spawn((
                Health::new_full(100.0),
                ActiveDeBuffs {
                    list: vec![DebuffEffect::Poison(PoisonDeBuffData {
                        stacks: 3,
                        max_stacks: 5,
                        damage_per_tick: 5.0,
                        timer: Timer::from_seconds(1.0, TimerMode::Repeating),
                    })],
                },
            ))
            .id();

        (world, entity)
    }

    #[test]
    fn test_poison_applies_debuff() {
        let (world, entity) = setup_poison_test();

        // Verify the entity has the poison debuff
        let debuffs = world.entity(entity).get::<ActiveDeBuffs>().unwrap();
        assert_eq!(debuffs.list.len(), 1);

        if let DebuffEffect::Poison(data) = &debuffs.list[0] {
            assert_eq!(data.stacks, 3);
            assert_eq!(data.damage_per_tick, 5.0);
        } else {
            panic!("Expected Poison debuff");
        }
    }

    #[test]
    fn test_poison_reduces_health_on_tick() {
        let (mut world, entity) = setup_poison_test();

        // Manually tick the poison for 1 second
        manual_poison_tick(&mut world, entity, Duration::from_secs(1));

        // Check that health was reduced
        // Initial: 100 HP, 3 stacks, 5 damage per tick
        // Expected damage: 5 * 3 = 15
        // Expected health: 100 - 15 = 85
        let health = world.entity(entity).get::<Health>().unwrap();
        assert_eq!(health.get_current(), 85.0);
    }

    #[test]
    fn test_poison_decrements_stacks() {
        let (mut world, entity) = setup_poison_test();

        // Manually tick the poison for 1 second
        manual_poison_tick(&mut world, entity, Duration::from_secs(1));

        // Check that stacks were decremented
        let debuffs = world.entity(entity).get::<ActiveDeBuffs>().unwrap();
        if let DebuffEffect::Poison(data) = &debuffs.list[0] {
            assert_eq!(data.stacks, 2); // Should decrease from 3 to 2
        } else {
            panic!("Expected Poison debuff");
        }
    }

    #[test]
    fn test_poison_multiple_ticks() {
        let (mut world, entity) = setup_poison_test();

        // First tick: 100 - (5 * 3) = 85, stacks: 3 -> 2
        manual_poison_tick(&mut world, entity, Duration::from_secs(1));

        let health = world.entity(entity).get::<Health>().unwrap();
        assert_eq!(health.get_current(), 85.0);

        // Second tick: 85 - (5 * 2) = 75, stacks: 2 -> 1
        manual_poison_tick(&mut world, entity, Duration::from_secs(1));

        let health = world.entity(entity).get::<Health>().unwrap();
        assert_eq!(health.get_current(), 75.0);

        // Third tick: 75 - (5 * 1) = 70, stacks: 1 -> 0
        manual_poison_tick(&mut world, entity, Duration::from_secs(1));

        let health = world.entity(entity).get::<Health>().unwrap();
        assert_eq!(health.get_current(), 70.0);

        // Verify stacks are now 0
        let debuffs = world.entity(entity).get::<ActiveDeBuffs>().unwrap();
        if let DebuffEffect::Poison(data) = &debuffs.list[0] {
            assert_eq!(data.stacks, 0);
        }
    }

    #[test]
    fn test_poison_stops_at_zero_stacks() {
        let (mut world, entity) = setup_poison_test();

        // Simulate 3 ticks to deplete all stacks
        manual_poison_tick(&mut world, entity, Duration::from_secs(1));
        manual_poison_tick(&mut world, entity, Duration::from_secs(1));
        manual_poison_tick(&mut world, entity, Duration::from_secs(1));

        let health_after_poison = world.entity(entity).get::<Health>().unwrap().get_current();
        assert_eq!(health_after_poison, 70.0);

        // One more tick should NOT reduce health (stacks are 0)
        manual_poison_tick(&mut world, entity, Duration::from_secs(1));

        let health_after_zero = world.entity(entity).get::<Health>().unwrap().get_current();
        assert_eq!(health_after_zero, 70.0); // Should remain the same
    }

    #[test]
    fn test_poison_with_different_damage_values() {
        let mut world = World::new();

        // Create poison with higher damage
        let entity = world
            .spawn((
                Health::new_full(100.0),
                ActiveDeBuffs {
                    list: vec![DebuffEffect::Poison(PoisonDeBuffData {
                        stacks: 2,
                        max_stacks: 5,
                        damage_per_tick: 10.0, // Higher damage
                        timer: Timer::from_seconds(1.0, TimerMode::Repeating),
                    })],
                },
            ))
            .id();

        manual_poison_tick(&mut world, entity, Duration::from_secs(1));

        // Expected: 100 - (10 * 2) = 80
        let health = world.entity(entity).get::<Health>().unwrap();
        assert_eq!(health.get_current(), 80.0);
    }

    #[test]
    fn test_no_damage_before_timer_expires() {
        let (mut world, entity) = setup_poison_test();

        // Tick for only 0.5 seconds (timer needs 1 second)
        manual_poison_tick(&mut world, entity, Duration::from_millis(500));

        // Health should remain unchanged
        let health = world.entity(entity).get::<Health>().unwrap();
        assert_eq!(health.get_current(), 100.0);

        // Stacks should also remain unchanged
        let debuffs = world.entity(entity).get::<ActiveDeBuffs>().unwrap();
        if let DebuffEffect::Poison(data) = &debuffs.list[0] {
            assert_eq!(data.stacks, 3);
        }
    }
}
