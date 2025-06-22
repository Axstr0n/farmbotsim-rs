use egui::Color32;
use std::collections::VecDeque;

use crate::{movement_module::pose::Pose, units::{angle::Angle, length::Length}};
use super::station_config::StationConfig;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StationPosType {
    ChargingSlot,
    QueueSlot,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Station {
    pub id: u32,
    pub pose: Pose,
    pub queue_direction: Angle,
    pub waiting_offset: Length,
    pub color: Color32,
    
    pub n_slots: u32,
    pub slots_pose: Vec<Pose>,
    pub slots: Vec<Option<u32>>,
    pub queue: VecDeque<u32>,
}

impl Default for Station {
    fn default() -> Self {
        let config = StationConfig::default();
        Self {
            id: 0,
            pose: config.pose,
            queue_direction: config.queue_direction,
            waiting_offset: config.waiting_offset,
            color: Color32::RED,

            n_slots: config.n_slots,
            slots_pose: config.slots_pose,
            slots: vec![None; config.n_slots as usize],
            queue: VecDeque::new(),
        }
    }
}

impl Station {
    pub fn from_config(id: u32, color: Color32, config: StationConfig) -> Self {
        Self {
            id,
            pose: config.pose,
            queue_direction: config.queue_direction,
            waiting_offset: config.waiting_offset,
            color,
            n_slots: config.n_slots,
            slots_pose: config.slots_pose,
            slots: vec![None; config.n_slots as usize],
            queue: VecDeque::new(),
        }
    }
    pub fn to_config(&self) -> StationConfig {
        StationConfig::new(self.pose.clone(), self.queue_direction, self.waiting_offset, self.n_slots, self.slots_pose.clone())
    }
}

impl Station {
    pub fn new(id: u32, pose: Pose, queue_direction: Angle, waiting_offset: Length, color: Color32, n_slots: u32, slots_pose: Vec<Pose>) -> Self {
        Self {
            id,
            pose,
            queue_direction,
            waiting_offset,
            color,

            n_slots,
            slots_pose,
            slots: vec![None; n_slots as usize],
            queue: VecDeque::new(),
        }
    }
    pub fn reset(&mut self) {
        self.slots = vec![None; self.n_slots as usize];
        self.queue.clear();
    }
    pub fn n_occupied_slots(&self) -> u32 {
        let mut n_occupied = 0;
        for slot in &self.slots {
            if slot.is_some() { n_occupied += 1; }
        }
        n_occupied
    }
    pub fn get_empty_slot(&self) -> Option<usize> {
        self.slots.iter().position(|x| x.is_none())
    }
    pub fn request_charge(&mut self, agent_id: u32) -> (Pose, StationPosType) {
        if let Some(index) = self.get_empty_slot() {
            self.slots[index] = Some(agent_id);
            if let Some(pose) = self.get_pose_for_slot(index) {
                return (pose, StationPosType::ChargingSlot)
            } else {
                self.slots[index] = None;
            }
        }
        self.queue.push_back(agent_id);
        (self.get_waiting_pose(self.queue.len()-1), StationPosType::QueueSlot)
    }
    pub fn release_agent(&mut self, agent_id: u32) -> bool {
        // Remove from slot or queue
        let mut successfully_removed = false;
        successfully_removed |= self.remove_agent_from_slots(agent_id);
        successfully_removed |= self.remove_agent_from_queue(agent_id);
        successfully_removed
    }
    pub fn move_agent_from_queue_to_slot(&mut self, agent_id: u32) -> Option<Pose> {
        if let Some(index) = self.get_empty_slot() {
            if self.remove_agent_from_queue(agent_id) {
                self.slots[index] = Some(agent_id);
                return self.get_pose_for_slot(index)
            }
        }
        None
    }
    fn remove_agent_from_slots(&mut self, agent_id: u32) -> bool {
        if self.slots.iter().any(|slot| *slot == Some(agent_id)) {
            self.slots.iter_mut().for_each(|slot| {
                if *slot == Some(agent_id) {
                    *slot = None;
                }
            });
            return true
        }
        false
    }
    fn remove_agent_from_queue(&mut self, agent_id: u32) -> bool {
        if self.queue.contains(&agent_id) {
            self.queue.retain(|&id| id != agent_id);
            return true
        }
        false
    }

    pub fn get_waiting_pose(&self, queue_index: usize) -> Pose {
        let distance = (queue_index as f32 + 1.0) * self.waiting_offset;
        let orientation = self.pose.orientation + self.queue_direction;
        let position = self.pose.position + orientation.to_vec2() * distance;
        Pose::new(position, orientation+Angle::degrees(180.0))
    }
    pub fn get_pose_for_slot(&self, index: usize) -> Option<Pose> {
        if index >= self.slots_pose.len() {None}
        else {
            Some(self.slots_pose[index].clone() + self.pose.clone())
        }
    }
}

