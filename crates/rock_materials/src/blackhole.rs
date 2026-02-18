use bevy::{
    asset::{embedded_asset, Asset},
    prelude::*,
    reflect::TypePath,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{AlphaMode2d, Material2d, Material2dPlugin},
};

/// A custom material for blackhole effects with swirl distortion applied to an image
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct BlackholeMaterial {
    /// The texture to apply the effect to
    #[texture(0)]
    #[sampler(1)]
    pub texture: Handle<Image>,

    /// Control the distortion strength (0.1 ~ 0.8)
    #[uniform(2)]
    pub distortion_strength: f32,

    /// Control the rotation speed
    #[uniform(2)]
    pub rotation_speed: f32,
}

impl Default for BlackholeMaterial {
    fn default() -> Self {
        Self {
            texture: Handle::default(),
            distortion_strength: 0.3,
            rotation_speed: 2.0,
        }
    }
}

impl Material2d for BlackholeMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://rock_materials/black_hole.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

/// Plugin to register the blackhole material
pub fn plugin(app: &mut App) {
    embedded_asset!(app, "black_hole.wgsl");
    app.add_plugins(Material2dPlugin::<BlackholeMaterial>::default());
}
