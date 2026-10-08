use crate::asset::id::TextureId;

pub mod id;
pub mod loader;
pub mod manager;
pub mod source;

#[repr(C)]
#[derive(Default, Debug, PartialEq, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct MeshAsset {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct SamplerDesc {
    pub address_u: wgpu::AddressMode,
    pub address_v: wgpu::AddressMode,
    pub mag_filter: wgpu::FilterMode,
    pub min_filter: wgpu::FilterMode,
    pub mipmap_filter: wgpu::MipmapFilterMode,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct TextureAsset {
    pub width: u32,
    pub height: u32,
    pub rgba8: Vec<u8>,
    pub sampler: SamplerDesc,
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct MaterialAsset {
    pub base_color: [f32; 4],
    pub base_color_texture: Option<TextureId>,
    pub normal_texture: Option<TextureId>,
}
