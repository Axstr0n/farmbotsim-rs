pub enum AppMode {
    Simulation,
    Path,
    Task,
    Editor,
    Battery,
    FarmEntityPlanEditor
}
impl AppMode {
    pub fn variants() -> Vec<AppMode> {
        vec![
            AppMode::Simulation,
            AppMode::Path,
            AppMode::Task,
            AppMode::Editor,
            AppMode::Battery,
            AppMode::FarmEntityPlanEditor,
        ]
    }
    pub fn to_string(&self) -> &str {
        match self {
            AppMode::Simulation => "Simulation",
            AppMode::Path => "Path",
            AppMode::Task => "Task",
            AppMode::Editor => "Editor",
            AppMode::Battery => "Battery",
            AppMode::FarmEntityPlanEditor => "FarmEntityPlanEditor"
        }
    }
}