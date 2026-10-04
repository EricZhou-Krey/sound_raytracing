use crate::renderer::{mesh::GPUMeshKey, resource::GPUTransform, texture::GPUTextureKey};

#[derive(Debug, Clone, bevy_ecs::component::Component, PartialEq)]
pub struct Mesh {
    pub key: GPUMeshKey,
}

#[derive(Debug, Clone, bevy_ecs::component::Component, PartialEq)]
pub struct Texture {
    pub key: GPUTextureKey,
}

#[derive(Default, bevy_ecs::component::Component, Debug, Clone, PartialEq)]
pub struct Transform {
    pub scale: glam::Vec3,
    pub translation: glam::Vec3,
    pub rotation: glam::Quat,
}

impl Transform {
    pub fn to_gpu_transform(&self) -> GPUTransform {
        GPUTransform {
            model: glam::Mat4::from_scale_rotation_translation(
                self.scale,
                self.rotation,
                self.translation,
            )
            .to_cols_array_2d(),
        }
    }
}
