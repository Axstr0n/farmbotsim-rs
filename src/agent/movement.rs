use egui::{Pos2, Vec2};
use std::f32::consts::PI;

use crate::utilities::utils::{TOLERANCE_ANGLE, TOLERANCE_DISTANCE};
use crate::utilities:: vec2::Vec2Rotate;


pub trait Movement {
    fn calculate_inputs_for_target(self, position: Pos2, direction: Vec2, target_position: Pos2, target_direction: Option<Vec2>) -> Vec<f32>;
    fn calculate_new_pose_from_inputs(&self, simulation_step: u32, inputs: Vec<f32>, position: Pos2, direction: Vec2, max_velocity: f32) -> (Pos2, Vec2, f32, f32);
}

#[derive(Clone, PartialEq, Copy, Debug)]
pub struct RombaMovement {
    pub max_velocity: f32,
    pub max_angular_velocity: f32,
    pub wheel_distance: f32,
    pub wheel_radius: f32
}

impl Default for RombaMovement {
    fn default() -> Self {
        let converter_kmh_ms = 0.277_777_8;
        Self {
            max_velocity: 10.0 * converter_kmh_ms,
            max_angular_velocity: 0.1 * converter_kmh_ms,
            wheel_distance: 0.2,
            wheel_radius: 0.05
        }
    }
}

impl Movement for RombaMovement {
    fn calculate_new_pose_from_inputs(&self, simulation_step: u32, inputs: Vec<f32>, position: Pos2, direction: Vec2, max_velocity: f32) -> (Pos2, Vec2, f32, f32) {
        if inputs.len() != 2 { assert_eq!(2, inputs.len()) }
        // Clamp if it is not
        let m1 = inputs[0].clamp(-1.0, 1.0);
        let m2 = inputs[1].clamp(-1.0, 1.0);

        let v_left = m1 * max_velocity;
        let v_right = m2 * max_velocity;

        let v = (v_left + v_right) / 2.0;
        let omega = (v_right - v_left) / self.wheel_distance * self.max_angular_velocity;

        let angle = omega * (simulation_step as f32);
        let new_direction = direction.rotate_radians(angle).normalized();

        let new_position = position + direction * (v * (simulation_step as f32));

        let current_velocity = v;

        (new_position, new_direction, current_velocity, omega)
    }
    
    fn calculate_inputs_for_target(self, position: Pos2, direction: Vec2, target_position: Pos2, target_direction: Option<Vec2>) -> Vec<f32> {
        
        let mut m1 = 0.0;
        let mut m2 = 0.0;
        
        let distance = position.distance(target_position);

        if distance > TOLERANCE_DISTANCE {
            // Compute angle to target
            let direction_to_target = (target_position - position).normalized();
            let angle_to_target = direction_to_target.angle()  * (180.0 / PI); // degrees
            let angle_of_agent = direction.angle() * (180.0 / PI);// degrees

            // Compute angle difference
            let delta_angle = (angle_to_target - angle_of_agent + 180.0).rem_euclid(360.0) - 180.0;

            // Normalize delta_angle to -1...1 range
            let normalized_delta = delta_angle / 180.0;

            // Basic differential drive control for turning and moving forward
            if delta_angle.abs() > TOLERANCE_ANGLE {
                // If we need to turn, adjust motor speeds accordingly
                let turn_strength = f32::min(1.0, normalized_delta.abs()); // Scale turn strength

                if normalized_delta < 0.0 {  // Turn right
                    m1 = turn_strength;
                    m2 = -turn_strength;
                } else {  // Turn left
                    m1 = -turn_strength;
                    m2 = turn_strength;
                }
            } else {
                // Move straight towards the target (no turning needed)
                let speed = f32::min(distance * 0.05, 1.0);  // Scale speed based on distance, up to a maximum of 1
                m1 = speed;
                m2 = speed;
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
    
        let threshold = 1e-4;
        if (-threshold..threshold).contains(&m1) {
            m1 = 0.0;
        }
        if (-threshold..threshold).contains(&m2) {
            m2 = 0.0;
        }
        
        vec![m1, m2]
    }
}
