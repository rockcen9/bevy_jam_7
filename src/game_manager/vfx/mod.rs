mod play;

// mod firework;
// pub use firework::*;

pub mod camera_vfx;
pub use camera_vfx::*;

// mod aseprite;

mod hit_flash;
pub use hit_flash::*;

pub use rock_particles::VfxEvent;

pub(crate) fn plugin(app: &mut bevy::app::App) {
    play::plugin(app);
    camera_vfx::plugin(app);
    // firework::plugin(app);
    // aseprite::plugin(app);
    hit_flash::plugin(app);
    if std::env::var("DISABLE_HANABI").is_err() {
        rock_particles::plugin(app);
    }
}
