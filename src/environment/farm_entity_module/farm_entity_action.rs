use egui::Pos2;
use serde::{Deserialize, Serialize};

use crate::{
    task_module::task::{Intent, Task},
    units::{duration::Duration, linear_velocity::LinearVelocity, power::Power}
};



#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FarmEntityAction {
    Point {
        #[serde(rename = "action_name")]
        action_name: String,
        #[serde(rename = "duration")]
        duration: Duration,
        #[serde(rename = "power")]
        power: Power,
    },
    Line {
        #[serde(rename = "action_name")]
        action_name: String,
        #[serde(rename = "velocity")]
        velocity: LinearVelocity,
        #[serde(rename = "power")]
        power: Power,
    },
    Wait {
        #[serde(rename = "action_name")]
        action_name: String,
        #[serde(rename = "duration")]
        duration: Duration,
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FarmEntityActionInstance {
    Point {
        id: u32, // crop_id
        field_id: u32,
        line_id: u32,
        pos: Pos2,
        duration: Duration,
        power: Power,
        action_name: String,
    },
    Line {
        id: u32, // row_id
        field_id: u32,
        path: Vec<Pos2>,
        velocity: LinearVelocity,
        power: Power,
        action_name: String,
    },
    Wait {
        id: u32, // crop_id / row_id
        duration: Duration,
        action_name: String,
    },
}
impl FarmEntityActionInstance {
    pub fn point(id: u32, field_id: u32, line_id: u32, pos: Pos2, duration: Duration, power: Power, action_name: String) -> Self {
        Self::Point { id, field_id, line_id, pos, duration, power, action_name }
    }
    pub fn line(id: u32, field_id: u32, path: Vec<Pos2>, velocity: LinearVelocity, power: Power, action_name: String) -> Self {
        Self::Line { id, field_id, path, velocity, power, action_name }
    }
    pub fn wait(id: u32, duration: Duration) -> Self {
        Self::Wait { id, duration, action_name: "waiting".to_string() }
    }
    
    pub fn to_task(&self, task_id: u32) -> Option<Task> {
        match self {
            FarmEntityActionInstance::Point { id, field_id, line_id, pos, duration, power, action_name } => {
                Some(Task::Stationary {
                id: task_id,
                pos: *pos,
                duration: *duration,
                intent: Intent::Work,
                farm_entity_id: *id,
                field_id: *field_id,
                line_id: *line_id,
                power: *power,
                info: action_name.clone(),
                })
            },
            FarmEntityActionInstance::Line { id, field_id, path, velocity, power, action_name } => {
                Some(Task::Moving {
                id: task_id,
                path: path.clone(),
                velocity: *velocity,
                intent: Intent::Work,
                field_id: *field_id,
                farm_entity_id: *id,
                power: *power,
                info: action_name.clone(),
                })
            },
            _ => None
        }
    }

}