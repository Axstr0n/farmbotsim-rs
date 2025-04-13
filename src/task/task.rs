use egui::Pos2;


#[derive(Clone, PartialEq)]
pub struct Task {
    id: u32,
    path: Vec<Pos2>,
    velocity: f32,
    duration: f32,
    info: String,
}
impl Task {
    pub fn stationary(id: u32, pos: Pos2, duration: f32) -> Self {
        Self {
            id,
            path: vec![pos],
            velocity: 0.0,
            duration,
            info: "".to_string(),
        }
    }
    pub fn moving(id: u32, path: Vec<Pos2>, velocity: f32) -> Self {
        let duration = Self::path_length(&path) / velocity;
        Self {
            id,
            path,
            velocity,
            duration,
            info: "".to_string(),
        }
    }
    fn path_length(path: &[Pos2]) -> f32 {
        path.windows(2)
            .map(|w| w[0].distance(w[1]))
            .sum()
    }
}