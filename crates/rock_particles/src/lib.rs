//! # Hanabi VFX
//!
//! Vertical-sliced VFX crate with headless mode support.
//!
//! ## Usage
//!
//! ```rust
//! use bevy::prelude::*;
//! use rock_particles::{VfxEvent, VfxType};
//!
//! fn spawn_effects(mut commands: Commands) {
//!     commands.trigger(VfxEvent::new(VfxType::Explosion, Vec2::ZERO));
//!     commands.trigger(VfxEvent::new(VfxType::Coin, Vec2::new(100.0, 0.0)));
//! }
//! ```

use bevy::prelude::*;

// ============================================================================
// Vertical Slices - Each effect is self-contained
// ============================================================================

pub mod coin;
pub mod combo_ring;
pub mod dust;
pub mod explosion;
pub mod poison;
pub mod sparks;
pub mod spread;
pub mod steam;
pub mod suck;
pub mod vortex;

// ============================================================================
// Shared Contracts (Always Compiled)
// ============================================================================

/// VFX effect types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VfxType {
    Explosion,
    Coin,
    Dust,
    Sparks,
    Steam,
    Vortex,
    Spread,
    Poison,
    Blackhole,
}

/// Generic VFX event - triggers particle effects by type
#[derive(Event, Clone, Copy, Debug)]
pub struct VfxEvent {
    pub vfx_type: VfxType,
    pub position: Vec2,
}

impl VfxEvent {
    pub fn new(vfx_type: VfxType, position: Vec2) -> Self {
        Self { vfx_type, position }
    }

    // Convenience constructors
    pub fn explosion(position: Vec2) -> Self {
        Self::new(VfxType::Explosion, position)
    }

    pub fn coin(position: Vec2) -> Self {
        Self::new(VfxType::Coin, position)
    }

    pub fn dust(position: Vec2) -> Self {
        Self::new(VfxType::Dust, position)
    }

    pub fn sparks(position: Vec2) -> Self {
        Self::new(VfxType::Sparks, position)
    }

    pub fn steam(position: Vec2) -> Self {
        Self::new(VfxType::Steam, position)
    }

    pub fn vortex(position: Vec2) -> Self {
        Self::new(VfxType::Vortex, position)
    }

    pub fn spread(position: Vec2) -> Self {
        Self::new(VfxType::Spread, position)
    }

    pub fn poison(position: Vec2) -> Self {
        Self::new(VfxType::Poison, position)
    }

    pub fn blackhole(position: Vec2) -> Self {
        Self::new(VfxType::Blackhole, position)
    }
}

/// Marker component for one-shot particle effects that should despawn when done
#[derive(Component, Clone, Copy, Debug)]
pub struct HanabiOneShot;

// Re-export combo ring types
pub use combo_ring::{ComboRingColor, ComboRingEvent};

// ============================================================================
// Main Plugin
// ============================================================================

#[cfg(feature = "backend")]
pub fn plugin(app: &mut App) {
    if !app.is_plugin_added::<bevy_hanabi::HanabiPlugin>() {
        app.add_plugins(bevy_hanabi::HanabiPlugin);
    }
    app.add_systems(Update, cleanup_oneshot_effects);
    coin::plugin(app);
    combo_ring::plugin(app);
    dust::plugin(app);
    explosion::plugin(app);
    poison::plugin(app);
    sparks::plugin(app);
    spread::plugin(app);
    steam::plugin(app);
    vortex::plugin(app);
    suck::plugin(app);
}

#[cfg(not(feature = "backend"))]
pub fn plugin(_app: &mut App) {
}

// ============================================================================
// Backend Systems (Only compiled with backend feature)
// ============================================================================

#[cfg(feature = "backend")]
fn cleanup_oneshot_effects(
    mut commands: Commands,
    query: Query<(Entity, &bevy_hanabi::EffectSpawner), With<HanabiOneShot>>,
) {
    for (entity, spawner) in query.iter() {
        // Despawn when spawner is inactive
        if !spawner.active {
            commands.entity(entity).despawn();
        }
    }
}
