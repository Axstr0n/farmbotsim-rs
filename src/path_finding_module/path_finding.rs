use egui::Pos2;


pub trait PathFinding {
    fn find_path(&mut self, start: Pos2, end: Pos2) -> Option<Vec<Pos2>>;
}