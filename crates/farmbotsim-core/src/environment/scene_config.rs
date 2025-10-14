use crate::environment::{
    spawn_area_module::spawn_area_config::SpawnAreaConfig,
    station_module::station_config::StationConfig,
};

/// Configuration data for a scene.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SceneConfig {
    /// Path to the field configuration file.
    pub field_config_path: String,
    /// List of configurations for stations in the scene
    pub station_configs: Vec<StationConfig>,
    /// Configuration for the spawn area within the scene.
    pub spawn_area_config: SpawnAreaConfig,
}
