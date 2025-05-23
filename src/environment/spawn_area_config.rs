use egui::Pos2;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SpawnAreaConfig {
    pub left_top_pos: Pos2,
    pub angle: f32,
    pub width: f32,
    pub height: f32,
}
