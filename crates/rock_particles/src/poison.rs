//! Poison VFX
//!
//! Vertical slice: Contains backend implementation and plugin registration.
//!
//! Vertical slice: Contains contracts, backend implementation, and plugin in one file.

use bevy::prelude::*;

// ============================================================================
// BACKEND IMPLEMENTATION (Conditional - Only with "backend" feature)
// Heavy Hanabi dependencies only compile when backend feature is enabled.
// ============================================================================
#[cfg(feature = "backend")]
mod backend {
    use super::*;
    use bevy_hanabi::prelude::*;

    const Z_LAYER: f32 = 500.;

    #[derive(Resource)]
    pub(super) struct PoisonEffectHandle(Handle<EffectAsset>);

    pub fn setup(mut commands: Commands, mut effects: ResMut<Assets<EffectAsset>>) {
        let effect_handle = effects.add(create_poison_effect());
        commands.insert_resource(PoisonEffectHandle(effect_handle));
    }

    pub fn on_vfx_event(
        trigger: On<crate::VfxEvent>,
        mut commands: Commands,
        effect_res: Res<PoisonEffectHandle>,
    ) {
        let event = trigger.event();
        if event.vfx_type != crate::VfxType::Poison {
            return;
        }
                let transform = Transform::from_translation(event.position.extend(Z_LAYER));

        // Poison cloud is a continuous emitter, not one-shot
        // Spawns particles over time for a lingering cloud effect
        commands.spawn((
            Name::new("Hanabi_poison"),
            ParticleEffect::new(effect_res.0.clone()),
            transform,
        ));
    }

    /// Create poison cloud particle effect
    /// Based on Godot PoisonCloud.tscn: circular emission, upward drift,
    /// weak upward gravity, green color gradient, continuous emission
    fn create_poison_effect() -> EffectAsset {
        let writer = ExprWriter::new();

        // Spawn from a circle (radius 15)
        let init_pos = SetPositionCircleModifier {
            center: writer.lit(Vec3::ZERO).expr(),
            axis: writer.lit(Vec3::Z).expr(), // 2D game uses Z as normal
            radius: writer.lit(15.).expr(),
            dimension: ShapeDimension::Volume,
        };

        // Initial velocity: upward direction (0, -1 in Godot = 0, 1 in Bevy)
        // No specific spread angle, so use a wide spread for cloud effect
        let angle = writer.lit(0.).uniform(writer.lit(std::f32::consts::TAU)); // Full 360 degrees
        let speed = writer.lit(10.).uniform(writer.lit(30.));
        let cos_angle = angle.clone().cos();
        let sin_angle = angle.sin();
        let vel_x = cos_angle * speed.clone();
        let vel_y = sin_angle * speed;
        let init_vel = SetAttributeModifier::new(
            Attribute::VELOCITY,
            vel_x.vec3(vel_y, writer.lit(0.)).expr(),
        );

        // Age starts at 0
        let init_age = SetAttributeModifier::new(Attribute::AGE, writer.lit(0.).expr());

        // Lifetime: 2.0 seconds with randomness 0.4
        let lifetime = writer.lit(1.6).uniform(writer.lit(2.4)).expr();
        let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

        // Weak upward gravity (gravity: Vector2(0, -10) in Godot)
        // In Bevy, positive Y is up, so upward gravity is positive
        let accel = writer.lit(Vec3::Y * 10.).expr();
        let update_accel = AccelModifier::new(accel);

        // Color gradient from Godot - green poison cloud:
        // 0.0: (0.2, 0.8, 0.2, 0.6) - green, semi-transparent
        // 0.4: (0.3, 0.9, 0.3, 0.4) - lighter green, more transparent
        // 1.0: (0.4, 1.0, 0.4, 0.0) - bright green, fully transparent
        let mut color_gradient = bevy_hanabi::Gradient::new();
        color_gradient.add_key(0.0, Vec4::new(0.2, 0.8, 0.2, 0.6));
        color_gradient.add_key(0.4, Vec4::new(0.3, 0.9, 0.3, 0.4));
        color_gradient.add_key(1.0, Vec4::new(0.4, 1.0, 0.4, 0.0));

        // Size: scale_amount_curve from Godot
        // 0.0: 0.3 (small start)
        // 0.4: 1.0 (peak size)
        // 1.0: 0.7 (shrink slightly but not to zero)
        // Using scale_amount_min: 3.0, scale_amount_max: 6.0
        let mut size_gradient = bevy_hanabi::Gradient::new();
        size_gradient.add_key(0.0, Vec3::splat(3.0 * 0.3)); // 0.9
        size_gradient.add_key(0.4, Vec3::splat(6.0 * 1.0)); // 6.0 (peak)
        size_gradient.add_key(1.0, Vec3::splat(6.0 * 0.7)); // 4.2 (shrink slightly)

        // Continuous emission: 25 particles over lifetime
        // Spawn rate: amount / lifetime = 25 / 2.0 = 12.5 particles/sec
        let spawner = SpawnerSettings::rate(12.5.into());

        EffectAsset::new(32, spawner, writer.finish())
            .with_name("poison")
            .with_simulation_space(SimulationSpace::Local)
            .init(init_pos)
            .init(init_vel)
            .init(init_age)
            .init(init_lifetime)
            .update(update_accel)
            .render(ColorOverLifetimeModifier {
                gradient: color_gradient,
                blend: ColorBlendMode::Overwrite,
                mask: ColorBlendMask::RGBA,
            })
            .render(SizeOverLifetimeModifier {
                gradient: size_gradient,
                screen_space_size: false,
            })
    }
}

// ============================================================================
// ============================================================================
// PLUGIN REGISTRATION
// ============================================================================

#[cfg(feature = "backend")]
pub fn plugin(app: &mut App) {
    app.add_systems(Startup, backend::setup);
    app.add_observer(backend::on_vfx_event);
}

#[cfg(not(feature = "backend"))]
pub fn plugin(_app: &mut App) {
}
