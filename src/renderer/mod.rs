pub mod callback;
pub mod extraction;
pub mod mesh;
pub mod resource;
pub mod texture;

pub struct Renderer;
impl Renderer {
    pub fn show(ui: &mut egui::Ui, extracted_vertices: Vec<Vertex>) {
        let (rect, _response) = ui.allocate_exact_size(ui.available_size(), egui::Sense::drag());

        ui.painter().add(egui_wgpu::Callback::new_paint_callback(
            rect,
            RoomRenderCallback { extracted_vertices },
        ));
    }
}
