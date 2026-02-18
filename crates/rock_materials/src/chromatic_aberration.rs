use bevy::{
    asset::{embedded_asset, Asset},
    prelude::*,
    reflect::TypePath,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{AlphaMode2d, Material2d, Material2dPlugin},
};

/// A custom material for chromatic aberration effect with animated RGB channel separation
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct ChromaticAberrationMaterial {
    /// The texture to apply the effect to
    #[texture(0)]
    #[sampler(1)]
    pub texture: Handle<Image>,

    /// Control the aberration strength (0.0 ~ 0.1, default: 0.05)
    #[uniform(2)]
    pub amount: f32,

    /// Control the opacity (0.0 = fully transparent, 1.0 = fully opaque, default: 1.0)
    #[uniform(2)]
    pub alpha: f32,
}

impl Default for ChromaticAberrationMaterial {
    fn default() -> Self {
        Self {
            texture: Handle::default(),
            amount: 0.05,
            alpha: 1.0,
        }
    }
}

impl Material2d for ChromaticAberrationMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://rock_materials/chromatic_aberration.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

/// Plugin to register the chromatic aberration material
pub fn plugin(app: &mut App) {
    embedded_asset!(app, "chromatic_aberration.wgsl");
    app.add_plugins(Material2dPlugin::<ChromaticAberrationMaterial>::default());
}
