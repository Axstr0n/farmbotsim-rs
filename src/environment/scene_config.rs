use crate::environment::{spawn_area_module::spawn_area_config::SpawnAreaConfig, station_module::station_config::StationConfig};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SceneConfig {
    pub field_config_path: String,
    pub station_configs: Vec<StationConfig>,
    pub spawn_area_config: SpawnAreaConfig,
}