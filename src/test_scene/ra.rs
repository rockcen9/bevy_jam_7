use bevy::input::common_conditions::input_just_pressed;

use crate::prelude::*;
use crate::screens::Screen;

pub(crate) fn plugin(app: &mut bevy::app::App) {
    app.add_systems(
        Update,
        spawn_ra_on_space
            .run_if(in_state(Screen::Gameplay).and(input_just_pressed(KeyCode::Space))),
    );
}

fn spawn_ra_on_space(mut commands: Commands) {
    commands.spawn((
        // RA,
        PlayerMemory,
        Transform::from_translation(Vec3::new(0., 1000., 0.)),
        // RALifetime::new(),
    ));
    info!("Spawned RA at origin via Space key");
}
