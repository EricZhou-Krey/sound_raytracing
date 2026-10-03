use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
}

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

pub struct Instance {
    position: glam::Vec3,
    rotation: glam::Quat,
}

impl Instance {
    pub fn to_raw(&self) -> InstanceRaw {
        InstanceRaw {
            model: glam::Mat4::from_rotation_translation(self.rotation, self.position)
                .to_cols_array_2d(),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct InstanceRaw {
    model: [[f32; 4]; 4],
}

impl InstanceRaw {
    const ATTRIBUTES: [wgpu::VertexAttribute; 4] = wgpu::vertex_attr_array![
        3 => Float32x4,
        4 => Float32x4,
        5 => Float32x4,
        5 => Float32x4,
    ];

    pub fn layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRIBUTES,
        }
    }
}

slotmap::new_key_type! {
    struct GPUMeshKey;
}

#[derive(Debug, PartialEq)]
pub struct GPUMesh {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
}

#[derive(Debug)]
pub struct MeshManager {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub meshes: slotmap::SlotMap<GPUMeshKey, GPUMesh>,
}

struct GPUMeshLoadError;

impl MeshManager {
    pub fn load_mesh_from_bytes(&mut self, bytes: &[u8]) -> Result<GPUMeshKey, GPUMeshLoadError> {
        todo!()
    }

    pub fn load_mesh(
        &mut self,
        path: impl AsRef<std::path::Path>,
    ) -> Result<GPUMeshKey, GPUMeshLoadError> {
        todo!()
    }

    pub fn get_mesh(&self, key: &GPUMeshKey) -> Option<GPUMesh> {
        todo!()
    }

    pub fn remove_mesh(&mut self, key: &GPUMeshKey) -> Option<GPUMesh> {
        todo!()
    }
}
