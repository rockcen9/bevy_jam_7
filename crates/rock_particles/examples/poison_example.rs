use bevy::prelude::*;
use rock_particles::VfxEvent;

/// Marker component for poison clouds to track them
#[derive(Component)]
struct PoisonCloud {
    lifetime: Timer,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Hanabi VFX - Poison Cloud".to_string(),
                resolution: (1280, 720).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(rock_particles::plugin)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (spawn_effect, tick_poison_clouds, tag_poison_clouds),
        )
        .run();
}

fn setup(mut commands: Commands) {
    // Spawn camera
    commands.spawn(Camera2d);

    // Spawn instruction text
    commands.spawn((
        Text::new(
            "Poison Cloud Effect (Continuous Emitter)\n\n\
            Press SPACE - Spawn poison cloud at cursor\n\
            Press D - Despawn all clouds\n\
            Press ESC - Exit\n\n\
            Clouds auto-despawn after 5 seconds",
        ),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        TextColor(Color::WHITE),
        TextFont {
            font_size: 20.0,
            ..default()
        },
    ));
}

fn spawn_effect(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    window_q: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    poison_clouds: Query<Entity, With<PoisonCloud>>,
) {
    // Exit on ESC
    if keyboard.just_pressed(KeyCode::Escape) {
        std::process::exit(0);
    }

    // Despawn all clouds on D
    if keyboard.just_pressed(KeyCode::KeyD) {
        for entity in poison_clouds.iter() {
            commands.entity(entity).despawn();
        }
        info!("Despawned all poison clouds");
        return;
    }

    // Get cursor position for spawning effects
    let Ok(window) = window_q.single() else {
        return;
    };
    let Ok((camera, transform)) = camera_q.single() else {
        return;
    };

    let cursor_world_pos: Vec2 = window
        .cursor_position()
        .and_then(|cursor_pos: Vec2| camera.viewport_to_world_2d(transform, cursor_pos).ok())
        .unwrap_or(Vec2::ZERO);

    // Spawn poison cloud effect
    if keyboard.just_pressed(KeyCode::Space) {
        info!("Spawning poison cloud at {:?}", cursor_world_pos);
        commands.trigger(VfxEvent::poison(cursor_world_pos));
    }
}

/// Tag newly spawned poison clouds with tracking component
fn tag_poison_clouds(
    mut commands: Commands,
    new_clouds: Query<Entity, (With<Name>, Without<PoisonCloud>)>,
) {
    for entity in new_clouds.iter() {
        commands.entity(entity).insert(PoisonCloud {
            lifetime: Timer::from_seconds(5.0, TimerMode::Once),
        });
    }
}

/// Tick poison cloud lifetimes and despawn expired ones
fn tick_poison_clouds(
    mut commands: Commands,
    mut clouds: Query<(Entity, &mut PoisonCloud)>,
    time: Res<Time>,
) {
    for (entity, mut cloud) in clouds.iter_mut() {
        cloud.lifetime.tick(time.delta());

        if cloud.lifetime.is_finished() {
            commands.entity(entity).despawn();
            info!("Auto-despawned poison cloud");
        }
    }
}
