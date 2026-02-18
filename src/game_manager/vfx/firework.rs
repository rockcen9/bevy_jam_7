//! Firework VFX for GameWin celebration
//!
//! Spawns celebratory firework particle effects when entering the GameWin state.
//! Uses bevy_hanabi for 2D particle effects with:
//! - Multiple rockets launching from random positions
//! - Burst explosions with colorful particles
//! - Gravity and drag for realistic motion

use bevy_hanabi::prelude::*;

use crate::prelude::*;

/// Marker component for firework effect entities
#[derive(Component)]
pub struct FireworkEffect;

pub(crate) fn plugin(app: &mut bevy::app::App) {
    app.add_systems(OnEnter(GameState::Win), spawn_firework_effect);
    app.add_systems(OnEnter(GameState::ManageArmy), despawn_firework_effect);
    app.add_systems(OnExit(GameState::Win), stop_firework_effect);
}

/// Create the rocket effect that launches upward and explodes
fn create_rocket_effect(center: Vec3) -> EffectAsset {
    let writer = ExprWriter::new();

    // Start from random positions at the bottom of the screen (2D, using Z axis as normal)
    let init_pos = SetPositionCircleModifier {
        center: writer.lit(center).expr(),
        axis: writer.lit(Vec3::Z).expr(), // 2D game uses Z as the "up" axis for circles
        radius: writer.lit(400.).expr(),
        dimension: ShapeDimension::Surface,
    };

    // Launch upward with some variation
    let x_vel = writer.lit(-30.).uniform(writer.lit(30.));
    let y_vel = writer.lit(300.).uniform(writer.lit(400.));
    let v = x_vel.vec3(y_vel, writer.lit(0.));
    let init_vel = SetAttributeModifier::new(Attribute::VELOCITY, v.expr());

    let age = writer.lit(0.).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);

    // Store a random color per rocket for the explosion
    let rgb = writer.rand(VectorType::VEC3F) * writer.lit(0.8) + writer.lit(0.2);
    let color = rgb.vec4_xyz_w(writer.lit(1.)).pack4x8unorm();
    let init_explosion_color = SetAttributeModifier::new(Attribute::U32_0, color.expr());

    // Rockets live for 0.6-1.0 seconds before exploding
    let lifetime = writer.lit(0.6).uniform(writer.lit(1.0)).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    // Gravity pulls rockets down (but they start with enough upward velocity)
    let accel = writer.lit(Vec3::Y * -200.).expr();
    let update_accel = AccelModifier::new(accel);

    // Slight drag
    let drag = writer.lit(1.).expr();
    let update_drag = LinearDragModifier::new(drag);

    // When the rocket dies, spawn explosion particles
    let update_spawn_on_die = EmitSpawnEventModifier {
        condition: EventEmitCondition::OnDie,
        count: writer.lit(200u32).expr(),
        child_index: 0,
    };

    // Spawn rockets at a rate
    let spawner = SpawnerSettings::rate(3.0.into());

    EffectAsset::new(32, spawner, writer.finish())
        .with_name("firework_rocket")
        .with_simulation_space(SimulationSpace::Global)
        .init(init_pos)
        .init(init_vel)
        .init(init_age)
        .init(init_lifetime)
        .init(init_explosion_color)
        .update(update_drag)
        .update(update_accel)
        .update(update_spawn_on_die)
        .render(ColorOverLifetimeModifier {
            gradient: bevy_hanabi::Gradient::constant(Vec4::new(4.0, 4.0, 4.0, 1.0)),
            blend: ColorBlendMode::Overwrite,
            mask: ColorBlendMask::RGBA,
        })
        .render(SizeOverLifetimeModifier {
            gradient: bevy_hanabi::Gradient::constant(Vec3::splat(3.0)),
            screen_space_size: false,
        })
}

/// Create the explosion effect for when rockets die
fn create_explosion_effect() -> EffectAsset {
    let writer = ExprWriter::new();

    // Inherit position from the dying rocket
    let init_pos = InheritAttributeModifier::new(Attribute::POSITION);

    // Inherit color from the rocket's stored color
    let init_color = SetAttributeModifier::new(
        Attribute::COLOR,
        writer.parent_attr(Attribute::U32_0).expr(),
    );

    // Explode in all directions (2D circle on XY plane)
    let speed = writer.lit(100.).uniform(writer.lit(250.));
    let dir = writer
        .rand(VectorType::VEC3F)
        .mul(writer.lit(2.0))
        .sub(writer.lit(1.0));
    // Flatten to 2D by zeroing Z component
    let dir_2d = writer.lit(Vec3::new(1., 1., 0.)) * dir;
    let dir_normalized = dir_2d.normalized();
    let init_vel = SetAttributeModifier::new(Attribute::VELOCITY, (dir_normalized * speed).expr());

    let age = writer.lit(0.).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);

    // Particles live for 0.8-1.5 seconds
    let lifetime = writer.lit(0.8).uniform(writer.lit(1.5)).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    // Gravity pulls particles down
    let accel = writer.lit(Vec3::Y * -150.).expr();
    let update_accel = AccelModifier::new(accel);

    // Drag slows particles down for the "explosion" effect
    let drag = writer.lit(3.).expr();
    let update_drag = LinearDragModifier::new(drag);

    // Spawner is controlled by parent (rocket death)
    let spawner = SpawnerSettings::default();

    // Color fades out over lifetime
    let mut color_gradient = bevy_hanabi::Gradient::new();
    color_gradient.add_key(0.0, Vec4::new(3.0, 3.0, 3.0, 1.0));
    color_gradient.add_key(0.5, Vec4::new(2.0, 2.0, 2.0, 1.0));
    color_gradient.add_key(1.0, Vec4::new(1.0, 1.0, 1.0, 0.0));

    // Size shrinks over lifetime
    let mut size_gradient = bevy_hanabi::Gradient::new();
    size_gradient.add_key(0.0, Vec3::splat(4.0));
    size_gradient.add_key(1.0, Vec3::splat(1.0));

    EffectAsset::new(8000, spawner, writer.finish())
        .with_name("firework_explosion")
        .init(init_pos)
        .init(init_vel)
        .init(init_age)
        .init(init_lifetime)
        .init(init_color)
        .update(update_drag)
        .update(update_accel)
        .render(ColorOverLifetimeModifier {
            gradient: color_gradient,
            blend: ColorBlendMode::Modulate,
            mask: ColorBlendMask::RGBA,
        })
        .render(SizeOverLifetimeModifier {
            gradient: size_gradient,
            screen_space_size: false,
        })
}

fn spawn_firework_effect(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
    camera_q: Query<&Transform, With<Camera2d>>,
) {
    // Get camera position to spawn fireworks relative to the view
    let Ok(camera_transform) = camera_q.single() else {
        return;
    };
    let camera_pos = camera_transform.translation;
    // Offset downward from camera center so fireworks launch from bottom of screen
    let center = Vec3::new(camera_pos.x, camera_pos.y - 100., 0.);

    // Create the rocket effect
    let rocket_effect = effects.add(create_rocket_effect(center));
    let rocket_entity = commands
        .spawn((
            Name::new("Firework Rocket"),
            ParticleEffect::new(rocket_effect),
            FireworkEffect,
            SpriteLayer::VFX,
            FireworkEffectParent,
        ))
        .id();

    // Create the explosion effect as a child of the rocket
    let explosion_effect = effects.add(create_explosion_effect());
    commands.spawn((
        Name::new("Firework Explosion"),
        ParticleEffect::new(explosion_effect),
        EffectParent::new(rocket_entity),
        FireworkEffect,
        SpriteLayer::VFX,
        ChildOf(rocket_entity),
    ));
}

#[derive(Component, Default)]
pub struct FireworkEffectParent;

fn despawn_firework_effect(
    mut commands: Commands,
    firework_q: Query<Entity, With<FireworkEffectParent>>,
) {
    for entity in &firework_q {
        commands.entity(entity).despawn();
    }
}
fn stop_firework_effect(
    _commands: Commands,

    mut firework_q: Query<(Entity, &mut EffectSpawner), With<FireworkEffect>>,
) {
    for (_entity, mut effect) in &mut firework_q {
        effect.active = false;

        // commands.entity(entity).despawn();
    }
}
