use crate::task_module::task_manager::{ChargingStrat, ChooseStationStrat};


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TaskManagerConfig {
    pub charging_strat: ChargingStrat,
    pub choose_station_strat: ChooseStationStrat,
}
impl Default for TaskManagerConfig {
    fn default() -> Self {
        Self {
            charging_strat: ChargingStrat::CriticalOnly,
            choose_station_strat: ChooseStationStrat::Closest,
        }
    }
}