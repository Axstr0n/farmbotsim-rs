use serde::{Deserialize, Serialize};

use crate::{cfg::{DEFAULT_LINE_FARM_ENTITY_PLAN_PATH, DEFAULT_POINT_FARM_ENTITY_PLAN_PATH}, environment::farm_entity_module::farm_entity_action::FarmEntityAction, utilities::utils::load_json_or_panic};


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FarmEntityPlan {
    #[serde(rename = "name")]
    pub crop_name: String,
    #[serde(rename = "type")]
    pub type_ : String,
    #[serde(rename = "cycle")]
    pub cycle : Option<u32>,
    #[serde(rename = "plan")]
    pub schedule: Vec<FarmEntityAction>,
}

impl FarmEntityPlan {
    pub fn from_json_file(path: &str) -> Self {
        load_json_or_panic(path)
    }
    pub fn default_point() -> Self {
        Self::from_json_file(DEFAULT_POINT_FARM_ENTITY_PLAN_PATH)
    }
    pub fn default_line() -> Self {
        Self::from_json_file(DEFAULT_LINE_FARM_ENTITY_PLAN_PATH)
    }
}