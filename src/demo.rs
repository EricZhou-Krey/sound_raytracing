use crate::{
    asset::{
        builtin::BuiltinAssets,
        id::{AssetId, MaterialId, MeshId, TextureId},
        loader::GPUAssetUploader,
        manager::{MaterialManager, TextureManager},
        FrameDelta, MaterialAsset, TextureAsset,
    },
    component::{
        camera::{ActiveCamera, Camera, CameraProjection},
        Light, Material, Mesh, Transform,
    },
};
use bevy_ecs::{
    entity::Entity,
    query::With,
    schedule::Schedule,
    system::{Query, Res},
    world::World,
};
use glam::Vec3;

pub trait DemoWorldExtension {
    fn setup_demo(&mut self);
}

impl DemoWorldExtension for World {
    fn setup_demo(&mut self) {
        let camera_position: Vec3 = Vec3::new(5.0, -5.0, 5.0);
        let direction: Vec3 = (Vec3::ZERO - camera_position).normalize();

        let camera: Entity = self
            .spawn((
                Camera,
                Transform {
                    translation: camera_position,
                    rotation: glam::Quat::from_rotation_arc(glam::Vec3::NEG_Z, direction),
                    scale: Vec3::ONE,
                },
            ))
            .id();

        self.insert_resource(ActiveCamera { camera });
        self.insert_resource(CameraProjection {
            vertical_fov: 60.0_f32.to_radians(),
            aspect_ratio: 16.0 / 9.0,
            z_near: 0.1,
            z_far: 1000.0,
        });

        self.spawn((
            Light {
                color: [100.0, 50.0, 50.0],
                intensity: 0.01,
                range: 1000.0,
            },
            Transform {
                translation: Vec3::new(5.0, 0.0, 0.0),
                rotation: glam::Quat::IDENTITY,
                scale: Vec3::ONE,
            },
        ));

        self.spawn((
            Light {
                color: [50.0, 100.0, 50.0],
                intensity: 0.01,
                range: 1000.0,
            },
            Transform {
                translation: Vec3::new(0.0, 5.0, 0.0),
                rotation: glam::Quat::IDENTITY,
                scale: Vec3::ONE,
            },
        ));

        self.spawn((
            Light {
                color: [50.0, 50.0, 100.0],
                intensity: 0.01,
                range: 1000.0,
            },
            Transform {
                translation: Vec3::new(0.0, 0.0, 5.0),
                rotation: glam::Quat::IDENTITY,
                scale: Vec3::ONE,
            },
        ));

        let cube_mesh_id: MeshId = self.resource::<BuiltinAssets>().meshes.sphere;
        let flat_normal_id: TextureId = self.resource::<BuiltinAssets>().textures.flat_normal;

        let base_color_texture_id: TextureId = self
            .resource_mut::<TextureManager>()
            .textures
            .insert(TextureAsset {
                width: 2,
                height: 2,
                rgba8: vec![
                    230, 90, 30, 255, 245, 210, 120, 255, 245, 210, 120, 255, 230, 90, 30, 255,
                ],
            });

        let material_id: MaterialId =
            self.resource_mut::<MaterialManager>()
                .materials
                .insert(MaterialAsset {
                    base_color: [1.0, 1.0, 1.0, 1.0],
                    base_color_texture: base_color_texture_id,
                    normal_texture: flat_normal_id,
                });

        self.resource_mut::<GPUAssetUploader>().asset_ids.extend([
            AssetId::Texture(base_color_texture_id),
            AssetId::Material(material_id),
        ]);

        self.spawn((
            Transform {
                translation: Vec3::ZERO,
                rotation: glam::Quat::IDENTITY,
                scale: Vec3::splat(1.5),
            },
            Mesh { id: cube_mesh_id },
            Material { id: material_id },
        ));
    }
}

pub trait DemoScheduleExtension {
    fn setup_demo(&mut self);
}

impl DemoScheduleExtension for Schedule {
    fn setup_demo(&mut self) {
        self.add_systems(rotate_objects);
    }
}

fn rotate_objects(
    mut transforms: Query<&mut Transform, (With<Material>, With<Mesh>)>,
    dt: Res<FrameDelta>,
) {
    for mut transform in transforms.iter_mut() {
        transform.rotation *= glam::Quat::from_axis_angle(Vec3::Y, dt.0);
    }
}
