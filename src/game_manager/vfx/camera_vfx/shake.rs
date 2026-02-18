use std::time::Duration;

use bevy_rand::{global::GlobalRng, prelude::ChaCha8Rng};
use rand::Rng;

use crate::prelude::*;

// const CAMERA_DECAY_RATE: f32 = 0.9; // Adjust this for smoother or snappier decay
const SHAKE_DURATION: f32 = 0.3; // Duration of shake effect in seconds

const MAX_ANGLE: f32 = 1.;
const MAX_OFFSET: f32 = 25.0;
const TRAUMA: f32 = 0.5;
#[derive(Event)]
pub struct CameraShakeEvent;
#[derive(Resource, Default)]
pub struct ScreenShake {
    max_angle: f32,
    max_offset: f32,
    trauma: f32,
    timer: Timer,
    is_active: bool,
    original_position: Vec3,
    original_rotation: Quat,
}

impl ScreenShake {
    pub fn start_light_shake(&mut self, current_position: Vec3, current_rotation: Quat) {
        self.max_angle = MAX_ANGLE;
        self.max_offset = MAX_OFFSET;
        self.trauma = TRAUMA;
        self.timer = Timer::new(Duration::from_secs_f32(SHAKE_DURATION), TimerMode::Once);
        self.is_active = true;
        self.original_position = current_position;
        self.original_rotation = current_rotation;
    }

    pub fn is_shaking(&self) -> bool {
        self.is_active && !self.timer.is_finished()
    }
}

pub fn trigger_camera_shake(
    _trigger: On<CameraShakeEvent>,
    mut screen_shake: ResMut<ScreenShake>,
    camera_query: Query<&Transform, With<Camera>>,
) {
    if let Ok(transform) = camera_query.single() {
        if screen_shake.is_shaking() {
            return;
        }
        screen_shake.start_light_shake(transform.translation, transform.rotation);
    }
}

pub fn apply_camera_shake(
    time: Res<Time>,
    mut screen_shake: ResMut<ScreenShake>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    rng: Single<&mut ChaCha8Rng, With<GlobalRng>>,
) {
    if !screen_shake.is_active {
        return;
    }

    // Update timer
    screen_shake.timer.tick(time.delta());

    if screen_shake.timer.is_finished() {
        // Reset camera to original position when shake ends
        if let Ok(mut transform) = camera_query.single_mut() {
            transform.translation = screen_shake.original_position;
            transform.rotation = screen_shake.original_rotation;
        }
        screen_shake.is_active = false;
        return;
    }

    // Calculate shake intensity based on remaining time
    let time_progress = screen_shake.timer.elapsed_secs() / SHAKE_DURATION;
    let shake_intensity = screen_shake.trauma * (1.0 - time_progress); // Fade out over time

    let shake = shake_intensity * shake_intensity;

    let mut rng = rng.into_inner();
    let angle = (screen_shake.max_angle * shake).to_radians() * rng.random_range(-1.0..1.0);
    let offset_x = screen_shake.max_offset * shake * rng.random_range(-1.0..1.0);
    let offset_y = screen_shake.max_offset * shake * rng.random_range(-1.0..1.0);

    if let Ok(mut transform) = camera_query.single_mut() {
        // Apply shake offset to position
        let shake_offset = Vec3::new(offset_x, offset_y, 0.0);
        transform.translation = screen_shake.original_position + shake_offset;

        // Apply shake rotation
        transform.rotation = screen_shake.original_rotation * Quat::from_rotation_z(angle);
    }
}

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(Update, apply_camera_shake);
    app.add_observer(trigger_camera_shake);
    app.init_resource::<ScreenShake>();

    // app.add_observer(trigger_camera_red);
    // app.add_observer(trigger_camera_shake_for_next_wave);
}

// Commented out until NextWaveEvent is defined
// pub fn trigger_camera_shake_for_next_wave(
//     _trigger: On<NextWaveEvent>,
//     mut commands: Commands,
// ) {
//     commands.trigger(CameraShakeEvent);
// }
// pub fn fade_in_red(commands: &mut Commands, fade_entity: Entity) {
//     let tween = Tween::new(
//         EaseFunction::QuadraticInOut,
//         Duration::from_secs_f32(0.3),
//         UiBackgroundColorLens {
//             start: Color::srgb(0.3, 0.0, 0.0).with_alpha(1.0),
//             end: Color::srgb(1.0, 0.0, 0.0).with_alpha(0.0),
//         },
//     );

//     commands.entity(fade_entity).insert(TweenAnim::new(tween));

//     // Update the resource state
//     commands.insert_resource(FadeState {
//         fade_entity: Some(fade_entity),
//         is_faded_out: false,
//     });
// }
