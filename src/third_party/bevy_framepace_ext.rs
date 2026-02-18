use bevy_framepace::{FramepaceSettings, Limiter};

pub(crate) fn plugin(app: &mut bevy::app::App) {
    app.add_plugins(bevy_framepace::FramepacePlugin);
    app.insert_resource(FramepaceSettings {
        limiter: Limiter::from_framerate(60.0),
    });
}
