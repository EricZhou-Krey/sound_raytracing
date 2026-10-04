use crate::{
    component::{
        camera::{ActiveCamera, CameraProjection},
        object::Transform,
    },
    renderer::resource::{
        GPUCamera, GPUObject, GPUTransform, RenderCallbackObjectQueryState, RenderResource,
    },
};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct RenderCallback {
    pub camera: GPUCamera,
    pub objects: Vec<GPUObject>,
}

impl RenderCallback {
    pub fn extract_from_world(world: &mut bevy_ecs::world::World) -> Self {
        world.resource_scope(
            |world,
             mut query_resource: bevy_ecs::change_detection::Mut<
                RenderCallbackObjectQueryState,
            >| {
                let objects: Vec<GPUObject> = query_resource
                    .query_state
                    .iter_mut(world)
                    .map(|(transform, mesh, texture)| GPUObject {
                        transform: GPUTransform {
                            model: transform.to_raw(),
                        },
                        mesh: mesh.key,
                        texture: texture.key,
                    })
                    .collect();

                let active_camera: bevy_ecs::entity::Entity =
                    world.resource::<ActiveCamera>().camera;
                let camera_transform: &Transform = world.get::<Transform>(active_camera).unwrap();
                let projection: &CameraProjection = world.resource::<CameraProjection>();
                let camera: GPUCamera = GPUCamera {
                    view: camera_transform.to_inverse_raw(),
                    proj: projection.to_raw(),
                };

                RenderCallback { camera, objects }
            },
        )
    }
}

impl eframe::egui_wgpu::CallbackTrait for RenderCallback {
    fn prepare(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        _screen_descriptor: &eframe::egui_wgpu::ScreenDescriptor,
        _egui_encoder: &mut wgpu::CommandEncoder,
        callback_resources: &mut eframe::egui_wgpu::CallbackResources,
    ) -> Vec<wgpu::CommandBuffer> {
        let render_resource: &mut RenderResource = callback_resources
            .get_mut()
            .expect("RenderResource missing");
        queue.write_buffer(
            &render_resource.camera_buffer,
            0,
            bytemuck::bytes_of(&self.camera),
        );

        render_resource
            .instance_manager
            .update_batches(device, &self.objects);

        Vec::new()
    }

    fn paint(
        &self,
        _info: egui::PaintCallbackInfo,
        render_pass: &mut wgpu::RenderPass<'static>,
        callback_resources: &eframe::egui_wgpu::CallbackResources,
    ) {
        let render_resource: &RenderResource =
            callback_resources.get().expect("RenderResource missing");

        render_pass.set_pipeline(&render_resource.pipeline);
        render_pass.set_bind_group(0, &render_resource.camera_bind_group, &[]);

        for batch in &render_resource.instance_manager.instance_batches {
            let mesh = &render_resource.mesh_manager.meshes[batch.mesh];
            let texture = &render_resource.texture_manager.textures[batch.texture];

            render_pass.set_bind_group(1, &texture.bind_group, &[]);
            render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, batch.instance_buffer.slice(..));
            render_pass.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

            render_pass.draw_indexed(0..mesh.num_indices, 0, 0..batch.instance_count);
        }
    }
}
