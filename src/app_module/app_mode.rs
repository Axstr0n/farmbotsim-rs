/// Represents the current mode of the application.
#[derive(Debug, PartialEq)]
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
    PerformanceMatrix,
}
impl AppMode {
    /// Returns a list of all available application modes.
    pub fn variants() -> Vec<AppMode> {
        vec![
            AppMode::MovementConfigEditor,
            AppMode::Battery,
            AppMode::AgentConfigEditor,
            AppMode::Simulation,
            AppMode::Path,
            AppMode::Task,
            AppMode::FarmEntityPlanEditor,
            AppMode::FieldConfigEditor,
            AppMode::SceneConfigEditor,
            AppMode::PerformanceMatrix,
        ]
    }
    /// Converts the application mode to its displayable name.
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
            AppMode::PerformanceMatrix => "PerformanceMatrix"
        }
    }
}