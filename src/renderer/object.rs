use crate::renderer::{mesh::MeshHandle, texture::TextureHandle};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ObjectHandle {
    mesh: MeshHandle,
    texture: TextureHandle,
}

#[derive(Debug, PartialEq)]
struct Object {
    mesh: MeshHandle,
    texture: TextureHandle,
    transform: [[f32; 4]; 4],
}

#[derive(Debug, PartialEq)]
struct ObjectSlot {
    generation: u32,
    object: Option<Object>,
}
#[derive(Debug, PartialEq)]
struct ObjectManager {
    slots: Vec<ObjectSlot>,
}
