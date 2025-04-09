pub enum AppMode {
    Simulation,
    Path,
    Editor,
    ConfigEditor,
}
impl AppMode {
    pub fn variants() -> Vec<AppMode> {
        vec![
            AppMode::Simulation,
            AppMode::Path,
            AppMode::Editor,
            AppMode::ConfigEditor,
        ]
    }
    pub fn to_string(&self) -> &str {
        match self {
            AppMode::Simulation => "Simulation",
            AppMode::Path => "Path",
            AppMode::Editor => "Editor",
            AppMode::ConfigEditor => "ConfigEditor",
        }
    }
}