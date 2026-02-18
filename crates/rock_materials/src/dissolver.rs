use bevy::{
    asset::{embedded_asset, Asset},
    prelude::*,
    reflect::TypePath,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{AlphaMode2d, Material2d, Material2dPlugin},
};

/// Turbo-rainbow dissolver: animated fBm noise shapes that pop in and out
/// with a glowing edge, colored via the Turbo colormap.
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct DissolverMaterial {
    /// Resolution of the rendering area (width, height)
    #[uniform(0)]
    pub resolution: Vec2,

    /// Overall opacity (0.0 – 1.0, default 1.0)
    #[uniform(0)]
    pub alpha: f32,

    /// Animation speed multiplier (default 1.0)
    #[uniform(0)]
    pub speed: f32,

    /// Spatial scale of the noise pattern (default 3.0)
    #[uniform(0)]
    pub scale: f32,

    /// Width of the glowing pop edge (default 0.05)
    #[uniform(0)]
    pub pop_width: f32,

    /// Padding to satisfy 16-byte uniform alignment
    #[uniform(0)]
    pub _padding: Vec2,
}

impl Default for DissolverMaterial {
    fn default() -> Self {
        Self {
            resolution: Vec2::new(1280.0, 720.0),
            alpha: 1.0,
            speed: 1.0,
            scale: 3.0,
            pop_width: 0.05,
            _padding: Vec2::ZERO,
        }
    }
}

impl Material2d for DissolverMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://rock_materials/dissolver.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

/// Plugin to register the dissolver material.
pub fn plugin(app: &mut App) {
    embedded_asset!(app, "dissolver.wgsl");
    app.add_plugins(Material2dPlugin::<DissolverMaterial>::default());
}
