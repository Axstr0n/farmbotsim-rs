use egui::Pos2;

use super::spawn_area_config::SpawnAreaConfig;


#[derive(Clone, Debug, PartialEq)]
pub struct SpawnArea {
    pub left_top_pos: Pos2,
    pub angle: f32,
    pub width: f32,
    pub height: f32,
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
            angle: 5.0,
            width: 3.0,
            height: 1.0,
        }
    }
}
