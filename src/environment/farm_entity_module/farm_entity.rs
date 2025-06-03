use crate::environment::farm_entity_module::{
    crop::Crop,
    farm_entity_action::FarmEntityActionInstance,
    farm_stages::FarmStages, row::Row
};


#[derive(Debug, Clone, PartialEq)]
pub enum FarmEntity {
    Crop(Crop),
    Row(Row),
}

impl FarmEntity {
    pub fn get_id(&self) -> u32 {
        match self {
            FarmEntity::Crop(crop) => crop.id,
            FarmEntity::Row(row) => row.id,
        }
    }
}

impl FarmStages for FarmEntity {
    fn get_stage(&self) -> Option<u32> {
        match self {
            FarmEntity::Crop(crop) => crop.get_stage(),
            FarmEntity::Row(row) => row.get_stage(),
        }
    }

    fn set_stage(&mut self, stage: Option<u32>) {
        match self {
            FarmEntity::Crop(crop) => crop.set_stage(stage),
            FarmEntity::Row(row) => row.set_stage(stage),
        }
    }

    fn stages(&self) -> &Vec<FarmEntityActionInstance> {
        match self {
            FarmEntity::Crop(crop) => crop.stages(),
            FarmEntity::Row(row) => row.stages(),
        }
    }

    fn cycle(&self) -> Option<u32> {
        match self {
            FarmEntity::Crop(crop) => crop.cycle(),
            FarmEntity::Row(row) => row.cycle(),
        }
    }
}
