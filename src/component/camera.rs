#[derive(Debug, Clone, bevy_ecs::resource::Resource, PartialEq)]
pub struct CameraProjection {
    pub vertical_fov: f32,
    pub aspect_ratio: f32,
    pub z_near: f32,
    pub z_far: f32,
}

#[derive(Debug, Clone, bevy_ecs::resource::Resource, PartialEq)]
pub struct ActiveCamera {
    pub camera: bevy_ecs::entity::Entity,
}

#[derive(Debug, Clone, bevy_ecs::component::Component, PartialEq)]
pub struct Camera;
