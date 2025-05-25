use egui::Pos2;


#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Crop {
    pub id: u32,
    pub row_id: u32,
    pub position: Pos2,
    growth_time: u32,
}

impl Crop {
    pub fn new(id: u32, row_id: u32, position: Pos2) -> Self {
        Self {
            id,
            row_id,
            position,
            growth_time: 0,
        }
    }
    pub fn grow(&mut self, dt: u32) {
        self.growth_time += dt;
    }
}
