use bevy::{
    asset::{embedded_asset, Asset},
    prelude::*,
    reflect::TypePath,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{AlphaMode2d, Material2d, Material2dPlugin},
};

/// A custom material for glitch snake effect with oscillating RGB channel separation
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct GlitchSnakeMaterial {
    /// The texture to apply the effect to
    #[texture(0)]
    #[sampler(1)]
    pub texture: Handle<Image>,

    /// Control the snake effect strength (0.0 ~ 0.1, default: 0.01)
    #[uniform(2)]
    pub strength: f32,

    /// Control the oscillation frequency (1.0 ~ 20.0, default: 10.0)
    #[uniform(2)]
    pub frequency: f32,

    /// Control the opacity (0.0 = fully transparent, 1.0 = fully opaque, default: 1.0)
    #[uniform(2)]
    pub alpha: f32,
}

impl Default for GlitchSnakeMaterial {
    fn default() -> Self {
        Self {
            texture: Handle::default(),
            strength: 0.01,
            frequency: 10.0,
            alpha: 1.0,
        }
    }
}

impl Material2d for GlitchSnakeMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://rock_materials/glitch_snake.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

/// Plugin to register the glitch snake material
pub fn plugin(app: &mut App) {
    embedded_asset!(app, "glitch_snake.wgsl");
    app.add_plugins(Material2dPlugin::<GlitchSnakeMaterial>::default());
}
