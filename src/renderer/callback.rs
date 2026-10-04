use crate::renderer::{
    mesh::{GPUVertex, MeshManager},
    resource::RenderResource,
    texture::TextureManager,
};

pub fn init_callback_resources(cc: &eframe::CreationContext) {
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
            texture_manager: TextureManager {
                device: device.clone(),
                queue: queue.clone(),
                textures: slotmap::SlotMap::with_key(),
            },
            mesh_manager: MeshManager {
                device: device.clone(),
                queue: queue.clone(),
                meshes: slotmap::SlotMap::with_key(),
            },
        });
}
