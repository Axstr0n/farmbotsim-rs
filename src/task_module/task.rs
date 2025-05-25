use egui::Pos2;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Intent {
    Work,
    Charge,
    Queue,
    Idle
}


#[derive(Clone, PartialEq, Debug)]
pub enum Task {
    Stationary {
        id: u32,
        pos: Pos2,
        duration: u32,
        field_id: u32,
        line_id: u32,
        power_w: f32,
        intent: Intent,
    },
    Moving {
        id: u32,
        path: Vec<Pos2>,
        velocity: f32,
        field_id: u32,
        line_id: u32,
        power_w: f32,
        intent: Intent,
    },
    Travel {
        path: Vec<Pos2>,
        velocity: f32,
        intent: Intent,
    },
    WaitDuration {
        pos: Pos2,
        duration: u32,
        intent: Intent,
    },
    WaitInfinite {
        pos: Pos2,
        intent: Intent,
    }
}
impl Task {
    pub fn stationary(id: u32, pos: Pos2, duration: u32, field_id: u32, line_id: u32, power_w: f32) -> Self {
        Task::Stationary {
            id,
            pos,
            duration,
            field_id,
            line_id,
            power_w,
            intent: Intent::Work,
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
            intent: Intent::Work,
        }
    }

    pub fn travel(path: Vec<Pos2>, velocity: f32, intent: Intent) -> Self {
        Task::Travel {
            path,
            velocity,
            intent,
        }
    }
    pub fn wait_duration(pos: Pos2, duration: u32, intent: Intent) -> Self {
        Task::WaitDuration {
            pos,
            duration,
            intent
        }
    }
    pub fn wait_infinite(pos: Pos2, intent: Intent) -> Self {
        Task::WaitInfinite {
            pos,
            intent
        }
    }

    pub fn get_id(&self) -> Option<&u32> {
        match self {
            Task::Stationary {id, .. } => { Some(id) },
            Task::Moving {id, .. } => { Some(id) },
            Task::Travel {.. } => { None },
            Task::WaitDuration {.. } => { None },
            Task::WaitInfinite {.. } => { None },
        }
    }
    pub fn get_path(&self) -> Vec<Pos2> {
        match self {
            Task::Stationary {pos, .. } => { vec![*pos] },
            Task::Moving {path, .. } => { path.clone() },
            Task::Travel {path, .. } => { path.clone() },
            Task::WaitDuration { pos, .. } => { vec![*pos] },
            Task::WaitInfinite { pos, .. } => { vec![*pos] },
        }
    }
    pub fn get_first_pos(&self) -> &Pos2 {
        match self {
            Task::Stationary {pos, .. } => { pos },
            Task::Moving {path, .. } => { &path[0] },
            Task::Travel {path, .. } => { &path[0] },
            Task::WaitDuration { pos, .. } => { pos },
            Task::WaitInfinite { pos, .. } => { pos },
        }
    }
    pub fn get_velocity(&self) -> &f32 {
        match self {
            Task::Stationary {..} => { &0.0 },
            Task::Moving {velocity, .. } => { velocity },
            Task::Travel {velocity, .. } => { velocity },
            Task::WaitDuration { .. } => { &0.0 },
            Task::WaitInfinite { .. } => { &0.0 },
        }
    }
    pub fn get_intent(&self) -> &Intent {
        match self {
            Task::Stationary { intent,.. } => { intent },
            Task::Moving { intent,.. } => { intent },
            Task::Travel { intent,.. } => { intent },
            Task::WaitDuration { intent,.. } => { intent },
            Task::WaitInfinite { intent,.. } => { intent },
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
            Task::WaitDuration { .. } => false,
            Task::WaitInfinite { .. } => false,
        }
    }
    pub fn is_travel(&self) -> bool {
        matches!(self, Task::Travel { .. })
    }
    pub fn is_wait(&self) -> bool {
        matches!(self, Task::WaitDuration { .. } | Task::WaitInfinite { .. })
    }

    pub fn is_charge_intent(&self) -> bool {
        matches!(self, Task::WaitDuration { intent: Intent::Charge,.. } | Task::WaitInfinite { intent: Intent::Charge,.. })
    }
}
