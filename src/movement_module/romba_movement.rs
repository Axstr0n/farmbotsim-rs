use egui::Vec2;
use serde::{Deserialize, Serialize};

use crate::{
    cfg::{TOLERANCE_ANGLE, TOLERANCE_DISTANCE},
    movement_module::{is_movement::IsMovement, movement::MovementInputs, pose::Pose},
    units::{angle::Angle, angular_velocity::{AngularVelocity, AngularVelocityUnit}, duration::Duration, length::{Length, LengthUnit}, linear_velocity::{LinearVelocity, LinearVelocityUnit}}
};


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RombaMovementInputs {
    pub left: f32,
    pub right: f32,
}

impl RombaMovementInputs {
    pub fn new(left: f32, right: f32) -> Self {
        Self {
            left, right
        }
    }
    pub fn clamped(self) -> Self {
        Self {
            left: self.left.clamp(-1.0, 1.0),
            right: self.right.clamp(-1.0, 1.0),
        }
    }

    pub fn as_vec(&self) -> Vec<f32> {
        vec![self.left, self.right]
    }

    pub fn from_vec(vec: Vec<f32>) -> Option<Self> {
        if vec.len() != 2 {
            None
        } else {
            Some(Self { left: vec[0], right: vec[1] })
        }
    }
}

#[derive(Clone, PartialEq, Copy, Debug, Serialize, Deserialize)]
pub struct RombaMovement {
    pub max_velocity: LinearVelocity,
    pub max_angular_velocity: AngularVelocity,
    pub wheel_distance: Length,
    pub wheel_radius: Length
}
impl Default for RombaMovement {
    fn default() -> Self {
        Self {
            max_velocity: LinearVelocity { value: 10.0, unit: LinearVelocityUnit::KilometersPerHour },
            max_angular_velocity: AngularVelocity { value: 0.1, unit: AngularVelocityUnit::RadiansPerSecond },
            wheel_distance: Length { value: 0.2, unit: LengthUnit::Meters },
            wheel_radius: Length { value: 0.05, unit: LengthUnit::Meters },
        }
    }
}

impl IsMovement for RombaMovement {
    fn calculate_new_pose_from_inputs(&self, simulation_step: Duration, inputs: MovementInputs, current_pose: Pose, max_velocity: LinearVelocity) -> (Pose, LinearVelocity, AngularVelocity) {
        match inputs {
            MovementInputs::Romba(romba_inputs) => {
                // Clamp if it is not
                let romba_inputs = romba_inputs.clamped();
                let mut m1 = romba_inputs.left;
                let mut m2 = romba_inputs.right;

                let max_velocity = if max_velocity > self.max_velocity {self.max_velocity} else {max_velocity};

                let mut v_left = m1 * max_velocity;
                let mut v_right = m2 * max_velocity;

                let mut omega = (v_right - v_left) / self.wheel_distance;

                // Clamp omega if needed
                if omega.to_base_unit().abs() > self.max_angular_velocity.to_base_unit() {
                    let scale = self.max_angular_velocity.to_base_unit() / omega.to_base_unit().abs();
                    m1 *= scale;
                    m2 *= scale;

                    v_left = m1 * max_velocity;
                    v_right = m2 * max_velocity;
                    omega = (v_right - v_left) / self.wheel_distance;
                }

                let v = (v_left + v_right) / 2.0;
                let new_orientation = Angle::degrees((current_pose.orientation + omega * simulation_step).to_degrees().rem_euclid(360.0));

                let direction = Vec2::new(new_orientation.to_radians().cos(), new_orientation.to_radians().sin());
                let new_position = current_pose.position + direction * (v * simulation_step);
                let current_velocity = v;

                let new_pose = Pose::new(new_position, new_orientation);

                (new_pose, current_velocity, omega)
            },
            // _ => {
            //     let msg = "Invalid inputs for RombaMovement";
            //     log_error_and_panic(msg);
            // },
        }
    }
    
    fn calculate_inputs_for_target(&self, current_pose: Pose, target_pose: Pose) -> MovementInputs {
        let position_error = current_pose.position.distance(target_pose.position);
        let desired_orientation = if position_error > TOLERANCE_DISTANCE.to_base_unit() {
            // Face the next position when far away
            let direction = (target_pose.position - current_pose.position).normalized();
            Angle::radians(direction.angle())
        } else {
            // Close enough, face the final orientation
            target_pose.orientation
        };

        let should_turn = !desired_orientation.is_close_to(current_pose.orientation, TOLERANCE_ANGLE);
        let should_move = position_error > TOLERANCE_DISTANCE.to_base_unit();

        let (left, right) = match (should_move, should_turn) {
            (true, true) => {
                // Need to rotate toward the direction of the next target position
                let desired_direction = (target_pose.position - current_pose.position).normalized();
                let angle_to_target = Angle::radians(desired_direction.angle());
                Self::turning_inputs(current_pose.orientation, angle_to_target)
            }
            (true, false) => {
                // Drive straight toward the target
                let forward_strength = (position_error * 0.3).clamp(0.0, 1.0);
                (forward_strength, forward_strength)
            }
            (false, true) => {
                // We're close to the target position, now match final orientation
                Self::turning_inputs(current_pose.orientation, target_pose.orientation)
            }
            (false, false) => (0.0, 0.0), // Done
        };

        MovementInputs::Romba(RombaMovementInputs::new(left, right))
    }
}

impl RombaMovement {
    fn turning_inputs(current: Angle, target: Angle) -> (f32, f32) {
        let delta = (target.to_degrees() - current.to_degrees() + 180.0).rem_euclid(360.0) - 180.0;
        let norm = delta / 180.0;
        let strength = (norm.abs() * 0.1).clamp(0.0, 1.0);

        if norm < 0.0 {
            (strength, -strength) // Turn right
        } else {
            (-strength, strength) // Turn left
        }
    }
}