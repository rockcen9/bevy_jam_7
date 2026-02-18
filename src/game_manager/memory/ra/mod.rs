mod ra;
mod portal;

pub use ra::{RALifetime, RA};

pub(crate) fn plugin(app: &mut bevy::app::App) {
    ra::plugin(app);
    portal::plugin(app);
}
