use egui::Pos2;


#[derive(PartialEq, Clone, Copy)]
pub struct FieldConfig {
    pub left_top_pos: Pos2,
    pub angle: f32,
    pub n_rows: u32,
    pub n_crops_per_row: u32,
    pub row_spacing: f32,
    pub crop_spacing: f32,
}

impl Default for FieldConfig {
    fn default() -> Self {
        Self {
            left_top_pos: Pos2 { x: 3.0, y: 4.0 },
            angle: 4.0,
            n_rows: 7,
            n_crops_per_row: 10,
            row_spacing: 0.5,
            crop_spacing: 0.3,
        }
    }
}