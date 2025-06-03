use egui::Pos2;

use crate::units::{angle::Angle, length::Length};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SpawnAreaConfig {
    pub left_top_pos: Pos2,
    pub angle: Angle,
    pub width: Length,
    pub height: Length,
}
