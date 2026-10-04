use std::collections::HashMap;
use wgpu::util::DeviceExt;

use crate::renderer::{
    mesh::GPUMeshKey,
    resource::{GPUObject, GPUTransform},
    texture::GPUTextureKey,
};

#[derive(Debug, PartialEq)]
pub struct InstanceBatch {
    pub mesh: GPUMeshKey,
    pub texture: GPUTextureKey,
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

    pub fn update_batches(&mut self, device: &wgpu::Device, objects: &[GPUObject]) {
        let mut batches: HashMap<(GPUMeshKey, GPUTextureKey), Vec<GPUTransform>> = HashMap::new();

        for object in objects {
            batches
                .entry((object.mesh, object.texture))
                .or_default()
                .push(object.transform);
        }

        self.instance_batches = batches
            .into_iter()
            .map(|((mesh, texture), transforms)| {
                let instance_buffer =
                    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("Instance Buffer"),
                        contents: bytemuck::cast_slice(&transforms),
                        usage: wgpu::BufferUsages::VERTEX,
                    });

                InstanceBatch {
                    mesh,
                    texture,
                    instance_buffer,
                    instance_count: transforms.len() as u32,
                }
            })
            .collect();
    }
}
