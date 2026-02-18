use bevy::{
    asset::{Asset, embedded_asset},
    prelude::*,
    reflect::TypePath,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{AlphaMode2d, Material2d, Material2dPlugin},
};

/// A custom 2D material that renders a ghostly, flowing soul effect.

///

/// Procedural plasma noise (superimposed sine waves) creates a rippling,

/// liquid texture that drifts upward over time.  A 2D Fresnel approximation

/// makes the silhouette edge glow while the centre stays semi-transparent,

/// giving the classic "spectral presence" look.

///

/// # Usage

/// ```rust,ignore

/// commands.spawn((

///     Mesh2d(meshes.add(Rectangle::default())),

///     MeshMaterial2d(soul_materials.add(SoulMaterial::default())),

///     Transform::from_scale(Vec3::splat(64.0)),

/// ));

/// ```

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]

pub struct SoulMaterial {
    /// Base color of the soul (default: pale cyan — typical ghost/spirit hue).

    /// Use any Bevy `Color`:

    /// ```rust,ignore

    /// color: Color::hsl(270.0, 1.0, 0.6).into(), // purple

    /// color: Color::srgb(0.2, 1.0, 0.4).into(),  // green

    /// ```

    #[uniform(0)]
    pub color: LinearRgba,

    /// Animation speed multiplier — higher values make the soul churn faster

    /// (default: 1.0)

    #[uniform(0)]
    pub speed: f32,

    /// Overall brightness multiplier (default: 1.0)

    #[uniform(0)]
    pub intensity: f32,

    /// Corner radius in UV space (0.0 = sharp corners, 0.1 = soft rounded, 0.5 = pill)

    /// (default: 0.15)

    #[uniform(0)]
    pub corner_radius: f32,

    /// Per-tile seed that shifts the noise sample position.

    /// Use world position (e.g. `transform.translation().x`) to make every tile unique.

    #[uniform(0)]
    pub seed: f32,
}

impl Default for SoulMaterial {
    fn default() -> Self {
        Self {
            color: Color::hsl(195.0, 0.8, 0.7).into(), // pale cyan

            speed: 1.0,

            intensity: 1.0,

            corner_radius: 0.15,

            seed: 0.0,
        }
    }
}

impl Material2d for SoulMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://rock_materials/soul.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

pub fn plugin(app: &mut App) {
    embedded_asset!(app, "soul.wgsl");

    app.add_plugins(Material2dPlugin::<SoulMaterial>::default());
}
