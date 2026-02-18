use std::time::Duration;

use bevy::math::curve::easing::EaseFunction;
use bevy_tweening::{Lens, Tween, TweenAnim};

use crate::{prelude::*, screens::Screen};

/// Event to trigger a fade-in effect (from black to transparent)
#[derive(Event)]
pub struct FadeInEvent {
    /// Duration of the fade in seconds (default: 1.0)
    pub duration: f32,
    /// Starting color (default: black)
    pub start_color: Color,
}

impl Default for FadeInEvent {
    fn default() -> Self {
        Self {
            duration: 1.0,
            start_color: Color::BLACK,
        }
    }
}

/// Event to trigger a fade-out effect (from transparent to black)
#[derive(Event)]
pub struct FadeOutEvent {
    /// Duration of the fade in seconds (default: 1.0)
    pub duration: f32,
    /// Target color (default: black)
    pub target_color: Color,
}

impl Default for FadeOutEvent {
    fn default() -> Self {
        Self {
            duration: 1.0,
            target_color: Color::BLACK,
        }
    }
}

/// Resource tracking the fade overlay entity
#[derive(Resource, Debug)]
pub struct FadeOverlay {
    pub entity: Option<Entity>,
}

impl Default for FadeOverlay {
    fn default() -> Self {
        Self { entity: None }
    }
}

/// Marker component for the fade overlay UI
#[derive(Component)]
pub struct FadeOverlayMarker;

/// Lens for animating BackgroundColor
struct BackgroundColorLens {
    start: Color,
    end: Color,
}

impl Lens<BackgroundColor> for BackgroundColorLens {
    fn lerp(&mut self, mut target: bevy::ecs::world::Mut<BackgroundColor>, ratio: f32) {
        let r = self
            .start
            .to_srgba()
            .red
            .lerp(self.end.to_srgba().red, ratio);
        let g = self
            .start
            .to_srgba()
            .green
            .lerp(self.end.to_srgba().green, ratio);
        let b = self
            .start
            .to_srgba()
            .blue
            .lerp(self.end.to_srgba().blue, ratio);
        let a = self
            .start
            .to_srgba()
            .alpha
            .lerp(self.end.to_srgba().alpha, ratio);

        target.0 = Color::srgba(r, g, b, a);
    }
}

/// System to handle fade-in event
pub fn handle_fade_in(
    trigger: On<FadeInEvent>,
    mut commands: Commands,
    mut fade_overlay: ResMut<FadeOverlay>,
    overlay_query: Query<Entity, With<FadeOverlayMarker>>,
) {
    // Get the event data
    let fade_event = &trigger;

    // Remove existing overlay if any
    if let Some(existing_entity) = fade_overlay.entity {
        if let Ok(entity) = overlay_query.get(existing_entity) {
            commands.entity(entity).despawn();
        }
    }

    // Create full-screen overlay
    let overlay_entity = commands
        .spawn((
            Name::new("Fade Overlay"),
            FadeOverlayMarker,
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            BackgroundColor(fade_event.start_color),
            ZIndex(1000), // High z-index to appear on top
        ))
        .id();

    // Create fade-in tween (from start_color to transparent)
    let tween = Tween::new(
        EaseFunction::QuadraticInOut,
        Duration::from_secs_f32(fade_event.duration),
        BackgroundColorLens {
            start: fade_event.start_color,
            end: fade_event.start_color.with_alpha(0.0),
        },
    );

    commands
        .entity(overlay_entity)
        .insert(TweenAnim::new(tween));

    // Update resource
    fade_overlay.entity = Some(overlay_entity);
}

/// System to handle fade-out event
pub fn handle_fade_out(
    trigger: On<FadeOutEvent>,
    mut commands: Commands,
    mut fade_overlay: ResMut<FadeOverlay>,
    overlay_query: Query<Entity, With<FadeOverlayMarker>>,
) {
    // Get the event data
    let fade_event = &trigger;

    // Remove existing overlay if any
    if let Some(existing_entity) = fade_overlay.entity {
        if let Ok(entity) = overlay_query.get(existing_entity) {
            commands.entity(entity).despawn();
        }
    }

    // Create full-screen overlay (start transparent)
    let overlay_entity = commands
        .spawn((
            Name::new("Fade Overlay"),
            FadeOverlayMarker,
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            BackgroundColor(fade_event.target_color.with_alpha(0.0)),
            ZIndex(1000),
        ))
        .id();

    // Create fade-out tween (from transparent to target_color)
    let tween = Tween::new(
        EaseFunction::QuadraticInOut,
        Duration::from_secs_f32(fade_event.duration),
        BackgroundColorLens {
            start: fade_event.target_color.with_alpha(0.0),
            end: fade_event.target_color,
        },
    );

    commands
        .entity(overlay_entity)
        .insert(TweenAnim::new(tween));

    // Update resource
    fade_overlay.entity = Some(overlay_entity);
}

/// System to clean up the overlay after fade-in completes
pub fn cleanup_fade_overlay_on_complete(
    mut commands: Commands,
    mut fade_overlay: ResMut<FadeOverlay>,
    overlay_query: Query<(Entity, &BackgroundColor), With<FadeOverlayMarker>>,
) {
    if let Some(overlay_entity) = fade_overlay.entity {
        if let Ok((entity, bg_color)) = overlay_query.get(overlay_entity) {
            // If overlay is fully transparent, remove it
            if bg_color.0.to_srgba().alpha <= 0.01 {
                commands.entity(entity).despawn();
                fade_overlay.entity = None;
            }
        }
    }
}

pub(crate) fn plugin(app: &mut App) {
    app.init_resource::<FadeOverlay>();
    app.add_observer(handle_fade_in);
    app.add_observer(handle_fade_out);
    app.add_systems(Update, cleanup_fade_overlay_on_complete);
    app.add_systems(OnExit(Screen::Gameplay), cleanup_fade_on_exit);

    app.add_systems(OnExit(Screen::Gameplay), unpause_time_on_exit);
}
fn unpause_time_on_exit(mut time: ResMut<Time<Virtual>>) {
    if time.is_paused() {
        time.unpause();
    }
}
/// Clean up fade overlay when exiting gameplay
fn cleanup_fade_on_exit(
    mut commands: Commands,
    mut fade_overlay: ResMut<FadeOverlay>,
    overlay_query: Query<Entity, With<FadeOverlayMarker>>,
) {
    if let Some(entity) = fade_overlay.entity {
        if let Ok(overlay_entity) = overlay_query.get(entity) {
            commands.entity(overlay_entity).despawn();
        }
        fade_overlay.entity = None;
    }
}
