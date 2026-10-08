use crate::asset::{id::MeshId, MeshAsset, Vertex};
use wgpu::util::DeviceExt;

impl Vertex {
    const ATTRIBUTES: [wgpu::VertexAttribute; 3] = wgpu::vertex_attr_array![
        0 => Float32x3,
        1 => Float32x3,
        2 => Float32x2,
    ];

    pub fn layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct GPUMesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
}

#[derive(Debug)]
pub struct MeshManager {
    pub meshes: slotmap::SecondaryMap<MeshId, GPUMesh>,
}

impl Default for MeshManager {
    fn default() -> Self {
        Self::new()
    }
}

impl MeshManager {
    pub fn new() -> Self {
        Self {
            meshes: slotmap::SecondaryMap::new(),
        }
    }
    pub fn init_mesh(&mut self, device: &wgpu::Device, id: MeshId, asset: &MeshAsset) {
        let vertex_buffer: wgpu::Buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&asset.vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });

        let index_buffer: wgpu::Buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&asset.indices),
                usage: wgpu::BufferUsages::INDEX,
            });

        self.meshes.insert(
            id,
            GPUMesh {
                vertex_buffer,
                index_buffer,
                num_indices: asset.indices.len() as u32,
            },
        );
    }
}
