use bevy_rand::prelude::{ChaCha8Rng, EntropyPlugin};

pub(crate) fn plugin(app: &mut bevy::app::App) {
    app.add_plugins(EntropyPlugin::<ChaCha8Rng>::default());
}
