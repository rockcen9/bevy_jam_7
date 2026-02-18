use bevy::{
    asset::{embedded_asset, Asset},
    prelude::*,
    reflect::TypePath,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{AlphaMode2d, Material2d, Material2dPlugin},
};

/// Procedural red laser beam with dramatic zoom, screen shake, and voronoi-based stripes.
/// Loops every 6 seconds. No texture required.
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct LaserBeamMaterial {
    /// Resolution of the rendering area (width, height in pixels)
    #[uniform(0)]
    pub resolution: Vec2,

    /// Overall opacity (0.0 = transparent, 1.0 = opaque)
    #[uniform(0)]
    pub alpha: f32,

    /// Padding for 16-byte uniform alignment
    #[uniform(0)]
    pub _padding: f32,
}

impl Default for LaserBeamMaterial {
    fn default() -> Self {
        Self {
            resolution: Vec2::new(1920.0, 1080.0),
            alpha: 1.0,
            _padding: 0.0,
        }
    }
}

impl Material2d for LaserBeamMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://rock_materials/laser_beam.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

pub fn plugin(app: &mut App) {
    embedded_asset!(app, "laser_beam.wgsl");
    app.add_plugins(Material2dPlugin::<LaserBeamMaterial>::default());
}
