pub enum AppMode {
    Simulation,
    Path,
    Task,
    Battery,
    FarmEntityPlanEditor,
    MovementConfigEditor,
    AgentConfigEditor,
    FieldConfigEditor,
    SceneConfigEditor,
    PerformanceMatrixTool,
}
impl AppMode {
    pub fn variants() -> Vec<AppMode> {
        vec![
            AppMode::Simulation,
            AppMode::Path,
            AppMode::Task,
            AppMode::Battery,
            AppMode::FarmEntityPlanEditor,
            AppMode::MovementConfigEditor,
            AppMode::AgentConfigEditor,
            AppMode::FieldConfigEditor,
            AppMode::SceneConfigEditor,
            AppMode::PerformanceMatrixTool,
        ]
    }
    pub fn to_string(&self) -> &str {
        match self {
            AppMode::Simulation => "Simulation",
            AppMode::Path => "Path",
            AppMode::Task => "Task",
            AppMode::Battery => "Battery",
            AppMode::FarmEntityPlanEditor => "FarmEntityPlanEditor",
            AppMode::MovementConfigEditor => "MovementConfigEditor",
            AppMode::AgentConfigEditor => "AgentConfigEditor",
            AppMode::FieldConfigEditor => "FieldConfigEditor",
            AppMode::SceneConfigEditor => "SceneConfigEditor",
            AppMode::PerformanceMatrixTool => "PerformanceMatrixTool"
        }
    }
}