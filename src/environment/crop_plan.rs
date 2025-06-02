use std::{fs, path::Path};

use egui::Pos2;
use serde::{Deserialize, Serialize};


use crate::{cfg::{DEFAULT_LINE_CROP_PLAN_PATH, DEFAULT_POINT_CROP_PLAN_PATH}, task_module::task::{Intent, Task}, units::{duration::Duration, linear_velocity::LinearVelocity, power::Power}};

use super::{crop::Crop, row::Row};


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CropAction {
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
pub enum CropActionInstance {
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
impl CropActionInstance {
    pub fn point(id: u32, field_id: u32, line_id: u32, pos: Pos2, duration: Duration, power: Power, action_name: String) -> Self {
        Self::Point { id, field_id, line_id, pos, duration, power, action_name }
    }
    pub fn line(id: u32, field_id: u32, path: Vec<Pos2>, velocity: LinearVelocity, power: Power, action_name: String) -> Self {
        Self::Line { id, field_id, path, velocity, power, action_name }
    }
    pub fn wait(id: u32, duration: Duration) -> Self {
        Self::Wait { id, duration, action_name: "waiting".to_string() }
    }
    // todo!()  make single fn that checks cropactioninstance type and return task (combine to_stationary_task and to_moving_task)
    pub fn to_stationary_task(&self, task_id: u32) -> Option<Task> {
        if let CropActionInstance::Point { id, field_id, line_id, pos,  duration, power, action_name } = self {
            Some(Task::Stationary {
                id: task_id,
                pos: *pos,
                duration: *duration,
                intent: Intent::Work,
                crop_id: *id,
                field_id: *field_id,
                line_id: *line_id,
                power: *power,
                info: action_name.clone(),
            })
        } else {
            None
        } 
    }
    pub fn to_moving_task(&self, task_id: u32) -> Option<Task> {
        if let CropActionInstance::Line { id, field_id, path, velocity, power , action_name} = self {
            Some(Task::Moving {
                id: task_id,
                path: path.clone(),
                velocity: *velocity,
                intent: Intent::Work,
                field_id: *field_id,
                line_id: *id,
                power: *power,
                info: action_name.clone(),
            })
        } else {
            None
        }
    }

}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CropPlan {
    #[serde(rename = "name")]
    pub crop_name: String,
    #[serde(rename = "type")]
    pub type_ : String,
    #[serde(rename = "cycle")]
    pub cycle : Option<u32>,
    #[serde(rename = "plan")]
    pub schedule: Vec<CropAction>,
}

impl CropPlan {
    pub fn from_json_file(path: &str) -> Self {
        let path = Path::new(path);
        let json_str = fs::read_to_string(path).expect("File not found");
        let plan: CropPlan = serde_json::from_str(&json_str).expect("Can't deserialize crop plan.");
        plan
    }
    pub fn default_point() -> Self {
        Self::from_json_file(DEFAULT_POINT_CROP_PLAN_PATH)
    }
    pub fn default_line() -> Self {
        Self::from_json_file(DEFAULT_LINE_CROP_PLAN_PATH)
    }
}