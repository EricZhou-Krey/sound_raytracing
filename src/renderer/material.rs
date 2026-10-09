use crate::{
    asset::{
        id::{MaterialId, TextureId},
        MaterialAsset,
    },
    renderer::texture::TextureManager,
};
use wgpu::util::DeviceExt;

#[derive(Debug)]
pub struct GPUMaterial {
    pub uniform_buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
}

#[derive(Debug)]
pub struct DefaultTextures {
    pub white: TextureId,
    pub flat_normal: TextureId,
}

#[derive(Debug)]
pub struct MaterialManager {
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub materials: slotmap::SecondaryMap<MaterialId, GPUMaterial>,
}

impl MaterialManager {
    pub fn new(bind_group_layout: wgpu::BindGroupLayout) -> Self {
        Self {
            bind_group_layout,
            materials: slotmap::SecondaryMap::new(),
        }
    }

    pub fn init_material(
        &mut self,
        device: &wgpu::Device,
        id: MaterialId,
        asset: &MaterialAsset,
        texture_manager: &TextureManager,
    ) {
        let uniform_buffer: wgpu::Buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Material Uniform Buffer"),
                contents: bytemuck::cast_slice(&asset.base_color),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

        let base_color_view: &wgpu::TextureView =
            &texture_manager.textures[asset.base_color_texture].view;
        let normal_view: &wgpu::TextureView = &texture_manager.textures[asset.normal_texture].view;

        let bind_group: wgpu::BindGroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Material Bind Group"),
            layout: &self.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(base_color_view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(normal_view),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::Sampler(&texture_manager.sampler),
                },
            ],
        });

        self.materials.insert(
            id,
            GPUMaterial {
                uniform_buffer,
                bind_group,
            },
        );
    }
}
