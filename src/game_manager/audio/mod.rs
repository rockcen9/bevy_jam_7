use crate::prelude::*;
#[cfg(feature = "backend")]
use bevy_seedling::prelude::*;

#[cfg(feature = "backend")]
pub(crate) mod perceptual;
mod music;
mod sfx;
pub(crate) use sfx::*;
#[allow(unused_imports)]
pub(crate) use music::{BGMEvent, CurrentBGM};

#[cfg(feature = "backend")]
#[derive(PoolLabel, Reflect, PartialEq, Eq, Debug, Hash, Clone)]
#[reflect(Component)]
pub(crate) struct SpatialPool;

#[cfg(feature = "backend")]
#[derive(PoolLabel, Reflect, PartialEq, Eq, Debug, Hash, Clone)]
#[reflect(Component)]
pub(crate) struct SfxPool;

#[cfg(feature = "backend")]
#[derive(PoolLabel, Reflect, PartialEq, Eq, Debug, Hash, Clone)]
#[reflect(Component)]
pub(crate) struct MusicPool;

#[cfg(feature = "backend")]
/// Set somewhere below 0 dB so that the user can turn the volume up if they want to.
pub(crate) const DEFAULT_MAIN_VOLUME: Volume = Volume::Linear(0.25);

#[cfg(feature = "backend")]
pub fn plugin(app: &mut App) {
    app.add_systems(Startup, initialize_audio);
    if std::env::var("DISABLE_SEEDLING").is_err() && std::env::var("DISABLE_MUSIC").is_err() {
        music::plugin(app);
    }
    if std::env::var("DISABLE_SEEDLING").is_err() {
        sfx::plugin(app);
    }
}

#[cfg(not(feature = "backend"))]
pub fn plugin(_app: &mut App) {}

#[cfg(feature = "backend")]
fn initialize_audio(
    mut master: Single<&mut VolumeNode, With<MainBus>>,
    mut default_spatial_scale: ResMut<DefaultSpatialScale>,
    mut commands: Commands,
) {
    master.volume = DEFAULT_MAIN_VOLUME;
    // Game units are pixels (1 unit = 1px, units are 64px wide).
    // Seedling's default attenuates to -6dB at 10 units — far too aggressive for pixel coords.
    // Scale of 1/64 makes 1 game-unit behave like 1 meter for audio purposes.
    default_spatial_scale.0 = Vec3::splat(1.0 / 64.0);
    // Tuned by ear
    const DEFAULT_POOL_VOLUME: Volume = Volume::Linear(1.6);

    // For each new pool, we can provide non-default initial values for the volume.
    commands.spawn((
        Name::new("Music audio sampler pool"),
        SamplerPool(MusicPool),
        VolumeNode {
            volume: DEFAULT_POOL_VOLUME,
            ..default()
        },
    ));
    commands.spawn((
        Name::new("SFX audio sampler pool"),
        SamplerPool(SpatialPool),
        sample_effects![SpatialBasicNode::default()],
        VolumeNode {
            volume: DEFAULT_POOL_VOLUME,
            ..default()
        },
    ));
    commands.spawn((
        Name::new("UI SFX audio sampler pool"),
        SamplerPool(SfxPool),
        VolumeNode {
            volume: DEFAULT_POOL_VOLUME,
            ..default()
        },
    ));
}
