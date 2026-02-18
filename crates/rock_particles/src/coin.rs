//! Coin VFX - Golden particle burst effect
//!
//! Vertical slice: Contains backend implementation and plugin registration.

use bevy::prelude::*;

// ============================================================================
// BACKEND IMPLEMENTATION (Conditional - Only with "backend" feature)
// ============================================================================
#[cfg(feature = "backend")]
mod backend {
    use super::*;
    use bevy_hanabi::prelude::*;

    #[derive(Resource)]
    pub(super) struct CoinEffectHandle(pub Handle<EffectAsset>);

    pub fn setup(mut commands: Commands, mut effects: ResMut<Assets<EffectAsset>>) {
        let effect_handle = effects.add(create_coin_effect());
        commands.insert_resource(CoinEffectHandle(effect_handle));
    }

    pub fn on_vfx_event(
        trigger: On<crate::VfxEvent>,
        mut commands: Commands,
        effect_res: Res<CoinEffectHandle>,
    ) {
        let event = trigger.event();
        if event.vfx_type != crate::VfxType::Coin {
            return;
        }

        let transform = Transform::from_translation(Vec3::new(
            event.position.x,
            event.position.y + 20.0,
            0.0,
        ));

        commands.spawn((
            Name::new("Hanabi_coin"),
            ParticleEffect::new(effect_res.0.clone()),
            transform,
            crate::HanabiOneShot,
        ));
    }

    fn create_coin_effect() -> EffectAsset {
        let writer = ExprWriter::new();

        let init_pos = SetPositionCircleModifier {
            center: writer.lit(Vec3::ZERO).expr(),
            axis: writer.lit(Vec3::Z).expr(),
            radius: writer.lit(8.).expr(),
            dimension: ShapeDimension::Volume,
        };

        let vel_x = writer.lit(-150.).uniform(writer.lit(150.));
        let vel_y = writer.lit(100.).uniform(writer.lit(300.));
        let init_vel = SetAttributeModifier::new(
            Attribute::VELOCITY,
            vel_x.vec3(vel_y, writer.lit(0.)).expr(),
        );

        let init_age = SetAttributeModifier::new(Attribute::AGE, writer.lit(0.).expr());
        let lifetime = writer.lit(0.25).uniform(writer.lit(0.35)).expr();
        let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

        let accel = writer.lit(Vec3::Y * -800.).expr();
        let update_accel = AccelModifier::new(accel);

        let mut color_gradient = bevy_hanabi::Gradient::new();
        color_gradient.add_key(0.0, Vec4::new(1.0, 0.8, 0.0, 1.0));
        color_gradient.add_key(0.7, Vec4::new(1.0, 0.8, 0.0, 0.8));
        color_gradient.add_key(1.0, Vec4::new(1.0, 0.8, 0.0, 0.0));

        let mut size_gradient = bevy_hanabi::Gradient::new();
        size_gradient.add_key(0.0, Vec3::splat(20.0));
        size_gradient.add_key(0.5, Vec3::splat(18.0));
        size_gradient.add_key(1.0, Vec3::splat(12.0));

        let spawner = SpawnerSettings::once(8.0.into());

        EffectAsset::new(16, spawner, writer.finish())
            .with_name("coin")
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
