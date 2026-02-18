pub(crate) mod bottom_middle;
mod bottom_right;
pub(crate) mod root;
mod top_middle;
mod top_right;

pub(crate) mod bottom_left;

pub(crate) fn plugin(app: &mut bevy::app::App) {
    app.add_plugins(root::plugin);
    app.add_plugins(top_right::plugin);
    app.add_plugins(top_middle::plugin);
    app.add_plugins(bottom_right::plugin);
    app.add_plugins(bottom_middle::plugin);
    app.add_plugins(bottom_left::plugin);
}
