use wgpu::util::DeviceExt;

slotmap::new_key_type! {
    pub struct GPUTextureKey;
}

#[derive(Debug, PartialEq)]
pub struct GPUTexture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub bind_group: wgpu::BindGroup,
}

#[derive(Debug)]
pub struct TextureManager {
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub sampler: wgpu::Sampler,
    pub textures: slotmap::SlotMap<GPUTextureKey, GPUTexture>,
}

impl TextureManager {
    pub fn new(bind_group_layout: wgpu::BindGroupLayout, sampler: wgpu::Sampler) -> Self {
        Self {
            bind_group_layout,
            sampler,
            textures: slotmap::SlotMap::with_key(),
        }
    }

    pub fn init_texture(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bytes: &[u8],
    ) -> anyhow::Result<GPUTextureKey> {
        let image: image::DynamicImage = image::load_from_memory(bytes)?;
        let image_rgba: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = image.to_rgba8();

        use image::GenericImageView;
        let dimensions = image.dimensions();

        let texture_size: wgpu::Extent3d = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        let texture: wgpu::Texture = device.create_texture_with_data(
            queue,
            &wgpu::TextureDescriptor {
                label: Some("Texture"),
                size: texture_size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            },
            wgpu::wgt::TextureDataOrder::MipMajor,
            &image_rgba,
        );

        let view: wgpu::TextureView = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let bind_group: wgpu::BindGroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Texture Bind Group"),
            layout: &self.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.sampler),
                },
            ],
        });

        Ok(self.textures.insert(GPUTexture {
            texture,
            view,
            bind_group,
        }))
    }
}
