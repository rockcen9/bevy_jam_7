use crate::prelude::*;

#[derive(Component)]
pub struct MainMesh2d;

#[derive(Component, Default)]
pub struct RequiredMesh2d;

pub(crate) fn plugin(app: &mut bevy::app::App) {
    app.add_observer(setup_mesh);
    app.add_systems(Update, setup_mesh_material);
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
            Name::new("MainMesh2d"),
            MainMesh2d,
            ChildOf(model),
            BelongTo(belong_to.0),
        ));
    }
}

pub fn setup_mesh_material(
    q_actor: Query<Entity, With<RequiredMesh2d>>,
    q_belong_to: Query<(Entity, &BelongTo), With<MainMesh2d>>,
    q_prefab: Query<&PrefabId>,
    mut commands: Commands,
    server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (main_mesh, belong_to) in q_belong_to.iter() {
        let Ok(actor) = q_actor.get(belong_to.0) else {
            continue;
        };
        let Ok(prefab_id) = q_prefab.get(actor) else {
            continue;
        };

        let path = format!("procreate/{}.png", prefab_id.id);

        // Create a quad mesh (64x64 pixels based on unit size)
        let mesh = Mesh::from(Rectangle::new(64.0, 64.0));

        // Create a material with the loaded texture
        let material = ColorMaterial {
            texture: Some(server.load(path)),
            ..Default::default()
        };

        commands.entity(main_mesh).insert((
            Pickable::default(),
            Mesh2d(meshes.add(mesh)),
            MeshMaterial2d(materials.add(material)),
            SpriteLayer::Pawn,
        ));
        commands.entity(actor).remove::<RequiredMesh2d>();
    }
}
