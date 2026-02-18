//! Dust VFX
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
    pub(super) struct DustEffectHandle(Handle<EffectAsset>);

    pub fn setup(mut commands: Commands, mut effects: ResMut<Assets<EffectAsset>>) {
        let effect_handle = effects.add(create_dust_effect());
        commands.insert_resource(DustEffectHandle(effect_handle));
    }

    pub fn on_vfx_event(
        trigger: On<crate::VfxEvent>,
        mut commands: Commands,
        effect_res: Res<DustEffectHandle>,
    ) {
        let event = trigger.event();
        if event.vfx_type != crate::VfxType::Dust {
            return;
        }
                let transform = Transform::from_translation(event.position.extend(0.0));

        commands.spawn((
            Name::new("Hanabi_dust"),
            ParticleEffect::new(effect_res.0.clone()),
            transform,
            crate::HanabiOneShot,
        ));
    }

    /// Create dust particle effect
    /// Based on enoki dust.ron: circle emission, upward drift, gravity, scale/color animation
    fn create_dust_effect() -> EffectAsset {
        let writer = ExprWriter::new();

        // Spawn from a circle (radius 256, matching enoki config)
        let init_pos = SetPositionCircleModifier {
            center: writer.lit(Vec3::ZERO).expr(),
            axis: writer.lit(Vec3::Z).expr(), // 2D game uses Z as normal
            radius: writer.lit(256.).expr(),
            dimension: ShapeDimension::Volume,
        };

        // Initial velocity: upward with variation (direction (0, 1) with spread 0.8)
        let speed_x = writer.lit(-24.).uniform(writer.lit(24.)); // horizontal spread
        let speed_y = writer.lit(30.).uniform(writer.lit(45.)); // upward speed
        let init_vel = SetAttributeModifier::new(
            Attribute::VELOCITY,
            speed_x.vec3(speed_y, writer.lit(0.)).expr(),
        );

        // Age starts at 0
        let init_age = SetAttributeModifier::new(Attribute::AGE, writer.lit(0.).expr());

        // Lifetime: 0.6 ± 0.2 (so 0.4 to 0.8)
        let lifetime = writer.lit(0.4).uniform(writer.lit(0.8)).expr();
        let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

        // Gravity pulls particles down (gravity_speed: 20)
        let accel = writer.lit(Vec3::Y * -20.).expr();
        let update_accel = AccelModifier::new(accel);

        // Linear drag (linear_damp: 30)
        let drag = writer.lit(30.).expr();
        let update_drag = LinearDragModifier::new(drag);

        // Color gradient: fade in then out with brownish color (0.6, 0.5, 0.4)
        let mut color_gradient = bevy_hanabi::Gradient::new();
        color_gradient.add_key(0.0, Vec4::new(0.6, 0.5, 0.4, 0.0)); // start transparent
        color_gradient.add_key(0.2, Vec4::new(0.6, 0.5, 0.4, 0.8)); // fade in
        color_gradient.add_key(1.0, Vec4::new(0.6, 0.5, 0.4, 0.0)); // fade out

        // Size gradient: 8 -> 16 (peak at 0.3) -> 8 with easing
        let mut size_gradient = bevy_hanabi::Gradient::new();
        size_gradient.add_key(0.0, Vec3::splat(8.0));
        size_gradient.add_key(0.3, Vec3::splat(16.0));
        size_gradient.add_key(1.0, Vec3::splat(8.0));

        // One-shot burst: spawn all particles at once (spawn_amount: 56)
        let spawner = SpawnerSettings::once(56.0.into());

        EffectAsset::new(64, spawner, writer.finish())
            .with_name("dust")
            .with_simulation_space(SimulationSpace::Local)
            .init(init_pos)
            .init(init_vel)
            .init(init_age)
            .init(init_lifetime)
            .update(update_drag)
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
