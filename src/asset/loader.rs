use bevy_ecs::{
    event::Event,
    observer::On,
    prelude::{Commands, Component, Entity, Query, ResMut},
};

use crate::{
    asset::manager::{
        MaterialAsset, MaterialId, MaterialManager, MeshAsset, MeshId, MeshManager, TextureAsset,
        TextureId, TextureManager,
    },
    component::object::{Material, Mesh, Transform},
};

#[derive(Debug, Clone)]
pub enum MeshSource {
    Path(String),
}

#[derive(Debug, Clone)]
pub enum MaterialSource {
    Color {
        rgba: [f32; 4],
        metallic: f32,
        roughness: f32,
    },
    Textured {
        base_color: [f32; 4],
        metallic: f32,
        roughness: f32,
        base_color_texture: Option<String>,
        normal_texture: Option<String>,
        metallic_roughness_texture: Option<String>,
        occlusion_texture: Option<String>,
    },
}

#[derive(Debug, Clone, Component)]
pub struct PendingMesh {
    pub source: MeshSource,
}

#[derive(Debug, Clone, Component)]
pub struct PendingMaterial {
    pub source: MaterialSource,
}

#[derive(Debug, Event)]
pub struct RequestLoadObject {
    pub mesh: MeshSource,
    pub material: MaterialSource,
}

pub fn request_init_object(trigger: On<RequestLoadObject>, mut commands: Commands) {
    let event: &RequestLoadObject = trigger.event();

    commands
        .spawn(Transform::default())
        .insert(PendingMesh {
            source: event.mesh.clone(),
        })
        .insert(PendingMaterial {
            source: event.material.clone(),
        });
}

pub fn load_mesh(
    mut commands: Commands,
    mut manager: ResMut<MeshManager>,
    query: Query<(Entity, &PendingMesh)>,
) {
    for (entity, pending) in query.iter() {
        let Some(asset): Option<MeshAsset> = load_mesh_asset(&pending.source) else {
            continue;
        };

        let id: MeshId = manager.meshes.insert(asset);

        commands
            .entity(entity)
            .remove::<PendingMesh>()
            .insert(Mesh { id });
    }
}

pub fn load_material(
    mut commands: Commands,
    mut material_manager: ResMut<MaterialManager>,
    mut texture_manager: ResMut<TextureManager>,
    query: Query<(Entity, &PendingMaterial)>,
) {
    for (entity, pending) in query.iter() {
        let mut load_texture = |path_option: &Option<String>| -> Option<TextureId> {
            if let Some(path) = path_option {
                if let Some(asset) = load_texture_asset(path) {
                    return Some(texture_manager.textures.insert(asset));
                }
            }
            None
        };

        let asset: MaterialAsset = match &pending.source {
            MaterialSource::Color {
                rgba,
                metallic,
                roughness,
            } => MaterialAsset {
                base_color: *rgba,
                metallic: *metallic,
                roughness: *roughness,
                base_color_texture: None,
                normal_texture: None,
                metallic_roughness_texture: None,
                occlusion_texture: None,
                gpu_key: None,
            },
            MaterialSource::Textured {
                base_color,
                metallic,
                roughness,
                base_color_texture,
                normal_texture,
                metallic_roughness_texture,
                occlusion_texture,
            } => MaterialAsset {
                base_color: *base_color,
                metallic: *metallic,
                roughness: *roughness,
                base_color_texture: load_texture(base_color_texture),
                normal_texture: load_texture(normal_texture),
                metallic_roughness_texture: load_texture(metallic_roughness_texture),
                occlusion_texture: load_texture(occlusion_texture),
                gpu_key: None,
            },
        };

        let id: MaterialId = material_manager.materials.insert(asset);

        commands
            .entity(entity)
            .remove::<PendingMaterial>()
            .insert(Material { id });
    }
}

fn load_mesh_asset(source: &MeshSource) -> Option<MeshAsset> {
    match source {
        MeshSource::Path(_path) => todo!(),
    }
}

fn load_texture_asset(path: &str) -> Option<TextureAsset> {
    let _path: &str = path;
    todo!()
}
