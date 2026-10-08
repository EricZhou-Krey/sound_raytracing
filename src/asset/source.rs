use crate::{
    asset::{MeshAsset, TextureAsset},
    component::object::Transform,
};

#[derive(Debug, Clone, Default)]
pub struct SceneSource {
    pub meshes: Vec<MeshSource>,
    pub textures: Vec<TextureSource>,
    pub materials: Vec<MaterialSource>,
    pub objects: Vec<ObjectSource>,
}

#[derive(Debug, Clone)]
pub enum MeshSource {
    Description(MeshAsset),
}

#[derive(Debug, Clone)]
pub enum TextureSource {
    Description(TextureAsset),
}

#[derive(Debug, Clone)]
pub enum MaterialSource {
    Description {
        base_color: [f32; 4],
        base_color_texture: Option<usize>,
        normal_texture: Option<usize>,
    },
}

#[derive(Debug, Clone)]
pub struct ObjectSource {
    pub transform: Transform,
    pub mesh: usize,
    pub material: usize,
}
