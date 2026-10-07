use crate::{
    asset::manager::{
        MaterialAsset, MaterialId, MaterialManager, MeshAsset, MeshId, MeshManager, TextureAsset,
        TextureId,
    },
    component::{
        camera::{ActiveCamera, CameraProjection},
        object::Transform,
    },
    renderer::{
        material::GPUMaterial,
        mesh::GPUMesh,
        resource::{
            GPUCamera, GPULight, GPUObject, GPUTransform, RenderCallbackObjectQueryState,
            RenderResource,
        },
    },
};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct RenderCallback<'a> {
    pub camera: GPUCamera,
    // pub lights: Vec<GPULight>,
    pub objects: Vec<GPUObject>,
    pub pending_meshes: Vec<(MeshId, &'a MeshAsset)>,
    pub pending_textures: Vec<(TextureId, &'a TextureAsset)>,
    pub pending_materials: Vec<(MaterialId, &'a MaterialAsset)>,
}

impl<'a> RenderCallback<'a> {
    pub fn extract_from_world(world: &mut bevy_ecs::world::World) -> Self {
        let camera: GPUCamera = Self::extract_camera(world);
        // let lights: Vec<GPULight> = Self::extract_lights(world);
        let objects: Vec<GPUObject> = Self::extract_objects(world);

        // Grab pending meshes, textures and materials, remove them from the world then create a
        // GPU handle when inserting them

        Self {
            camera,
            // lights,
            objects,
        }
    }

    fn extract_objects(world: &mut bevy_ecs::world::World) -> Vec<GPUObject> {
        world.resource_scope(
                |world,
                 mut query_state: bevy_ecs::change_detection::Mut<
                    RenderCallbackObjectQueryState,
                >| {
                    query_state
                        .query_state
                        .iter(world)
                        .map(|(transform, mesh, material)| {
                            GPUObject {
                                transform: GPUTransform {
                                    model: transform.to_raw(),
                                },
                                mesh: mesh.id,
                                material: material.id,
                            }
                        })
                        .collect()
                },
            )
    }

    fn extract_lights(world: &bevy_ecs::world::World) -> Vec<GPULight> {
        todo!()
    }

    fn extract_camera(world: &bevy_ecs::world::World) -> GPUCamera {
        let active_camera: bevy_ecs::entity::Entity = world.resource::<ActiveCamera>().camera;

        let camera_transform: &Transform = world
            .get::<Transform>(active_camera)
            .expect("Active camera entity does not have a Transform component");

        let projection: &CameraProjection = world.resource::<CameraProjection>();

        GPUCamera {
            view: camera_transform.to_inverse_raw(),
            proj: projection.to_raw(),
        }
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

        render_resource.instance_manager.update_batches(
            device,
            &self.objects,
            &render_resource.mesh_manager.id_key_mapping,
            &render_resource.material_manager.id_key_mapping,
        );

        Vec::new()
    }

    fn paint(
        &self,
        _info: egui::PaintCallbackInfo,
        render_pass: &mut wgpu::RenderPass,
        callback_resources: &eframe::egui_wgpu::CallbackResources,
    ) {
        let render_resource: &RenderResource = callback_resources.get().unwrap();
        render_pass.set_pipeline(&render_resource.pipeline);
        render_pass.set_bind_group(0, &render_resource.camera_bind_group, &[]);
        // Light bind group

        for batch in &render_resource.instance_manager.instance_batches {
            let mesh: &GPUMesh = render_resource.mesh_manager.meshes.get(batch.mesh).unwrap();

            let material: &GPUMaterial = render_resource
                .material_manager
                .materials
                .get(batch.material)
                .unwrap();

            render_pass.set_bind_group(1, &material.bind_group, &[]);

            render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, batch.instance_buffer.slice(..));
            render_pass.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

            render_pass.draw_indexed(0..mesh.num_indices, 0, 0..batch.instance_count);
        }
    }
}
