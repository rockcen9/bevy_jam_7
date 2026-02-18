mod bottom_right;
mod root;

pub(crate) fn plugin(app: &mut bevy::app::App) {
    root::plugin(app);
    bottom_right::plugin(app);
}
