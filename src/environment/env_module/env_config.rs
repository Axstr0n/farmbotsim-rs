use std::path::Path;
use serde::{Deserialize, Serialize};

use crate::{
    environment::{
        datetime::DateTimeConfig,
    }, task_module::task_manager_config::TaskManagerConfig, utilities::utils::load_json_or_panic
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
