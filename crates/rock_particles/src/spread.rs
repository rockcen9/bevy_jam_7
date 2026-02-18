//! Spread VFX
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


    #[derive(Resource)]
    pub(super) struct BlackholeEffect(Handle<EffectAsset>);

    pub fn setup(mut commands: Commands, mut effects: ResMut<Assets<EffectAsset>>) {
        let effect_handle = effects.add(create_blackhole_effect());
        commands.insert_resource(BlackholeEffect(effect_handle));
    }

    pub fn on_vfx_event(
        trigger: On<crate::VfxEvent>,
        mut commands: Commands,
        effect_res: Res<BlackholeEffect>,
    ) {
        let event = trigger.event();
        if event.vfx_type != crate::VfxType::Spread {
            return;
        }
                let transform = Transform::from_translation(event.position.extend(0.0));
        let effect = effect_res.0.clone();

        commands.spawn((
            Name::new("Hanabi_blackhole"),
            ParticleEffect::new(effect),
            transform,
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
            Transform::from_translation(event.position.extend(0.0) + Vec3::Z), // Layer above particles
            crate::HanabiOneShot,
        ));
    }

    /// Create black hole accretion disk particle effect
    /// Ring-shaped emission with rotational velocity and gravity pull
    fn create_blackhole_effect() -> EffectAsset {
        let writer = ExprWriter::new();

        // Color gradient: bright orange-yellow -> deep red -> fade to black
        let mut color_gradient = bevy_hanabi::Gradient::new();
        color_gradient.add_key(0.0, Vec4::new(1.0, 0.8, 0.2, 1.0)); // Bright orange-yellow start
        color_gradient.add_key(0.2, Vec4::new(0.8, 0.2, 0.1, 1.0)); // Deep red mid
        color_gradient.add_key(1.0, Vec4::new(0.1, 0.0, 0.0, 0.0)); // Fade to black

        // Spawn particles from outside in a thick ring (accretion disk)
        // Use varying radius to create orbital layers (80-150 range)
        let spawn_radius = writer.lit(80.0).uniform(writer.lit(150.0)).expr();
        let init_pos = SetPositionCircleModifier {
            center: writer.lit(Vec3::ZERO).expr(),
            axis: writer.lit(Vec3::Z).expr(), // 2D game uses Z as normal
            radius: spawn_radius,
            dimension: ShapeDimension::Surface, // Spawn on circle perimeter
        };

        // Set orbital velocity (tangential to the ring)
        // Closer particles orbit faster (like real physics)
        let init_vel = SetVelocityCircleModifier {
            center: writer.lit(Vec3::ZERO).expr(),
            axis: writer.lit(Vec3::Z).expr(),
            speed: writer.lit(150.0).uniform(writer.lit(250.0)).expr(), // Orbital speed
        };

        // Age starts at 0
        let init_age = SetAttributeModifier::new(Attribute::AGE, writer.lit(0.).expr());

        // Longer lifetime for continuous orbiting (2.5 to 4 seconds)
        let lifetime = writer.lit(2.5).uniform(writer.lit(4.0)).expr();
        let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

        // Very light drag to keep particles moving smoothly
        let drag = writer.lit(0.2).expr();
        let update_drag = LinearDragModifier::new(drag);

        // Gentle inward spiral (much weaker than before)
        // This creates a slow spiral inward effect over time
        let radial_accel = RadialAccelModifier::new(
            writer.lit(Vec3::ZERO).expr(), // Center point
            writer.lit(-15.0).expr(),      // Gentle inward pull for spiral effect
        );

        // Size gradient: constant small particles
        let mut size_gradient = bevy_hanabi::Gradient::new();
        size_gradient.add_key(0.0, Vec3::splat(3.0));
        size_gradient.add_key(1.0, Vec3::splat(3.0));

        // Continuous spawner: 2000 particles per second for dense orbital effect
        let spawner = SpawnerSettings::rate(2000.0.into());

        EffectAsset::new(16384, spawner, writer.finish())
            .with_name("blackhole_accretion_disk")
            .with_simulation_space(SimulationSpace::Local)
            .init(init_pos)
            .init(init_vel)
            .init(init_age)
            .init(init_lifetime)
            .update(update_drag)
            .update(radial_accel)
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
