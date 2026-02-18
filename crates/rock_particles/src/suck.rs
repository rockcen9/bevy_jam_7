//! Blackhole VFX
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


    pub fn on_vfx_event(
        trigger: On<crate::VfxEvent>,
        mut commands: Commands,
        mut effects: ResMut<Assets<EffectAsset>>,
    ) {
        let event = trigger.event();
        if event.vfx_type != crate::VfxType::Blackhole {
            return;
        }
                let center = Vec3::new(event.position.x, event.position.y, 0.0);
        let effect = effects.add(create_blackhole_effect(center));

        commands.spawn((
            Name::new("Hanabi_blackhole"),
            ParticleEffect::new(effect),
            crate::HanabiOneShot,
        ));

        // Spawn the black hole event horizon (black circle sprite in the center)
        commands.spawn((
            Name::new("BlackHole_EventHorizon"),
            Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::splat(120.0)), // Large enough to hide inner area
                ..default()
            },
            Transform::from_translation(center + Vec3::Z), // Layer above particles
            crate::HanabiOneShot,
        ));
    }

    /// Create black hole accretion disk particle effect
    /// Ring-shaped emission with proper orbital physics
    fn create_blackhole_effect(center: Vec3) -> EffectAsset {
        let writer = ExprWriter::new();

        // Color gradient: core blue-white (hot) -> purple -> edge dark red (cool)
        let mut color_gradient = bevy_hanabi::Gradient::new();
        color_gradient.add_key(0.0, Vec4::new(0.2, 0.6, 1.0, 1.0)); // Core: cyan-blue high-energy particles
        color_gradient.add_key(0.3, Vec4::new(0.5, 0.2, 0.8, 1.0)); // Mid: purple
        color_gradient.add_key(1.0, Vec4::new(0.1, 0.0, 0.0, 0.0)); // Edge: dark red transparent

        // Spawn particles in wide ring (60-140 radius)
        let spawn_radius = writer.lit(60.0).uniform(writer.lit(140.0)).expr();
        let init_pos = SetPositionCircleModifier {
            center: writer.lit(center).expr(),
            axis: writer.lit(Vec3::Z).expr(), // 2D game uses Z as normal
            radius: spawn_radius,
            dimension: ShapeDimension::Surface, // Spawn on circle perimeter
        };

        // High-speed orbital velocity
        let orbital_speed = writer.lit(200.0).uniform(writer.lit(250.0)).expr();
        let init_vel = SetVelocityCircleModifier {
            center: writer.lit(center).expr(),
            axis: writer.lit(Vec3::Z).expr(),
            speed: orbital_speed,
        };

        // Age and lifetime
        let init_age = SetAttributeModifier::new(Attribute::AGE, writer.lit(0.).expr());
        let init_lifetime = SetAttributeModifier::new(
            Attribute::LIFETIME,
            writer.lit(3.0).uniform(writer.lit(5.0)).expr(),
        );

        // KEY FIX: Strong centripetal force to maintain orbit
        // Physics: a = v²/r. With v≈225 and r≈100, we need a≈500
        // Using -600 to create spiral inward motion (not just circular orbit)
        let radial_accel = RadialAccelModifier::new(
            writer.lit(center).expr(),
            writer.lit(-600.0).expr(), // Strong inward pull for proper orbital mechanics
        );

        // Tangent acceleration: simulates friction/drag in accretion disk
        // Positive value accelerates particles along their rotation
        let tangent_accel = TangentAccelModifier::new(
            writer.lit(center).expr(),
            writer.lit(Vec3::Z).expr(),
            writer.lit(100.0).expr(),
        );

        // Moderate drag to prevent infinite acceleration
        let update_drag = LinearDragModifier::new(writer.lit(2.0).expr());

        // Size gradient: particles grow slightly then shrink as they fade
        let mut size_gradient = bevy_hanabi::Gradient::new();
        size_gradient.add_key(0.0, Vec3::splat(3.0));
        size_gradient.add_key(0.5, Vec3::splat(5.0));
        size_gradient.add_key(1.0, Vec3::splat(1.0)); // Shrink when fading

        // High particle density for visible disk
        let spawner = SpawnerSettings::rate(3000.0.into());

        EffectAsset::new(16384, spawner, writer.finish())
            .with_name("blackhole_accretion_disk")
            .with_simulation_space(SimulationSpace::Global)
            .init(init_pos)
            .init(init_vel)
            .init(init_age)
            .init(init_lifetime)
            .update(update_drag)
            .update(radial_accel) // Critical: strong centripetal force
            .update(tangent_accel) // Maintains rotation with friction
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
    app.add_observer(backend::on_vfx_event);
}

#[cfg(not(feature = "backend"))]
pub fn plugin(_app: &mut App) {
}
