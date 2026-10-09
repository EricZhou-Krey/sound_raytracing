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
        let camera_position: glam::Vec3 = glam::Vec3::new(2.0, -2.0, 5.0);
        let direction: glam::Vec3 = (glam::Vec3::ZERO - camera_position).normalize();
        let camera: Entity = self
            .spawn((
                Camera,
                Transform {
                    translation: camera_position,
                    rotation: glam::Quat::from_rotation_arc(glam::Vec3::NEG_Z, direction),
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

fn cube_mesh() -> MeshAsset {
    let mut vertices = Vec::with_capacity(24);
    let mut indices = Vec::with_capacity(36);

    fn add_face(
        vertices: &mut Vec<Vertex>,
        indices: &mut Vec<u32>,
        corners: [[f32; 3]; 4],
        normal: [f32; 3],
    ) {
        let base = vertices.len() as u32;
        let uvs = [[0.0, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0]];

        for (position, uv) in corners.into_iter().zip(uvs) {
            vertices.push(Vertex {
                position,
                normal,
                uv,
            });
        }

        indices.extend([base, base + 1, base + 2, base, base + 2, base + 3]);
    }

    add_face(
        &mut vertices,
        &mut indices,
        [
            [-0.5, -0.5, 0.5],
            [0.5, -0.5, 0.5],
            [0.5, 0.5, 0.5],
            [-0.5, 0.5, 0.5],
        ],
        [0.0, 0.0, 1.0],
    );

    add_face(
        &mut vertices,
        &mut indices,
        [
            [0.5, -0.5, -0.5],
            [-0.5, -0.5, -0.5],
            [-0.5, 0.5, -0.5],
            [0.5, 0.5, -0.5],
        ],
        [0.0, 0.0, -1.0],
    );

    add_face(
        &mut vertices,
        &mut indices,
        [
            [0.5, -0.5, 0.5],
            [0.5, -0.5, -0.5],
            [0.5, 0.5, -0.5],
            [0.5, 0.5, 0.5],
        ],
        [1.0, 0.0, 0.0],
    );

    add_face(
        &mut vertices,
        &mut indices,
        [
            [-0.5, -0.5, -0.5],
            [-0.5, -0.5, 0.5],
            [-0.5, 0.5, 0.5],
            [-0.5, 0.5, -0.5],
        ],
        [-1.0, 0.0, 0.0],
    );

    add_face(
        &mut vertices,
        &mut indices,
        [
            [-0.5, 0.5, 0.5],
            [0.5, 0.5, 0.5],
            [0.5, 0.5, -0.5],
            [-0.5, 0.5, -0.5],
        ],
        [0.0, 1.0, 0.0],
    );

    add_face(
        &mut vertices,
        &mut indices,
        [
            [-0.5, -0.5, -0.5],
            [0.5, -0.5, -0.5],
            [0.5, -0.5, 0.5],
            [-0.5, -0.5, 0.5],
        ],
        [0.0, -1.0, 0.0],
    );

    MeshAsset { vertices, indices }
}

fn demo_scene() -> SceneSource {
    let mesh: MeshAsset = cube_mesh();

    let base_color_texture: TextureAsset = TextureAsset {
        width: 2,
        height: 2,
        rgba8: vec![
            230, 90, 30, 255, 245, 210, 120, 255, 245, 210, 120, 255, 230, 90, 30, 255,
        ],
    };

    let normal_texture: TextureAsset = TextureAsset {
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
        objects: vec![ObjectSource {
            transform: Transform {
                translation: glam::Vec3::ZERO,
                rotation: glam::Quat::IDENTITY,
                scale: glam::Vec3::splat(1.5),
            },
            mesh: 0,
            material: 0,
        }],
    }
}
