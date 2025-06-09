use std::path::Path;
use serde::{Deserialize, Serialize};

use crate::{
    environment::{
        datetime::DateTimeConfig,
        field_config::VariantFieldConfig,
        spawn_area_module::spawn_area_config::SpawnAreaConfig,
        station_module::station_config::StationConfig
    }, task_module::task_manager_config::TaskManagerConfig, utilities::utils::load_json_or_panic
};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvConfig {
    pub n_agents: u32,
    pub agent_path: String,
    #[serde(rename = "date_time")]
    pub datetime_config: DateTimeConfig,
    #[serde(rename = "field")]
    pub field_configs: Vec<VariantFieldConfig>,
    #[serde(rename = "stations")]
    pub station_configs: Vec<StationConfig>,
    #[serde(rename = "spawn_area")]
    pub spawn_area_config: SpawnAreaConfig,
    #[serde(rename = "task_manager")]
    pub task_manager_config: TaskManagerConfig,
}

impl EnvConfig {
    pub fn new(n_agents: u32, agent_path: String, datetime_config: DateTimeConfig, field_configs: Vec<VariantFieldConfig>, station_configs: Vec<StationConfig>, spawn_area_config: SpawnAreaConfig, task_manager_config: TaskManagerConfig) -> Self {
        Self {
            n_agents,
            agent_path,
            datetime_config,
            field_configs,
            station_configs,
            spawn_area_config,
            task_manager_config,
        }
    }
}

impl EnvConfig {
    pub fn from_json_file<P: AsRef<Path>>(file_path: P) -> Self {
        load_json_or_panic(file_path)
    }
}
