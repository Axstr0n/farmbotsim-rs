use egui::Pos2;


#[derive(Clone, PartialEq, Debug)]
pub struct Task {
    id: u32,
    pub path: Vec<Pos2>,
    pub velocity: f32,
    pub duration: f32,
    info: String,
    pub field_id: Option<u32>,
    pub line_id: Option<u32>,
}
impl Task {
    pub fn stationary(id: u32, pos: Pos2, duration: f32, field_id: Option<u32>, line_id: Option<u32>) -> Self {
        Self {
            id,
            path: vec![pos],
            velocity: 0.0,
            duration,
            info: "".to_string(),
            field_id,
            line_id,
        }
    }
    pub fn moving(id: u32, path: Vec<Pos2>, velocity: f32, field_id: Option<u32>, line_id: Option<u32>) -> Self {
        let duration = Self::path_length(&path) / velocity;
        Self {
            id,
            path,
            velocity,
            duration,
            info: "".to_string(),
            field_id,
            line_id,
        }
    }
    fn path_length(path: &[Pos2]) -> f32 {
        path.windows(2)
            .map(|w| w[0].distance(w[1]))
            .sum()
    }
    pub fn is_moving(&self) -> bool {
        self.velocity > 0.0
    }
}
