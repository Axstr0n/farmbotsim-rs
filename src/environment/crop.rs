use egui::Pos2;


#[derive(PartialEq, Debug, Clone, Copy)]
pub enum CropState{
    Unprocessed,
    // Scanning,
    // Scanned,
    // Processing,
    // Processed
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Crop {
    pub id: u32,
    pub row_id: u32,
    pub position: Pos2,
    state: CropState,
    worked_time: u32,
    required_scan_time: u32,
    required_process_time: u32,
}

impl Crop {
    pub fn new(id: u32, row_id: u32, position: Pos2) -> Self {
        Self {
            id,
            row_id,
            position,
            state: CropState::Unprocessed,
            worked_time: 0,
            required_scan_time: 60,
            required_process_time: 2 * 60
        }
    }
}
