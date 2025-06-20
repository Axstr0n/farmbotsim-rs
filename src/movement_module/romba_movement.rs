use std::f32::consts::PI;
use serde::{Deserialize, Serialize};

use crate::{cfg::{TOLERANCE_ANGLE, TOLERANCE_DISTANCE}, movement_module::{is_movement::IsMovement, movement::MovementInputs, pose::Pose}, units::{angle::Angle, angular_velocity::{AngularVelocity, AngularVelocityUnit}, duration::Duration, length::{Length, LengthUnit}, linear_velocity::{LinearVelocity, LinearVelocityUnit}}, utilities::vec2::Vec2Rotate};


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
                let angle = omega * simulation_step;
                let new_direction = current_pose.direction.rotate(angle).normalized();

                let new_position = current_pose.position + current_pose.direction * (v * simulation_step);
                let current_velocity = v;

                let new_pose = Pose::new(new_position, new_direction);

                (new_pose, current_velocity, omega)
            },
            // _ => {
            //     let msg = "Invalid inputs for RombaMovement";
            //     log_error_and_panic(msg);
            // },
        }
    }
    
    fn calculate_inputs_for_target(&self, current_pose: Pose, target_pose: Pose) -> MovementInputs {
        
        let distance = Length::meters(current_pose.position.distance(target_pose.position));

        let (m1, m2) = if distance > TOLERANCE_DISTANCE {
            // Compute angle to target
            let direction_to_target = (target_pose.position - current_pose.position).normalized();
            let angle_to_target = Angle::radians(direction_to_target.angle());
            let angle_of_agent = Angle::radians(current_pose.direction.angle());

            // Compute angle difference
            let delta_angle_value = (angle_to_target.to_degrees() - angle_of_agent.to_degrees() + 180.0).rem_euclid(360.0) - 180.0;

            // Normalize delta_angle to -1...1 range
            let normalized_delta = delta_angle_value / 180.0;

            // Basic differential drive control for turning and moving forward
            if Angle::degrees(delta_angle_value.abs()) > TOLERANCE_ANGLE {
                // If we need to turn, adjust motor speeds accordingly
                let turn_strength = f32::min(1.0, normalized_delta.abs()/10.0); // Scale turn strength

                if normalized_delta < 0.0 {  // Turn right
                    (turn_strength, -turn_strength)
                } else {  // Turn left
                    (-turn_strength, turn_strength)
                }
            } else {
                // Move straight towards the target (no turning needed)
                let input_strength = f32::min(distance.to_base_unit()*0.3, 1.0);  // Scale speed based on distance, up to a maximum of 1
                (input_strength, input_strength)
            }
        }

        // If at the target position
        else {
            let angle_of_target = target_pose.direction.angle() * (180.0 / PI);
            let angle_of_agent = current_pose.direction.angle() * (180.0 / PI);
            let delta_angle = (angle_of_target - angle_of_agent + 180.0).rem_euclid(360.0) - 180.0;
            
            // Normalize delta_angle to -1...1 range
            let normalized_delta = delta_angle / 180.0;
            
            // Only turn in place when adjusting final heading
            let turn_strength = f32::min(1.0, normalized_delta.abs()*0.5);
            
            if normalized_delta < 0.0 { // Turn right
                (turn_strength, -turn_strength)
            } else { // Turn left
                (-turn_strength, turn_strength)
            }
        };
    
        // let threshold = 1e-4;
        // if (-threshold..threshold).contains(&m1) {
        //     m1 = 0.0;
        // }
        // if (-threshold..threshold).contains(&m2) {
        //     m2 = 0.0;
        // }
        let romba_inputs = RombaMovementInputs::new(m1, m2);
        MovementInputs::Romba(romba_inputs)
    }
}