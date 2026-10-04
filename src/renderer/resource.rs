use crate::{
    component::object::{Mesh, Texture, Transform},
    renderer::{
        mesh::{GPUVertex, MeshManager},
        texture::TextureManager,
    },
};

#[repr(C)]
#[derive(Default, Debug, PartialEq, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GPUCamera {
    pub view: [[f32; 4]; 4],
    pub proj: [[f32; 4]; 4],
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
        5 => Float32x4,
    ];

    pub fn layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRIBUTES,
        }
    }
}

#[derive(Debug)]
pub struct RenderResource {
    pub pipeline: wgpu::RenderPipeline,
    pub texture_manager: TextureManager,
    pub mesh_manager: MeshManager,
}

#[derive(Debug, bevy_ecs::resource::Resource)]
pub struct RenderCallbackObjectQueryState {
    pub query_state:
        bevy_ecs::query::QueryState<(&'static Transform, &'static Mesh, &'static Texture)>,
}

impl RenderCallbackObjectQueryState {
    pub fn new(world: &mut bevy_ecs::world::World) -> Self {
        Self {
            query_state: world.query::<(&Transform, &Mesh, &Texture)>(),
        }
    }
}

pub fn init_callback_resources(cc: &eframe::CreationContext, world: &mut bevy_ecs::world::World) {
    let object_query: RenderCallbackObjectQueryState = RenderCallbackObjectQueryState::new(world);
    world.insert_resource(object_query);

    let wgpu_state: &eframe::egui_wgpu::RenderState =
        cc.wgpu_render_state.as_ref().expect("wgpu not enabled");
    let device: &wgpu::Device = &wgpu_state.device;
    let queue: &wgpu::Queue = &wgpu_state.queue;

    let shader: wgpu::ShaderModule = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("shader/shader.wgsl").into()),
    });

    let pipeline_layout: wgpu::PipelineLayout =
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            immediate_size: 0,
        });

    let pipeline: wgpu::RenderPipeline =
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Room Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Some(GPUVertex::layout())],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu_state.target_format.into())],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
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
            texture_manager: TextureManager::new(device.clone(), queue.clone()),
            mesh_manager: MeshManager::new(device.clone(), queue.clone()),
        });
}
