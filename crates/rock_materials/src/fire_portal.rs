use bevy::{
    asset::{embedded_asset, Asset},
    prelude::*,
    reflect::TypePath,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{AlphaMode2d, Material2d, Material2dPlugin},
};

/// A custom material for a fire portal effect.
///
/// Renders an animated swirling fire ring using procedural FBM noise
/// in polar coordinates. Fully transparent outside the portal circle.
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct FirePortalMaterial {
    /// Overall brightness multiplier (default: 1.0)
    #[uniform(0)]
    pub intensity: f32,

    /// Overall opacity (0.0 = invisible, 1.0 = fully opaque, default: 1.0)
    #[uniform(0)]
    pub alpha: f32,

    /// Padding for 16-byte alignment
    #[uniform(0)]
    pub _pad: Vec2,
}

impl Default for FirePortalMaterial {
    fn default() -> Self {
        Self {
            intensity: 1.0,
            alpha: 1.0,
            _pad: Vec2::ZERO,
        }
    }
}

impl Material2d for FirePortalMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://rock_materials/fire_portal.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

pub fn plugin(app: &mut App) {
    embedded_asset!(app, "fire_portal.wgsl");
    app.add_plugins(Material2dPlugin::<FirePortalMaterial>::default());
}
