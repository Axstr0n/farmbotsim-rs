use egui::{Pos2, Vec2};
use serde::{Deserialize, Serialize};
use std::f32::consts::PI;

use crate::{
    cfg::{TOLERANCE_ANGLE, TOLERANCE_DISTANCE}, units::{
        angle::Angle,
        angular_velocity::AngularVelocity,
        duration::Duration,
        length::Length,
        linear_velocity::LinearVelocity,
    }, utilities::{utils::load_json_or_panic, vec2::Vec2Rotate}
};

pub trait IsMovement {
    fn calculate_inputs_for_target(self, position: Pos2, direction: Vec2, target_position: Pos2, target_direction: Option<Vec2>) -> Vec<f32>;
    fn calculate_new_pose_from_inputs(&self, simulation_step: Duration, inputs: Vec<f32>, position: Pos2, direction: Vec2, max_velocity: LinearVelocity) -> (Pos2, Vec2, LinearVelocity, AngularVelocity);
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "params")]
pub enum Movement {
    RombaMovement(RombaMovement),
}
impl IsMovement for Movement {
    fn calculate_inputs_for_target(self, position: Pos2, direction: Vec2, target_position: Pos2, target_direction: Option<Vec2>) -> Vec<f32> {
        match self {
            Movement::RombaMovement(romba) => romba.calculate_inputs_for_target(position, direction, target_position, target_direction),
        }
    }
    fn calculate_new_pose_from_inputs(&self, simulation_step: Duration, inputs: Vec<f32>, position: Pos2, direction: Vec2, max_velocity: LinearVelocity) -> (Pos2, Vec2, LinearVelocity, AngularVelocity) {
        match self {
            Movement::RombaMovement(romba) => romba.calculate_new_pose_from_inputs(simulation_step, inputs, position, direction, max_velocity),
        }
    }
}
impl Movement {
    pub fn from_json_file(file_path: String) -> Self {
        load_json_or_panic(file_path)
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
            max_velocity: LinearVelocity::kilometers_per_hour(10.0),
            max_angular_velocity: AngularVelocity::radians_per_second(0.1),
            wheel_distance: Length::meters(0.2),
            wheel_radius: Length::meters(0.05)
        }
    }
}

impl IsMovement for RombaMovement {
    fn calculate_new_pose_from_inputs(&self, simulation_step: Duration, inputs: Vec<f32>, position: Pos2, direction: Vec2, max_velocity: LinearVelocity) -> (Pos2, Vec2, LinearVelocity, AngularVelocity) {
        if inputs.len() != 2 { assert_eq!(2, inputs.len()) }
        // Clamp if it is not
        let mut m1 = inputs[0].clamp(-1.0, 1.0);
        let mut m2 = inputs[1].clamp(-1.0, 1.0);

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
        let new_direction = direction.rotate(angle).normalized();

        let new_position = position + direction * (v * simulation_step);
        let current_velocity = v;

        (new_position, new_direction, current_velocity, omega)
    }
    
    fn calculate_inputs_for_target(self, position: Pos2, direction: Vec2, target_position: Pos2, target_direction: Option<Vec2>) -> Vec<f32> {
        
        let mut m1 = 0.0;
        let mut m2 = 0.0;
        
        let distance = Length::meters(position.distance(target_position));

        if distance > TOLERANCE_DISTANCE {
            // Compute angle to target
            let direction_to_target = (target_position - position).normalized();
            let angle_to_target = Angle::radians(direction_to_target.angle());
            let angle_of_agent = Angle::radians(direction.angle());

            // Compute angle difference
            let delta_angle_value = (angle_to_target.to_degrees() - angle_of_agent.to_degrees() + 180.0).rem_euclid(360.0) - 180.0;

            // Normalize delta_angle to -1...1 range
            let normalized_delta = delta_angle_value / 180.0;

            // Basic differential drive control for turning and moving forward
            if Angle::degrees(delta_angle_value.abs()) > TOLERANCE_ANGLE {
                // If we need to turn, adjust motor speeds accordingly
                let turn_strength = f32::min(1.0, normalized_delta.abs()/10.0); // Scale turn strength

                if normalized_delta < 0.0 {  // Turn right
                    m1 = turn_strength;
                    m2 = -turn_strength;
                } else {  // Turn left
                    m1 = -turn_strength;
                    m2 = turn_strength;
                }
            } else {
                // Move straight towards the target (no turning needed)
                let input_strength = f32::min(distance.to_base_unit()*0.3, 1.0);  // Scale speed based on distance, up to a maximum of 1
                m1 = input_strength;
                m2 = input_strength;
            }
        }

        // If at the target position
        else if let Some(dir) = target_direction { // a target direction is provided, adjust heading to match target direction
            let angle_of_target = dir.angle() * (180.0 / PI);
            let angle_of_agent = direction.angle() * (180.0 / PI);
            let delta_angle = (angle_of_target - angle_of_agent + 180.0).rem_euclid(360.0) - 180.0;
            
            // Normalize delta_angle to -1...1 range
            let normalized_delta = delta_angle / 180.0;
            
            // Only turn in place when adjusting final heading
            let turn_strength = f32::min(1.0, normalized_delta.abs()*0.5);
            
            if normalized_delta < 0.0 { // Turn right
                m1 = turn_strength;
                m2 = -turn_strength;
            } else { // Turn left
                m1 = -turn_strength;
                m2 = turn_strength;
            }
        }
    
        // let threshold = 1e-4;
        // if (-threshold..threshold).contains(&m1) {
        //     m1 = 0.0;
        // }
        // if (-threshold..threshold).contains(&m2) {
        //     m2 = 0.0;
        // }
        
        vec![m1, m2]
    }
}
