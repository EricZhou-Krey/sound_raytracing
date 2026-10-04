use crate::renderer::{mesh::GPUMeshKey, resource::GPUTransform, texture::GPUTextureKey};

#[derive(Debug, Clone, PartialEq)]
pub struct GPUObject {
    mesh: GPUMeshKey,
    texture: GPUTextureKey,
    transform: GPUTransform,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct RenderCallback {
    pub objects: Vec<GPUObject>,
}

impl RenderCallback {
    pub fn extract_from_world(world: &mut bevy_ecs::world::World) -> Self {
        todo!()
    }
}

impl eframe::egui_wgpu::CallbackTrait for RenderCallback {
    fn prepare(
        &self,
        _device: &wgpu::Device,
        _queue: &wgpu::Queue,
        _screen_descriptor: &eframe::egui_wgpu::ScreenDescriptor,
        _egui_encoder: &mut wgpu::CommandEncoder,
        _callback_resources: &mut eframe::egui_wgpu::CallbackResources,
    ) -> Vec<wgpu::CommandBuffer> {
        todo!()
    }

    fn finish_prepare(
        &self,
        _device: &wgpu::Device,
        _queue: &wgpu::Queue,
        _egui_encoder: &mut wgpu::CommandEncoder,
        _callback_resources: &mut eframe::egui_wgpu::CallbackResources,
    ) -> Vec<wgpu::CommandBuffer> {
        todo!()
    }

    fn paint(
        &self,
        info: egui::PaintCallbackInfo,
        render_pass: &mut wgpu::RenderPass<'static>,
        callback_resources: &eframe::egui_wgpu::CallbackResources,
    ) {
        todo!()
    }
}
