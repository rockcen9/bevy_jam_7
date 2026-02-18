//! Combo Ring VFX - Ring burst with color variants
//!
//! Vertical slice: Contains backend implementation and plugin registration.

use bevy::prelude::*;

// ============================================================================
// CONTRACTS (Always Compiled)
// ============================================================================

/// Color options for combo ring effect
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComboRingColor {
    Purple,
    Pink,
}

/// Combo ring event (separate from generic VfxEvent due to color parameter)
#[derive(Event, Clone, Copy, Debug)]
pub struct ComboRingEvent {
    pub position: Vec2,
    pub color: ComboRingColor,
}

impl ComboRingEvent {
    pub fn new(position: Vec2, color: ComboRingColor) -> Self {
        Self { position, color }
    }

    pub fn purple(position: Vec2) -> Self {
        Self::new(position, ComboRingColor::Purple)
    }

    pub fn pink(position: Vec2) -> Self {
        Self::new(position, ComboRingColor::Pink)
    }
}

// ============================================================================
// BACKEND IMPLEMENTATION (Conditional)
// ============================================================================
#[cfg(feature = "backend")]
mod backend {
    use super::*;
    use bevy_hanabi::prelude::*;

    const Z_LAYER: f32 = 500.;

    #[derive(Resource)]
    pub(super) struct ComboRingEffectHandles {
        pub purple: Handle<EffectAsset>,
        pub pink: Handle<EffectAsset>,
    }

    fn hex_to_vec4(hex: &str, alpha: f32) -> Vec4 {
        let hex = hex.trim_start_matches('#');
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap() as f32 / 255.0;
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap() as f32 / 255.0;
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap() as f32 / 255.0;
        Vec4::new(r, g, b, alpha)
    }

    pub fn setup(mut commands: Commands, mut effects: ResMut<Assets<EffectAsset>>) {
        let purple_colors = [
            hex_to_vec4("#ffe54c", 1.0),
            hex_to_vec4("#ff9900", 0.7),
            hex_to_vec4("#ebd5b5", 0.0),
        ];

        let pink_colors = [
            hex_to_vec4("#ff80ab", 1.0),
            hex_to_vec4("#ffb2cf", 0.7),
            hex_to_vec4("#fce4ec", 0.0),
        ];

        let purple_handle = effects.add(create_combo_ring_effect(purple_colors));
        let pink_handle = effects.add(create_combo_ring_effect(pink_colors));

        commands.insert_resource(ComboRingEffectHandles {
            purple: purple_handle,
            pink: pink_handle,
        });
    }

    pub fn on_combo_ring_event(
        trigger: On<ComboRingEvent>,
        mut commands: Commands,
        effects: Res<ComboRingEffectHandles>,
    ) {
        let event = trigger.event();

        let effect_handle = match event.color {
            ComboRingColor::Purple => effects.purple.clone(),
            ComboRingColor::Pink => effects.pink.clone(),
        };

        let transform = Transform::from_translation(event.position.extend(Z_LAYER));

        let color_name = match event.color {
            ComboRingColor::Purple => "purple",
            ComboRingColor::Pink => "pink",
        };

        commands.spawn((
            Name::new(format!("Hanabi_combo_ring_{}", color_name)),
            ParticleEffect::new(effect_handle),
            transform,
            crate::HanabiOneShot,
        ));
    }

    fn create_combo_ring_effect(colors: [Vec4; 3]) -> EffectAsset {
        let writer = ExprWriter::new();

        let init_pos = SetPositionCircleModifier {
            center: writer.lit(Vec3::ZERO).expr(),
            axis: writer.lit(Vec3::Z).expr(),
            radius: writer.lit(25.).expr(),
            dimension: ShapeDimension::Surface,
        };

        let angle = writer.lit(0.).uniform(writer.lit(std::f32::consts::TAU));
        let speed = writer.lit(80.).uniform(writer.lit(120.));
        let vel_x = speed.clone() * angle.clone().cos();
        let vel_y = speed * angle.sin();
        let init_vel = SetAttributeModifier::new(
            Attribute::VELOCITY,
            vel_x.vec3(vel_y, writer.lit(0.)).expr(),
        );

        let init_age = SetAttributeModifier::new(Attribute::AGE, writer.lit(0.).expr());
        let lifetime = writer.lit(0.5).expr();
        let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

        let radial_accel =
            RadialAccelModifier::new(writer.lit(Vec3::ZERO).expr(), writer.lit(-80.).expr());

        let mut color_gradient = bevy_hanabi::Gradient::new();
        color_gradient.add_key(0.0, colors[0]);
        color_gradient.add_key(0.5, colors[1]);
        color_gradient.add_key(1.0, colors[2]);

        let mut size_gradient = bevy_hanabi::Gradient::new();
        size_gradient.add_key(0.0, Vec3::splat(16.0));
        size_gradient.add_key(1.0, Vec3::splat(8.0));

        let spawner = SpawnerSettings::once(20.0.into());

        EffectAsset::new(32, spawner, writer.finish())
            .with_name("combo_ring")
            .with_simulation_space(SimulationSpace::Local)
            .init(init_pos)
            .init(init_vel)
            .init(init_age)
            .init(init_lifetime)
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
// PLUGIN REGISTRATION
// ============================================================================

#[cfg(feature = "backend")]
pub fn plugin(app: &mut App) {
    app.add_systems(Startup, backend::setup);
    app.add_observer(backend::on_combo_ring_event);
}

#[cfg(not(feature = "backend"))]
pub fn plugin(_app: &mut App) {
}
