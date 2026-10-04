use crate::renderer::{
    mesh::GPUMeshKey,
    resource::{GPUTransform, RenderCallbackObjectQueryState},
    texture::GPUTextureKey,
};

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
        world.resource_scope(
            |world,
             mut query_resource: bevy_ecs::change_detection::Mut<
                RenderCallbackObjectQueryState,
            >| {
                let objects = query_resource
                    .query_state
                    .iter_mut(world)
                    .map(|(transform, mesh, texture)| GPUObject {
                        transform: transform.to_gpu_transform(),
                        mesh: mesh.key,
                        texture: texture.key,
                    })
                    .collect();

                RenderCallback { objects }
            },
        )
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
        _info: egui::PaintCallbackInfo,
        _render_pass: &mut wgpu::RenderPass<'static>,
        _callback_resources: &eframe::egui_wgpu::CallbackResources,
    ) {
        todo!()
    }
}
