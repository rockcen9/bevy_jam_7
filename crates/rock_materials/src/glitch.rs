use bevy::{
    asset::{embedded_asset, Asset},
    prelude::*,
    reflect::TypePath,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{AlphaMode2d, Material2d, Material2dPlugin},
};

/// A custom material for glitch effect with chromatic aberration and random displacement
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct GlitchMaterial {
    /// The texture to apply the effect to
    #[texture(0)]
    #[sampler(1)]
    pub texture: Handle<Image>,

    /// Control the glitch intensity (0.0 ~ 1.0, default: 0.5)
    #[uniform(2)]
    pub glitch_amount: f32,

    /// Control the opacity (0.0 = fully transparent, 1.0 = fully opaque, default: 1.0)
    #[uniform(2)]
    pub alpha: f32,
}

impl Default for GlitchMaterial {
    fn default() -> Self {
        Self {
            texture: Handle::default(),
            glitch_amount: 0.5,
            alpha: 1.0,
        }
    }
}

impl Material2d for GlitchMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://rock_materials/glitch.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

/// Plugin to register the glitch material
pub fn plugin(app: &mut App) {
    embedded_asset!(app, "glitch.wgsl");
    app.add_plugins(Material2dPlugin::<GlitchMaterial>::default());
}
