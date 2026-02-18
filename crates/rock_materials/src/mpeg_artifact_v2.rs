use bevy::{
    asset::{embedded_asset, Asset},
    prelude::*,
    reflect::TypePath,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{AlphaMode2d, Material2d, Material2dPlugin},
};

/// MPEG artifact material with a fill bar overlay (bottom → top).
///
/// - `fill`: 0.0 = empty, 1.0 = full
/// - `fill_color`: color mixed with the texture in the filled region (alpha = blend strength)
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct MpegArtifactV2Material {
    /// The texture to apply the effect to
    #[texture(0)]
    #[sampler(1)]
    pub texture: Handle<Image>,

    /// Overall effect intensity (0.0 ~ 1.0, default: 1.0)
    #[uniform(2)]
    pub intensity: f32,

    /// Overall opacity (0.0 = transparent, 1.0 = opaque, default: 1.0)
    #[uniform(2)]
    pub alpha: f32,

    /// Fill bar percentage (0.0 = empty, 1.0 = full, bottom → top)
    #[uniform(2)]
    pub fill: f32,

    /// Color mixed into the texture inside the filled region.
    /// The alpha channel controls blend strength (0.0 = invisible, 1.0 = opaque overlay).
    #[uniform(3)]
    pub fill_color: LinearRgba,
}

impl Default for MpegArtifactV2Material {
    fn default() -> Self {
        Self {
            texture: Handle::default(),
            intensity: 1.0,
            alpha: 1.0,
            fill: 1.0,
            fill_color: LinearRgba::new(0.2, 0.8, 1.0, 0.5),
        }
    }
}

impl Material2d for MpegArtifactV2Material {
    fn fragment_shader() -> ShaderRef {
        "embedded://rock_materials/mpeg_artifact_v2.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

/// Plugin to register the V2 MPEG artifact material
pub fn plugin(app: &mut App) {
    embedded_asset!(app, "mpeg_artifact_v2.wgsl");
    app.add_plugins(Material2dPlugin::<MpegArtifactV2Material>::default());
}
