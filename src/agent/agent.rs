use egui::{Pos2, Vec2};

use super::movement::Movement;
use crate::path::visibility_graph::VisibilityGraph;
use crate::utilities::pos2::ExtendedPos2;
use crate::utilities::utils::TOLERANCE_DISTANCE;


#[derive(Clone)]
pub struct Agent {
    pub id: u32,
    pub position: Pos2,
    pub direction: Vec2,
    pub movement: Movement,
    pub visibility_graph: VisibilityGraph,
    pub velocity_lin: f32,
    pub velocity_ang: f32,

    pub path: Option<Vec<Pos2>>,
}

impl Agent {
    pub fn new(id: u32, 
        position: Pos2,
        direction: Vec2,
        movement: Movement,
        visibility_graph: VisibilityGraph) -> Self {
        Self {
            id: id,
            position: position,
            direction: direction,
            movement: movement,
            visibility_graph: visibility_graph,
            velocity_lin: 0.0,
            velocity_ang: 0.0,

            path: None,
        }
    }
    pub fn update(&mut self, simulation_step: u32) {
        self.update_path();
        let (m1,m2) = self._get_inputs();
        self._move(simulation_step, m1, m2);
    }
    fn _move(&mut self, simulation_step: u32, m1: f32, m2: f32) {
        let (new_position, new_direction, new_velocity_l, new_velocity_r) = self.movement.calculate_new_values(
            simulation_step, m1, m2, self.position, self.direction
        );
    
        // Now assign the new values
        self.position = new_position;
        self.direction = new_direction;
        self.velocity_lin = new_velocity_l;
        self.velocity_ang = new_velocity_r;
    }
    
    fn _get_inputs(&self) -> (f32,f32) {
        let next_position;
        let next_direction: Option<Vec2> = None;

        match &self.path {
            Some(path) if &path.len() > &(0 as usize) => {
                // Follow path normally
                next_position = path[0];
            },
            _ => {
                next_position = self.position;
            }
        };
        let (m1, m2) = self.movement.calculate_inputs_for_target(
            self.position, self.direction, next_position, next_direction
        );

        return (m1, m2)
    }
    
    fn update_path(&mut self) {
        match self.path.take() {
            Some(mut path) => {
                while !path.is_empty() {
                    if self.position.is_close_to(path[0], TOLERANCE_DISTANCE) {
                        path.remove(0);
                    } else {
                        break;
                    }
                }
                self.path = Some(path);
            },
            None => {}
        }
    }

    pub fn set_path(&mut self, end: Pos2) {
        self.path = self.visibility_graph.find_path(self.position, end);
    }

}


