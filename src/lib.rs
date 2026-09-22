use bevy_ecs::world::{Mut, World};
use eframe::CreationContext;
use rook_terminal::{command::HelpCommand, Terminal, TerminalCommandEvent, TerminalWorldExtension};

use crate::renderer::{Renderer, Vertex};

pub mod command;
pub mod component;
pub mod renderer;
pub mod system;

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

        Renderer::init(cc);

        Self { world }
    }
}

impl eframe::App for RaytraceApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::bottom("terminal").show(ui, |ui: &mut egui::Ui| {
            let pending_event: Option<TerminalCommandEvent> = self
                .world
                .resource_scope(|_world: &mut World, mut terminal: Mut<Terminal>| terminal.ui(ui));

            if let Some(event) = pending_event {
                self.world.trigger(event);
                self.world.flush();
            }
        });
        egui::CentralPanel::default().show(ui, |ui: &mut egui::Ui| {
            let extracted_vertices = vec![
                Vertex {
                    position: [0.0, 0.5, 0.0],
                    color: [1.0, 0.0, 0.0],
                },
                Vertex {
                    position: [-0.5, -0.5, 0.0],
                    color: [0.0, 1.0, 0.0],
                },
                Vertex {
                    position: [0.5, -0.5, 0.0],
                    color: [0.0, 0.0, 1.0],
                },
            ];

            Renderer::show(ui, extracted_vertices);
        });
    }
}
