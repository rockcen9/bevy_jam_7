use bevy::prelude::*;
use bevy::ui::Val::*;
use pyri_tooltip::prelude::*;

pub(crate) fn plugin(app: &mut App) {
    let world = app.world_mut();

    let text_entity = world
        .spawn((
            Name::new("PrimaryTooltipText"),
            Node::default(),
            RichText::default(),
        ))
        .id();

    let container_entity = world
        .spawn((
            Name::new("PrimaryTooltip"),
            Node {
                position_type: PositionType::Absolute,
                padding: UiRect::all(Px(8.0)),
                border_radius: BorderRadius::all(Px(8.0)),
                ..Default::default()
            },
            BackgroundColor(Color::srgba(0.106, 0.118, 0.122, 0.9)),
            Visibility::Hidden,
            GlobalZIndex(999),
            Pickable::IGNORE,
        ))
        .id();

    world
        .entity_mut(text_entity)
        .insert(ChildOf(container_entity));

    app.add_plugins(TooltipPlugin {
        container: container_entity,
        text: text_entity,
        enabled: true,
    });
}
