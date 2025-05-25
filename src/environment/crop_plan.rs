use serde::{Deserialize, Serialize};

use crate::utilities::{duration::Duration, power::Power, velocity::Velocity};



#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
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
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CropPlan {
    #[serde(rename = "name")]
    crop_name: String,
    #[serde(rename = "plan")]
    schedule: Vec<CropAction>,
}