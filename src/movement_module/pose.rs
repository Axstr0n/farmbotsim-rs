use egui::{Pos2, Vec2};

#[derive(Clone, Debug, PartialEq)]
pub struct Pose {
    pub position: Pos2,
    pub direction: Vec2,
}

impl Pose {
    pub fn new(position: Pos2, direction: Vec2) -> Self {
        Self { position, direction }
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

        poses.push(Pose { position, direction });
    }

    poses
}