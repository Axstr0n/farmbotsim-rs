use egui::{Color32, Pos2, Vec2};
use std::collections::VecDeque;

use super::movement::{Movement, RombaMovement};
use crate::task::task::Task;
use crate::utilities::pos2::ExtendedPos2;
use crate::utilities::utils::TOLERANCE_DISTANCE;


#[derive(Clone)]
pub struct Agent {
    pub id: u32,
    pub position: Pos2,
    pub direction: Vec2,
    pub movement: RombaMovement,
    pub velocity_lin: f32,
    pub velocity_ang: f32,
    pub color: Color32,

    pub work_schedule: VecDeque<Task>,
    pub current_task: Option<Task>,
}

impl Agent {
    pub fn new(id: u32, 
        position: Pos2,
        direction: Vec2,
        movement: RombaMovement,
        color: Color32) -> Self {
        Self {
            id,
            position,
            direction,
            movement,
            velocity_lin: 0.0,
            velocity_ang: 0.0,
            color,

            work_schedule: VecDeque::new(),
            current_task: None,
        }
    }
    pub fn update(&mut self, simulation_step: u32) {
        self.update_path();
        let inputs = self._get_inputs();
        self._move(simulation_step, inputs);
    }
    fn _move(&mut self, simulation_step: u32, inputs: Vec<f32>) {
        let current_task_velocity = self.current_task.as_ref().map(|task| task.velocity).unwrap_or_default();
        let (new_position, new_direction, new_velocity_l, new_velocity_r) = self.movement.calculate_new_pose_from_inputs(
            simulation_step, inputs, self.position, self.direction, current_task_velocity
        );
    
        // Now assign the new values
        self.position = new_position;
        self.direction = new_direction;
        self.velocity_lin = new_velocity_l;
        self.velocity_ang = new_velocity_r;
    }
    
    fn _get_inputs(&self) -> Vec<f32> {
        let next_direction: Option<Vec2> = None;

        let next_position = match &self.current_task {
            Some(task) => {
                if !task.path.is_empty() {
                    // Follow path normally
                    task.path[0]
                } else {
                    self.position
                }
                
            },
            _ => {
                self.position
            }
        };
        self.movement.calculate_inputs_for_target(
            self.position, self.direction, next_position, next_direction
        )
    }
    
    fn update_path(&mut self) {
        if let Some(task) = &mut self.current_task {
            while !task.path.is_empty() {
                if self.position.is_close_to(task.path[0], TOLERANCE_DISTANCE) {
                    task.path.remove(0);
                } else {
                    break;
                }
            }
            if task.path.is_empty() { self.current_task = self.work_schedule.pop_front(); }
        }
        else { self.current_task = self.work_schedule.pop_front(); }
    }

}


