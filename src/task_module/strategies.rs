/// Strategies for selecting a charging station.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ChooseStationStrategy {
    /// Choose first charging station
    First,
    /// Choose closest charging station
    Closest,
    /// Choose closest charging station with min queue
    ClosestMinQueue,
}
impl ChooseStationStrategy {
    pub fn variants() -> Vec<ChooseStationStrategy> {
        vec![ChooseStationStrategy::First, ChooseStationStrategy::Closest, ChooseStationStrategy::ClosestMinQueue]
    }
}
impl std::fmt::Display for ChooseStationStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let str = match self {
            Self::First => "First".to_string(),
            Self::Closest => "Closest".to_string(),
            Self::ClosestMinQueue => "ClosestMinQueue".to_string(),
        };
        write!(f, "{str}")
    }
}

/// Strategies with behavior when to charge.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ChargingStrategy {
    /// Go charging only if battery is bellow critical
    CriticalOnly,
    /// Go charging if battery is bellow threshold and station is available
    /// Go charging if battery is bellow critical
    ThresholdWithLimit,
}
impl ChargingStrategy {
    pub fn variants() -> Vec<ChargingStrategy> {
        vec![ChargingStrategy::CriticalOnly, ChargingStrategy::ThresholdWithLimit]
    }
}
impl std::fmt::Display for ChargingStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let str = match self {
            Self::CriticalOnly => "CriticalOnly".to_string(),
            Self::ThresholdWithLimit => "ThresholdWithLimit".to_string(),
        };
        write!(f, "{str}")
    }
}