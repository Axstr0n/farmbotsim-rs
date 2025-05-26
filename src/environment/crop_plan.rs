use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::utilities::{duration::Duration, power::Power, velocity::Velocity};
use crate::cfg::{DEFAULT_LINE_CROP_PLAN_PATH, DEFAULT_POINT_CROP_PLAN_PATH};


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CropAction {
    Point {
        #[serde(rename = "action_name")]
        action_name: String,
        #[serde(rename = "duration")]
        duration: Duration,
        #[serde(rename = "power", default)]
        power: Power,
    },
    Line {
        #[serde(rename = "action_name")]
        action_name: String,
        #[serde(rename = "velocity")]
        velocity: Velocity,
        #[serde(rename = "power", default)]
        power: Power,
    },
    Wait {
        #[serde(rename = "action_name")]
        action_name: String,
        #[serde(rename = "duration")]
        duration: Duration,
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CropPlan {
    #[serde(rename = "name")]
    pub crop_name: String,
    #[serde(rename = "type")]
    pub type_ : String,
    #[serde(rename = "plan")]
    schedule: Vec<CropAction>,
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