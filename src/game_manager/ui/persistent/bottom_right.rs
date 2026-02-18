use crate::prelude::*;
use super::root::PersistentUiRoot;

#[derive(Component)]
struct VersionText;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_bottom_right_ui.after(super::root::spawn_persistent_ui_root));
}

fn spawn_bottom_right_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    root_query: Query<Entity, With<PersistentUiRoot>>,
) {
    let Ok(root_entity) = root_query.single() else {
        error!("PersistentUiRoot not found");
        return;
    };

    let font = asset_server.load("fonts/Quicksand-Regular.ttf");

    commands.entity(root_entity).with_children(|parent| {
        parent
            .spawn((
                Node {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(10.0),
                    right: Val::Px(10.0),
                    ..default()
                },
                Name::new("Bottom Right Container"),
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new(format!("v{}", GAME_VERSION)),
                    TextFont {
                        font,
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    VersionText,
                    Name::new("Version Text"),
                ));
            });
    });
}
