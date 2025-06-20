use crate::{
    movement_module::pose::Pose, units::{duration::Duration, linear_velocity::LinearVelocity, power::Power}
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
        pose: Pose,
        duration: Duration,
        intent: Intent,
        farm_entity_id: u32,
        field_id: u32,
        line_id: u32,
        power: Power,
        info: String,
    },
    Moving {
        id: u32,
        path: Vec<Pose>,
        velocity: LinearVelocity,
        intent: Intent,
        field_id: u32,
        farm_entity_id: u32,
        power: Power,
        info: String,
    },
    Travel {
        path: Vec<Pose>,
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
    pub fn travel(path: Vec<Pose>, velocity: LinearVelocity, intent: Intent) -> Self {
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
    pub fn get_farm_entity_id(&self) -> Option<u32> {
        match self {
            Task::Stationary { farm_entity_id,.. } => Some(*farm_entity_id),
            Task::Moving { farm_entity_id,.. } => Some(*farm_entity_id),
            _ => None,
        }
    }
    pub fn get_path(&self) -> Option<Vec<Pose>> {
        match self {
            Task::Stationary {pose, .. } => { Some(vec![pose.clone()]) },
            Task::Moving {path, .. } => { Some(path.clone()) },
            Task::Travel {path, .. } => { Some(path.clone()) },
            Task::WaitDuration { .. } => { None },
            Task::WaitInfinite { .. } => { None },
        }
    }
    pub fn get_first_pose(&self) -> Option<&Pose> {
        match self {
            Task::Stationary {pose, .. } => { Some(pose) },
            Task::Moving {path, .. } => { Some(&path[0]) },
            Task::Travel {path, .. } => { Some(&path[0]) },
            Task::WaitDuration { .. } => { None },
            Task::WaitInfinite { .. } => { None },
        }
    }
    pub fn get_velocity(&self) -> LinearVelocity {
        match self {
            Task::Stationary {..} => { LinearVelocity::ZERO },
            Task::Moving {velocity, .. } => { *velocity },
            Task::Travel {velocity, .. } => { *velocity },
            Task::WaitDuration { .. } => { LinearVelocity::ZERO },
            Task::WaitInfinite { .. } => { LinearVelocity::ZERO },
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
