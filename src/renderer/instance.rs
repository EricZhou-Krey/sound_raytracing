use crate::{
    asset::manager::{MaterialId, MeshId},
    renderer::{material::GPUMaterialKey, mesh::GPUMeshKey, resource::GPUTransform},
};
use std::collections::HashMap;
use wgpu::util::DeviceExt;

#[derive(Debug)]
pub struct InstanceBatch {
    pub mesh: GPUMeshKey,
    pub material: GPUMaterialKey,
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
        objects: &[crate::renderer::resource::GPUObject],
        mesh_mapping: &HashMap<MeshId, GPUMeshKey>,
        material_mapping: &HashMap<MaterialId, GPUMaterialKey>,
    ) {
        let mut batches: HashMap<(GPUMeshKey, GPUMaterialKey), Vec<GPUTransform>> = HashMap::new();

        for object in objects {
            batches
                .entry((
                    *mesh_mapping.get(&object.mesh).unwrap(),
                    *material_mapping.get(&object.material).unwrap(),
                ))
                .or_default()
                .push(object.transform);
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
