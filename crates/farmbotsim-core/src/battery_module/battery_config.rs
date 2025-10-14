use crate::{
    units::{energy::Energy, voltage::Voltage},
    utilities::utils::load_json_or_panic,
};

/// Configuration for a battery model, including capacity, voltage, and seasonal characteristics.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct BatteryConfig {
    pub name: String,
    pub capacity: Energy,
    pub voltage: Voltage,
    pub jan_max: String,
    pub jan_min: String,
    pub jun_max: String,
}
impl BatteryConfig {
    /// Loads a BatteryConfig from a config.json file inside the given folder.
    /// Panics if the file is missing or invalid.
    pub fn from_json_file(folder_name: String) -> Self {
        let path_str = format!("{folder_name}/config.json");
        load_json_or_panic(path_str)
    }
}
