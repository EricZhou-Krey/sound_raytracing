use crate::renderer::extraction::GPUTransform;

#[derive(Default, bevy_ecs::component::Component, Debug, Clone, PartialEq)]
pub struct Transform {
    scale: glam::Vec3,
    translation: glam::Vec3,
    rotation: glam::Quat,
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
