use crate::{
    component::{
        camera::{ActiveCamera, Camera, CameraProjection},
        object::Transform,
    },
    renderer::{resource::init_callback_resources, Renderer},
};
use bevy_ecs::{
    entity::Entity,
    world::{Mut, World},
};
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

        init_callback_resources(cc, &mut world);

        let camera: Entity = world
            .spawn((
                Camera,
                Transform {
                    translation: glam::Vec3::new(0.0, 0.0, -5.0),
                    rotation: glam::Quat::IDENTITY,
                    scale: glam::Vec3::ONE,
                },
            ))
            .id();

        world.insert_resource(ActiveCamera { camera });

        world.insert_resource(CameraProjection {
            vertical_fov: 60.0_f32.to_radians(),
            aspect_ratio: 16.0 / 9.0,
            z_near: 0.1,
            z_far: 1000.0,
        });

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
            Renderer::ui(ui, &mut self.world);
        });
    }
}
