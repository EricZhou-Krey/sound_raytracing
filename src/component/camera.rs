#[derive(Debug, Clone, bevy_ecs::resource::Resource, PartialEq)]
pub struct CameraProjection {
    pub vertical_fov: f32,
    pub aspect_ratio: f32,
    pub z_near: f32,
    pub z_far: f32,
}

impl CameraProjection {
    pub fn to_raw(&self) -> [[f32; 4]; 4] {
        glam::camera::rh::proj::vulkan::perspective(
            self.vertical_fov,
            self.aspect_ratio,
            self.z_near,
            self.z_far,
        )
        .to_cols_array_2d()
    }
}

#[derive(Debug, Clone, bevy_ecs::resource::Resource, PartialEq)]
pub struct ActiveCamera {
    pub camera: bevy_ecs::entity::Entity,
}

#[derive(Debug, Clone, bevy_ecs::component::Component, PartialEq)]
pub struct Camera;
