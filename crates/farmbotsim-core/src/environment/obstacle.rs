use egui::Pos2;

/// Represents a polygonal obstacle defined by a series of 2D points.
#[derive(PartialEq, Debug, Clone)]
pub struct Obstacle {
    pub points: Vec<Pos2>,
}

impl Obstacle {
    /// Creates a new `Obstacle` from a list of 2D points.
    pub fn new(points: Vec<Pos2>) -> Self {
        Self { points }
    }
}
