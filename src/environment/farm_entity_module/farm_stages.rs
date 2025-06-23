use crate::environment::farm_entity_module::{
    crop::Crop,
    farm_entity_action_instance::FarmEntityActionInstance,
    row::Row
};

/// Trait defining common behavior for farm entities with staged actions.
pub trait FarmStages {
    /// Returns the current stage index, if any.
    fn get_stage(&self) -> Option<u32>;

    /// Sets the current stage index.
    fn set_stage(&mut self, stage: Option<u32>);

    /// Returns a reference to the vector of action instances representing stages.
    fn stages(&self) -> &Vec<FarmEntityActionInstance>;

    /// Returns an optional cycle stage index for repeating the schedule.
    fn cycle(&self) -> Option<u32>;

    /// Returns the next stage index, accounting for cycles.
    fn next_stage_val(&self) -> Option<u32> {
        match self.get_stage() {
            Some(val) => {
                let mut next_stage = val + 1;
                if next_stage >= self.stages().len() as u32 {
                    if let Some(cycle) = self.cycle() {
                        next_stage = cycle;
                    } else {
                        return None;
                    }
                }
                Some(next_stage)
            }
            None => Some(0),
        }
    }

    /// Advances to the next stage, returning whether the stage was updated.
    fn increment_stage(&mut self) -> bool {
        if let Some(next_stage_val) = self.next_stage_val() {
            self.set_stage(Some(next_stage_val));
            true
        } else {
            false
        }
    }

    /// Gets the next action instance in the schedule, if any.
    fn get_next_action_instance(&self) -> Option<FarmEntityActionInstance> {
        self.next_stage_val()
            .map(|val| self.stages()[val as usize].clone())
    }
}

impl FarmStages for Row {
    fn get_stage(&self) -> Option<u32> {
        self.stage
    }

    fn set_stage(&mut self, stage: Option<u32>) {
        self.stage = stage;
    }

    fn stages(&self) -> &Vec<FarmEntityActionInstance> {
        &self.stages
    }

    fn cycle(&self) -> Option<u32> {
        self.plan.cycle
    }
}

impl FarmStages for Crop {
    fn get_stage(&self) -> Option<u32> {
        self.stage
    }

    fn set_stage(&mut self, stage: Option<u32>) {
        self.stage = stage;
    }

    fn stages(&self) -> &Vec<FarmEntityActionInstance> {
        &self.stages
    }

    fn cycle(&self) -> Option<u32> {
        self.plan.cycle
    }
}