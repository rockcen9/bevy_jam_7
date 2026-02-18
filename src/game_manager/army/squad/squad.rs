use crate::prelude::*;

pub(crate) fn plugin(app: &mut App) {
    app.add_message::<SquadLossThresholdMessage>();
    app.add_systems(
        Update,
        reduce_squad_count_on_unit_death.in_set(AttackSet::DeathRecord),
    );

    // Example consumer - can be customized or moved to other modules
    #[cfg(debug_assertions)]
    app.add_systems(Update, example_squad_loss_listener);
}

/// Data component for squads - stores the unit type and count
#[derive(Component, Default, Reflect)]
#[require(Transform, Visibility, SquadHitCount, SquadTakeHitCount)]
pub struct Squad {
    pub child_prefab_name: String,
    pub max_unit_count: usize,
    pub current_unit_count: usize,
}

impl Squad {
    pub fn new(child_prefab_name: String, max_unit_count: usize) -> Self {
        Self {
            child_prefab_name,
            current_unit_count: 0,
            max_unit_count,
        }
    }

    /// Returns the current loss percentage (0-100)
    pub fn loss_percentage(&self) -> u8 {
        if self.max_unit_count == 0 {
            return 0;
        }
        let lost = self.max_unit_count.saturating_sub(self.current_unit_count);
        ((lost * 100) / self.max_unit_count) as u8
    }
}

/// Component to track which loss thresholds have been crossed
#[derive(Component, Default, Reflect, Debug)]
pub struct SquadLossTracker {
    pub threshold_30_crossed: bool,
    pub threshold_60_crossed: bool,
    pub threshold_90_crossed: bool,
}

/// Relationship component that links a unit to its squad
#[derive(Component, Reflect, Debug)]
#[relationship(relationship_target = RootStationSquad)]
pub struct BelongToSquad(pub Entity);

/// Root station component that tracks all units in a squad
#[derive(Component, Deref, Default, Reflect)]
#[relationship_target(relationship = BelongToSquad)]
pub struct RootStationSquad(Vec<Entity>);

/// Constant for default squad size
pub const DEFAULT_SQUAD_SIZE: usize = 50;

/// Message sent when a squad crosses a loss threshold (30%, 60%, 90%)
#[derive(Message, Debug)]
pub struct SquadLossThresholdMessage {
    pub squad_entity: Entity,
    pub loss_percentage: u8,
    pub remaining_units: usize,
    pub max_units: usize,
}

/// System that listens for unit death messages and reduces the squad's current unit count
fn reduce_squad_count_on_unit_death(
    mut death_messages: MessageReader<UnitDeathMessage>,
    q_belong_to_squad: Query<&BelongToSquad>,
    mut q_squad: Query<(&mut Squad, Option<&mut SquadLossTracker>), Without<BelongToSquad>>,
    mut loss_messages: MessageWriter<SquadLossThresholdMessage>,
    mut commands: Commands,
) {
    for msg in death_messages.read() {
        // Get the squad entity that the dead unit belonged to
        let Ok(belong_to_squad) = q_belong_to_squad.get(msg._entity) else {
            continue;
        };
        let squad_entity = belong_to_squad.0;

        // Reduce the squad's current unit count
        let Ok((mut squad, tracker)) = q_squad.get_mut(squad_entity) else {
            continue;
        };

        if squad.current_unit_count > 0 {
            squad.current_unit_count -= 1;
            debug!(
                "Squad {:?} unit died. Current count: {} / {}",
                squad_entity, squad.current_unit_count, squad.max_unit_count
            );

            // Check for loss threshold crossings
            let loss_pct = squad.loss_percentage();

            // Get or create tracker
            let mut tracker = match tracker {
                Some(t) => t,
                None => {
                    commands
                        .entity(squad_entity)
                        .insert(SquadLossTracker::default());
                    return; // Will be processed next frame
                }
            };

            // Check 30% threshold
            if loss_pct >= 30 && !tracker.threshold_30_crossed {
                tracker.threshold_30_crossed = true;
                loss_messages.write(SquadLossThresholdMessage {
                    squad_entity,
                    loss_percentage: 30,
                    remaining_units: squad.current_unit_count,
                    max_units: squad.max_unit_count,
                });
                info!(
                    "Squad {:?} crossed 30% loss threshold ({}/{})",
                    squad_entity, squad.current_unit_count, squad.max_unit_count
                );
            }

            // Check 60% threshold
            if loss_pct >= 60 && !tracker.threshold_60_crossed {
                tracker.threshold_60_crossed = true;
                loss_messages.write(SquadLossThresholdMessage {
                    squad_entity,
                    loss_percentage: 60,
                    remaining_units: squad.current_unit_count,
                    max_units: squad.max_unit_count,
                });
                info!(
                    "Squad {:?} crossed 60% loss threshold ({}/{})",
                    squad_entity, squad.current_unit_count, squad.max_unit_count
                );
            }

            // Check 90% threshold
            if loss_pct >= 90 && !tracker.threshold_90_crossed {
                tracker.threshold_90_crossed = true;
                loss_messages.write(SquadLossThresholdMessage {
                    squad_entity,
                    loss_percentage: 90,
                    remaining_units: squad.current_unit_count,
                    max_units: squad.max_unit_count,
                });
                info!(
                    "Squad {:?} crossed 90% loss threshold ({}/{})",
                    squad_entity, squad.current_unit_count, squad.max_unit_count
                );
            }
        }
    }
}

/// Example system showing how to consume SquadLossThresholdMessage
/// This can be used to trigger audio, VFX, UI updates, or game logic
#[cfg(debug_assertions)]
fn example_squad_loss_listener(mut loss_messages: MessageReader<SquadLossThresholdMessage>) {
    for msg in loss_messages.read() {
        debug!(
            "🚨 Squad {:?} has lost {}% of units! ({}/{} remaining)",
            msg.squad_entity, msg.loss_percentage, msg.remaining_units, msg.max_units
        );

        // Example actions based on threshold:
        match msg.loss_percentage {
            30 => {
                // Could trigger: warning sound, yellow UI indicator
                debug!("  → Squad morale should decrease slightly");
            }
            60 => {
                // Could trigger: alarm sound, orange UI indicator, retreat suggestion
                debug!("  → Squad is critically weakened");
            }
            90 => {
                // Could trigger: panic sound, red UI indicator, automatic retreat
                debug!("  → Squad is nearly destroyed!");
            }
            _ => {}
        }
    }
}
