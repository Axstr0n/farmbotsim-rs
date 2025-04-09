use egui::Vec2;
use rand::Rng;
use std::f32::consts::PI;

pub fn random_vec2() -> Vec2 {
    let mut rng = rand::rng(); // Random number generator

    // Generate a random angle in radians between 0 and 2π
    let angle = rng.random_range(0.0..2.0 * PI);

    // Convert the angle to a direction (unit vector)
    let x = angle.cos();
    let y = angle.sin();

    Vec2::new(x, y)
}

pub trait Vec2Rotate {
    fn rotate_radians(&self, angle_rad: f32) -> Vec2;
    fn rotate_degrees(&self, angle_deg: f32) -> Vec2;
}

impl Vec2Rotate for Vec2 {
    fn rotate_radians(&self, angle_rad: f32) -> Vec2 {
        let cos = angle_rad.cos();
        let sin = angle_rad.sin();
        Vec2::new(
            self.x * cos - self.y * sin,
            self.x * sin + self.y * cos,
        )
    }

    fn rotate_degrees(&self, angle_deg: f32) -> Vec2 {
        self.rotate_radians(angle_deg.to_radians())
    }
}


pub trait ExtendedVec2 {
    fn fmt(&self, n_decimals: usize) -> String;
}

impl ExtendedVec2 for Vec2 {
    fn fmt(&self, n_decimals: usize) -> String {
        format!("({:.*}, {:.*})", n_decimals, self.x, n_decimals, self.y)
    }
}
