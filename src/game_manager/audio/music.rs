use crate::prelude::*;
#[cfg(feature = "backend")]
use crate::asset_tracking::LoadResource;
#[cfg(feature = "backend")]
use super::MusicPool;
#[cfg(feature = "backend")]
use bevy_seedling::prelude::*;
#[cfg(feature = "backend")]
use bevy_seedling::sample::{AudioSample, SamplePlayer};

#[cfg(feature = "backend")]
#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub(crate) struct MusicAssets {
    #[dependency]
    pub(crate) prepare: Handle<AudioSample>,
    #[dependency]
    pub(crate) battle: Handle<AudioSample>,
}

#[cfg(feature = "backend")]
impl FromWorld for MusicAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            prepare: assets.load("audio/music/prepare.ogg"),
            battle: assets.load("audio/music/battle.ogg"),
        }
    }
}

/// Resource that tracks the currently playing background music instance
#[derive(Resource, Default)]
pub struct CurrentBGM {
    pub id: Option<String>,
    pub entity: Option<Entity>,
}

/// Marker component for music that needs to fade in
#[cfg(feature = "backend")]
#[derive(Component)]
struct FadeInMusic;

#[derive(Event)]
pub struct BGMEvent {
    pub id: String,
}

impl BGMEvent {
    pub fn new(id: &str) -> Self {
        Self { id: id.to_string() }
    }
}

#[cfg(feature = "backend")]
pub(crate) fn plugin(app: &mut App) {
    app.init_resource::<CurrentBGM>();

    app.load_resource::<MusicAssets>();

    app.add_observer(play_bgm);
    app.add_systems(Update, fade_in_new_music);
    app.add_systems(OnEnter(GameState::Preparing), |mut commands: Commands| {
        commands.trigger(BGMEvent::new("prepare"));
    });

    app.add_systems(OnEnter(GameState::Battle), |mut commands: Commands| {
        commands.trigger(BGMEvent::new("battle"));
    });
}

#[cfg(not(feature = "backend"))]
pub(crate) fn plugin(_app: &mut App) {}

/// Plays background music with fade transitions
#[cfg(feature = "backend")]
fn play_bgm(
    trigger: On<BGMEvent>,
    music_assets: Res<MusicAssets>,
    mut current_music: ResMut<CurrentBGM>,
    mut volume_nodes: Query<(&VolumeNode, &mut AudioEvents)>,
    sample_effects: Query<&SampleEffects>,
    mut commands: Commands,
) {
    debug!(
        "BGM event triggered: '{}' | Currently playing: {:?}",
        trigger.id, current_music.id
    );

    // Skip if the same BGM is already playing
    if current_music.id.as_ref() == Some(&trigger.id) {
        debug!("BGM '{}' is already playing, skipping", trigger.id);
        return;
    }

    let fade_duration = DurationSeconds(1.5);

    // Fade out the previous music if there is one
    if let Some(prev_entity) = current_music.entity {
        if let Ok(effects) = sample_effects.get(prev_entity) {
            debug!(
                "Fading out previous BGM: {:?} (entity: {:?})",
                current_music.id, prev_entity
            );
            if let Ok((volume, mut events)) = volume_nodes.get_effect_mut(effects) {
                volume.fade_to(Volume::SILENT, fade_duration, &mut events);
            }
        }
        // Despawn the old music entity after fade duration
        commands.entity(prev_entity).despawn();
    }

    // Get the music handle based on the ID
    let music_handle = match trigger.id.as_str() {
        "prepare" => music_assets.prepare.clone(),
        "battle" => music_assets.battle.clone(),
        _ => {
            warn!("Unknown BGM ID: {}", trigger.id);
            return;
        }
    };

    // Spawn the new music entity with looping and volume effects starting at silent
    let entity = commands
        .spawn((
            Name::new(format!("BGM: {}", trigger.id)),
            SamplePlayer::new(music_handle).looping(),
            MusicPool,
            sample_effects![VolumeNode {
                volume: Volume::SILENT,
                ..default()
            }],
            FadeInMusic, // Mark this music to be faded in
        ))
        .id();

    debug!("Started BGM '{}' (entity: {:?})", trigger.id, entity);

    // Store the new music entity and ID
    current_music.id = Some(trigger.id.clone());
    current_music.entity = Some(entity);
}

/// System that fades in newly spawned music
#[cfg(feature = "backend")]
fn fade_in_new_music(
    new_music: Query<(Entity, &SampleEffects), With<FadeInMusic>>,
    mut volume_nodes: Query<(&VolumeNode, &mut AudioEvents)>,
    mut commands: Commands,
) {
    for (entity, effects) in new_music.iter() {
        // Use get_effect_mut to access the volume node for this music's effects
        if let Ok((volume, mut events)) = volume_nodes.get_effect_mut(effects) {
            let fade_duration = DurationSeconds(1.5);
            volume.fade_to(Volume::UNITY_GAIN, fade_duration, &mut events);
            debug!("Fading in new BGM (entity: {:?})", entity);
        }

        // Remove the marker component so we don't fade in again
        commands.entity(entity).remove::<FadeInMusic>();
    }
}
