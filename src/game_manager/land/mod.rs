mod space;

mod dark;
pub(crate) fn plugin(app: &mut bevy::app::App) {
    space::plugin(app);
    dark::plugin(app);
}
