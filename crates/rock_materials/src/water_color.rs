use bevy::{
    asset::{embedded_asset, Asset},
    prelude::*,
    reflect::TypePath,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{AlphaMode2d, Material2d, Material2dPlugin},
};

/// Procedural water color material using a bitwise integer pattern algorithm.
///
/// Scrolls horizontally over time to simulate water movement.
/// No texture required — color is fully generated in the shader.
///
/// - `iters = 9.0` → normal world map look
/// - `iters = 12.0` → swamp world look
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct WaterColorMaterial {
    /// Resolution of the rendering area (width, height)
    #[uniform(0)]
    pub resolution: Vec2,

    /// Number of iterations (9.0 = normal, 12.0 = swamp, default: 9.0)
    #[uniform(0)]
    pub iters: f32,

    /// Scroll speed multiplier (1.0 = default, 2.0 = 2x faster, 0.0 = frozen)
    #[uniform(0)]
    pub speed: f32,

    /// Opacity (0.0 = fully transparent, 1.0 = fully opaque, default: 1.0)
    #[uniform(0)]
    pub alpha: f32,

    /// Padding for uniform alignment
    #[uniform(0)]
    pub _padding: f32,
}

impl Default for WaterColorMaterial {
    fn default() -> Self {
        Self {
            resolution: Vec2::new(1280.0, 720.0),
            iters: 9.0,
            speed: 1.0,
            alpha: 1.0,
            _padding: 0.0,
        }
    }
}

impl Material2d for WaterColorMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://rock_materials/water_color.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

/// Plugin to register the water color material
pub fn plugin(app: &mut App) {
    embedded_asset!(app, "water_color.wgsl");
    app.add_plugins(Material2dPlugin::<WaterColorMaterial>::default());
}
