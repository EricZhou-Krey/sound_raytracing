use crate::{
    asset::{
        id::{AssetId, MaterialId, MeshId, TextureId},
        loader::GPUAssetUploader,
        manager::{MaterialManager, MeshManager, TextureManager},
        MaterialAsset, MeshAsset, TextureAsset,
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

#[derive(Default, Debug, PartialEq)]
pub struct RenderCallback<'a> {
    pub camera: GPUCamera,
    // pub lights: Vec<GPULight>,
    pub objects: Vec<GPUObject>,
    pub uploaded_meshes: Vec<(MeshId, &'a MeshAsset)>,
    pub uploaded_textures: Vec<(TextureId, &'a TextureAsset)>,
    pub uploaded_materials: Vec<(MaterialId, &'a MaterialAsset)>,
}

impl<'a> RenderCallback<'a> {
    pub fn extract_from_world(world: &'a mut bevy_ecs::world::World) -> Self {
        let camera: GPUCamera = Self::extract_camera(world);
        // let lights: Vec<GPULight> = Self::extract_lights(world);
        let objects: Vec<GPUObject> = Self::extract_objects(world);
        let (uploaded_meshes, uploaded_textures, uploaded_materials): (
            Vec<(MeshId, &'a MeshAsset)>,
            Vec<(TextureId, &'a TextureAsset)>,
            Vec<(MaterialId, &'a MaterialAsset)>,
        ) = Self::extract_uploaded_assets(world);

        Self {
            camera,
            // lights,
            objects,
            uploaded_meshes,
            uploaded_textures,
            uploaded_materials,
        }
    }

    fn extract_uploaded_assets(
        world: &'a mut bevy_ecs::world::World,
    ) -> (
        Vec<(MeshId, &'a MeshAsset)>,
        Vec<(TextureId, &'a TextureAsset)>,
        Vec<(MaterialId, &'a MaterialAsset)>,
    ) {
        let asset_ids: Vec<AssetId> =
            std::mem::take(&mut world.resource_mut::<GPUAssetUploader>().asset_ids);

        let meshes: &MeshManager = world.resource::<MeshManager>();
        let textures: &TextureManager = world.resource::<TextureManager>();
        let materials: &MaterialManager = world.resource::<MaterialManager>();

        let mut uploaded_meshes: Vec<(MeshId, &'a MeshAsset)> = Vec::new();
        let mut uploaded_textures: Vec<(TextureId, &'a TextureAsset)> = Vec::new();
        let mut uploaded_materials: Vec<(MaterialId, &'a MaterialAsset)> = Vec::new();

        for asset_id in asset_ids {
            match asset_id {
                AssetId::Mesh(id) => {
                    let asset = meshes.meshes.get(id).expect("missing mesh asset");
                    uploaded_meshes.push((id, asset));
                }
                AssetId::Texture(id) => {
                    let asset = textures.textures.get(id).expect("missing texture asset");
                    uploaded_textures.push((id, asset));
                }
                AssetId::Material(id) => {
                    let asset = materials.materials.get(id).expect("missing material asset");
                    uploaded_materials.push((id, asset));
                }
            }
        }

        (uploaded_meshes, uploaded_textures, uploaded_materials)
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

    fn extract_lights(_world: &bevy_ecs::world::World) -> Vec<GPULight> {
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

impl eframe::egui_wgpu::CallbackTrait for RenderCallback<'_> {
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
