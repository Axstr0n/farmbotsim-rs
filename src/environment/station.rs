use egui::{Color32, Pos2, Vec2};
use std::collections::VecDeque;

use crate::agent::agent::Agent;

#[derive(Clone)]
pub struct Station {
    pub id: u32,
    pub position: Pos2,
    pub queue_direction: Vec2,
    pub waiting_offset: f32,
    pub queue: VecDeque<Agent>,
    pub color: Color32,
}

impl Default for Station {
    fn default() -> Self {
        Self {
            id: 0,
            position: Pos2::new(1.0, 1.0),
            queue_direction: Vec2::new(1.0,0.0),
            waiting_offset: 0.5,
            queue: VecDeque::new(),
            color: Color32::RED,
        }
    }
}

impl Station {
    pub fn new(id: u32, position: Pos2, queue_direction: Vec2, waiting_offset: f32, color: Color32) -> Self {
        Self {
            id,
            position,
            queue_direction,
            waiting_offset,
            queue: VecDeque::new(),
            color,
        }
    }
}

