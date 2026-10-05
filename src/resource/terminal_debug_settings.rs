#[derive(Debug, bevy_ecs::resource::Resource)]
pub struct TerminalDebugSettings {
    pub active: bool,
}

impl Default for TerminalDebugSettings {
    fn default() -> Self {
        Self { active: true }
    }
}
