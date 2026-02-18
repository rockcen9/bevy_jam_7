use crate::prelude::*;
use rock_materials::{
    BlackholeMaterial, ChromaticAberrationMaterial, GlitchMaterial, GlitchSnakeMaterial,
    MpegArtifactMaterial, PoisonMaterial, WaveDistortionMaterial,
};

#[derive(Component, Debug, Clone)]
pub enum RequiredCustomMaterial {
    _BlackHole {
        distortion_strength: f32,
        rotation_speed: f32,
    },
    _ChromaticAberration {
        amount: f32,
        alpha: f32,
    },
    _Glitch {
        glitch_amount: f32,
        alpha: f32,
    },
    _GlitchSnake {
        strength: f32,
        frequency: f32,
        alpha: f32,
    },
    _MpegArtifact {
        intensity: f32,
        alpha: f32,
    },
    _Poison {
        poison_amount: f32,
        pulse_speed: f32,
        poison_color: LinearRgba,
    },
    _WaveDistortion {
        wave_center: Vec2,
        wave_params: Vec3,
        alpha: f32,
    },
}

// impl RequiredCustomMaterial {
//     pub fn chromatic() -> Self {
//         Self::ChromaticAberration {
//             amount: 0.05,
//             alpha: 1.0,
//         }
//     }
//     pub fn glitch() -> Self {
//         Self::Glitch {
//             glitch_amount: 0.5,
//             alpha: 1.0,
//         }
//     }
//     pub fn glitch_snake() -> Self {
//         Self::GlitchSnake {
//             strength: 0.01,
//             frequency: 10.0,
//             alpha: 1.0,
//         }
//     }
//     pub fn mpeg_artifact() -> Self {
//         Self::MpegArtifact {
//             intensity: 0.7,
//             alpha: 1.0,
//         }
//     }
//     pub fn poison() -> Self {
//         Self::Poison {
//             poison_amount: 0.8,
//             pulse_speed: 3.0,
//             poison_color: LinearRgba::new(0.3, 1.0, 0.3, 1.0), // Bright green
//         }
//     }
// }

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(Update, setup_custom_material);
}

pub fn setup_custom_material(
    q_actor: Query<(Entity, &RequiredCustomMaterial)>,
    q_belong_to: Query<(Entity, &BelongTo), With<MainMesh>>,
    q_prefab: Query<&PrefabId>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut blackhole_materials: ResMut<Assets<BlackholeMaterial>>,
    mut chromatic_materials: ResMut<Assets<ChromaticAberrationMaterial>>,
    mut glitch_materials: ResMut<Assets<GlitchMaterial>>,
    mut glitch_snake_materials: ResMut<Assets<GlitchSnakeMaterial>>,
    mut mpeg_materials: ResMut<Assets<MpegArtifactMaterial>>,
    mut poison_materials: ResMut<Assets<PoisonMaterial>>,
    mut wave_distortion_materials: ResMut<Assets<WaveDistortionMaterial>>,
    images: Res<Assets<Image>>,
    server: Res<AssetServer>,
) {
    for (main_mesh, belong_to) in q_belong_to.iter() {
        let Ok((actor, required_material)) = q_actor.get(belong_to.0) else {
            continue;
        };
        let Ok(prefab_id) = q_prefab.get(actor) else {
            continue;
        };

        match required_material {
            RequiredCustomMaterial::_BlackHole {
                distortion_strength,
                rotation_speed,
            } => {
                let path = format!("procreate/{}.png", prefab_id.id);
                let texture = server.load(path);

                // Wait for texture to load before creating mesh
                let Some(image) = images.get(&texture) else {
                    continue; // Texture not loaded yet, will retry next frame
                };

                let material = blackhole_materials.add(BlackholeMaterial {
                    texture,
                    distortion_strength: *distortion_strength,
                    rotation_speed: *rotation_speed,
                });

                // Use original texture dimensions for mesh size
                let texture_size = image.size_f32();
                let width = texture_size.x;
                let height = texture_size.y;

                commands.entity(main_mesh).insert((
                    Mesh2d(meshes.add(Rectangle::new(width, height))),
                    MeshMaterial2d(material),
                    Pickable::default(),
                    SpriteLayer::Pawn,
                ));
            }
            RequiredCustomMaterial::_ChromaticAberration { amount, alpha } => {
                let path = format!("procreate/{}.png", prefab_id.id);
                let texture = server.load(path);

                // Wait for texture to load before creating mesh
                let Some(image) = images.get(&texture) else {
                    continue; // Texture not loaded yet, will retry next frame
                };

                let material = chromatic_materials.add(ChromaticAberrationMaterial {
                    texture,
                    amount: *amount,
                    alpha: *alpha,
                });

                // Use original texture dimensions for mesh size
                let texture_size = image.size_f32();
                let width = texture_size.x;
                let height = texture_size.y;

                info!(
                    "Creating mesh for {} with original texture size: {}x{}",
                    prefab_id.id, width, height
                );

                commands.entity(main_mesh).insert((
                    Mesh2d(meshes.add(Rectangle::new(width, height))),
                    MeshMaterial2d(material),
                    Pickable::default(),
                    SpriteLayer::Pawn,
                ));
            }
            RequiredCustomMaterial::_Glitch {
                glitch_amount,
                alpha,
            } => {
                let path = format!("procreate/{}.png", prefab_id.id);
                let texture = server.load(path);

                // Wait for texture to load before creating mesh
                let Some(image) = images.get(&texture) else {
                    continue; // Texture not loaded yet, will retry next frame
                };

                let material = glitch_materials.add(GlitchMaterial {
                    texture,
                    glitch_amount: *glitch_amount,
                    alpha: *alpha,
                });

                // Use original texture dimensions for mesh size
                let texture_size = image.size_f32();
                let width = texture_size.x;
                let height = texture_size.y;

                info!(
                    "Creating mesh for {} with original texture size: {}x{}",
                    prefab_id.id, width, height
                );

                commands.entity(main_mesh).insert((
                    Mesh2d(meshes.add(Rectangle::new(width, height))),
                    MeshMaterial2d(material),
                    Pickable::default(),
                    SpriteLayer::Pawn,
                ));
            }
            RequiredCustomMaterial::_GlitchSnake {
                strength,
                frequency,
                alpha,
            } => {
                let path = format!("procreate/{}.png", prefab_id.id);
                let texture = server.load(path);

                // Wait for texture to load before creating mesh
                let Some(image) = images.get(&texture) else {
                    continue; // Texture not loaded yet, will retry next frame
                };

                let material = glitch_snake_materials.add(GlitchSnakeMaterial {
                    texture,
                    strength: *strength,
                    frequency: *frequency,
                    alpha: *alpha,
                });

                // Use original texture dimensions for mesh size
                let texture_size = image.size_f32();
                let width = texture_size.x;
                let height = texture_size.y;

                info!(
                    "Creating mesh for {} with original texture size: {}x{}",
                    prefab_id.id, width, height
                );

                commands.entity(main_mesh).insert((
                    Mesh2d(meshes.add(Rectangle::new(width, height))),
                    MeshMaterial2d(material),
                    Pickable::default(),
                    SpriteLayer::Pawn,
                ));
            }
            RequiredCustomMaterial::_MpegArtifact { intensity, alpha } => {
                let path = format!("procreate/{}.png", prefab_id.id);
                let texture = server.load(path);

                // Wait for texture to load before creating mesh
                let Some(image) = images.get(&texture) else {
                    continue; // Texture not loaded yet, will retry next frame
                };

                let material = mpeg_materials.add(MpegArtifactMaterial {
                    texture,
                    intensity: *intensity,
                    alpha: *alpha,
                });

                // Use original texture dimensions for mesh size
                let texture_size = image.size_f32();
                let width = texture_size.x;
                let height = texture_size.y;

                info!(
                    "Creating mesh for {} with original texture size: {}x{}",
                    prefab_id.id, width, height
                );

                commands.entity(main_mesh).insert((
                    Mesh2d(meshes.add(Rectangle::new(width, height))),
                    MeshMaterial2d(material),
                    Pickable::default(),
                    SpriteLayer::Pawn,
                ));
            }
            RequiredCustomMaterial::_Poison {
                poison_amount,
                pulse_speed,
                poison_color,
            } => {
                let path = format!("procreate/{}.png", prefab_id.id);
                let texture = server.load(path);

                // Wait for texture to load before creating mesh
                let Some(image) = images.get(&texture) else {
                    continue; // Texture not loaded yet, will retry next frame
                };

                let material = poison_materials.add(PoisonMaterial {
                    texture,
                    poison_amount: *poison_amount,
                    pulse_speed: *pulse_speed,
                    poison_color: *poison_color,
                });

                // Use original texture dimensions for mesh size
                let texture_size = image.size_f32();
                let width = texture_size.x;
                let height = texture_size.y;

                info!(
                    "Creating poisoned mesh for {} with texture size: {}x{}",
                    prefab_id.id, width, height
                );

                commands.entity(main_mesh).insert((
                    Mesh2d(meshes.add(Rectangle::new(width, height))),
                    MeshMaterial2d(material),
                    Pickable::default(),
                    SpriteLayer::Pawn,
                ));
            }
            RequiredCustomMaterial::_WaveDistortion {
                wave_center,
                wave_params,
                alpha,
            } => {
                let path = format!("procreate/{}.png", prefab_id.id);
                let texture = server.load(path);

                // Wait for texture to load before creating mesh
                let Some(image) = images.get(&texture) else {
                    continue; // Texture not loaded yet, will retry next frame
                };

                let material = wave_distortion_materials.add(WaveDistortionMaterial {
                    texture,
                    wave_center: *wave_center,
                    wave_params: *wave_params,
                    alpha: *alpha,
                    start_time: 0.0,
                });

                // Use original texture dimensions for mesh size
                let texture_size = image.size_f32();
                let width = texture_size.x;
                let height = texture_size.y;

                info!(
                    "Creating mesh for {} with original texture size: {}x{}",
                    prefab_id.id, width, height
                );

                commands.entity(main_mesh).insert((
                    Mesh2d(meshes.add(Rectangle::new(width, height))),
                    MeshMaterial2d(material),
                    Pickable::default(),
                    SpriteLayer::Pawn,
                ));
            }
        }

        commands.entity(actor).remove::<RequiredCustomMaterial>();
    }
}
