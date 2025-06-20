use crate::{movement_module::{movement::MovementInputs, pose::Pose}, units::{angular_velocity::AngularVelocity, duration::Duration, linear_velocity::LinearVelocity}};

pub trait IsMovement {
    fn calculate_inputs_for_target(&self, current_pose: Pose, target_pose: Pose) -> MovementInputs;
    fn calculate_new_pose_from_inputs(&self, simulation_step: Duration, inputs: MovementInputs, current_pose: Pose, max_velocity: LinearVelocity) -> (Pose, LinearVelocity, AngularVelocity);
}