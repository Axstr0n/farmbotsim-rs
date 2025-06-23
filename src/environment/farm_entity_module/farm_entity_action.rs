use serde::{Deserialize, Serialize};

use crate::{
    units::{duration::Duration, linear_velocity::LinearVelocity, power::Power}
};


/// Represents an action that can be performed on farm entity.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FarmEntityAction {
    /// A point action with a fixed duration and power (stationary action).
    Point {
        #[serde(rename = "action_name")]
        action_name: String,
        #[serde(rename = "duration")]
        duration: Duration,
        #[serde(rename = "power")]
        power: Power,
    },
    /// A line action representing movement along a path with velocity and power (moving action).
    Line {
        #[serde(rename = "action_name")]
        action_name: String,
        #[serde(rename = "velocity")]
        velocity: LinearVelocity,
        #[serde(rename = "power")]
        power: Power,
    },
    /// A wait action with a specified duration.
    Wait {
        #[serde(rename = "action_name")]
        action_name: String,
        #[serde(rename = "duration")]
        duration: Duration,
    }
}
impl FarmEntityAction {
    /// Returns a default point action with preset duration and power.
    pub fn default_point() -> Self {
        FarmEntityAction::Point {
            action_name: "point".to_string(),
            duration: Duration::seconds(40.0),
            power: Power::watts(100.0),
        }
    }
    /// Returns a default line action with preset velocity and power.
    pub fn default_line() -> Self {
        FarmEntityAction::Line {
            action_name: "line".to_string(),
            velocity: LinearVelocity::kilometers_per_hour(2.0),
            power: Power::watts(150.0),
        }
    }
    /// Returns a default wait action with preset duration.
    pub fn default_wait() -> Self {
        FarmEntityAction::Wait {
            action_name: "wait".to_string(),
            duration: Duration::minutes(5.0),
        }
    }
}