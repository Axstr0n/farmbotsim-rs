use egui::Pos2;

/// Trait defining pathfinding functionality.
pub trait PathFinding {
    /// Finds a path from `start` to `end`.
    /// If no path returns None.
    fn find_path(&mut self, start: Pos2, end: Pos2) -> Option<Vec<Pos2>>;
}