use crate::{
    component::{
        camera::{ActiveCamera, CameraProjection},
        object::Transform,
    },
    renderer::{
        mesh::GPUMeshKey,
        resource::{GPUCamera, GPUTransform, RenderCallbackObjectQueryState, RenderResource},
        texture::GPUTextureKey,
    },
};

#[derive(Debug, Clone, PartialEq)]
pub struct GPUObject {
    mesh: GPUMeshKey,
    texture: GPUTextureKey,
    transform: GPUTransform,
}

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
                        transform: transform.to_gpu_transform(),
                        mesh: mesh.key,
                        texture: texture.key,
                    })
                    .collect();

                let active_camera: bevy_ecs::entity::Entity =
                    world.resource::<ActiveCamera>().camera;
                let camera_transform: &Transform = world.get::<Transform>(active_camera).unwrap();
                let projection: &CameraProjection = world.resource::<CameraProjection>();
                let camera: GPUCamera = GPUCamera {
                    view: glam::Mat4::from_rotation_translation(
                        camera_transform.rotation,
                        camera_transform.translation,
                    )
                    .to_cols_array_2d(),
                    proj: glam::camera::rh::proj::vulkan::perspective(
                        projection.vertical_fov,
                        projection.aspect_ratio,
                        projection.z_near,
                        projection.z_far,
                    )
                    .to_cols_array_2d(),
                };

                RenderCallback { camera, objects }
            },
        )
    }
}

impl eframe::egui_wgpu::CallbackTrait for RenderCallback {
    fn paint(
        &self,
        _info: egui::PaintCallbackInfo,
        render_pass: &mut wgpu::RenderPass<'static>,
        callback_resources: &eframe::egui_wgpu::CallbackResources,
    ) {
        let resources: &RenderResource = callback_resources.get().unwrap();
        render_pass.set_pipeline(&resources.pipeline);
    }
}
