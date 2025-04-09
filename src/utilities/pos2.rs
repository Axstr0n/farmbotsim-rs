use egui::{Pos2, Rect};
use rand::Rng;

pub fn random_pos2_in_rect(rect: Rect) -> Pos2 {
    let mut rng = rand::rng(); // Random number generator

    // Generate random x and y coordinates within the bounds of the rectangle
    let x = rng.random_range(rect.min.x..rect.max.x);
    let y = rng.random_range(rect.min.y..rect.max.y);

    Pos2::new(x, y)
}

pub trait ExtendedPos2 {
    fn fmt(&self, n_decimals: usize) -> String;
    fn is_close_to(&self, other: Pos2, tolerance: f32) -> bool;
}

impl ExtendedPos2 for Pos2 {
    fn fmt(&self, n_decimals: usize) -> String {
        format!("({:.*}, {:.*})", n_decimals, self.x, n_decimals, self.y)
    }

    fn is_close_to(&self, other: Pos2, tolerance: f32) -> bool {
        self.distance(other) <= tolerance
    }
}
