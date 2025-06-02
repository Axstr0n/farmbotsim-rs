use egui::Pos2;

use crate::{
    units::{duration::Duration, linear_velocity::LinearVelocity, power::Power}
};

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
        duration: Duration,
        intent: Intent,
        crop_id: u32,
        field_id: u32,
        line_id: u32,
        power: Power,
        info: String,
    },
    Moving {
        id: u32,
        path: Vec<Pos2>,
        velocity: LinearVelocity,
        intent: Intent,
        field_id: u32,
        line_id: u32,
        power: Power,
        info: String,
    },
    Travel {
        path: Vec<Pos2>,
        velocity: LinearVelocity,
        intent: Intent,
    },
    WaitDuration {
        duration: Duration,
        intent: Intent,
    },
    WaitInfinite {
        intent: Intent,
    }
}

impl Task {
    pub fn travel(path: Vec<Pos2>, velocity: LinearVelocity, intent: Intent) -> Self {
        Task::Travel {
            path,
            velocity,
            intent,
        }
    }
    pub fn wait_duration(duration: Duration, intent: Intent) -> Self {
        Task::WaitDuration {
            duration,
            intent
        }
    }
    pub fn wait_infinite(intent: Intent) -> Self {
        Task::WaitInfinite {
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
    pub fn get_path(&self) -> Option<Vec<Pos2>> {
        match self {
            Task::Stationary {pos, .. } => { Some(vec![*pos]) },
            Task::Moving {path, .. } => { Some(path.clone()) },
            Task::Travel {path, .. } => { Some(path.clone()) },
            Task::WaitDuration { .. } => { None },
            Task::WaitInfinite { .. } => { None },
        }
    }
    pub fn get_first_pos(&self) -> Option<&Pos2> {
        match self {
            Task::Stationary {pos, .. } => { Some(pos) },
            Task::Moving {path, .. } => { Some(&path[0]) },
            Task::Travel {path, .. } => { Some(&path[0]) },
            Task::WaitDuration { .. } => { None },
            Task::WaitInfinite { .. } => { None },
        }
    }
    pub fn get_velocity(&self) -> LinearVelocity {
        let vel_zero = LinearVelocity::kilometers_per_hour(0.0);
        match self {
            Task::Stationary {..} => { vel_zero },
            Task::Moving {velocity, .. } => { *velocity },
            Task::Travel {velocity, .. } => { *velocity },
            Task::WaitDuration { .. } => { vel_zero },
            Task::WaitInfinite { .. } => { vel_zero },
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

    pub fn is_path_empty(&self) -> bool {
        if let Some(path) = self.get_path() {
            if path.is_empty() { return true }
            return false
        }
        true
    }
}
