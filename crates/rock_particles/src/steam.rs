//! Steam VFX
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
    pub(super) struct SteamEffectHandle(Handle<EffectAsset>);

    pub fn setup(mut commands: Commands, mut effects: ResMut<Assets<EffectAsset>>) {
        let effect_handle = effects.add(create_steam_effect());
        commands.insert_resource(SteamEffectHandle(effect_handle));
    }

    pub fn on_vfx_event(
        trigger: On<crate::VfxEvent>,
        mut commands: Commands,
        effect_res: Res<SteamEffectHandle>,
    ) {
        let event = trigger.event();
        if event.vfx_type != crate::VfxType::Steam {
            return;
        }
                let transform = Transform::from_translation(event.position.extend(Z_LAYER));

        commands.spawn((
            Name::new("Hanabi_steam"),
            ParticleEffect::new(effect_res.0.clone()),
            transform,
            crate::HanabiOneShot,
        ));
    }

    /// Create steam particle effect
    /// Based on Godot Steam.tscn: circular emission, upward direction with 30 degree spread,
    /// negative gravity (upward pull), velocity 20-40, scale curve (small->large->small),
    /// gray color fading to white transparent
    fn create_steam_effect() -> EffectAsset {
        let writer = ExprWriter::new();

        // Spawn from a small circle (radius 8, scaled to match game units)
        let init_pos = SetPositionCircleModifier {
            center: writer.lit(Vec3::ZERO).expr(),
            axis: writer.lit(Vec3::Z).expr(), // 2D game uses Z as normal
            radius: writer.lit(8.).expr(),
            dimension: ShapeDimension::Volume,
        };

        // Initial velocity: upward direction (0, -1 in Godot = 0, 1 in Bevy) with 30 degree spread
        // Convert 30 degrees to radians: ~0.524 radians, spread is ±15 degrees
        let spread_radians = 30f32.to_radians();
        let base_angle = std::f32::consts::FRAC_PI_2; // 90 degrees (upward)
        let angle = writer
            .lit(base_angle - spread_radians / 2.0)
            .uniform(writer.lit(base_angle + spread_radians / 2.0));
        let speed = writer.lit(20.).uniform(writer.lit(40.));
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

        // Lifetime: 2.0 seconds
        let lifetime = writer.lit(2.0).expr();
        let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

        // Negative gravity: pulls particles upward (gravity: Vector2(0, -50) in Godot)
        // In Bevy, positive Y is up, so upward gravity is positive
        let accel = writer.lit(Vec3::Y * 50.).expr();
        let update_accel = AccelModifier::new(accel);

        // Color gradient from Godot:
        // 0.0: (0.8, 0.8, 0.8, 0.6) - gray, semi-transparent
        // 0.7: (0.9, 0.9, 0.9, 0.4) - lighter gray, more transparent
        // 1.0: (1.0, 1.0, 1.0, 0.0) - white, fully transparent
        let mut color_gradient = bevy_hanabi::Gradient::new();
        color_gradient.add_key(0.0, Vec4::new(0.8, 0.8, 0.8, 0.6));
        color_gradient.add_key(0.7, Vec4::new(0.9, 0.9, 0.9, 0.4));
        color_gradient.add_key(1.0, Vec4::new(1.0, 1.0, 1.0, 0.0));

        // Size: scale_amount_curve from Godot
        // 0.0: 0.2 (small start)
        // 0.5: 1.0 (peak size)
        // 1.0: 0.0 (shrink to nothing)
        // Using scale_amount_min: 3.0, scale_amount_max: 6.0
        let mut size_gradient = bevy_hanabi::Gradient::new();
        size_gradient.add_key(0.0, Vec3::splat(3.0 * 0.2)); // 0.6
        size_gradient.add_key(0.5, Vec3::splat(6.0 * 1.0)); // 6.0 (peak)
        size_gradient.add_key(1.0, Vec3::splat(3.0 * 0.0)); // 0.0 (invisible)

        // One-shot burst: spawn particles with explosiveness 0.8
        // amount: 20, explosiveness: 0.8, randomness: 0.3
        // Most particles spawn at once, some spread over time
        let spawner = SpawnerSettings::once(20.0.into());

        EffectAsset::new(32, spawner, writer.finish())
            .with_name("steam")
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
