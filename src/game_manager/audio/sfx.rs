use crate::prelude::*;
#[cfg(feature = "backend")]
use super::{SfxPool, SpatialPool};
#[cfg(feature = "backend")]
use crate::asset_tracking::LoadResource;
#[cfg(feature = "backend")]
use bevy_seedling::sample::AudioSample;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum SFXCategory {
    UI,
    Combat,
}

#[cfg(feature = "backend")]
#[allow(dead_code)]
#[derive(Resource, Asset, Clone, TypePath)]
pub(crate) struct UISfxAssets {
    #[dependency]
    pub(crate) coin: Handle<AudioSample>,
    pub(crate) imbuse: Handle<AudioSample>,
    pub(crate) invalid: Handle<AudioSample>,
    pub(crate) invalid2: Handle<AudioSample>,
    pub(crate) pick: Handle<AudioSample>,
    pub(crate) put: Handle<AudioSample>,
}

#[cfg(feature = "backend")]
impl FromWorld for UISfxAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();

        Self {
            coin: assets.load("audio/ui/coin.wav"),
            imbuse: assets.load("audio/ui/imbuse.wav"),
            invalid: assets.load("audio/ui/invalid.wav"),
            invalid2: assets.load("audio/ui/invalid2.wav"),
            pick: assets.load("audio/ui/pick.wav"),
            put: assets.load("audio/ui/put.wav"),
        }
    }
}

#[cfg(feature = "backend")]
#[allow(dead_code)]
#[derive(Resource, Asset, Clone, TypePath)]
pub(crate) struct SfxAssets {
    #[dependency]
    pub(crate) arrow: Handle<AudioSample>,
    pub(crate) hit: Handle<AudioSample>,
    pub(crate) calm: Handle<AudioSample>,
    pub(crate) heart_beat: Handle<AudioSample>,
    pub(crate) beam: Handle<AudioSample>,
    pub(crate) wave: Handle<AudioSample>,
}

#[cfg(feature = "backend")]
impl FromWorld for SfxAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();

        Self {
            arrow: assets.load("audio/sfx/arrow.wav"),
            hit: assets.load("audio/sfx/hit.wav"),
            calm: assets.load("audio/sfx/calm.wav"),
            heart_beat: assets.load("audio/sfx/heart_beat.wav"),
            beam: assets.load("audio/sfx/beam.wav"),
            wave: assets.load("audio/sfx/wave.wav"),
        }
    }
}

#[derive(Event)]
pub struct SFXEvent {
    pub id: String,
    pub(crate) category: SFXCategory,
    pub(crate) random_pitch: Option<(f32, f32)>,
    pub(crate) space: Option<Vec2>,
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

#[derive(Message)]
struct SFXMessage {
    id: String,
    random_pitch: Option<(f32, f32)>,
    category: SFXCategory,
    space: Option<Vec2>,
}

#[cfg(feature = "backend")]
pub(crate) fn plugin(app: &mut App) {
    if std::env::var("DISABLE_SEEDLING").is_err() {
        app.load_resource::<UISfxAssets>();
        app.load_resource::<SfxAssets>();
    }
    app.add_message::<SFXMessage>();
    app.add_observer(sfx_event_system);
    app.add_systems(Update, sfx_message_system);

    // Play SFX on game state transitions
    app.add_systems(
        OnEnter(GameState::WinAndNextDay),
        |mut commands: Commands| {
            commands.trigger(SFXEvent::new("calm"));
        },
    );

    app.add_systems(OnEnter(GameState::Lose), |mut commands: Commands| {
        commands.trigger(SFXEvent::new("heart_beat"));
    });
}

#[cfg(not(feature = "backend"))]
pub(crate) fn plugin(_app: &mut App) {}

#[cfg(feature = "backend")]
fn sfx_event_system(trigger: On<SFXEvent>, mut message_writer: MessageWriter<SFXMessage>) {
    message_writer.write(SFXMessage {
        id: trigger.id.clone(),
        random_pitch: trigger.random_pitch,
        category: trigger.category,
        space: trigger.space,
    });
}

#[cfg(feature = "backend")]
fn sfx_message_system(
    mut message_reader: MessageReader<SFXMessage>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut cooldowns: Local<std::collections::HashMap<String, u32>>,
    mut rng: Single<&mut bevy_rand::prelude::ChaCha8Rng, With<bevy_rand::global::GlobalRng>>,
) {
    // Decrement all cooldowns each frame
    for cooldown in cooldowns.values_mut() {
        if *cooldown > 0 {
            *cooldown -= 1;
        }
    }

    // Process SFX messages with cooldown check
    for message in message_reader.read() {
        let current_cooldown = cooldowns.get(&message.id).copied().unwrap_or(0);

        if current_cooldown == 0 {
            debug!("SFX event triggered: '{}'", message.id);

            // Dynamically load the SFX handle based on category and ID
            let sfx_path = match message.category {
                SFXCategory::UI => format!("audio/ui/{}.wav", message.id),
                SFXCategory::Combat => format!("audio/sfx/{}.wav", message.id),
            };
            let sfx_handle: Handle<AudioSample> = asset_server.load(&sfx_path);

            // Apply category-specific volume
            let volume = match message.category {
                SFXCategory::UI => bevy_seedling::firewheel::Volume::Linear(1.),
                SFXCategory::Combat => bevy_seedling::firewheel::Volume::Linear(1.),
            };

            // Spawn the SFX entity with optional random pitch and optional spatial position
            let name = Name::new(format!("SFX: {} ({:?})", message.id, message.category));
            let player = bevy_seedling::sample::SamplePlayer::new(sfx_handle).with_volume(volume);
            let playback = message
                .random_pitch
                .map(|(min, max)| {
                    use rand::Rng;
                    let pitch = rng.random_range(min..max);
                    bevy_seedling::sample::PlaybackSettings::default().with_speed(pitch as f64)
                })
                .unwrap_or_default();

            if let Some(pos) = message.space {
                commands.spawn((
                    name,
                    player,
                    playback,
                    SpatialPool,
                    Transform::from_xyz(pos.x, pos.y, 0.),
                ));
            } else {
                commands.spawn((name, player, playback, SfxPool));
            }

            debug!("Played SFX '{}'", message.id);

            // Set cooldown to 6 frames (prevents spam)
            cooldowns.insert(message.id.clone(), 6);
        }
    }
}
