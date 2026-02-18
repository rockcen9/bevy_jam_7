use bevy::{
    asset::{embedded_asset, Asset},
    prelude::*,
    reflect::TypePath,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{AlphaMode2d, Material2d, Material2dPlugin},
};

/// A custom material for wave distortion effect with pulsing ripples from center
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct WaveDistortionMaterial {
    /// The texture to apply the effect to
    #[texture(0)]
    #[sampler(1)]
    pub texture: Handle<Image>,

    /// Wave center position (0.0 ~ 1.0, default: 0.5, 0.5)
    #[uniform(2)]
    pub wave_center: Vec2,

    /// Wave parameters: [frequency, falloff, thickness] (default: 10.0, 0.8, 0.1)
    #[uniform(2)]
    pub wave_params: Vec3,

    /// Control the opacity (0.0 = fully transparent, 1.0 = fully opaque, default: 1.0)
    #[uniform(2)]
    pub alpha: f32,

    /// Global time at which this wave was spawned (set to Time::elapsed_secs())
    #[uniform(2)]
    pub start_time: f32,
}

impl Default for WaveDistortionMaterial {
    fn default() -> Self {
        Self {
            texture: Handle::default(),
            wave_center: Vec2::new(0.5, 0.5),
            wave_params: Vec3::new(10.0, 0.8, 0.1),
            alpha: 1.0,
            start_time: 0.0,
        }
    }
}

impl Material2d for WaveDistortionMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://rock_materials/wave_distortion.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

/// Plugin to register the wave distortion material
pub fn plugin(app: &mut App) {
    embedded_asset!(app, "wave_distortion.wgsl");
    app.add_plugins(Material2dPlugin::<WaveDistortionMaterial>::default());
}
