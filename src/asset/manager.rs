use crate::asset::{
    id::{MaterialId, MeshId, TextureId},
    MaterialAsset, MeshAsset, TextureAsset,
};

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
