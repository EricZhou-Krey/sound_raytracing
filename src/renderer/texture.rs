use crate::asset::id::TextureId;
use wgpu::util::DeviceExt;

#[derive(Debug, PartialEq)]
pub struct GPUTexture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
}

#[derive(Debug)]
pub struct TextureManager {
    pub sampler: wgpu::Sampler,
    pub textures: slotmap::SlotMap<TextureId, GPUTexture>,
}

impl TextureManager {
    pub fn new(sampler: wgpu::Sampler) -> Self {
        Self {
            sampler,
            textures: slotmap::SlotMap::with_key(),
        }
    }

    pub fn init_texture(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        image: image::DynamicImage,
    ) -> TextureId {
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

        self.textures.insert(GPUTexture { texture, view })
    }
}
