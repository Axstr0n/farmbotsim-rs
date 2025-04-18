use egui::{Color32, Pos2, Vec2};
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StationPosType {
    ChargingSlot,
    QueueSlot,
}

#[derive(Clone)]
pub struct Station {
    pub id: u32,
    pub position: Pos2,
    pub queue_direction: Vec2,
    pub waiting_offset: f32,
    pub color: Color32,
    
    pub n_slots: u32,
    pub slots: Vec<u32>,
    pub queue: VecDeque<u32>,
}

impl Default for Station {
    fn default() -> Self {
        Self {
            id: 0,
            position: Pos2::new(1.0, 1.0),
            queue_direction: Vec2::new(1.0,0.0),
            waiting_offset: 0.5,
            color: Color32::RED,

            n_slots: 1,
            slots: Vec::new(),
            queue: VecDeque::new(),
        }
    }
}

impl Station {
    pub fn new(id: u32, position: Pos2, queue_direction: Vec2, waiting_offset: f32, color: Color32, n_slots: u32) -> Self {
        Self {
            id,
            position,
            queue_direction,
            waiting_offset,
            color,

            n_slots,
            slots: Vec::new(),
            queue: VecDeque::new(),
        }
    }
    pub fn reset(&mut self) {
        self.slots.clear();
        self.queue.clear();
    }
    pub fn request_charge(&mut self, agent_id: u32) -> (Pos2, StationPosType) {
        if self.slots.len() < self.n_slots as usize {
            self.slots.push(agent_id);
            return (self.position, StationPosType::ChargingSlot)
        }
        self.queue.push_back(agent_id);
        (self.get_waiting_position(self.queue.len()-1), StationPosType::QueueSlot)
    }

    pub fn release_agent(&mut self, agent_id: u32) -> bool {
        // Remove from slot or queue
        let mut succesfully_removed: bool = false;
        if self.slots.contains(&agent_id) { succesfully_removed = true; }
        if self.queue.contains(&agent_id) { succesfully_removed = true; }
        self.slots.retain(|&id| id != agent_id);
        self.queue.retain(|&id| id != agent_id);
        
        succesfully_removed
    }

    pub fn get_waiting_position(&self, queue_index: usize) -> Pos2 {
        let distance = (queue_index as f32 + 1.0) * self.waiting_offset;
        self.position + self.queue_direction * distance
    }
}

