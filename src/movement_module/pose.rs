use egui::{Pos2, Vec2};

use crate::{
    units::{angle::Angle, length::Length},
    utilities::pos2::ExtendedPos2,
};

/// Represents a 2D pose consisting of a position and an orientation.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Pose {
    /// The 2D position (x, y) of the pose.
    pub position: Pos2,
    /// The orientation angle (rad or degrees)
    pub orientation: Angle,
}

impl Pose {
    /// Creates a new pose from a position and an orientation.
    pub fn new(position: Pos2, orientation: Angle) -> Self {
        Self {
            position,
            orientation,
        }
    }
    /// Returns true if this pose is close to another pose within given tolerances.
    pub fn is_close_to(&self, other: &Pose, tol_dist: Length, tol_ang: Angle) -> bool {
        self.position.is_close_to(other.position, tol_dist)
            && self.orientation.is_close_to(other.orientation, tol_ang)
    }
}

impl std::ops::Add for Pose {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Pose {
            position: self.position + other.position.to_vec2(),
            orientation: self.orientation + other.orientation,
        }
    }
}

/// Converts a list of positions into poses by inferring orientation from path direction.
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

        poses.push(Pose {
            position,
            orientation,
        });
    }

    poses
}
