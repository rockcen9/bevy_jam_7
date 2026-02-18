use bevy::{
    asset::{embedded_asset, Asset},
    prelude::*,
    reflect::TypePath,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{AlphaMode2d, Material2d, Material2dPlugin},
};

/// A custom material for MPEG-style compression artifact effect with noise-based distortion,
/// scanlines, and chromatic aberration
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct MpegArtifactMaterial {
    /// The texture to apply the effect to
    #[texture(0)]
    #[sampler(1)]
    pub texture: Handle<Image>,

    /// Control the overall effect intensity (0.0 ~ 1.0, default: 1.0)
    #[uniform(2)]
    pub intensity: f32,

    /// Control the opacity (0.0 = fully transparent, 1.0 = fully opaque, default: 1.0)
    #[uniform(2)]
    pub alpha: f32,
}

impl Default for MpegArtifactMaterial {
    fn default() -> Self {
        Self {
            texture: Handle::default(),
            intensity: 1.0,
            alpha: 1.0,
        }
    }
}

impl Material2d for MpegArtifactMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://rock_materials/mpeg_artifact.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

/// Plugin to register the MPEG artifact material
pub fn plugin(app: &mut App) {
    embedded_asset!(app, "mpeg_artifact.wgsl");
    app.add_plugins(Material2dPlugin::<MpegArtifactMaterial>::default());
}
