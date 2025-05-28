use egui::{Pos2, Rect};
use rand::Rng;

use crate::units::{angle::Angle, length::Length};

pub fn random_pos2_in_rect(rect: Rect, angle: Angle) -> Pos2 {
    let mut rng = rand::rng(); // Random number generator

    // Generate random x and y coordinates within the bounds of the rectangle
    let x = rng.random_range(rect.min.x..rect.max.x);
    let y = rng.random_range(rect.min.y..rect.max.y);

    let dx = x - rect.min.x;
    let dy = y - rect.min.y;

    let cos_a = angle.to_radians().cos();
    let sin_a = angle.to_radians().sin();

    let rotated_x = dx * cos_a - dy * sin_a;
    let rotated_y = dx * sin_a + dy * cos_a;

    Pos2::new(rect.min.x + rotated_x, rect.min.y + rotated_y)
}

pub trait ExtendedPos2 {
    fn fmt(&self, n_decimals: usize) -> String;
    fn is_close_to(&self, other: Pos2, tolerance: Length) -> bool;
}

impl ExtendedPos2 for Pos2 {
    fn fmt(&self, n_decimals: usize) -> String {
        format!("({:.*}, {:.*})", n_decimals, self.x, n_decimals, self.y)
    }

    fn is_close_to(&self, other: Pos2, tolerance: Length) -> bool {
        self.distance(other) <= tolerance.to_base_unit()
    }
}
