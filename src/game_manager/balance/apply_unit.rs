use crate::{game_manager::BattleSystems, prelude::*};

/// Marker for units that have had their stats initialized from CSV.
#[derive(Component)]
pub struct StatsInitialized;

pub(crate) fn plugin(app: &mut bevy::app::App) {
    app.add_systems(
        Update,
        apply_unit_stats_from_csv.in_set(BattleSystems::UpdateUnitValue),
    );
}

fn apply_unit_stats_from_csv(
    mut commands: Commands,
    mut q_units: Query<
        (
            Entity,
            &mut UnitStats,
            &mut Health,
            &mut UnitCollider,
            Option<&Shield>,
            Option<&Archer>,
            Option<&Cavalry>,
            Option<&Spear>,
            Option<&EnemyUnit>,
        ),
        (With<Unit>, Without<StatsInitialized>),
    >,
    cache: Res<UnitStatsCache>,
) {
    if cache.stats.is_empty() {
        return;
    }

    for (
        entity,
        mut stats,
        mut health,
        mut collider,
        is_shield,
        is_archer,
        is_cavalry,
        is_spear,
        _is_enemy,
    ) in &mut q_units
    {
        // Determine unit ID (must match CSV id column)
        let unit_id = if is_shield.is_some() {
            "Shield"
        } else if is_archer.is_some() {
            "Archer"
        } else if is_cavalry.is_some() {
            "Cavalry"
        } else if is_spear.is_some() {
            "Spear"
        } else {
            // Skip unknown unit types
            continue;
        };

        // Apply stats from cache
        if let Some(row) = cache.stats.get(unit_id) {
            // Apply UnitStats
            stats.damage = row.atk;
            stats.attack_speed = row.atk_speed;
            stats.speed = row.move_speed;
            stats.attack_range = row.range;
            stats.defense = row.def;
            stats.unity_kind = row.unity_type;
            stats.counter = row.counter;

            // Apply Health
            *health = Health::new_full(row.hp);

            // Apply weight to collider
            collider.push_strength = row.weight;

            commands.entity(entity).insert(
                UnitGameName(row.game_unit_name.as_str().into()),
            );

            // info!("Applied CSV stats to {:?} ({})", entity, unit_id);
        }

        // Mark as initialized
        commands.entity(entity).insert(StatsInitialized);
    }
}
