use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::{
    cfg::{DEFAULT_AGENT_CONFIG_PATH, DEFAULT_SCENE_CONFIG_PATH, DEFAULT_TASK_MANAGER_CONFIG_PATH},
    environment::datetime::DateTimeConfig,
    utilities::utils::load_json_or_panic,
};

/// Configuration settings for the environment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvConfig {
    /// Number of agents in the environment.
    pub n_agents: u32,
    /// Path to the agent configuration file.
    pub agent_config_path: String,
    /// Configuration for date and time settings.
    #[serde(rename = "date_time")]
    pub datetime_config: DateTimeConfig,
    /// Path to the scene configuration file.
    pub scene_config_path: String,
    /// Path to task manager configuration file.
    pub task_manager_config_path: String,
}

impl Default for EnvConfig {
    /// Creates a default environment configuration with preset values.
    fn default() -> Self {
        Self {
            n_agents: 1,
            agent_config_path: DEFAULT_AGENT_CONFIG_PATH.to_string(),
            datetime_config: DateTimeConfig::from_string("01.01.2025 00:00:00".to_string()),
            scene_config_path: DEFAULT_SCENE_CONFIG_PATH.to_string(),
            task_manager_config_path: DEFAULT_TASK_MANAGER_CONFIG_PATH.to_string(),
        }
    }
}

impl EnvConfig {
    /// Creates a new `EnvConfig` with specified parameters.
    pub fn new(
        n_agents: u32,
        agent_config_path: String,
        datetime_config: DateTimeConfig,
        scene_config_path: String,
        task_manager_config_path: String,
    ) -> Self {
        Self {
            n_agents,
            agent_config_path,
            datetime_config,
            scene_config_path,
            task_manager_config_path,
        }
    }
}

impl EnvConfig {
    /// Loads an `EnvConfig` from a JSON file at the given path.
    /// Panics if the JSON file cannot be loaded or parsed.
    pub fn from_json_file<P: AsRef<Path>>(file_path: P) -> Self {
        load_json_or_panic(file_path)
    }
}
