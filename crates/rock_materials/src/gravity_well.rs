use bevy::{
    asset::{embedded_asset, Asset},
    prelude::*,
    reflect::TypePath,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{AlphaMode2d, Material2d, Material2dPlugin},
};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct GravityWellMaterial {
    #[uniform(0)]
    pub time: f32,
    #[uniform(1)]
    pub center_position: Vec2,
    #[texture(2)]
    #[sampler(3)]
    pub base_texture: Option<Handle<Image>>,
}

impl Material2d for GravityWellMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://rock_materials/gravity_well.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

fn update_gravity_well_time(
    time: Res<Time>,
    mut materials: ResMut<Assets<GravityWellMaterial>>,
) {
    let elapsed = time.elapsed_secs();
    for (_, material) in materials.iter_mut() {
        material.time = elapsed;
    }
}

pub fn plugin(app: &mut App) {
    embedded_asset!(app, "gravity_well.wgsl");
    app.add_plugins(Material2dPlugin::<GravityWellMaterial>::default())
        .add_systems(Update, update_gravity_well_time);
}
