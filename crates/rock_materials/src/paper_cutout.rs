use bevy::{
    asset::{embedded_asset, Asset},
    prelude::*,
    reflect::TypePath,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{AlphaMode2d, Material2d, Material2dPlugin},
};

/// Paper cutout material — draws a solid white outline around the sprite's
/// alpha boundary, giving it a hand-cut paper look.
///
/// # Usage
/// Replace the sprite's `Mesh2d` + `Sprite` with a `Mesh2d` +
/// `MeshMaterial2d<PaperCutoutMaterial>`.
///
/// ```rust
/// commands.spawn((
///     Mesh2d(meshes.add(Rectangle::new(64.0, 64.0))),
///     MeshMaterial2d(materials.add(PaperCutoutMaterial {
///         texture: asset_server.load("my_sprite.png"),
///         ..default()
///     })),
/// ));
/// ```
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct PaperCutoutMaterial {
    /// The sprite texture to apply the effect to
    #[texture(0)]
    #[sampler(1)]
    pub texture: Handle<Image>,

    /// Width of the white outline in UV space.
    /// For a 64-px sprite try 0.04–0.08; larger values = thicker border.
    #[uniform(2)]
    pub outline_width: f32,

    /// Overall opacity (0.0 = fully transparent, 1.0 = fully opaque)
    #[uniform(2)]
    pub alpha: f32,
}

impl Default for PaperCutoutMaterial {
    fn default() -> Self {
        Self {
            texture: Handle::default(),
            outline_width: 0.05,
            alpha: 1.0,
        }
    }
}

impl Material2d for PaperCutoutMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://rock_materials/paper_cutout.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

/// Plugin to register the paper cutout material
pub fn plugin(app: &mut App) {
    embedded_asset!(app, "paper_cutout.wgsl");
    app.add_plugins(Material2dPlugin::<PaperCutoutMaterial>::default());
}
