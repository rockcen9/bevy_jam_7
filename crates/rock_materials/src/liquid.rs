use bevy::{
    asset::{Asset, embedded_asset},
    prelude::*,
    reflect::TypePath,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{AlphaMode2d, Material2d, Material2dPlugin},
};

/// A custom 2D material that renders a wobbly liquid blob effect.
///
/// Value noise sampled along polar coordinates drives the outline to ripple
/// organically over time. A specular highlight in the upper-left gives the
/// surface a wet, glossy look.
///
/// # Usage
/// ```rust,ignore
/// commands.spawn((
///     Mesh2d(meshes.add(Rectangle::default())),
///     MeshMaterial2d(liquid_materials.add(LiquidMaterial::default())),
///     Transform::from_scale(Vec3::splat(64.0)),
/// ));
/// ```
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct LiquidMaterial {
    /// Base color of the liquid (default: water blue).
    ///
    /// ```rust,ignore
    /// color: Color::hsl(120.0, 0.9, 0.4).into(), // green slime
    /// color: Color::srgb(0.8, 0.1, 0.1).into(),  // red potion
    /// ```
    #[uniform(0)]
    pub color: LinearRgba,

    /// Animation speed multiplier — higher values make the blob ripple faster
    /// (default: 1.0).
    #[uniform(0)]
    pub time_scale: f32,
}

impl Default for LiquidMaterial {
    fn default() -> Self {
        Self {
            color: Color::hsl(210.0, 0.8, 0.55).into(), // water blue
            time_scale: 1.0,
        }
    }
}

impl Material2d for LiquidMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://rock_materials/liquid.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

pub fn plugin(app: &mut App) {
    embedded_asset!(app, "liquid.wgsl");
    app.add_plugins(Material2dPlugin::<LiquidMaterial>::default());
}
