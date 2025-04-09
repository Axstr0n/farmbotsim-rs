use egui::{Ui, Slider};

use crate::tool::tool::Tool;
use crate::environment::env::Env;
use crate::environment::field_config::FieldConfig;

use crate::rendering::camera::Camera;
use crate::rendering::render::{render_coordinate_system, render_crops, render_grid, render_obstacles, render_visibility_graph};
use crate::utilities::pos2::ExtendedPos2;


macro_rules! add_slider {
    ($ui:expr, $value:expr, $range:expr, $text:expr, $step:expr, $field:expr, $graph:expr) => {
        let response = $ui.add(
            Slider::new($value, $range)
                .text($text)
                .step_by($step)
        );
        if response.changed() {
            $field.recalculate_field();
            $graph.recalculate(&$field.get_graph_points(), &$field.obstacles);
        }
    };
}


pub struct EditorTool {
    tick: u32,
    env: Env,
    camera: Camera,
}

impl Default for EditorTool {
    fn default() -> Self {
        Self {
            tick: 0,
            env: Env::new(0, Some(FieldConfig::default())),
            camera: Camera::default(),
        }
    }
}

impl Tool for EditorTool {
    fn update(&mut self) {
        self.tick += 1;
        
    }

    fn render_main(&mut self, ui: &mut Ui) {
        self.camera.handle_events(ui);
        render_grid(ui, &self.camera);
        render_coordinate_system(ui, &self.camera);
        render_obstacles(ui, &self.camera, &self.env.field.obstacles);
        render_visibility_graph(ui, &self.camera, &self.env.visibility_graph);
        render_crops(ui, &self.camera, &self.env.field.crops);
    }

    fn render_ui(&mut self, ui: &mut Ui) {
        if ui.button("Save").clicked() {
            println!("Save not implemented");
        }

        let (mouse_pos, scene_pos) = match self.camera.mouse_position {
            Some(pos) => {
                let scene_pos = self.camera.screen_to_scene_pos(pos);
                (Some(pos), Some(scene_pos))
            },
            None => {
                (None, None)
            },
        };
        ui.label(format!("Mouse pos: {}", mouse_pos.map_or("None".to_string(), |p| p.fmt(2))));
        ui.label(format!("Scene pos: {}", scene_pos.map_or("None".to_string(), |p| p.fmt(2))));

        ui.label(format!("Camera pos: {}", self.camera.position.fmt(2)));
        ui.label(format!("Zoom: {}", self.camera.zoom_level));
        ui.label(format!("Dragging: {}", self.camera.dragging));

        ui.separator();

        egui::CollapsingHeader::new("Field")
            .default_open(true)
            .show(ui, |ui| {
                // LEFT_TOP_POS
                add_slider!(
                    ui,
                    &mut self.env.field.config.left_top_pos.x,
                    0.0..=10.0,
                    "Left top pos x",
                    0.2,
                    &mut self.env.field,
                    &mut self.env.visibility_graph
                );

                add_slider!(
                    ui,
                    &mut self.env.field.config.left_top_pos.y,
                    0.0..=10.0,
                    "Left top pos y",
                    0.2,
                    &mut self.env.field,
                    &mut self.env.visibility_graph
                );

                // ANGLE
                add_slider!(
                    ui,
                    &mut self.env.field.config.angle,
                    0.0..=360.0,
                    "Angle",
                    1.0,
                    &mut self.env.field,
                    &mut self.env.visibility_graph
                );

                // N_ROWS
                add_slider!(
                    ui,
                    &mut self.env.field.config.n_rows,
                    3..=10,
                    "N rows",
                    1.0,
                    &mut self.env.field,
                    &mut self.env.visibility_graph
                );

                // N_CROPS_PER_ROW
                add_slider!(
                    ui,
                    &mut self.env.field.config.n_crops_per_row,
                    3..=10,
                    "N crops per row",
                    1.0,
                    &mut self.env.field,
                    &mut self.env.visibility_graph
                );

                // ROW_SPACING
                add_slider!(
                    ui,
                    &mut self.env.field.config.row_spacing,
                    0.4..=1.0,
                    "Row spacing",
                    0.1,
                    &mut self.env.field,
                    &mut self.env.visibility_graph
                );

                // CROP_SPACING
                add_slider!(
                    ui,
                    &mut self.env.field.config.crop_spacing,
                    0.3..=1.0,
                    "Crop spacing",
                    0.1,
                    &mut self.env.field,
                    &mut self.env.visibility_graph
                );
            });
    }
    
}
