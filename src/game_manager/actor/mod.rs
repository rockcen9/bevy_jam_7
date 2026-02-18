mod model;
pub(crate) use actor::*;
pub(crate) use model::*;
pub(crate) use relation::*;
mod actor;
mod relation;

mod sprite;
pub(crate) use sprite::*;

// mod mesh;

mod custom_material;

pub(crate) fn plugin(app: &mut bevy::app::App) {
    model::plugin(app);
    sprite::plugin(app);
    custom_material::plugin(app);
    // mesh::plugin(app);
}
