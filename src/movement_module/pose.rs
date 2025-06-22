use egui::{Pos2, Vec2};

use crate::units::angle::Angle;

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Pose {
    pub position: Pos2,
    pub orientation: Angle,
}

impl Pose {
    pub fn new(position: Pos2, orientation: Angle) -> Self {
        Self { position, orientation }
    }
}

pub fn path_to_poses(path: Vec<Pos2>) -> Vec<Pose> {
    let mut poses = Vec::with_capacity(path.len());

    for i in 0..path.len() {
        let position = path[i];

        // Determine direction
        let direction = if i + 1 < path.len() {
            (path[i + 1] - path[i]).normalized()
        } else if i > 0 {
            // Use previous direction for the last pose
            (path[i] - path[i - 1]).normalized()
        } else {
            // Fallback: default direction
            Vec2::X
        };

        let orientation_value = direction.y.atan2(direction.x);
        let orientation = Angle::radians(orientation_value);

        poses.push(Pose { position, orientation });
    }

    poses
}