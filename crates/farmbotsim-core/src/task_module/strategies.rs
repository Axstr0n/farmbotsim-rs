use std::cmp::Ordering;

/// Strategies for selecting a charging station.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum ChooseStationStrategy {
    /// Manhattan distance-based selection
    ///
    /// factor: 0.0 → prioritize distance, 1.0 → prioritize small queues
    Manhattan(f32),
    /// Path distance-based selection
    ///
    /// factor: 0.0 → prioritize distance, 1.0 → prioritize small queues
    Path(f32),
}

impl ChooseStationStrategy {
    pub fn variants() -> Vec<ChooseStationStrategy> {
        vec![
            ChooseStationStrategy::Manhattan(0.6),
            ChooseStationStrategy::Path(0.6),
        ]
    }
}
impl Default for ChooseStationStrategy {
    fn default() -> Self {
        Self::Manhattan(0.0)
    }
}
impl std::fmt::Display for ChooseStationStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let str = match self {
            Self::Manhattan(v) => format!("Manhattan({v})"),
            Self::Path(v) => format!("Path({v})"),
        };
        write!(f, "{str}")
    }
}
impl Eq for ChooseStationStrategy {}
impl PartialOrd for ChooseStationStrategy {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for ChooseStationStrategy {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            // 1. Same variant: compare f32 factor
            (ChooseStationStrategy::Manhattan(a), ChooseStationStrategy::Manhattan(b)) => {
                a.partial_cmp(b).unwrap_or(Ordering::Equal)
            }

            (ChooseStationStrategy::Path(a), ChooseStationStrategy::Path(b)) => {
                a.partial_cmp(b).unwrap_or(Ordering::Equal)
            }

            // 2. Different variants: define a fixed order
            (ChooseStationStrategy::Manhattan(_), ChooseStationStrategy::Path(_)) => Ordering::Less,
            (ChooseStationStrategy::Path(_), ChooseStationStrategy::Manhattan(_)) => {
                Ordering::Greater
            }
        }
    }
}

/// Strategies with behavior when to charge.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", content = "values")]
pub enum ChargingStrategy {
    /// Go charging only if battery is bellow critical
    ///
    /// Critical value: f32 (0.0 - 100.0)
    CriticalOnly(f32),
    /// Go charging if battery is bellow threshold and station is available
    /// Go charging if battery is bellow critical
    ///
    /// Threshold value: f32 (0.0 - 100.0)
    ///
    /// Critical value: f32 (0.0 - 100.0)
    ThresholdWithLimit(f32, f32),
}
impl Default for ChargingStrategy {
    fn default() -> Self {
        Self::CriticalOnly(60.0)
    }
}
impl ChargingStrategy {
    pub fn variants() -> Vec<ChargingStrategy> {
        vec![
            ChargingStrategy::CriticalOnly(45.0),
            ChargingStrategy::ThresholdWithLimit(60.0, 45.0),
        ]
    }
}
impl std::fmt::Display for ChargingStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let str = match self {
            Self::CriticalOnly(c) => format!("CriticalOnly({c})"),
            Self::ThresholdWithLimit(t, c) => format!("ThresholdWithLimit({t}, {c})"),
        };
        write!(f, "{str}")
    }
}
impl Eq for ChargingStrategy {}
impl PartialOrd for ChargingStrategy {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for ChargingStrategy {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            // Both CriticalOnly → compare first parameter
            (ChargingStrategy::CriticalOnly(a), ChargingStrategy::CriticalOnly(b)) => {
                a.partial_cmp(b).unwrap_or(Ordering::Equal)
            }

            // Both ThresholdWithLimit → compare first parameter
            (
                ChargingStrategy::ThresholdWithLimit(t1, _),
                ChargingStrategy::ThresholdWithLimit(t2, _),
            ) => t1.partial_cmp(t2).unwrap_or(Ordering::Equal),

            // Variant ordering: CriticalOnly < ThresholdWithLimit
            (ChargingStrategy::CriticalOnly(_), ChargingStrategy::ThresholdWithLimit(_, _)) => {
                Ordering::Less
            }
            (ChargingStrategy::ThresholdWithLimit(_, _), ChargingStrategy::CriticalOnly(_)) => {
                Ordering::Greater
            }
        }
    }
}
