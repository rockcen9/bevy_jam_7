use bevy::{
    asset::{embedded_asset, Asset},
    prelude::*,
    reflect::TypePath,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{AlphaMode2d, Material2d, Material2dPlugin},
};

/// Chromatic aberration with a fill bar overlay.
///
/// - `fill`: 0.0 = empty, 1.0 = full (fills left → right)
/// - `fill_color`: color mixed with the texture in the filled region (alpha controls blend strength)
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct ChromaticAberrationV2Material {
    /// The texture to apply the effect to
    #[texture(0)]
    #[sampler(1)]
    pub texture: Handle<Image>,

    /// Chromatic aberration strength (0.0 ~ 0.1, default: 0.05)
    #[uniform(2)]
    pub amount: f32,

    /// Overall opacity (0.0 = transparent, 1.0 = opaque, default: 1.0)
    #[uniform(2)]
    pub alpha: f32,

    /// Fill bar percentage (0.0 = empty bar, 1.0 = full bar, default: 1.0)
    #[uniform(2)]
    pub fill: f32,

    /// Color mixed into the texture inside the filled region.
    /// The alpha channel controls blend strength (0.0 = invisible, 1.0 = opaque overlay).
    #[uniform(3)]
    pub fill_color: LinearRgba,
}

impl Default for ChromaticAberrationV2Material {
    fn default() -> Self {
        Self {
            texture: Handle::default(),
            amount: 0.05,
            alpha: 1.0,
            fill: 1.0,
            fill_color: LinearRgba::new(0.2, 0.8, 1.0, 0.5),
        }
    }
}

impl Material2d for ChromaticAberrationV2Material {
    fn fragment_shader() -> ShaderRef {
        "embedded://rock_materials/chromatic_aberration_v2.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

/// Plugin to register the V2 chromatic aberration material
pub fn plugin(app: &mut App) {
    embedded_asset!(app, "chromatic_aberration_v2.wgsl");
    app.add_plugins(Material2dPlugin::<ChromaticAberrationV2Material>::default());
}
