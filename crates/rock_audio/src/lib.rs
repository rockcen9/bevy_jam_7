//! # Rock Audio
//!
//! Vertical-sliced audio crate with headless mode support.
//!
//! ## Usage
//!
//! ```rust
//! use bevy::prelude::*;
//! use rock_audio::{BGMEvent, SFXEvent};
//!
//! fn play_audio(mut commands: Commands) {
//!     // Play background music
//!     commands.trigger(BGMEvent::new("prepare"));
//!
//!     // Play sound effect
//!     commands.trigger(SFXEvent::new("coin"));
//!
//!     // Play UI sound effect
//!     commands.trigger(SFXEvent::ui("click"));
//!
//!     // Play spatial sound effect
//!     commands.trigger(SFXEvent::space("explosion", Vec2::new(100.0, 200.0)));
//! }
//! ```

use bevy::prelude::*;

// ============================================================================
// Vertical Slices - Each audio system is self-contained
// ============================================================================

pub mod audio_engine;
pub mod bgm;
pub mod perceptual;
pub mod sfx;

// Re-export commonly used types
#[cfg(feature = "backend")]
pub use audio_engine::{DEFAULT_MAIN_VOLUME, SfxPool};

// ============================================================================
// Shared Contracts (Always Compiled)
// ============================================================================

/// Background music event - triggers music playback
#[derive(Event, Clone, Debug)]
pub struct BGMEvent {
    pub id: String,
}

impl BGMEvent {
    pub fn new(id: &str) -> Self {
        Self { id: id.to_string() }
    }
}

/// Sound effect category
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum SFXCategory {
    UI,
    Combat,
}

/// Sound effect event - triggers SFX playback
#[derive(Event, Clone, Debug)]
pub struct SFXEvent {
    pub id: String,
    pub category: SFXCategory,
    pub random_pitch: Option<(f32, f32)>,
    pub space: Option<Vec2>,
}

impl SFXEvent {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            random_pitch: None,
            category: SFXCategory::Combat,
            space: None,
        }
    }

    pub fn ui(id: &str) -> Self {
        Self {
            id: id.to_string(),
            random_pitch: None,
            category: SFXCategory::UI,
            space: None,
        }
    }

    pub fn space(id: &str, space: Vec2) -> Self {
        Self {
            id: id.to_string(),
            random_pitch: None,
            category: SFXCategory::Combat,
            space: Some(space),
        }
    }

    pub fn with_random_pitch(mut self, min: f32, max: f32) -> Self {
        self.random_pitch = Some((min, max));
        self
    }
}

// ============================================================================
// Main Plugin
// ============================================================================

pub fn plugin(app: &mut App) {
    #[cfg(feature = "backend")]
    {
        #[cfg(not(target_arch = "web"))]
        app.add_plugins(bevy_seedling::SeedlingPlugin::default());

        #[cfg(target_arch = "web")]
        app.add_plugins(bevy_seedling::SeedlingPlugin::new_web_audio());
    }
    // Register all audio plugins (they self-manage based on feature flags)
    audio_engine::plugin(app);
    bgm::plugin(app);
    sfx::plugin(app);
}
