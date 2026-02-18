use crate::prelude::*;

use crate::game_manager::memory::{EnemyMemory, PlayerMemory};

pub(crate) fn plugin(app: &mut bevy::app::App) {
    app.add_systems(Update, spawn_big_hand_on_combat);
}

#[derive(Component, Default, Prefab)]
#[require(Actor, RequiredCustomMaterial::glitch(), SpriteLayer::Memory)]
pub struct BigHand;

/// System that spawns BigHand entities on combat events (RapidKills) and boosts morale
fn spawn_big_hand_on_combat(
    mut commands: Commands,
    mut messages: MessageReader<CombatMessage>,
    mut combat_flux: ResMut<CombatFlux>,
) {
    for message in messages.read() {
        // Only trigger on RapidKills events
        if message.trigger != BattleTrigger::RapidKills {
            continue;
        }

        let scale = 1.0;
        let morale_boost = 15.0; // Positive morale boost amount

        // Spawn BigHand based on who triggered the event
        // If Player caused RapidKills (Massacre), it's a positive event
        // If Enemy caused RapidKills (Wipeout), it's a negative event
        match message.source {
            Faction::Player => {
                // Player achieved Massacre - positive event
                let position = Vec3::new(-800., 0., 0.0);

                commands.spawn((
                    BigHand,
                    PlayerMemory,
                    Transform::from_translation(position).with_scale(Vec3::splat(scale)),
                ));

                // Boost player morale
                combat_flux.add_morale(morale_boost);

                info!(
                    "Spawned BigHand with PlayerMemory at {:?} - Player Massacre! Morale +{:.1} (now: {:.1})",
                    position,
                    morale_boost,
                    combat_flux.morale()
                );
            }
            Faction::Enemy => {
                // Enemy achieved Massacre (Wipeout for player) - negative event
                let position = Vec3::new(800., 0., 0.0);

                commands.spawn((
                    BigHand,
                    EnemyMemory,
                    Transform::from_translation(position).with_scale(Vec3::splat(scale)),
                ));

                // Decrease player morale
                combat_flux.add_morale(-morale_boost);

                info!(
                    "Spawned BigHand with EnemyMemory at {:?} - Enemy Massacre! Morale -{:.1} (now: {:.1})",
                    position,
                    morale_boost,
                    combat_flux.morale()
                );
            }
        }
    }
}
