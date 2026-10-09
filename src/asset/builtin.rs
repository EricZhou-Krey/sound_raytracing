use bevy_ecs::{
    prelude::{Commands, ResMut},
    resource::Resource,
};

use crate::asset::{
    id::{AssetId, MaterialId, MeshId, TextureId},
    loader::GPUAssetUploader,
    manager::{MaterialManager, MeshManager, TextureManager},
    MaterialAsset, MeshAsset, TextureAsset, Vertex,
};

#[derive(Debug, Resource)]
pub struct BuiltinAssets {
    pub meshes: BuiltinMeshes,
    pub textures: BuiltinTextures,
    pub materials: BuiltinMaterials,
}

#[derive(Debug)]
pub struct BuiltinMeshes {
    pub square: MeshId,
    pub cube: MeshId,
    pub sphere: MeshId,
    pub cylinder: MeshId,
}

#[derive(Debug)]
pub struct BuiltinTextures {
    pub white: TextureId,
    pub black: TextureId,
    pub flat_normal: TextureId,
}

#[derive(Debug)]
pub struct BuiltinMaterials {
    pub white_matte: MaterialId,
    pub black_matte: MaterialId,
}

pub fn install_builtin_assets_system(
    mut commands: Commands,
    mut mesh_manager: ResMut<MeshManager>,
    mut texture_manager: ResMut<TextureManager>,
    mut material_manager: ResMut<MaterialManager>,
    mut uploader: ResMut<GPUAssetUploader>,
) {
    let mut upload_ids: Vec<AssetId> = Vec::new();

    let white: TextureId = texture_manager
        .textures
        .insert(solid_texture([255, 255, 255, 255]));
    upload_ids.push(AssetId::Texture(white));

    let black: TextureId = texture_manager
        .textures
        .insert(solid_texture([0, 0, 0, 255]));
    upload_ids.push(AssetId::Texture(black));

    let flat_normal: TextureId = texture_manager
        .textures
        .insert(solid_texture([128, 128, 255, 255]));
    upload_ids.push(AssetId::Texture(flat_normal));

    let square: MeshId = mesh_manager.meshes.insert(square_mesh());
    upload_ids.push(AssetId::Mesh(square));

    let cube: MeshId = mesh_manager.meshes.insert(cube_mesh());
    upload_ids.push(AssetId::Mesh(cube));

    let sphere: MeshId = mesh_manager.meshes.insert(uv_sphere_mesh(16, 24));
    upload_ids.push(AssetId::Mesh(sphere));

    let cylinder: MeshId = mesh_manager.meshes.insert(cylinder_mesh(24));
    upload_ids.push(AssetId::Mesh(cylinder));

    let white_matte: MaterialId = material_manager.materials.insert(MaterialAsset {
        base_color: [1.0, 1.0, 1.0, 1.0],
        base_color_texture: white,
        normal_texture: flat_normal,
    });
    upload_ids.push(AssetId::Material(white_matte));

    let black_matte: MaterialId = material_manager.materials.insert(MaterialAsset {
        base_color: [1.0, 1.0, 1.0, 1.0],
        base_color_texture: black,
        normal_texture: flat_normal,
    });
    upload_ids.push(AssetId::Material(black_matte));

    uploader.asset_ids.append(&mut upload_ids);

    commands.insert_resource(BuiltinAssets {
        meshes: BuiltinMeshes {
            square,
            cube,
            sphere,
            cylinder,
        },
        textures: BuiltinTextures {
            white,
            black,
            flat_normal,
        },
        materials: BuiltinMaterials {
            white_matte,
            black_matte,
        },
    });
}

fn solid_texture(rgba: [u8; 4]) -> TextureAsset {
    TextureAsset {
        width: 1,
        height: 1,
        rgba8: rgba.to_vec(),
    }
}

fn square_mesh() -> MeshAsset {
    let normal = [0.0, 0.0, 1.0];

    MeshAsset {
        vertices: vec![
            Vertex {
                position: [-0.5, -0.5, 0.0],
                normal,
                uv: [0.0, 1.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.0],
                normal,
                uv: [1.0, 1.0],
            },
            Vertex {
                position: [0.5, 0.5, 0.0],
                normal,
                uv: [1.0, 0.0],
            },
            Vertex {
                position: [-0.5, 0.5, 0.0],
                normal,
                uv: [0.0, 0.0],
            },
        ],
        indices: vec![0, 1, 2, 0, 2, 3],
    }
}

fn cube_mesh() -> MeshAsset {
    let mut mesh = MeshAsset::default();

    add_cube_face(
        &mut mesh,
        [
            [-0.5, -0.5, -0.5],
            [-0.5, 0.5, -0.5],
            [0.5, 0.5, -0.5],
            [0.5, -0.5, -0.5],
        ],
        [0.0, 0.0, -1.0],
    );

    add_cube_face(
        &mut mesh,
        [
            [-0.5, -0.5, 0.5],
            [0.5, -0.5, 0.5],
            [0.5, 0.5, 0.5],
            [-0.5, 0.5, 0.5],
        ],
        [0.0, 0.0, 1.0],
    );

    add_cube_face(
        &mut mesh,
        [
            [-0.5, -0.5, -0.5],
            [-0.5, -0.5, 0.5],
            [-0.5, 0.5, 0.5],
            [-0.5, 0.5, -0.5],
        ],
        [-1.0, 0.0, 0.0],
    );

    add_cube_face(
        &mut mesh,
        [
            [0.5, -0.5, -0.5],
            [0.5, 0.5, -0.5],
            [0.5, 0.5, 0.5],
            [0.5, -0.5, 0.5],
        ],
        [1.0, 0.0, 0.0],
    );

    add_cube_face(
        &mut mesh,
        [
            [-0.5, -0.5, -0.5],
            [0.5, -0.5, -0.5],
            [0.5, -0.5, 0.5],
            [-0.5, -0.5, 0.5],
        ],
        [0.0, -1.0, 0.0],
    );

    add_cube_face(
        &mut mesh,
        [
            [-0.5, 0.5, -0.5],
            [-0.5, 0.5, 0.5],
            [0.5, 0.5, 0.5],
            [0.5, 0.5, -0.5],
        ],
        [0.0, 1.0, 0.0],
    );

    mesh
}

fn add_cube_face(mesh: &mut MeshAsset, positions: [[f32; 3]; 4], normal: [f32; 3]) {
    let base = mesh.vertices.len() as u32;

    let uvs = [[0.0, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0]];

    for (position, uv) in positions.into_iter().zip(uvs) {
        mesh.vertices.push(Vertex {
            position,
            normal,
            uv,
        });
    }

    mesh.indices
        .extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
}

fn uv_sphere_mesh(stacks: u32, sectors: u32) -> MeshAsset {
    assert!(stacks >= 2);
    assert!(sectors >= 3);

    let mut vertices = Vec::with_capacity(((stacks + 1) * (sectors + 1)) as usize);
    let mut indices = Vec::with_capacity((stacks * sectors * 6) as usize);

    for stack in 0..=stacks {
        let v = stack as f32 / stacks as f32;
        let phi = std::f32::consts::PI * v;
        let ring_radius = phi.sin();
        let y = phi.cos();

        for sector in 0..=sectors {
            let u = sector as f32 / sectors as f32;
            let theta = std::f32::consts::TAU * u;

            let normal = [ring_radius * theta.cos(), y, ring_radius * theta.sin()];

            vertices.push(Vertex {
                position: [0.5 * normal[0], 0.5 * normal[1], 0.5 * normal[2]],
                normal,
                uv: [u, v],
            });
        }
    }

    let row_len = sectors + 1;

    for stack in 0..stacks {
        for sector in 0..sectors {
            let a = stack * row_len + sector;
            let a_next = a + 1;
            let b = a + row_len;
            let b_next = b + 1;

            if stack != 0 {
                indices.extend_from_slice(&[a, a_next, b]);
            }

            if stack + 1 != stacks {
                indices.extend_from_slice(&[a_next, b_next, b]);
            }
        }
    }

    MeshAsset { vertices, indices }
}

fn cylinder_mesh(sectors: u32) -> MeshAsset {
    assert!(sectors >= 3);

    let mut mesh = MeshAsset::default();
    let half_height = 0.5;
    let radius = 0.5;

    for sector in 0..=sectors {
        let u = sector as f32 / sectors as f32;
        let theta = std::f32::consts::TAU * u;
        let (sin_theta, cos_theta) = theta.sin_cos();
        let normal = [cos_theta, 0.0, sin_theta];

        mesh.vertices.push(Vertex {
            position: [radius * cos_theta, -half_height, radius * sin_theta],
            normal,
            uv: [u, 1.0],
        });
        mesh.vertices.push(Vertex {
            position: [radius * cos_theta, half_height, radius * sin_theta],
            normal,
            uv: [u, 0.0],
        });
    }

    for sector in 0..sectors {
        let bottom = sector * 2;
        let top = bottom + 1;
        let next_bottom = bottom + 2;
        let next_top = bottom + 3;

        mesh.indices
            .extend_from_slice(&[bottom, next_top, next_bottom, bottom, top, next_top]);
    }

    for is_top in [false, true] {
        let y = if is_top { half_height } else { -half_height };
        let normal = if is_top {
            [0.0, 1.0, 0.0]
        } else {
            [0.0, -1.0, 0.0]
        };
        let center = mesh.vertices.len() as u32;

        mesh.vertices.push(Vertex {
            position: [0.0, y, 0.0],
            normal,
            uv: [0.5, 0.5],
        });

        let ring_start = mesh.vertices.len() as u32;
        for sector in 0..=sectors {
            let theta = std::f32::consts::TAU * sector as f32 / sectors as f32;
            let (sin_theta, cos_theta) = theta.sin_cos();
            let x = radius * cos_theta;
            let z = radius * sin_theta;

            mesh.vertices.push(Vertex {
                position: [x, y, z],
                normal,
                uv: [0.5 + 0.5 * cos_theta, 0.5 + 0.5 * sin_theta],
            });
        }

        for sector in 0..sectors {
            let current = ring_start + sector;
            let next = current + 1;

            if is_top {
                mesh.indices.extend_from_slice(&[center, next, current]);
            } else {
                mesh.indices.extend_from_slice(&[center, current, next]);
            }
        }
    }

    mesh
}
