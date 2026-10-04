use crate::renderer::{callback::init_callback_resources, Renderer};
use bevy_ecs::world::{Mut, World};
use eframe::CreationContext;
use rook_terminal::{command::HelpCommand, Terminal, TerminalCommandEvent, TerminalWorldExtension};

pub struct RaytraceApp {
    pub world: World,
}

impl RaytraceApp {
    pub fn new(cc: &CreationContext) -> Self {
        let mut world: World = World::new();
        world.setup_terminal();

        world.trigger(TerminalCommandEvent {
            raw_command: HelpCommand::name().to_string(),
        });
        world.flush();

        init_callback_resources(cc);

        Self { world }
    }
}

impl eframe::App for RaytraceApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::left("terminal").show(ui, |ui: &mut egui::Ui| {
            let pending_event: Option<TerminalCommandEvent> = self
                .world
                .resource_scope(|_world: &mut World, mut terminal: Mut<Terminal>| terminal.ui(ui));

            if let Some(event) = pending_event {
                self.world.trigger(event);
                self.world.flush();
            }
        });
        egui::CentralPanel::default().show(ui, |ui: &mut egui::Ui| {
            todo!();
        });
    }
}
