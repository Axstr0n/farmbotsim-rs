use egui::{Pos2};

use crate::{movement_module::pose::Pose, units::{angle::Angle, length::Length}, utilities::utils::line_positions};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StationConfig {
    pub position: Pos2,
    pub angle: Angle,
    pub queue_direction: Angle,
    pub waiting_offset: Length,
    pub n_slots: u32,
    pub slots_pose: Vec<Pose>,
}

impl Default for StationConfig {
    fn default() -> Self {
        Self {
            position: Pos2::ZERO,
            angle: Angle::degrees(0.0),
            queue_direction: Angle::degrees(0.0),
            waiting_offset: Length::meters(1.0),
            n_slots: 1,
            slots_pose: vec![Pose::new(Pos2::ZERO, Angle::degrees(0.0))],
        }
    }
}

impl StationConfig {
    pub fn new(position: Pos2, angle: Angle, queue_direction: Angle, waiting_offset: Length, n_slots: u32, slots_pose: Vec<Pose>) -> Self {
        Self {
            position,
            angle,
            queue_direction,
            waiting_offset,
            n_slots,
            slots_pose,
        }
    }

    pub fn update_slots_pose(&mut self) {
        if self.n_slots < self.slots_pose.len() as u32 {
            self.slots_pose.truncate(self.n_slots as usize);
            let positions = line_positions(self.n_slots as usize, Length::meters(0.3).to_base_unit(), self.angle.to_radians());
            let positions_with_center: Vec<Pos2> = positions.into_iter()
                .map(|pos| Pos2::new(pos.x + self.position.x, pos.y + self.position.y))
                .collect();
            for (i, slot_pose) in self.slots_pose.iter_mut().enumerate() {
                slot_pose.position = positions_with_center[i];
            }
        }
        else {
            let positions = line_positions(self.n_slots as usize, Length::meters(0.3).to_base_unit(), self.angle.to_radians());
            let positions_with_center: Vec<Pos2> = positions.into_iter()
                .map(|pos| Pos2::new(pos.x + self.position.x, pos.y + self.position.y))
                .collect();
            let diff = self.n_slots - self.slots_pose.len() as u32;
            self.slots_pose.extend((0..diff).map(|_| Pose::new(Pos2::ZERO, Angle::degrees(0.0))));
            for (i, slot_pose) in self.slots_pose.iter_mut().enumerate() {
                slot_pose.position = positions_with_center[i];
            }
        }
    }
}
