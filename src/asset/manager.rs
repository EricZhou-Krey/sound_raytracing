use crate::renderer::{material::GPUMaterialKey, mesh::GPUMeshKey, texture::GPUTextureKey};

slotmap::new_key_type! {
    pub struct MeshId;
    pub struct TextureId;
    pub struct MaterialId;
}

#[derive(Debug, PartialEq)]
pub struct MeshAsset {
    pub vertices: Vec<glam::Vec3>,
    pub indices: Vec<usize>,
    pub gpu_key: GPUMeshKey,
}

#[derive(Debug, PartialEq)]
pub struct TextureAsset {
    pub bytes: Vec<u8>,
    pub gpu_key: GPUTextureKey,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MaterialAsset {
    pub base_color: [f32; 4],
    pub metallic: f32,
    pub roughness: f32,

    pub base_color_texture: Option<TextureId>,
    pub normal_texture: Option<TextureId>,
    pub metallic_roughness_texture: Option<TextureId>,
    pub occlusion_texture: Option<TextureId>,

    pub gpu_key: Option<GPUMaterialKey>,
}

#[derive(Debug, Default, bevy_ecs::resource::Resource)]
pub struct MeshManager {
    pub meshes: slotmap::SlotMap<MeshId, MeshAsset>,
}

#[derive(Debug, Default, bevy_ecs::resource::Resource)]
pub struct TextureManager {
    pub textures: slotmap::SlotMap<TextureId, TextureAsset>,
}

#[derive(Debug, Default, bevy_ecs::resource::Resource)]
pub struct MaterialManager {
    pub materials: slotmap::SlotMap<MaterialId, MaterialAsset>,
}
