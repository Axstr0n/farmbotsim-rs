use egui::Pos2;

use crate::{
    environment::spawn_area_module::spawn_area_config::SpawnAreaConfig,
    units::{angle::Angle, length::Length}
};


#[derive(Clone, Debug, PartialEq)]
pub struct SpawnArea {
    pub left_top_pos: Pos2,
    pub angle: Angle,
    pub width: Length,
    pub height: Length,
}

impl SpawnArea {
    pub fn from_config(config: SpawnAreaConfig) -> Self {
        Self {
            left_top_pos: config.left_top_pos,
            angle: config.angle,
            width: config.width,
            height: config.height,
        }
    }
    pub fn to_config(&self) -> SpawnAreaConfig {
        SpawnAreaConfig {
            left_top_pos: self.left_top_pos,
            angle: self.angle,
            width: self.width,
            height: self.height
        }
    }
}

impl Default for SpawnArea {
    fn default() -> Self {
        Self {
            left_top_pos: Pos2::new(2.0, 1.0),
            angle: Angle::degrees(5.0),
            width: Length::meters(3.0),
            height: Length::meters(1.0),
        }
    }
}
