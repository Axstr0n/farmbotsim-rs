
#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct AgentConfig {
    pub movement: String,
    pub battery: String,
    pub battery_soc: f32,
}

impl AgentConfig {
    pub fn new(movement: String, battery: String, battery_soc: f32) -> Self {
        Self { movement, battery, battery_soc }
    }
}