use crate::{
    asset::{
        id::{MaterialId, MeshId, TextureId},
        manager::{MaterialAsset, MeshAsset, TextureAsset},
    },
    renderer::mesh::GPUVertex,
};

// consider using references cloning into GPU assets might be costly for loading times, probably
// figure out the format for objects before doing this

#[derive(Default, Debug, PartialEq, Clone)]
pub struct GPUMeshAsset {
    pub vertices: Vec<GPUVertex>,
    pub indices: Vec<u16>,
}

impl From<MeshAsset> for GPUMeshAsset {
    fn from(value: MeshAsset) -> Self {
        todo!()
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct GPUTextureAsset {
    pub image: image::DynamicImage,
}

impl From<TextureAsset> for GPUTextureAsset {
    fn from(value: TextureAsset) -> Self {
        todo!()
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct GPUMaterialAsset {
    pub base_color: [f32; 4],
    pub metallic: f32,
    pub roughness: f32,
    pub base_color_texture: Option<TextureId>,
    pub normal_texture: Option<TextureId>,
    pub metallic_roughness_texture: Option<TextureId>,
    pub occlusion_texture: Option<TextureId>,
}

impl From<MaterialAsset> for GPUMaterialAsset {
    fn from(value: MaterialAsset) -> Self {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum GPUAssetBundle {
    MeshAsset(MeshId, GPUMeshAsset),
    TextureAsset(TextureId, GPUTextureAsset),
    MaterialAsset(MaterialId, GPUMaterialAsset),
}
