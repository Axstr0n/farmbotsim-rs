use egui::Pos2;

use crate::environment::crop_plan::CropAction;
use super::crop_plan::{CropActionInstance, CropPlan};

#[derive(PartialEq, Debug, Clone)]
pub struct Row {
    pub id: u32,
    pub field_id: u32,
    pub path: Vec<Pos2>,
    pub stage: Option<u32>,
    pub plan: CropPlan,
    pub stages: Vec<CropActionInstance>,
}

impl Row {
    pub fn new(id: u32, field_id: u32, path: Vec<Pos2>, plan: CropPlan) -> Self {
        let mut stages = vec![];
        for action in &plan.schedule {
            let data = match action {
                CropAction::Line { action_name, velocity, power, .. } => {
                    CropActionInstance::line(id, field_id, path.clone(), *velocity, *power, action_name.clone())
                }
                CropAction::Wait { duration, .. } => {
                    CropActionInstance::wait(id, *duration)
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
    pub fn increment_stage(&mut self) -> bool {
        let next_stage = self.next_stage_val();
        if let Some(next_stage_val) = next_stage {
            self.stage = Some(next_stage_val);
            true
        } else {
            false
        }
    }
    fn next_stage_val(&self) -> Option<u32> {
        match self.stage {
            Some(val) => {
                let mut next_stage = val + 1;
                if next_stage >= self.stages.len() as u32 {
                    if let Some(cycle) = self.plan.cycle {
                        next_stage = cycle;
                    } else {
                        return None
                    }
                }
                Some(next_stage)
            },
            None => {
                Some(0)
            }
        }
    }
    pub fn get_next_action_instance(&self) -> Option<CropActionInstance> {
        let next_stage_val = self.next_stage_val();
        next_stage_val.map(|next_stage_val| self.stages[next_stage_val as usize].clone())
    }
}