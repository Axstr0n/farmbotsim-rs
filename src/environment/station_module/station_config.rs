use egui::{Pos2, Vec2};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StationConfig {
    pub position: Pos2,
    pub queue_direction: Vec2,
    pub n_slots: u32,
}

impl Default for StationConfig {
    fn default() -> Self {
        Self {
            position: Pos2::ZERO,
            queue_direction: Vec2::X,
            n_slots: 1,
        }
    }
}

impl StationConfig {
    pub fn new(position: Pos2, queue_direction: Vec2, n_slots: u32) -> Self {
        Self {
            position,
            queue_direction,
            n_slots
        }
    }
}
