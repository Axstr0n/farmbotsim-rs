use crate::task_module::strategies::{ChargingStrategy, ChooseStationStrategy};

/// Configuration for task manager strategies including charging and station selection.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TaskManagerConfig {
    pub charging_strat: ChargingStrategy,
    pub choose_station_strat: ChooseStationStrategy,
}
impl Default for TaskManagerConfig {
    fn default() -> Self {
        Self {
            charging_strat: ChargingStrategy::CriticalOnly,
            choose_station_strat: ChooseStationStrategy::Closest,
        }
    }
}