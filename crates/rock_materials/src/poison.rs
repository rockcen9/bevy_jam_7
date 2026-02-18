use bevy::{
    asset::{embedded_asset, Asset},
    prelude::*,
    reflect::TypePath,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{AlphaMode2d, Material2d, Material2dPlugin},
};

/// A custom material for poison effect with pulsing green overlay
///
/// This shader creates a poisoned/toxic look by:
/// - Mixing the original texture with a poison color (default: bright green)
/// - Pulsing the effect over time for a "breathing" toxic appearance
/// - Allowing full control over intensity and pulse speed
///
/// # Example
/// ```rust
/// let poison_material = PoisonMaterial {
///     texture: asset_server.load("character.png"),
///     poison_amount: 0.8,  // Strong poison effect
///     pulse_speed: 3.0,    // Default pulse speed
///     poison_color: LinearRgba::new(0.3, 1.0, 0.3, 1.0), // Bright green
/// };
/// ```
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct PoisonMaterial {
    /// The texture to apply the poison effect to
    #[texture(0)]
    #[sampler(1)]
    pub texture: Handle<Image>,

    /// Controls the poison effect intensity (0.0 = no effect, 1.0 = maximum poison)
    /// Default: 0.5
    #[uniform(2)]
    pub poison_amount: f32,

    /// Speed of the pulsing effect (higher = faster pulsing)
    /// Default: 3.0
    #[uniform(2)]
    pub pulse_speed: f32,

    /// The poison tint color (default: bright green for toxic effect)
    /// Default: LinearRgba(0.3, 1.0, 0.3, 1.0)
    #[uniform(2)]
    pub poison_color: LinearRgba,
}

impl Default for PoisonMaterial {
    fn default() -> Self {
        Self {
            texture: Handle::default(),
            poison_amount: 0.5,
            pulse_speed: 3.0,
            poison_color: LinearRgba::new(0.3, 1.0, 0.3, 1.0), // Bright green
        }
    }
}

impl Material2d for PoisonMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://rock_materials/poison.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

/// Plugin to register the poison material
pub fn plugin(app: &mut App) {
    embedded_asset!(app, "poison.wgsl");
    app.add_plugins(Material2dPlugin::<PoisonMaterial>::default());
}
