use bevy_ecs::{component::Component, resource::Resource};
use glam::Vec3;
use kira::sound::static_sound::StaticSoundData;

#[derive(Component)]
pub struct Point {
    pub position: Vec3,
}
#[derive(Component)]
pub enum Shape {
    // Circle { center: Vec3, radius: f32 },
    AxisAlignedBoundingBox { min: Vec3, max: Vec3 },
    // Plane { point: Vec3, normal: Vec3 },
}
#[derive(Component)]
pub struct FacingDirection {
    pub direction: Vec3,
}
#[derive(Component)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}
#[derive(Component)]
pub struct CollisionPath {
    pub path: Vec3,
}
#[derive(Component)]
pub struct Lifetime {
    pub time: f32,
}
#[derive(Resource)]
pub struct SpeedOfSoundRay {
    pub speed: f32,
}
#[derive(Component)]
pub struct SoundData {
    pub data: StaticSoundData,
}
#[derive(Component)]
pub struct Amplitude {
    pub amplitude: f32,
}
#[derive(Component)]
pub struct AudioOrigin {
    pub origin: Vec3,
}
