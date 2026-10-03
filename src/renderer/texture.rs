slotmap::new_key_type! {
    struct GPUTextureKey;
}

#[derive(Debug, PartialEq)]
pub struct GPUTexture {
    texture: wgpu::Texture,
    view: wgpu::TextureView,
    bind_group: wgpu::BindGroup,
}

#[derive(Debug)]
pub struct TextureManager {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub textures: slotmap::SlotMap<GPUTextureKey, GPUTexture>,
}

struct GPUTextureLoadError;

impl TextureManager {
    pub fn load_texture_from_bytes(
        &mut self,
        bytes: &[u8],
    ) -> Result<GPUTextureKey, GPUTextureLoadError> {
        todo!()
    }

    pub fn load_texture(
        &mut self,
        path: impl AsRef<std::path::Path>,
    ) -> Result<GPUTextureKey, GPUTextureLoadError> {
        todo!()
    }

    pub fn get_texture(&self, key: &GPUTextureKey) -> Option<GPUTextureLoadError> {
        todo!()
    }

    pub fn remove_texture(&mut self, key: &GPUTextureKey) -> Option<GPUTextureLoadError> {
        todo!()
    }
}

/*

// Add texture formatting

let image_bytes = include_bytes!("*.png");
let image = image::load_from_memory(image_bytes).unwrap();
let image_rgba = image.to_rgba8();

let texture_size = wgpu::Extent3d {
    width: dimensions.0,
    height: dimensions.1,
    depth_or_array_layers: 1,
};

let texture = device.create_texture(
    &wgpu::TextureDescriptor {
        size: texture_size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        label: Some("texture"),
        view_formats: &[],
    }
);

queue.write_texture(
    wgpu::TexelCopyTextureInfo {
        texture: &diffuse_texture,
        mip_level: 0,
        origin: wgpu::Origin3d::ZERO,
        aspect: wgpu::TextureAspect::All,
    },
    &diffuse_rgba,
    wgpu::TexelCopyBufferLayout {
        offset: 0,
        bytes_per_row: Some(4 * dimensions.0),
        rows_per_image: Some(dimensions.1),
    },
    texture_size,
);

use image::GenericImageView;
let dimensions = image.dimensions();

let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
let texture_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
    address_mode_u: wgpu::AddressMode::ClampToEdge,
    address_mode_v: wgpu::AddressMode::ClampToEdge,
    address_mode_w: wgpu::AddressMode::ClampToEdge,
    mag_filter: wgpu::FilterMode::Linear,
    min_filter: wgpu::FilterMode::Nearest,
    mipmap_filter: wgpu::MipmapFilterMode::Nearest,
    ..Default::default()
});

let texture_bind_group_layout: wgpu::BindGroupLayout =
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("texture_bind_group_layout"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
        ],
    });

let texture_bind_group: wgpu::BindGroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
    label: Some("texture_bind_group"),
    layout: &texture_bind_group_layout,
    entries: &[
        wgpu::BindGroupEntry {
            binding: 0,
            resource: wgpu::BindingResource::TextureView(&texture_bind_grou)
        }
    ],
});

*/
