use crate::{
    asset::AssetWorldExtension,
    demo::DemoWorldExtension,
    renderer::{resource::init_callback_resources, Renderer},
};
use bevy_ecs::{schedule::Schedule, world::World};
use eframe::CreationContext;
use rook_terminal::{
    command::HelpCommand, event::TerminalCommandRequested, TerminalInputState, TerminalSession,
    TerminalViewState, TerminalWorldExtension,
};

pub struct RaytraceApp {
    pub world: World,
    pub schedule: Schedule,
}

impl RaytraceApp {
    pub fn new(cc: &CreationContext) -> Self {
        let mut world: World = World::new();
        world.setup_terminal();
        world.setup_assets();
        world.setup_demo();

        world.trigger(TerminalCommandRequested {
            raw_command: HelpCommand::name().to_string(),
        });
        world.flush();

        init_callback_resources(cc, &mut world);

        let schedule: Schedule = Schedule::default();

        Self { world, schedule }
    }
}

impl eframe::App for RaytraceApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::left("Terminal").show(ui, |ui: &mut egui::Ui| {
            let pending_event: Option<TerminalCommandRequested> = self
                .world
                .resource_scope::<TerminalSession, _>(|world, mut terminal_session| {
                    world.resource_scope::<TerminalInputState, _>(|world, mut input_state| {
                        let view_state = world.resource::<TerminalViewState>();

                        terminal_session.ui(&mut input_state, view_state, ui)
                    })
                });

            if let Some(event) = pending_event {
                self.world.trigger(event);
                self.world.flush();
            }
        });

        egui::CentralPanel::default().show(ui, |ui: &mut egui::Ui| {
            Renderer::ui(ui, &mut self.world);
        });
    }

    fn logic(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.schedule.run(&mut self.world);
    }
}
