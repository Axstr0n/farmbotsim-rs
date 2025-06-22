use serde::{Deserialize, Serialize};

use crate::{
    movement_module::{is_movement::IsMovement, pose::Pose, romba_movement::{RombaMovement, RombaMovementInputs}}, units::{
        angular_velocity::AngularVelocity,
        duration::Duration,
        linear_velocity::LinearVelocity,
    }, utilities::utils::load_json_or_panic
};

#[derive(Debug, Clone, Copy)]
pub enum MovementInputs {
    Romba(RombaMovementInputs),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "params")]
pub enum Movement {
    RombaMovement(RombaMovement),
}
impl IsMovement for Movement {
    fn calculate_inputs_for_target(&self, current_pose: Pose, target_pose: Pose) -> MovementInputs {
        match self {
            Movement::RombaMovement(romba) => romba.calculate_inputs_for_target(current_pose, target_pose),
        }
    }
    fn calculate_new_pose_from_inputs(&self, simulation_step: Duration, inputs: MovementInputs, current_pose: Pose, max_velocity: LinearVelocity) -> (Pose, LinearVelocity, AngularVelocity) {
        match self {
            Movement::RombaMovement(romba) => romba.calculate_new_pose_from_inputs(simulation_step, inputs, current_pose, max_velocity),
        }
    }
}
impl Movement {
    pub fn from_json_file(file_path: String) -> Self {
        load_json_or_panic(file_path)
    }
    pub fn max_velocity(&self) -> LinearVelocity {
        match  &self {
            Movement::RombaMovement(rm) => rm.max_velocity
        }
    }
}
