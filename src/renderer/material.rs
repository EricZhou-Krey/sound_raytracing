use crate::renderer::texture::{GPUTextureKey, TextureManager};
use wgpu::util::DeviceExt;

#[derive(Debug, PartialEq, Clone)]
pub struct MaterialAsset {
    pub base_color: [f32; 4],
    pub metallic: f32,
    pub roughness: f32,
    pub base_color_texture: Option<GPUTextureKey>,
    pub normal_texture: Option<GPUTextureKey>,
    pub metallic_roughness_texture: Option<GPUTextureKey>,
    pub occlusion_texture: Option<GPUTextureKey>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GPUMaterialUniforms {
    pub base_color: [f32; 4],
    pub metallic_roughness: [f32; 4],
}

slotmap::new_key_type! {
    pub struct GPUMaterialKey;
}

#[derive(Debug)]
pub struct GPUMaterial {
    pub uniform_buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
}

pub struct DefaultTextures {
    pub white: GPUTextureKey,
    pub flat_normal: GPUTextureKey,
}

#[derive(Debug)]
pub struct MaterialManager {
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub materials: slotmap::SlotMap<GPUMaterialKey, GPUMaterial>,
}

impl MaterialManager {
    pub fn new(bind_group_layout: wgpu::BindGroupLayout) -> Self {
        Self {
            bind_group_layout,
            materials: slotmap::SlotMap::with_key(),
        }
    }

    pub fn init_material(
        &mut self,
        device: &wgpu::Device,
        material: &MaterialAsset,
        texture_manager: &TextureManager,
        default_textures: &DefaultTextures,
    ) -> GPUMaterialKey {
        let base_color_key: GPUTextureKey = material
            .base_color_texture
            .unwrap_or(default_textures.white);
        let normal_key: GPUTextureKey = material
            .normal_texture
            .unwrap_or(default_textures.flat_normal);
        let metallic_roughness_key: GPUTextureKey = material
            .metallic_roughness_texture
            .unwrap_or(default_textures.white);
        let occlusion_key: GPUTextureKey =
            material.occlusion_texture.unwrap_or(default_textures.white);

        let uniforms: GPUMaterialUniforms = GPUMaterialUniforms {
            base_color: material.base_color,
            metallic_roughness: [material.metallic, material.roughness, 0.0, 0.0],
        };

        let uniform_buffer: wgpu::Buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Material Uniform Buffer"),
                contents: bytemuck::bytes_of(&uniforms),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

        let base_color_view: &wgpu::TextureView = &texture_manager.textures[base_color_key].view;
        let normal_view: &wgpu::TextureView = &texture_manager.textures[normal_key].view;
        let metallic_roughness_view: &wgpu::TextureView =
            &texture_manager.textures[metallic_roughness_key].view;
        let occlusion_view: &wgpu::TextureView = &texture_manager.textures[occlusion_key].view;

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
                    resource: wgpu::BindingResource::TextureView(metallic_roughness_view),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: wgpu::BindingResource::TextureView(occlusion_view),
                },
                wgpu::BindGroupEntry {
                    binding: 5,
                    resource: wgpu::BindingResource::Sampler(&texture_manager.sampler),
                },
            ],
        });

        self.materials.insert(GPUMaterial {
            uniform_buffer,
            bind_group,
        })
    }
}
