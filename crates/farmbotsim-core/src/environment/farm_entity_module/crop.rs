use egui::Pos2;

use crate::{
    environment::farm_entity_module::{
        farm_entity_action::FarmEntityAction,
        farm_entity_action_instance::FarmEntityActionInstance, farm_entity_plan::FarmEntityPlan,
    },
    logger::log_error_and_panic,
};

/// Represents a point farm entity
#[derive(PartialEq, Debug, Clone)]
pub struct Crop {
    pub id: u32,
    pub field_id: u32,
    pub row_id: u32,
    pub position: Pos2,
    pub stage: Option<u32>,
    pub plan: FarmEntityPlan,
    pub stages: Vec<FarmEntityActionInstance>,
}

impl Crop {
    /// Creates a new `Crop` with explicit parameters.
    /// Panics if the plan contains line actions (unsupported for point crops).
    pub fn new(id: u32, field_id: u32, row_id: u32, position: Pos2, plan: FarmEntityPlan) -> Self {
        let mut stages = vec![];
        for action in &plan.schedule {
            let data = match action {
                FarmEntityAction::Point {
                    action_name,
                    duration,
                    power,
                    ..
                } => FarmEntityActionInstance::point(
                    id,
                    field_id,
                    row_id,
                    position,
                    *duration,
                    *power,
                    action_name.clone(),
                ),
                FarmEntityAction::Wait { duration, .. } => {
                    FarmEntityActionInstance::wait(id, *duration)
                }
                _ => {
                    let msg = "Can't have line action for point crop";
                    log_error_and_panic(msg)
                }
            };
            stages.push(data);
        }
        Self {
            id,
            field_id,
            row_id,
            position,
            stage: None,
            plan,
            stages,
        }
    }
}
