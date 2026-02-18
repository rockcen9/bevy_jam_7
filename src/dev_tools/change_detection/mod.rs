use crate::prelude::*;

// add and remove at the system will not be detected
// remove componenet can not be detected
pub fn change_detection_plugin(_app: &mut App) {
    // _app.add_systems(Update, _change_detection::<Transform>);
    // _app.add_observer(_react_on_removal::<ConfirmedPosition>);
    // _app.add_systems(Update, _change_detection::<Pentagram>);
    // _app.add_systems(Update, _detect_resource_change::<FadeOverlay>);
}
pub fn _change_detection<C: Component + std::fmt::Debug>(
    changed_components: Query<Ref<C>, (Changed<C>,)>,
) {
    for component in &changed_components {
        error!(
            "Change detected!\n\t-> value: {:?}\n\t-> added: {}\n\t-> changed: {}\n\t-> changed by: {}",
            component,
            component.is_added(),
            component.is_changed(),
            component.changed_by(),
        );
    }
}
fn _react_on_removal<C: Component + std::fmt::Debug>(trigger: On<Remove, C>) {
    // The `OnRemove` trigger was automatically called on the `Entity` that had its `MyComponent` removed.
    let entity = trigger.entity;
    warn!("Removed component detected: {:?}", entity);
}
pub fn _detect_resource_change<T: Resource + std::fmt::Debug>(my_res: Res<T>) {
    if my_res.is_changed() {
        error!(
            "Resource changed! value: {:?}, added: {}, changed: {}, changed by: {}",
            *my_res,
            my_res.is_added(),
            my_res.is_changed(),
            my_res.changed_by()
        );
    }
}
