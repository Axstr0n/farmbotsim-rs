use serde::{Deserialize, Serialize};

use crate::{
    cfg::{DEFAULT_LINE_FARM_ENTITY_PLAN_PATH, DEFAULT_POINT_FARM_ENTITY_PLAN_PATH},
    environment::farm_entity_module::farm_entity_action::FarmEntityAction,
    utilities::utils::load_json_or_panic,
};

/// Represents a plan for a farm entity, detailing its crop type, action cycle, and scheduled actions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FarmEntityPlan {
    #[serde(rename = "name")]
    pub crop_name: String,
    /// Plan type weather is "point" or "line"
    #[serde(rename = "type")]
    pub type_: String,
    /// Represents if the schedule cycles and from which index
    #[serde(rename = "cycle")]
    pub cycle: Option<u32>,
    /// Plan of actions
    #[serde(rename = "plan")]
    pub schedule: Vec<FarmEntityAction>,
}

impl FarmEntityPlan {
    /// Loads a farm entity plan from a JSON file, panicking on failure.
    pub fn from_json_file(path: &str) -> Self {
        load_json_or_panic(path)
    }
    /// Loads the default point-type farm entity plan, panicking on failure.
    pub fn default_point() -> Self {
        Self::from_json_file(DEFAULT_POINT_FARM_ENTITY_PLAN_PATH)
    }
    /// Loads the default line-type farm entity plan, panicking on failure.
    pub fn default_line() -> Self {
        Self::from_json_file(DEFAULT_LINE_FARM_ENTITY_PLAN_PATH)
    }
}
