use eframe::NativeOptions;
use sound_raytracing::RaytraceApp;

fn main() -> eframe::Result {
    let native_options: NativeOptions = NativeOptions {
        viewport: egui::ViewportBuilder::default().with_maximized(true),
        ..Default::default()
    };

    eframe::run_native(
        "sound_raytrace",
        native_options,
        Box::new(|_cc| Ok(Box::new(RaytraceApp::new()))),
    )
}
