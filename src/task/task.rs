use egui::Pos2;


#[derive(Clone, PartialEq, Debug)]
pub enum Task {
    Stationary {
        id: u32,
        pos: Pos2,
        duration: f32,
        field_id: u32,
        line_id: u32,
        power_w: f32,
    },
    Moving {
        id: u32,
        path: Vec<Pos2>,
        velocity: f32,
        field_id: u32,
        line_id: u32,
        power_w: f32,
    },
    Travel {
        path: Vec<Pos2>,
        velocity: f32,
    },
}
impl Task {
    pub fn stationary(id: u32, pos: Pos2, duration: f32, field_id: u32, line_id: u32, power_w: f32) -> Self {
        Task::Stationary {
            id,
            pos,
            duration,
            field_id,
            line_id,
            power_w,
        }
    }
    pub fn moving(id: u32, path: Vec<Pos2>, velocity: f32, field_id: u32, line_id: u32, power_w: f32) -> Self {
        Task::Moving {
            id,
            path,
            velocity,
            field_id,
            line_id,
            power_w,
        }
    }

    pub fn travel(path: Vec<Pos2>, velocity: f32) -> Self {
        Task::Travel {
            path,
            velocity,
        }
    }

    pub fn get_id(&self) -> Option<&u32> {
        match self {
            Task::Stationary {id, .. } => { Some(id) },
            Task::Moving {id, .. } => { Some(id) },
            Task::Travel {.. } => { None },
        }
    }
    pub fn get_path(&self) -> Vec<Pos2> {
        match self {
            Task::Stationary {pos, .. } => { vec![*pos] },
            Task::Moving {path, .. } => { path.clone() },
            Task::Travel {path, .. } => { path.clone() },
        }
    }
    pub fn get_first_pos(&self) -> &Pos2 {
        match self {
            Task::Stationary {pos, .. } => { pos },
            Task::Moving {path, .. } => { &path[0] },
            Task::Travel {path, .. } => { &path[0] },
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
