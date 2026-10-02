pub mod callback;
pub mod extraction;
pub mod pipeline;
pub mod resource;

pub struct Renderer;
impl Renderer {
    pub fn write_callback(cc: &eframe::CreationContext) {
        let wgpu_state: &eframe::egui_wgpu::RenderState =
            cc.wgpu_render_state.as_ref().expect("wgpu not enabled");
        let device: &wgpu::Device = &wgpu_state.device;

        let vertex_buffer: wgpu::Buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Room Geometry Buffer"),
            size: 1024 * 1024,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let shader: wgpu::ShaderModule =
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Room Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
            });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            immediate_size: 0,
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Room Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Some(Vertex::desc())],
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
            .insert(RoomRenderResources {
                pipeline,
                vertex_buffer,
            });
    }

    pub fn show(ui: &mut egui::Ui, extracted_vertices: Vec<Vertex>) {
        let (rect, _response) = ui.allocate_exact_size(ui.available_size(), egui::Sense::drag());

        ui.painter().add(egui_wgpu::Callback::new_paint_callback(
            rect,
            RoomRenderCallback { extracted_vertices },
        ));
    }
}
