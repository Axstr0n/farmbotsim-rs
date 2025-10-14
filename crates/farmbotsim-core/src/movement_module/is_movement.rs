use crate::{
    movement_module::{movement::MovementInputs, pose::Pose},
    units::{
        angular_velocity::AngularVelocity, duration::Duration, linear_velocity::LinearVelocity,
    },
};

/// A trait defining movement behavior for navigating between poses.
pub trait IsMovement {
    /// Calculates the required movement inputs to reach a target pose from the current pose.
    fn calculate_inputs_for_target(
        &self,
        current_pose: &Pose,
        target_pose: &Pose,
    ) -> MovementInputs;
    /// Computes the new pose after applying movement inputs over a time step.
    fn calculate_new_pose_from_inputs(
        &self,
        simulation_step: Duration,
        inputs: MovementInputs,
        current_pose: Pose,
        max_velocity: LinearVelocity,
    ) -> (Pose, LinearVelocity, AngularVelocity);
}
