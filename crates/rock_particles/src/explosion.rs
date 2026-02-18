//! Explosion VFX
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
    pub(super) struct ExplosionEffectHandle(Handle<EffectAsset>);

    pub fn setup(mut commands: Commands, mut effects: ResMut<Assets<EffectAsset>>) {
        let effect_handle = effects.add(create_explosion_effect());
        commands.insert_resource(ExplosionEffectHandle(effect_handle));
    }

    pub fn on_vfx_event(
        trigger: On<crate::VfxEvent>,
        mut commands: Commands,
        effect_res: Res<ExplosionEffectHandle>,
    ) {
        let event = trigger.event();
        if event.vfx_type != crate::VfxType::Explosion {
            return;
        }
                let transform = Transform::from_translation(event.position.extend(0.0));

        commands.spawn((
            Name::new("Hanabi_explosion"),
            ParticleEffect::new(effect_res.0.clone()),
            transform,
            crate::HanabiOneShot,
        ));
    }

    fn create_explosion_effect() -> EffectAsset {
        let writer = ExprWriter::new();

        // Spawn from a small circle at the explosion center
        let init_pos = SetPositionCircleModifier {
            center: writer.lit(Vec3::ZERO).expr(),
            axis: writer.lit(Vec3::Z).expr(),
            radius: writer.lit(12.).expr(),
            dimension: ShapeDimension::Volume,
        };

        // Radial burst: particles fly outward in all directions
        let angle = writer.lit(0.).uniform(writer.lit(std::f32::consts::TAU));
        let speed = writer.lit(150.).uniform(writer.lit(400.));
        let vel_x = speed.clone() * angle.clone().cos();
        let vel_y = speed * angle.sin();
        let init_vel = SetAttributeModifier::new(
            Attribute::VELOCITY,
            vel_x.vec3(vel_y, writer.lit(0.)).expr(),
        );

        let init_age = SetAttributeModifier::new(Attribute::AGE, writer.lit(0.).expr());

        let lifetime = writer.lit(0.4).uniform(writer.lit(0.7)).expr();
        let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

        // Slight drag to slow particles over time
        let drag = LinearDragModifier::new(writer.lit(3.).expr());

        // Color gradient: bright white-orange core -> orange -> dark red -> transparent
        let mut color_gradient = bevy_hanabi::Gradient::new();
        color_gradient.add_key(0.0, Vec4::new(1.0, 1.0, 0.8, 1.0)); // bright white-yellow
        color_gradient.add_key(0.2, Vec4::new(1.0, 0.5, 0.1, 1.0)); // orange
        color_gradient.add_key(0.6, Vec4::new(0.8, 0.1, 0.0, 0.8)); // dark red
        color_gradient.add_key(1.0, Vec4::new(0.2, 0.0, 0.0, 0.0)); // fade out

        // Size: large burst shrinking to nothing
        let mut size_gradient = bevy_hanabi::Gradient::new();
        size_gradient.add_key(0.0, Vec3::splat(28.0));
        size_gradient.add_key(0.4, Vec3::splat(18.0));
        size_gradient.add_key(1.0, Vec3::splat(4.0));

        let spawner = SpawnerSettings::once(32.0.into());

        EffectAsset::new(64, spawner, writer.finish())
            .with_name("explosion")
            .with_simulation_space(SimulationSpace::Local)
            .init(init_pos)
            .init(init_vel)
            .init(init_age)
            .init(init_lifetime)
            .update(drag)
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
