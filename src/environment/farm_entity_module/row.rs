use egui::Pos2;

use crate::environment::farm_entity_module::{
    farm_entity_action::{FarmEntityAction, FarmEntityActionInstance},
    farm_entity_plan::FarmEntityPlan
};


#[derive(PartialEq, Debug, Clone)]
pub struct Row {
    pub id: u32,
    pub field_id: u32,
    pub path: Vec<Pos2>,
    pub stage: Option<u32>,
    pub plan: FarmEntityPlan,
    pub stages: Vec<FarmEntityActionInstance>,
}

impl Row {
    pub fn new(id: u32, field_id: u32, path: Vec<Pos2>, plan: FarmEntityPlan) -> Self {
        let mut stages = vec![];
        for action in &plan.schedule {
            let data = match action {
                FarmEntityAction::Line { action_name, velocity, power, .. } => {
                    FarmEntityActionInstance::line(id, field_id, path.clone(), *velocity, *power, action_name.clone())
                }
                FarmEntityAction::Wait { duration, .. } => {
                    FarmEntityActionInstance::wait(id, *duration)
                },
                _ => {
                    panic!("Can't have point action for line crop")
                }
            };
            stages.push(data);
        }
        Self {
            id,
            field_id,
            path,
            stage: None,
            plan,
            stages,
        }
    }
}