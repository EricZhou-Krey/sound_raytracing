use bevy_ecs::{
    schedule::Schedule,
    world::{Mut, World},
};
use rook_terminal::{Terminal, TerminalScheduleExtension, TerminalWorldExtension};

pub mod component;
pub mod system;

pub struct RaytraceApp {
    pub world: World,
    pub schedule: Schedule,
}

impl Default for RaytraceApp {
    fn default() -> Self {
        Self::new()
    }
}

impl RaytraceApp {
    pub fn new() -> Self {
        let mut world: World = World::new();
        let mut schedule: Schedule = Schedule::default();

        world.setup_terminal();
        schedule.setup_terminal();

        Self { world, schedule }
    }
}

impl eframe::App for RaytraceApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::bottom("terminal").show(ui, |ui| {
            self.world
                .resource_scope(|_world: &mut World, mut terminal: Mut<Terminal>| {
                    terminal.ui(ui);
                });
        });
    }

    fn logic(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.schedule.run(&mut self.world);
    }
}
