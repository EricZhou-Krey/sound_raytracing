slotmap::new_key_type! {
    pub struct MeshId;
    pub struct TextureId;
    pub struct MaterialId;
}

#[derive(Debug)]
pub enum AssetId {
    Mesh(MeshId),
    Texture(TextureId),
    Material(MaterialId),
}
