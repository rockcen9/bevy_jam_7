// mod bottom_right;
mod top_middle;
mod top_right;

mod root;
pub(crate) fn plugin(app: &mut bevy::app::App) {
    root::plugin(app);
    // bottom_right::plugin(app);
    top_middle::plugin(app);
    top_right::plugin(app);
}
