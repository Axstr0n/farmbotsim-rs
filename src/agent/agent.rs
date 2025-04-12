use egui::{Color32, Pos2, Vec2};

use super::movement::{Movement, RombaMovement};
use crate::path::visibility_graph::VisibilityGraph;
use crate::utilities::pos2::ExtendedPos2;
use crate::utilities::utils::TOLERANCE_DISTANCE;


#[derive(Clone)]
pub struct Agent {
    pub id: u32,
    pub position: Pos2,
    pub direction: Vec2,
    pub movement: RombaMovement,
    pub visibility_graph: VisibilityGraph,
    pub velocity_lin: f32,
    pub velocity_ang: f32,
    pub color: Color32,

    pub path: Option<Vec<Pos2>>,
}

impl Agent {
    pub fn new(id: u32, 
        position: Pos2,
        direction: Vec2,
        movement: RombaMovement,
        visibility_graph: VisibilityGraph,
        color: Color32) -> Self {
        Self {
            id,
            position,
            direction,
            movement,
            visibility_graph,
            velocity_lin: 0.0,
            velocity_ang: 0.0,
            color,

            path: None,
        }
    }
    pub fn update(&mut self, simulation_step: u32) {
        self.update_path();
        let inputs = self._get_inputs();
        self._move(simulation_step, inputs);
    }
    fn _move(&mut self, simulation_step: u32, inputs: Vec<f32>) {
        let (new_position, new_direction, new_velocity_l, new_velocity_r) = self.movement.calculate_new_pose_from_inputs(
            simulation_step, inputs, self.position, self.direction
        );
    
        // Now assign the new values
        self.position = new_position;
        self.direction = new_direction;
        self.velocity_lin = new_velocity_l;
        self.velocity_ang = new_velocity_r;
    }
    
    fn _get_inputs(&self) -> Vec<f32> {
        let next_direction: Option<Vec2> = None;

        let next_position = match &self.path {
            Some(path) if !path.is_empty() => {
                // Follow path normally
                path[0]
            },
            _ => {
                self.position
            }
        };
        let inputs = self.movement.calculate_inputs_for_target(
            self.position, self.direction, next_position, next_direction
        );

        inputs
    }
    
    fn update_path(&mut self) {
        if let Some(mut path) = self.path.take() {
            while !path.is_empty() {
                if self.position.is_close_to(path[0], TOLERANCE_DISTANCE) {
                    path.remove(0);
                } else {
                    break;
                }
            }
            self.path = Some(path);
        }
    }

    pub fn set_path(&mut self, end: Pos2) {
        self.path = self.visibility_graph.find_path(self.position, end);
    }

}


