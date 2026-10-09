use crate::{
    asset::{
        loader::RequestLoadScene,
        source::{MaterialSource, MeshSource, ObjectSource, SceneSource, TextureSource},
        MeshAsset, TextureAsset, Vertex,
    },
    component::{
        camera::{ActiveCamera, Camera, CameraProjection},
        object::Transform,
    },
};
use bevy_ecs::{entity::Entity, world::World};

pub trait DemoWorldExtension {
    fn setup_demo(&mut self);
}

impl DemoWorldExtension for World {
    fn setup_demo(&mut self) {
        let camera: Entity = self
            .spawn((
                Camera,
                Transform {
                    translation: glam::Vec3::new(0.0, 0.0, 5.0),
                    rotation: glam::Quat::IDENTITY,
                    scale: glam::Vec3::ONE,
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

        self.trigger(RequestLoadScene {
            scene: demo_scene(),
        });
    }
}

fn demo_scene() -> SceneSource {
    let mesh = MeshAsset {
        vertices: vec![
            Vertex {
                position: [-0.5, -0.5, 0.0],
                normal: [0.0, 0.0, -1.0],
                uv: [0.0, 1.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.0],
                normal: [0.0, 0.0, -1.0],
                uv: [1.0, 1.0],
            },
            Vertex {
                position: [0.5, 0.5, 0.0],
                normal: [0.0, 0.0, -1.0],
                uv: [1.0, 0.0],
            },
            Vertex {
                position: [-0.5, 0.5, 0.0],
                normal: [0.0, 0.0, -1.0],
                uv: [0.0, 0.0],
            },
        ],
        indices: vec![0, 2, 1, 0, 3, 2],
    };

    let base_color_texture = TextureAsset {
        width: 2,
        height: 2,
        rgba8: vec![
            230, 90, 30, 255, 245, 210, 120, 255, 245, 210, 120, 255, 230, 90, 30, 255,
        ],
    };

    let normal_texture = TextureAsset {
        width: 1,
        height: 1,
        rgba8: vec![128, 128, 255, 255],
    };

    SceneSource {
        meshes: vec![MeshSource::Description(mesh)],
        textures: vec![
            TextureSource::Description(base_color_texture),
            TextureSource::Description(normal_texture),
        ],
        materials: vec![MaterialSource::Description {
            base_color: [1.0, 1.0, 1.0, 1.0],
            base_color_texture: 0,
            normal_texture: 1,
        }],
        objects: vec![
            ObjectSource {
                transform: Transform {
                    translation: glam::Vec3::new(-1.2, 0.0, 0.0),
                    rotation: glam::Quat::IDENTITY,
                    scale: glam::Vec3::ONE,
                },
                mesh: 0,
                material: 0,
            },
            ObjectSource {
                transform: Transform {
                    translation: glam::Vec3::ZERO,
                    rotation: glam::Quat::IDENTITY,
                    scale: glam::Vec3::splat(1.4),
                },
                mesh: 0,
                material: 0,
            },
            ObjectSource {
                transform: Transform {
                    translation: glam::Vec3::new(1.2, 0.0, 0.0),
                    rotation: glam::Quat::IDENTITY,
                    scale: glam::Vec3::ONE,
                },
                mesh: 0,
                material: 0,
            },
        ],
    }
}
