use std::path::Path;

use crate::utilities::utils::load_json_or_panic;

/// Configuration for an agent, including movement type, battery type, and state of charge.
#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct AgentConfig {
    pub movement: String,
    pub battery: String,
    pub battery_soc: f32,
}

impl AgentConfig {
    /// Creates a new agent configuration.
    pub fn new(movement: String, battery: String, battery_soc: f32) -> Self {
        Self {
            movement,
            battery,
            battery_soc,
        }
    }

    /// Loads an agent configuration from a JSON file, panicking on failure.
    pub fn from_json_file<P: AsRef<Path>>(file_path: P) -> Self {
        load_json_or_panic(file_path)
    }
}
