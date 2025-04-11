use egui::Pos2;


#[derive(Clone)]
pub struct SpawnArea {
    pub left_top_pos: Pos2,
    pub angle: f32,
    pub length: f32,
    pub width: f32,
}

impl Default for SpawnArea {
    fn default() -> Self {
        Self {
            left_top_pos: Pos2::new(2.0, 1.0),
            angle: 5.0,
            length: 3.0,
            width: 1.0,
        }
    }
}
