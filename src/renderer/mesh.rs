use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Default, Debug, PartialEq, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GPUVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
}

impl GPUVertex {
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

slotmap::new_key_type! {
    pub struct GPUMeshKey;
}

#[derive(Debug, PartialEq)]
pub struct GPUMesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
}

#[derive(Debug)]
pub struct MeshManager {
    pub meshes: slotmap::SlotMap<GPUMeshKey, GPUMesh>,
}

impl Default for MeshManager {
    fn default() -> Self {
        Self::new()
    }
}

impl MeshManager {
    pub fn new() -> Self {
        Self {
            meshes: slotmap::SlotMap::with_key(),
        }
    }
    pub fn init_mesh(
        &mut self,
        device: &wgpu::Device,
        vertices: &[GPUVertex],
        indices: &[u16],
    ) -> GPUMeshKey {
        let vertex_buffer: wgpu::Buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });

        let index_buffer: wgpu::Buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(indices),
                usage: wgpu::BufferUsages::INDEX,
            });

        self.meshes.insert(GPUMesh {
            vertex_buffer,
            index_buffer,
            num_indices: indices.len() as u32,
        })
    }
}
