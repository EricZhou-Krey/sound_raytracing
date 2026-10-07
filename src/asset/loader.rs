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
    Description {
        vertices: Vec<glam::Vec3>,
        indicies: Vec<usize>,
    },
}
#[derive(Debug, Clone)]
pub enum TextureSource {
    Path(String),
}

#[derive(Debug, Clone)]
pub enum MaterialSource {
    Path(String),
    Description {
        base_color: [f32; 4],
        metallic: f32,
        roughness: f32,
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

#[derive(Debug, Clone, Component)]
pub struct PendingGPUMesh {
    pub id: MeshId,
}

#[derive(Debug, Clone, Component)]
pub struct PendingGPUTexture {
    pub id: TextureId,
}

#[derive(Debug, Clone, Component)]
pub struct PendingGPUMaterial {
    pub id: MaterialId,
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
        let Ok(asset): anyhow::Result<MeshAsset> = load_mesh_asset(&pending.source) else {
            continue;
        };

        let id: MeshId = manager.meshes.insert(asset);

        commands
            .entity(entity)
            .remove::<PendingMesh>()
            .insert(Mesh { id });

        commands.spawn(PendingGPUMesh { id });
    }
}

pub fn load_material(
    mut commands: Commands,
    mut material_manager: ResMut<MaterialManager>,
    mut texture_manager: ResMut<TextureManager>,
    query: Query<(Entity, &PendingMaterial)>,
) {
    for (entity, pending) in query.iter() {
        let Ok(asset): anyhow::Result<MaterialAsset> =
            load_material_asset(&mut texture_manager, &pending.source)
        else {
            continue;
        };

        let id: MaterialId = material_manager.materials.insert(asset);

        commands
            .entity(entity)
            .remove::<PendingMaterial>()
            .insert(Material { id });

        commands.spawn(PendingGPUMaterial { id });

        // NEED to upload a request to init the mesh CPU side taken from the GPU prepare
    }
}

// TODO: research obj and object file formats to parse, or find a create that parses the differnet
// types of resources for to describe an object or series of objects or scene, maybe a pub fn
// request_load_scene could be used, also need to load lights in the same way
// NEED access to callback resources

fn load_mesh_asset(source: &MeshSource) -> anyhow::Result<MeshAsset> {
    match source {
        MeshSource::Path(_path) => todo!(),
        MeshSource::Description { vertices, indicies } => Ok(MeshAsset {
            vertices: vertices.clone(),
            indices: indicies.clone(),
        }),
    }
}

fn load_texture_asset(source: &TextureSource) -> anyhow::Result<TextureAsset> {
    match source {
        TextureSource::Path(_path) => todo!(),
    }
}

fn load_material_asset(
    mut _texture_manager: &mut ResMut<TextureManager>,
    source: &MaterialSource,
) -> anyhow::Result<MaterialAsset> {
    match source {
        MaterialSource::Path(_path) => todo!(),
        MaterialSource::Description {
            base_color,
            metallic,
            roughness,
        } => Ok(MaterialAsset {
            base_color: *base_color,
            metallic: *metallic,
            roughness: *roughness,
            base_color_texture: None,
            normal_texture: None,
            metallic_roughness_texture: None,
            occlusion_texture: None,
        }),
    }
}
