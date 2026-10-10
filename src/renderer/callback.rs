use crate::{
    asset::{
        id::{AssetId, MaterialId, MeshId, TextureId},
        loader::GPUAssetUploader,
        manager::{MaterialManager, MeshManager, TextureManager},
        MaterialAsset, MeshAsset, TextureAsset,
    },
    component::{
        camera::{ActiveCamera, CameraProjection},
        Material, Mesh, PointLight, Transform,
    },
    renderer::{
        material::GPUMaterial,
        mesh::GPUMesh,
        resource::{
            GPUCamera, GPUDrawable, GPULight, GPUTransform, RenderCallbackDrawableQueryState,
            RenderResource,
        },
    },
};

#[derive(Default, Debug, PartialEq)]
pub struct RenderCallback {
    pub camera: GPUCamera,
    pub lights: Vec<GPULight>,
    pub drawables: Vec<GPUDrawable>,
    pub uploaded_meshes: Vec<(MeshId, MeshAsset)>,
    pub uploaded_textures: Vec<(TextureId, TextureAsset)>,
    pub uploaded_materials: Vec<(MaterialId, MaterialAsset)>,
}

type AssetCollection = (
    Vec<(MeshId, MeshAsset)>,
    Vec<(TextureId, TextureAsset)>,
    Vec<(MaterialId, MaterialAsset)>,
);

impl RenderCallback {
    pub fn extract_from_world(world: &mut bevy_ecs::world::World) -> Self {
        let camera: GPUCamera = Self::extract_camera(world);

        let (drawables, lights): (Vec<GPUDrawable>, Vec<GPULight>) =
            world.resource_scope(
                |world,
                 mut query_state: bevy_ecs::change_detection::Mut<
                    RenderCallbackDrawableQueryState,
                >| {
                    (
                        Self::extract_drawables(world, &mut query_state.drawable_query),
                        Self::extract_lights(world, &mut query_state.light_query),
                    )
                },
            );

        let (uploaded_meshes, uploaded_textures, uploaded_materials): AssetCollection =
            Self::extract_uploaded_assets(world);

        Self {
            camera,
            lights,
            drawables,
            uploaded_meshes,
            uploaded_textures,
            uploaded_materials,
        }
    }

    fn extract_uploaded_assets(world: &mut bevy_ecs::world::World) -> AssetCollection {
        let asset_ids: Vec<AssetId> =
            std::mem::take(&mut world.resource_mut::<GPUAssetUploader>().asset_ids);

        let meshes: &MeshManager = world.resource::<MeshManager>();
        let textures: &TextureManager = world.resource::<TextureManager>();
        let materials: &MaterialManager = world.resource::<MaterialManager>();

        let mut uploaded_meshes: Vec<(MeshId, MeshAsset)> = Vec::new();
        let mut uploaded_textures: Vec<(TextureId, TextureAsset)> = Vec::new();
        let mut uploaded_materials: Vec<(MaterialId, MaterialAsset)> = Vec::new();

        for asset_id in asset_ids {
            match asset_id {
                AssetId::Mesh(id) => {
                    let asset: &MeshAsset = meshes.meshes.get(id).expect("missing mesh asset");
                    uploaded_meshes.push((id, asset.clone()));
                }
                AssetId::Texture(id) => {
                    let asset: &TextureAsset =
                        textures.textures.get(id).expect("missing texture asset");
                    uploaded_textures.push((id, asset.clone()));
                }
                AssetId::Material(id) => {
                    let asset: &MaterialAsset =
                        materials.materials.get(id).expect("missing material asset");
                    uploaded_materials.push((id, asset.clone()));
                }
            }
        }

        (uploaded_meshes, uploaded_textures, uploaded_materials)
    }

    fn extract_drawables(
        world: &mut bevy_ecs::world::World,
        drawable_query: &mut bevy_ecs::query::QueryState<(
            &'static Transform,
            &'static Mesh,
            &'static Material,
        )>,
    ) -> Vec<GPUDrawable> {
        drawable_query
            .iter(world)
            .map(|(transform, mesh, material)| GPUDrawable {
                transform: GPUTransform {
                    model: transform.to_raw(),
                },
                mesh: mesh.id,
                material: material.id,
            })
            .collect()
    }

    fn extract_lights(
        world: &bevy_ecs::world::World,
        light_query: &mut bevy_ecs::query::QueryState<(&'static Transform, &'static PointLight)>,
    ) -> Vec<GPULight> {
        light_query
            .iter(world)
            .map(|(transform, point_light)| GPULight {
                position_range: [
                    transform.translation.x,
                    transform.translation.y,
                    transform.translation.z,
                    point_light.range,
                ],
                color_intensity: [
                    point_light.color[0],
                    point_light.color[1],
                    point_light.color[2],
                    point_light.intensity,
                ],
            })
            .collect()
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

        for (mesh_id, mesh_asset) in &self.uploaded_meshes {
            render_resource
                .mesh_manager
                .init_mesh(device, *mesh_id, mesh_asset);
        }
        for (texture_id, texture_asset) in &self.uploaded_textures {
            render_resource
                .texture_manager
                .init_texture(device, queue, *texture_id, texture_asset);
        }
        for (material_id, material_asset) in &self.uploaded_materials {
            render_resource.material_manager.init_material(
                device,
                *material_id,
                material_asset,
                &render_resource.texture_manager,
            );
        }

        queue.write_buffer(
            &render_resource.camera_buffer,
            0,
            bytemuck::bytes_of(&self.camera),
        );

        render_resource
            .instance_manager
            .update_batches(device, &self.drawables);

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
            render_pass.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);

            render_pass.draw_indexed(0..mesh.num_indices, 0, 0..batch.instance_count);
        }
    }
}
