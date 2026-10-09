use std::collections::HashMap;
use wgpu::util::DeviceExt;

use crate::{
    asset::id::{MaterialId, MeshId},
    renderer::resource::GPUTransform,
};

#[derive(Debug)]
pub struct InstanceBatch {
    pub mesh: MeshId,
    pub material: MaterialId,
    pub instance_buffer: wgpu::Buffer,
    pub instance_count: u32,
}

#[derive(Debug, Default)]
pub struct InstanceManager {
    pub instance_batches: Vec<InstanceBatch>,
}

impl InstanceManager {
    pub fn new() -> Self {
        Self {
            instance_batches: Vec::new(),
        }
    }

    pub fn update_batches(
        &mut self,
        device: &wgpu::Device,
        drawables: &[crate::renderer::resource::GPUDrawable],
    ) {
        let mut batches: HashMap<(MeshId, MaterialId), Vec<GPUTransform>> = HashMap::new();

        for drawable in drawables {
            batches
                .entry((drawable.mesh, drawable.material))
                .or_default()
                .push(drawable.transform);
        }

        self.instance_batches.clear();

        for ((mesh, material), transforms) in batches {
            let instance_buffer: wgpu::Buffer =
                device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Instance Buffer"),
                    contents: bytemuck::cast_slice(&transforms),
                    usage: wgpu::BufferUsages::VERTEX,
                });

            self.instance_batches.push(InstanceBatch {
                mesh,
                material,
                instance_buffer,
                instance_count: transforms.len() as u32,
            });
        }
    }
}
