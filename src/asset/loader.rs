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
    Description {},
}
#[derive(Debug, Clone)]
pub enum TextureSource {
    Path(String),
    Description {},
}

#[derive(Debug, Clone)]
pub enum MaterialSource {
    Path(String),
    Description {
        base_color: [f32; 4],
        metallic: f32,
        roughness: f32,
        base_color_texture: Option<TextureSource>,
        normal_texture: Option<TextureSource>,
        metallic_roughness_texture: Option<TextureSource>,
        occlusion_texture: Option<TextureSource>,
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
        let Some(asset): Option<MaterialAsset> =
            load_material_asset(&mut texture_manager, &pending.source)
        else {
            continue;
        };

        let id: MaterialId = material_manager.materials.insert(asset);

        commands
            .entity(entity)
            .remove::<PendingMaterial>()
            .insert(Material { id });
    }
}

fn load_mesh_asset(source: &MeshSource) -> Option<MeshAsset> {
    todo!()
}

fn load_texture_asset(source: &TextureSource) -> Option<TextureAsset> {
    todo!()
}

fn load_material_asset(
    mut texture_manager: &mut ResMut<TextureManager>,
    source: &MaterialSource,
) -> Option<MaterialAsset> {
    todo!()
}
