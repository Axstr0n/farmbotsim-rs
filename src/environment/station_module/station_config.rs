use egui::{Pos2};

use crate::{movement_module::pose::Pose, units::{angle::Angle, length::Length}, utilities::utils::line_positions};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StationConfig {
    pub pose: Pose,
    pub queue_direction: Angle,
    pub waiting_offset: Length,
    pub n_slots: u32,
    pub slots_pose: Vec<Pose>,
}

impl Default for StationConfig {
    fn default() -> Self {
        Self {
            pose: Pose::new(Pos2::new(1.0, 1.0), Angle::degrees(0.0)),
            queue_direction: Angle::degrees(270.0),
            waiting_offset: Length::meters(1.0),
            n_slots: 1,
            slots_pose: vec![Pose::new(Pos2::ZERO, Angle::degrees(90.0))],
        }
    }
}

impl StationConfig {
    pub fn new(pose: Pose, queue_direction: Angle, waiting_offset: Length, n_slots: u32, slots_pose: Vec<Pose>) -> Self {
        Self {
            pose,
            queue_direction,
            waiting_offset,
            n_slots,
            slots_pose,
        }
    }

    pub fn update_slots_pose(&mut self) {
        if self.n_slots < self.slots_pose.len() as u32 {
            self.slots_pose.truncate(self.n_slots as usize);
            let positions = line_positions(self.n_slots as usize, Length::meters(0.3).to_base_unit(), self.pose.orientation.to_radians());
            for (i, slot_pose) in self.slots_pose.iter_mut().enumerate() {
                slot_pose.position = positions[i];
            }
        }
        else {
            let positions = line_positions(self.n_slots as usize, Length::meters(0.3).to_base_unit(), self.pose.orientation.to_radians());
            let diff = self.n_slots - self.slots_pose.len() as u32;
            self.slots_pose.extend((0..diff).map(|_| Pose::new(Pos2::ZERO, Angle::degrees(90.0))));
            for (i, slot_pose) in self.slots_pose.iter_mut().enumerate() {
                slot_pose.position = positions[i];
            }
        }
    }
}
