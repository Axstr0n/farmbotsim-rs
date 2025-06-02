use egui::Pos2;

use super::crop_plan::{CropAction, CropActionInstance, CropPlan};


#[derive(PartialEq, Debug, Clone)]
pub struct Crop {
    pub id: u32,
    pub field_id: u32,
    pub row_id: u32,
    pub position: Pos2,
    pub stage: Option<u32>,
    pub plan: CropPlan,
    pub stages: Vec<CropActionInstance>,
}

impl Crop {
    pub fn new(id: u32, field_id: u32, row_id: u32, position: Pos2, plan: CropPlan) -> Self {
        let mut stages = vec![];
        for action in &plan.schedule {
            let data = match action {
                CropAction::Point { action_name, duration, power ,..} => {
                    CropActionInstance::point(id, field_id, row_id, position, *duration, *power, action_name.clone())
                },
                CropAction::Wait { duration, .. } => {
                    CropActionInstance::wait(id, *duration)
                },
                _ => {
                    panic!("Can't have line action for point crop")
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
    // todo!() move increment_stage, next_stage_val, get_next_action_instance to trait and impl for crop and row
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
