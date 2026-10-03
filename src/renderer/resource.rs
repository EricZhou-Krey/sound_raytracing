pub struct RenderResource {
    pub pipeline: wgpu::RenderPipeline,
    pub texture_manager: 
    pub mesh_manager: ...
}

impl RenderResource {
    fn prepare(&self, _device: &wgpu::Device, queue: &wgpu::Queue, angle: f32) {
        queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[angle, 0.0, 0.0, 0.0]),
        );
    }

    fn paint(&self, render_pass: &mut wgpu::RenderPass<'_>) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.draw(0..3, 0..1);
    }
}
