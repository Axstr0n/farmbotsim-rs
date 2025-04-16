use egui::Pos2;


#[derive(Clone, PartialEq, Debug)]
pub enum Task {
    Stationary {
        id: u32,
        path: Vec<Pos2>,
        duration: f32,
        field_id: u32,
        line_id: u32,
    },
    Moving {
        id: u32,
        path: Vec<Pos2>,
        velocity: f32,
        field_id: u32,
        line_id: u32,
    },
    Travel {
        path: Vec<Pos2>,
        velocity: f32,
    },
}
impl Task {
    pub fn stationary(id: u32, pos: Pos2, duration: f32, field_id: u32, line_id: u32) -> Self {
        Task::Stationary {
            id,
            path: vec![pos],
            duration,
            field_id,
            line_id,
        }
    }
    pub fn moving(id: u32, path: Vec<Pos2>, velocity: f32, field_id: u32, line_id: u32) -> Self {
        Task::Moving {
            id,
            path,
            velocity,
            field_id,
            line_id,
        }
    }

    pub fn travel(path: Vec<Pos2>, velocity: f32) -> Self {
        Task::Travel {
            path,
            velocity,
        }
    }

    pub fn get_id(&self) -> &u32 {
        match self {
            Task::Stationary {id, .. } => { id },
            Task::Moving {id, .. } => { id },
            Task::Travel {.. } => { &0 },
        }
    }
    pub fn get_path(&self) -> &Vec<Pos2> {
        match self {
            Task::Stationary {path, .. } => { path },
            Task::Moving {path, .. } => { path },
            Task::Travel {path, .. } => { path },
        }
    }
    pub fn get_path_mut(&mut self) -> &mut Vec<Pos2> {
        match self {
            Task::Stationary {path, .. } => { path },
            Task::Moving {path, .. } => { path },
            Task::Travel {path, .. } => { path },
        }
    }
    pub fn get_velocity(&self) -> &f32 {
        match self {
            Task::Stationary {..} => { &0.0 },
            Task::Moving {velocity, .. } => { velocity },
            Task::Travel {velocity, .. } => { velocity },
        }
    }

    fn path_length(path: &[Pos2]) -> f32 {
        path.windows(2)
            .map(|w| w[0].distance(w[1]))
            .sum()
    }

    pub fn is_work(&self) -> bool {
        match self {
            Task::Stationary { .. } => true,
            Task::Moving { .. } => true,
            Task::Travel { .. } => false,
        }
    }
}
