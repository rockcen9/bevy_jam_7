//! Vortex VFX
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

    #[derive(Component)]
    struct VortexLifetime {
        _stop_spawner_timer: Timer,
        _despawn_timer: Timer,
    }

    #[derive(Resource)]
    pub(super) struct VortexEffectHandle(Handle<EffectAsset>);

    pub fn setup(mut commands: Commands, mut effects: ResMut<Assets<EffectAsset>>) {
        let effect_handle = effects.add(create_vortex_effect());
        commands.insert_resource(VortexEffectHandle(effect_handle));
    }

    // fn tick_vortex_lifetime(
    //     time: Res<Time>,
    //     mut query: Query<(Entity, &mut VortexLifetime, &mut EffectSpawner)>,
    //     mut commands: Commands,
    // ) {
    //     for (entity, mut lifetime, mut spawner) in &mut query {
    //         lifetime.stop_spawner_timer.tick(time.delta());
    //         if lifetime.stop_spawner_timer.just_finished() {
    //             spawner.active = false;
    //         }

    //         lifetime.despawn_timer.tick(time.delta());
    //         if lifetime.despawn_timer.just_finished() {
    //             commands.entity(entity).despawn();
    //         }
    //     }
    // }

    pub fn on_vfx_event(
        trigger: On<crate::VfxEvent>,
        mut commands: Commands,
        effect_res: Res<VortexEffectHandle>,
    ) {
        let event = trigger.event();
        if event.vfx_type != crate::VfxType::Vortex {
            return;
        }
        let transform = Transform::from_translation(event.position.extend(0.0));

        debug!("Spawning vortex at {:?}", event.position);
        commands.spawn((
            Name::new("Hanabi_vortex"),
            ParticleEffect::new(effect_res.0.clone()),
            transform,
            crate::HanabiOneShot,
            VortexLifetime {
                _stop_spawner_timer: Timer::from_seconds(3.0, TimerMode::Once),
                // Max particle lifetime is 3.0s, so despawn after 3 + 3 = 6s
                _despawn_timer: Timer::from_seconds(6.0, TimerMode::Once),
            },
        ));
    }

    fn create_vortex_effect() -> EffectAsset {
        let writer = ExprWriter::new();

        // Wide outer ring — debris is picked up from the edges and sucked in
        let init_pos = SetPositionCircleModifier {
            center: writer.lit(Vec3::ZERO).expr(),
            axis: writer.lit(Vec3::Z).expr(),
            radius: writer.lit(60.0).uniform(writer.lit(160.0)).expr(),
            dimension: ShapeDimension::Surface,
        };

        // Fast initial orbital spin
        let orbital_speed = writer.lit(350.0).uniform(writer.lit(550.0)).expr();
        let init_vel = SetVelocityCircleModifier {
            center: writer.lit(Vec3::ZERO).expr(),
            axis: writer.lit(Vec3::Z).expr(),
            speed: orbital_speed,
        };

        let init_age = SetAttributeModifier::new(Attribute::AGE, writer.lit(0.).expr());
        let init_lifetime = SetAttributeModifier::new(
            Attribute::LIFETIME,
            writer.lit(1.5).uniform(writer.lit(3.0)).expr(),
        );

        // Strong inward pull — the defining tornado spiral
        let radial_accel =
            RadialAccelModifier::new(writer.lit(Vec3::ZERO).expr(), writer.lit(-700.0).expr());

        // Tangent boost — keeps spin going as particles get pulled tight
        let tangent_accel = TangentAccelModifier::new(
            writer.lit(Vec3::ZERO).expr(),
            writer.lit(Vec3::Z).expr(),
            writer.lit(200.0).expr(),
        );

        // Upward drift — debris lifts as it's sucked in
        let upward_accel = AccelModifier::new(writer.lit(Vec3::Y * 60.0).expr());

        // Light drag to stop infinite acceleration at the core
        let update_drag = LinearDragModifier::new(writer.lit(1.0).expr());

        // Palette: brown_dark #5e4f5d → blue_darkest #3d636b → blue_dark #58778e → blue_medium #748ab2
        let mut color_gradient = bevy_hanabi::Gradient::new();
        color_gradient.add_key(0.0, Vec4::new(0.369, 0.310, 0.365, 1.0)); // brown_dark   #5e4f5d
        color_gradient.add_key(0.3, Vec4::new(0.239, 0.388, 0.420, 0.9)); // blue_darkest #3d636b
        color_gradient.add_key(0.6, Vec4::new(0.345, 0.467, 0.557, 0.7)); // blue_dark    #58778e
        color_gradient.add_key(1.0, Vec4::new(0.455, 0.541, 0.698, 0.0)); // blue_medium  #748ab2, fade out

        // Large at spawn, shrink as they tighten into the core
        let mut size_gradient = bevy_hanabi::Gradient::new();
        size_gradient.add_key(0.0, Vec3::splat(10.0));
        size_gradient.add_key(0.4, Vec3::splat(7.0));
        size_gradient.add_key(0.8, Vec3::splat(4.0));
        size_gradient.add_key(1.0, Vec3::splat(1.0));

        let spawner = SpawnerSettings::rate(400.0.into());

        EffectAsset::new(2048, spawner, writer.finish())
            .with_name("vortex")
            .with_simulation_space(SimulationSpace::Local)
            .init(init_pos)
            .init(init_vel)
            .init(init_age)
            .init(init_lifetime)
            .update(update_drag)
            .update(radial_accel)
            .update(tangent_accel)
            .update(upward_accel)
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
