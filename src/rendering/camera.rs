use egui::{Pos2, Ui};


pub struct Camera {
    pub zoom_level: f32,
    zoom_factor: f32,
    min_zoom: f32,
    max_zoom: f32,
    last_zoom_level: f32,

    pub position: Pos2,
    pub mouse_position: Option<Pos2>,

    width: f32,
    height: f32,

}

impl Default for Camera {
    fn default() -> Self {
        let zoom_level = 70.0;
        let initial_pos = Pos2::new(4.0, 4.0);
        Self {
            zoom_level,
            zoom_factor: 1.1,
            min_zoom: 5.0,
            max_zoom: 500.0,
            position: initial_pos,
            last_zoom_level: zoom_level,
            mouse_position: None,
            width: 0.0,
            height: 0.0,

        }
    }
}

impl Camera {
    
    pub fn handle_events(&mut self, ui: &mut Ui) {
        let main_rect = ui.available_rect_before_wrap();
        let min_pos = main_rect.min;
        let max_pos = main_rect.max;
        self.width = max_pos.x - min_pos.x;
        self.height = max_pos.y - min_pos.y;

        // Get mouse pos in main scene
        if let Some(pos) = ui.ctx().pointer_hover_pos() {
            if main_rect.contains(pos) {
                self.mouse_position = Some(pos);
            } else {
                self.mouse_position = None;
            }
        } else {
            self.mouse_position = None;
        }

        // Dragging
        let response = ui.interact(ui.available_rect_before_wrap(), ui.id(), egui::Sense::click_and_drag());
        if response.dragged_by(egui::PointerButton::Middle) {
            let mut drag_delta = response.drag_delta();
            drag_delta.x *= -1.0;
            self.position += drag_delta/self.zoom_level;
        }
        
        // Detect scroll
        if self.mouse_position.is_some() {
            let scroll_delta = ui.input(|i| i.raw_scroll_delta);
            if scroll_delta.y != 0.0 {
                let mut new_zoom_level: f32 = self.zoom_level;
                if scroll_delta.y > 0.0 {
                    if self.zoom_level < self.max_zoom {
                        new_zoom_level = self.zoom_level * self.zoom_factor;
                    }
                }
                else { // scroll_delta.y < 0.0
                    if self.zoom_level > self.min_zoom {
                        new_zoom_level = self.zoom_level / self.zoom_factor;
                    }
                }
                self.last_zoom_level = self.zoom_level;
                self.zoom_level = new_zoom_level;
            }
        }
    }


    pub fn scene_to_screen_pos(&self, pos: Pos2) -> Pos2 {
        let screen_center =  Pos2::new(self.width/2.0, self.height/2.0);
        let mut camera_relative = (pos - self.position) * self.zoom_level;
        camera_relative.y *= -1.0;
        screen_center + camera_relative
    }
    pub fn scene_to_screen_val(&self, val: f32) -> f32 {
        val * self.zoom_level
    }
    pub fn screen_to_scene_pos(&self, pos: Pos2) -> Pos2 {
        let screen_center =  Pos2::new(self.width/2.0, self.height/2.0);
        let mut scene_relative = pos - screen_center;
        scene_relative.y *= -1.0;
        self.position + scene_relative / self.zoom_level
    }
}

