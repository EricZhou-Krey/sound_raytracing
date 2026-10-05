use bevy_ecs::{
    event::Event,
    observer::On,
    prelude::{Commands, Component, Entity, Query, ResMut},
};

use crate::{
    asset::{
        manager::{MeshAsset, MeshId, MeshManager, TextureAsset, TextureId, TextureManager},
        primitive::PrimitiveMesh,
    },
    component::object::{Mesh, Texture, Transform},
};

#[derive(Debug, Clone)]
pub enum MeshSource {
    Path(String),
    Primitive(PrimitiveMesh),
}

#[derive(Debug, Clone)]
pub enum TextureSource {
    Path(String),
    None,
}

#[derive(Debug, Clone, Component)]
pub struct PendingMesh {
    pub source: MeshSource,
}

#[derive(Debug, Clone, Component)]
pub struct PendingTexture {
    pub source: TextureSource,
}

#[derive(Debug, Event)]
pub struct RequestLoadObject {
    pub mesh: MeshSource,
    pub texture: TextureSource,
}

// Distisnguish between textures and materials of a base face colour, and do this for the GPU
// objects aswell during extraction

pub fn request_init_object(trigger: On<RequestLoadObject>, mut commands: Commands) {
    let event: &RequestLoadObject = trigger.event();

    let entity: Entity = commands
        .spawn(Transform::default())
        .insert(PendingMesh {
            source: event.mesh.clone(),
        })
        .id();

    if !matches!(event.texture, TextureSource::None) {
        commands.entity(entity).insert(PendingTexture {
            source: event.texture.clone(),
        });
    }
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

pub fn load_texture(
    mut commands: Commands,
    mut manager: ResMut<TextureManager>,
    query: Query<(Entity, &PendingTexture)>,
) {
    for (entity, pending) in query.iter() {
        let TextureSource::Path(path): &TextureSource = &pending.source else {
            commands.entity(entity).remove::<PendingTexture>();
            continue;
        };

        let Some(asset): Option<TextureAsset> = load_texture_asset(path) else {
            continue;
        };

        let id: TextureId = manager.textures.insert(asset);

        commands
            .entity(entity)
            .remove::<PendingTexture>()
            .insert(Texture { id });
    }
}

fn load_mesh_asset(source: &MeshSource) -> Option<MeshAsset> {
    match source {
        MeshSource::Path(path) => load_mesh_from_path(path),
        MeshSource::Primitive(primitive) => Some(load_mesh_from_primitive(primitive)),
    }
}

fn load_mesh_from_path(path: &str) -> Option<MeshAsset> {
    let _path: &str = path;

    todo!()
}

fn load_mesh_from_primitive(primitive: &PrimitiveMesh) -> MeshAsset {
    let _primitive: &PrimitiveMesh = primitive;

    todo!()
}

fn load_texture_asset(path: &str) -> Option<TextureAsset> {
    let _path: &str = path;

    todo!()
}
