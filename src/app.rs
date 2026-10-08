use crate::{
    asset::{
        loader::{load_material, load_mesh, request_init_object, GPUAssetUploader},
        manager::{MaterialManager, MeshManager, TextureManager},
    },
    component::{
        camera::{ActiveCamera, Camera, CameraProjection},
        object::Transform,
    },
    renderer::{resource::init_callback_resources, Renderer},
    resource::terminal_debug_settings::TerminalDebugSettings,
};
use bevy_ecs::{
    entity::Entity,
    schedule::{IntoScheduleConfigs, Schedule},
    world::{self, Mut, World},
};
use eframe::CreationContext;
use rook_terminal::{command::HelpCommand, Terminal, TerminalCommandEvent, TerminalWorldExtension};

pub struct RaytraceApp {
    pub world: World,
    pub schedule: Schedule,
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
        world.insert_resource(MeshManager::default());
        world.insert_resource(TextureManager::default());
        world.insert_resource(MaterialManager::default());
        world.insert_resource(GPUAssetUploader::default());
        world.insert_resource(TerminalDebugSettings::default());

        world.add_observer(request_init_object);
        let mut schedule: Schedule = Schedule::default();
        schedule.add_systems((load_mesh, load_material).chain());

        Self { world, schedule }
    }
}

impl eframe::App for RaytraceApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::left("Terminal").show(ui, |ui: &mut egui::Ui| {
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

    fn logic(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.schedule.run(&mut self.world);
    }
}
