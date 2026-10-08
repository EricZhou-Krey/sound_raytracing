use crate::asset::id::{MaterialId, MeshId, TextureId};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Vertex {
    pub position: glam::Vec3,
    pub normal: glam::Vec3,
    pub uv: glam::Vec2,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MeshAsset {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextureAsset {
    pub bytes: Vec<u8>,
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
