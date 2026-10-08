use crate::asset::manager::Vertex;

#[derive(Debug, Clone)]
pub enum MeshSource {
    Path(String),
    Description {
        vertices: Vec<Vertex>,
        indicies: Vec<usize>,
    },
}
#[derive(Debug, Clone)]
pub enum TextureSource {
    Path(String),
}

#[derive(Debug, Clone)]
pub enum MaterialSource {
    Path(String),
    Description {
        base_color: [f32; 4],
        metallic: f32,
        roughness: f32,
    },
}
