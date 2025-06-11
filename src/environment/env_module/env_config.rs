use std::path::Path;
use serde::{Deserialize, Serialize};

use crate::{
    cfg::{DEFAULT_AGENT_CONFIG_PATH, DEFAULT_SCENE_CONFIG_PATH}, environment::datetime::DateTimeConfig, task_module::task_manager_config::TaskManagerConfig, utilities::utils::load_json_or_panic
};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvConfig {
    pub n_agents: u32,
    pub agent_config_path: String,
    #[serde(rename = "date_time")]
    pub datetime_config: DateTimeConfig,
    pub scene_config_path: String,
    #[serde(rename = "task_manager")]
    pub task_manager_config: TaskManagerConfig,
}

impl Default for EnvConfig {
    fn default() -> Self {
        Self {
            n_agents: 1,
            agent_config_path: DEFAULT_AGENT_CONFIG_PATH.to_string(),
            datetime_config: DateTimeConfig::from_string("01.01.2025 00:00:00".to_string()),
            scene_config_path: DEFAULT_SCENE_CONFIG_PATH.to_string(),
            task_manager_config: TaskManagerConfig::default(),
        }
    }
}

impl EnvConfig {
    pub fn new(n_agents: u32, agent_config_path: String, datetime_config: DateTimeConfig, scene_config_path: String, task_manager_config: TaskManagerConfig) -> Self {
        Self {
            n_agents,
            agent_config_path,
            datetime_config,
            scene_config_path,
            task_manager_config,
        }
    }
}

impl EnvConfig {
    pub fn from_json_file<P: AsRef<Path>>(file_path: P) -> Self {
        load_json_or_panic(file_path)
    }
}
