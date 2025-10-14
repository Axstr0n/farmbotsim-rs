use std::collections::VecDeque;

use crate::{
    movement_module::pose::Pose,
    units::{duration::Duration, linear_velocity::LinearVelocity, power::Power},
};

/// Represents the intention of a task.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Intent {
    /// Performing work-related tasks
    Work,
    /// Charging at a station
    Charge,
    /// Waiting in a queue for a station slot
    Queue,
    /// Idle
    Idle,
}

/// Represents different types of tasks an agent can perform, including stationary/moving work, travel, and waiting.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Task {
    /// A work stationary task at a specific pose, with duration and associated metadata.
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
    /// A work moving task along a path at a specified velocity, with associated metadata.
    Moving {
        id: u32,
        path: VecDeque<Pose>,
        velocity: LinearVelocity,
        intent: Intent,
        field_id: u32,
        farm_entity_id: u32,
        power: Power,
        info: String,
    },
    /// A travel task representing movement along a path.
    Travel {
        path: VecDeque<Pose>,
        velocity: LinearVelocity,
        intent: Intent,
    },
    /// A waiting task for a specified duration with an intent.
    WaitDuration { duration: Duration, intent: Intent },
    /// A waiting task of indefinite length with an intent.
    WaitInfinite { intent: Intent },
}

impl Task {
    /// Creates a travel task along the given path with specified velocity and intent.
    pub fn travel(path: Vec<Pose>, velocity: LinearVelocity, intent: Intent) -> Self {
        Task::Travel {
            path: VecDeque::from(path),
            velocity,
            intent,
        }
    }
    /// Creates a wait task for a fixed duration with the specified intent.
    pub fn wait_duration(duration: Duration, intent: Intent) -> Self {
        Task::WaitDuration { duration, intent }
    }
    /// Creates a wait task with infinite duration with the specified intent.
    pub fn wait_infinite(intent: Intent) -> Self {
        Task::WaitInfinite { intent }
    }

    /// Returns the task's unique ID if applicable (stationary or moving tasks).
    pub fn get_id(&self) -> Option<&u32> {
        match self {
            Task::Stationary { id, .. } => Some(id),
            Task::Moving { id, .. } => Some(id),
            Task::Travel { .. } => None,
            Task::WaitDuration { .. } => None,
            Task::WaitInfinite { .. } => None,
        }
    }
    /// Returns the associated farm entity ID if applicable.
    pub fn get_farm_entity_id(&self) -> Option<u32> {
        match self {
            Task::Stationary { farm_entity_id, .. } => Some(*farm_entity_id),
            Task::Moving { farm_entity_id, .. } => Some(*farm_entity_id),
            _ => None,
        }
    }
    /// Returns the path of poses associated with the task if any.
    pub fn get_path(&self) -> Option<VecDeque<Pose>> {
        match self {
            Task::Stationary { pose, .. } => Some(VecDeque::from(vec![pose.clone()])),
            Task::Moving { path, .. } => Some(path.clone()),
            Task::Travel { path, .. } => Some(path.clone()),
            Task::WaitDuration { .. } => None,
            Task::WaitInfinite { .. } => None,
        }
    }
    /// Returns the first pose of the task's path or stationary position.
    pub fn get_first_pose(&self) -> Option<&Pose> {
        match self {
            Task::Stationary { pose, .. } => Some(pose),
            Task::Moving { path, .. } => Some(&path[0]),
            Task::Travel { path, .. } => Some(&path[0]),
            Task::WaitDuration { .. } => None,
            Task::WaitInfinite { .. } => None,
        }
    }
    /// Returns the velocity associated with the task (zero for stationary and waiting tasks).
    pub fn get_velocity(&self) -> LinearVelocity {
        match self {
            Task::Stationary { .. } => LinearVelocity::ZERO,
            Task::Moving { velocity, .. } => *velocity,
            Task::Travel { velocity, .. } => *velocity,
            Task::WaitDuration { .. } => LinearVelocity::ZERO,
            Task::WaitInfinite { .. } => LinearVelocity::ZERO,
        }
    }
    /// Returns a reference to the intent of the task.
    pub fn get_intent(&self) -> &Intent {
        match self {
            Task::Stationary { intent, .. } => intent,
            Task::Moving { intent, .. } => intent,
            Task::Travel { intent, .. } => intent,
            Task::WaitDuration { intent, .. } => intent,
            Task::WaitInfinite { intent, .. } => intent,
        }
    }
    /// Returns true if the task is a work-related task (stationary or moving).
    pub fn is_work(&self) -> bool {
        match self {
            Task::Stationary { .. } => true,
            Task::Moving { .. } => true,
            Task::Travel { .. } => false,
            Task::WaitDuration { .. } => false,
            Task::WaitInfinite { .. } => false,
        }
    }
    /// Returns true if the task is a travel task.
    pub fn is_travel(&self) -> bool {
        matches!(self, Task::Travel { .. })
    }
    /// Returns true if the task is a waiting task (either fixed duration or infinite).
    pub fn is_wait(&self) -> bool {
        matches!(self, Task::WaitDuration { .. } | Task::WaitInfinite { .. })
    }
    /// Returns true if the task has a charge intent.
    pub fn is_charge_intent(&self) -> bool {
        matches!(
            self,
            Task::WaitDuration {
                intent: Intent::Charge,
                ..
            } | Task::WaitInfinite {
                intent: Intent::Charge,
                ..
            }
        )
    }
    /// Returns true if the task's path is empty or non-existent.
    pub fn is_path_empty(&self) -> bool {
        if let Some(path) = self.get_path() {
            if path.is_empty() {
                return true;
            }
            return false;
        }
        true
    }
}
