//! Sparks VFX
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
    pub(super) struct SparksEffectHandle(Handle<EffectAsset>);

    pub fn setup(mut commands: Commands, mut effects: ResMut<Assets<EffectAsset>>) {
        let effect_handle = effects.add(create_sparks_effect());
        commands.insert_resource(SparksEffectHandle(effect_handle));
    }

    pub fn on_vfx_event(
        trigger: On<crate::VfxEvent>,
        mut commands: Commands,
        effect_res: Res<SparksEffectHandle>,
    ) {
        let event = trigger.event();
        if event.vfx_type != crate::VfxType::Sparks {
            return;
        }
                let transform = Transform::from_translation(event.position.extend(0.0));

        commands.spawn((
            Name::new("Hanabi_sparks"),
            ParticleEffect::new(effect_res.0.clone()),
            transform,
            crate::HanabiOneShot,
        ));
    }

    /// Create sparks particle effect
    /// Based on Godot sparks.tscn: sphere emission, upward direction with 180 spread,
    /// gravity, damping, color gradient (white -> yellow/orange -> transparent)
    fn create_sparks_effect() -> EffectAsset {
        let writer = ExprWriter::new();

        // Spawn from a circle (radius 64, scaled up from Godot's 5 to match game scale)
        let init_pos = SetPositionCircleModifier {
            center: writer.lit(Vec3::ZERO).expr(),
            axis: writer.lit(Vec3::Z).expr(), // 2D game uses Z as normal
            radius: writer.lit(64.).expr(),
            dimension: ShapeDimension::Volume,
        };

        // Initial velocity: upward direction with 180 degree spread
        // initial_velocity_min: 100, initial_velocity_max: 200
        let angle = writer
            .lit(-std::f32::consts::PI)
            .uniform(writer.lit(std::f32::consts::PI));
        let speed = writer.lit(100.).uniform(writer.lit(200.));
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

        // Lifetime: 0.5 seconds (with some randomness)
        let lifetime = writer.lit(0.4).uniform(writer.lit(0.6)).expr();
        let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

        // Gravity pulls particles down (gravity: 500 in Godot)
        let accel = writer.lit(Vec3::Y * -500.).expr();
        let update_accel = AccelModifier::new(accel);

        // Linear drag (damping_min: 50, damping_max: 100, using average 75)
        let drag = writer.lit(75.).expr();
        let update_drag = LinearDragModifier::new(drag);

        // Color gradient from Godot:
        // 0.039: white (1,1,1,1)
        // 0.18: yellow-white (1,1,0.5,1)
        // 0.51: yellow-white (1,1,0.5,1)
        // 0.83: orange (0.96,0.70,0.24,1)
        // 1.0: red-orange transparent (1,0.3,0,0)
        let mut color_gradient = bevy_hanabi::Gradient::new();
        color_gradient.add_key(0.0, Vec4::new(1.0, 1.0, 1.0, 1.0)); // white
        color_gradient.add_key(0.18, Vec4::new(1.0, 1.0, 0.5, 1.0)); // yellow-white
        color_gradient.add_key(0.51, Vec4::new(1.0, 1.0, 0.5, 1.0)); // yellow-white
        color_gradient.add_key(0.83, Vec4::new(0.96, 0.70, 0.24, 1.0)); // orange
        color_gradient.add_key(1.0, Vec4::new(1.0, 0.3, 0.0, 0.0)); // red-orange, fade out

        // Size: scale_amount_min: 0.2, keeping sparks small (4-8 pixels)
        let mut size_gradient = bevy_hanabi::Gradient::new();
        size_gradient.add_key(0.0, Vec3::splat(4.0));
        size_gradient.add_key(0.5, Vec3::splat(6.0));
        size_gradient.add_key(1.0, Vec3::splat(2.0));

        // One-shot burst: spawn all particles at once (amount: 30, explosiveness: 1.0)
        let spawner = SpawnerSettings::once(30.0.into());

        EffectAsset::new(32, spawner, writer.finish())
            .with_name("sparks")
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
