use std::{fs, path::Path};
use serde::{Deserialize, Serialize};

use crate::{cfg::{DEFAULT_LINE_FARM_ENTITY_PLAN_PATH, DEFAULT_POINT_FARM_ENTITY_PLAN_PATH}, environment::farm_entity_module::farm_entity_action::FarmEntityAction};


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
        let path = Path::new(path);
        let json_str = fs::read_to_string(path).expect("File not found");
        let plan: FarmEntityPlan = serde_json::from_str(&json_str).expect("Can't deserialize to FarmEntityPlan.");
        plan
    }
    pub fn default_point() -> Self {
        Self::from_json_file(DEFAULT_POINT_FARM_ENTITY_PLAN_PATH)
    }
    pub fn default_line() -> Self {
        Self::from_json_file(DEFAULT_LINE_FARM_ENTITY_PLAN_PATH)
    }
}