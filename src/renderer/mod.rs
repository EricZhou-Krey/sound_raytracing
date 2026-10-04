use crate::renderer::callback::RenderCallback;

pub mod callback;
pub mod mesh;
pub mod resource;
pub mod texture;

pub struct Renderer;
impl Renderer {
    pub fn ui(ui: &mut egui::Ui, world: &mut bevy_ecs::world::World) {
        let (rect, _response) = ui.allocate_exact_size(ui.available_size(), egui::Sense::drag());

        ui.painter()
            .add(eframe::egui_wgpu::Callback::new_paint_callback(
                rect,
                RenderCallback::extract_from_world(world),
            ));
    }
}
