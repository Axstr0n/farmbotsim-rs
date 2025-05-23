use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::utilities::datetime::DateTimeConfig;

use super::{field_config::VariantFieldConfig, spawn_area_config::SpawnAreaConfig, station_config::StationConfig};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvConfig {
    pub n_agents: u32,
    #[serde(rename = "date_time")]
    pub datetime_config: DateTimeConfig,
    #[serde(rename = "field")]
    pub field_configs: Vec<VariantFieldConfig>,
    #[serde(rename = "stations")]
    pub station_configs: Vec<StationConfig>,
    #[serde(rename = "spawn_area")]
    pub spawn_area_config: SpawnAreaConfig,
}

impl EnvConfig {
    pub fn new(n_agents: u32, datetime_config: DateTimeConfig, field_configs: Vec<VariantFieldConfig>, station_configs: Vec<StationConfig>, spawn_area_config: SpawnAreaConfig) -> Self {
        Self {
            n_agents,
            datetime_config,
            field_configs,
            station_configs,
            spawn_area_config
        }
    }
}

impl EnvConfig {
    pub fn from_json_file<P: AsRef<Path>>(file_path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let file = std::fs::File::open(file_path.as_ref())
            .map_err(|e| format!("Failed to open file: {}", e))?;
        
        let config = serde_json::from_reader(file)
            .map_err(|e| format!("Failed to parse JSON: {}", e))?;
        
        Ok(config)
    }
}
