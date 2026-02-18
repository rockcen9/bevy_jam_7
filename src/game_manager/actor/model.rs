use crate::prelude::*;
#[derive(Component)]
#[require(Transform, Visibility)]
pub struct Model;
pub(crate) fn plugin(app: &mut bevy::app::App) {
    app.add_observer(add_model);
}
pub fn add_model(trigger: On<Add, Actor>, mut commands: Commands) {
    let actor = trigger.event().entity;
    commands.spawn((
        Name::new("Model"),
        Model,
        ChildOf(actor),
        BelongTo(actor),
        Transform::default(),
        Visibility::default(),
    ));
}

pub fn setup_mesh(
    trigger: On<Add, Model>,
    q_model: Query<&BelongTo, With<Model>>,
    mut commands: Commands,
) {
    let model = trigger.event().entity;
    if let Ok(belong_to) = q_model.get(model) {
        commands.spawn((
            Transform::default(),
            Name::new("MainMesh"),
            MainMesh,
            ChildOf(model),
            BelongTo(belong_to.0),
        ));
    }
}
