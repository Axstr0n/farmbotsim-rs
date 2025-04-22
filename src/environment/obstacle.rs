use egui::Pos2;

#[derive(PartialEq, Debug, Clone)]
pub struct Obstacle {
    pub points: Vec<Pos2>,
}

impl Obstacle {
    pub fn new(points: Vec<Pos2>) -> Self {
        Self {
            points,
        }
    }
}
