use crate::{
    asset::{
        id::{MaterialId, MeshId},
        Vertex,
    },
    component::{Light, Material, Mesh, Transform},
    renderer::{
        instance::InstanceManager, material::MaterialManager, mesh::MeshManager,
        texture::TextureManager,
    },
};

pub const MAX_N_LIGHTS: usize = 256;

#[repr(C)]
#[derive(Default, Debug, PartialEq, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GPUCamera {
    pub view: [[f32; 4]; 4],
    pub proj: [[f32; 4]; 4],
}

#[repr(C)]
#[derive(Default, Debug, PartialEq, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GPULight {
    pub position_range: [f32; 4],
    pub color_intensity: [f32; 4],
}

#[repr(C)]
#[derive(Debug, PartialEq, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GPULightUniform {
    pub lights: [GPULight; MAX_N_LIGHTS],
    pub count: u32,
    pub _pad: [u32; 3],
}

#[repr(C)]
#[derive(Default, Debug, PartialEq, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GPUTransform {
    pub model: [[f32; 4]; 4],
}

impl GPUTransform {
    const ATTRIBUTES: [wgpu::VertexAttribute; 4] = wgpu::vertex_attr_array![
        3 => Float32x4,
        4 => Float32x4,
        5 => Float32x4,
        6 => Float32x4,
    ];

    pub fn layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRIBUTES,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GPUDrawable {
    pub mesh: MeshId,
    pub material: MaterialId,
    pub transform: GPUTransform,
}

#[derive(Debug)]
pub struct RenderResource {
    pub pipeline: wgpu::RenderPipeline,
    pub texture_manager: TextureManager,
    pub material_manager: MaterialManager,
    pub mesh_manager: MeshManager,
    pub instance_manager: InstanceManager,

    pub camera_buffer: wgpu::Buffer,
    pub camera_bind_group: wgpu::BindGroup,

    pub light_buffer: wgpu::Buffer,
    pub light_bind_group: wgpu::BindGroup,
}

#[derive(Debug, bevy_ecs::resource::Resource)]
pub struct RenderCallbackDrawableQueryState {
    pub drawable_query:
        bevy_ecs::query::QueryState<(&'static Transform, &'static Mesh, &'static Material)>,
    pub light_query: bevy_ecs::query::QueryState<(&'static Transform, &'static Light)>,
}

impl RenderCallbackDrawableQueryState {
    pub fn new(world: &mut bevy_ecs::world::World) -> Self {
        Self {
            drawable_query: world.query::<(&Transform, &Mesh, &Material)>(),
            light_query: world.query::<(&Transform, &Light)>(),
        }
    }
}

pub fn init_callback_resources(cc: &eframe::CreationContext, world: &mut bevy_ecs::world::World) {
    let drawable_query: RenderCallbackDrawableQueryState =
        RenderCallbackDrawableQueryState::new(world);
    world.insert_resource(drawable_query);

    let wgpu_state: &eframe::egui_wgpu::RenderState =
        cc.wgpu_render_state.as_ref().expect("wgpu not enabled");
    let device: &wgpu::Device = &wgpu_state.device;

    let shader: wgpu::ShaderModule = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("shader/shader.wgsl").into()),
    });

    let camera_buffer: wgpu::Buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Camera Buffer"),
        size: std::mem::size_of::<GPUCamera>() as u64,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let camera_bind_group_layout: wgpu::BindGroupLayout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Camera Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

    let camera_bind_group: wgpu::BindGroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Camera Bind Group"),
        layout: &camera_bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: camera_buffer.as_entire_binding(),
        }],
    });

    let material_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Material Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

    let sampler: wgpu::Sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Nearest,
        mipmap_filter: wgpu::MipmapFilterMode::Nearest,
        ..Default::default()
    });

    let light_buffer: wgpu::Buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Light Buffer"),
        size: std::mem::size_of::<GPULightUniform>() as u64,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let light_bind_group_layout: wgpu::BindGroupLayout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Light Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

    let light_bind_group: wgpu::BindGroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Light Bind Group"),
        layout: &light_bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: light_buffer.as_entire_binding(),
        }],
    });

    let pipeline_layout: wgpu::PipelineLayout =
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[
                Some(&camera_bind_group_layout),
                Some(&material_bind_group_layout),
                Some(&light_bind_group_layout),
            ],
            immediate_size: 0,
        });

    let pipeline: wgpu::RenderPipeline =
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Some(Vertex::layout()), Some(GPUTransform::layout())],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu_state.target_format.into())],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: Some(true),
                depth_compare: Some(wgpu::CompareFunction::Less),
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });

    wgpu_state
        .renderer
        .write()
        .callback_resources
        .insert(RenderResource {
            pipeline,
            texture_manager: TextureManager::new(sampler),
            material_manager: MaterialManager::new(material_bind_group_layout),
            mesh_manager: MeshManager::new(),
            instance_manager: InstanceManager::new(),

            camera_buffer,
            camera_bind_group,

            light_buffer,
            light_bind_group,
        });
}
