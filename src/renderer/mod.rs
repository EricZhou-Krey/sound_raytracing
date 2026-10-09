use crate::renderer::callback::RenderCallback;

pub mod callback;
pub mod instance;
pub mod material;
pub mod mesh;
pub mod resource;
pub mod texture;

pub struct Renderer;
impl Renderer {
    pub fn ui(ui: &mut egui::Ui, world: &mut bevy_ecs::world::World) {
        ui.painter()
            .add(eframe::egui_wgpu::Callback::new_paint_callback(
                ui.viewport_rect(),
                RenderCallback::extract_from_world(world),
            ));
    }
}
