//! Backend - Hanabi particle implementation (compiled only with "backend" feature)
//!
//! This module contains all the heavy shader code and Hanabi systems.
//! It should ONLY be imported when the "backend" feature is enabled.

mod coin;
mod combo_ring;
mod dust;
mod explosion;
mod poison;
mod sparks;
mod spread;
mod steam;
mod vortex;
mod suck;

use bevy::prelude::*;

/// Backend plugin - registers all Hanabi systems and observers
pub fn plugin(app: &mut App) {
    // Only add HanabiPlugin if it hasn't been added yet
    if !app.is_plugin_added::<bevy_hanabi::HanabiPlugin>() {
        app.add_plugins(bevy_hanabi::HanabiPlugin);
    }

    // Register all effect modules
    spread::plugin(app);
    coin::plugin(app);
    combo_ring::plugin(app);
    dust::plugin(app);
    explosion::plugin(app);
    poison::plugin(app);
    sparks::plugin(app);
    steam::plugin(app);
    vortex::plugin(app);
    suck::plugin(app);

    // Add one-shot cleanup system
    app.add_systems(Update, cleanup_oneshot_effects);
}

/// Cleanup system: despawn entities with HanabiOneShot when their effect is done
///
/// This is a simple timer-based cleanup. For more accurate cleanup, you could
/// track individual effect lifetimes or check particle counts.
fn cleanup_oneshot_effects(
    mut commands: Commands,
    query: Query<(Entity, &bevy_hanabi::EffectSpawner), With<crate::components::HanabiOneShot>>,
) {
    for (entity, spawner) in query.iter() {
        // Despawn when spawner is inactive (one-shot effects become inactive after spawning)
        if !spawner.active {
            commands.entity(entity).despawn();
        }
    }
}
