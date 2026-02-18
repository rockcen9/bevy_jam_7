use bevy::{
    asset::{embedded_asset, Asset},
    prelude::*,
    reflect::TypePath,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{AlphaMode2d, Material2d, Material2dPlugin},
};

/// A custom material for void pattern effect - complex mathematical visual pattern
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct VoidPatternMaterial {
    /// The texture to apply the effect to
    #[texture(0)]
    #[sampler(1)]
    pub texture: Handle<Image>,

    /// Resolution of the rendering area (width, height)
    #[uniform(2)]
    pub resolution: Vec2,

    /// Control the opacity (0.0 = fully transparent, 1.0 = fully opaque, default: 1.0)
    #[uniform(2)]
    pub alpha: f32,

    /// Padding for alignment
    #[uniform(2)]
    pub _padding: f32,
}

impl Default for VoidPatternMaterial {
    fn default() -> Self {
        Self {
            texture: Handle::default(),
            resolution: Vec2::new(1920.0, 1080.0),
            alpha: 1.0,
            _padding: 0.0,
        }
    }
}

impl Material2d for VoidPatternMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://rock_materials/void_pattern.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

/// Plugin to register the void pattern material
pub fn plugin(app: &mut App) {
    embedded_asset!(app, "void_pattern.wgsl");
    app.add_plugins(Material2dPlugin::<VoidPatternMaterial>::default());
}
