use egui::{Ui, Slider};

use crate::environment::station::Station;
use crate::tool::tool::Tool;
use crate::environment::env::Env;
use crate::environment::field_config::FieldConfig;

use crate::rendering::camera::Camera;
use crate::rendering::render::{render_coordinate_system, render_crops, render_grid, render_obstacles, render_stations, render_visibility_graph};
use crate::utilities::pos2::ExtendedPos2;
use crate::utilities::utils::generate_colors;


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
        render_stations(ui, &self.camera, &self.env.stations);
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
        
        egui::CollapsingHeader::new(
            format!("Stations ({})", self.env.stations.len())
        )
            .default_open(true)
            .show(ui, |ui| {
                ui.horizontal_top(|ui| {
                    if ui.button("Add station").clicked() {
                        self.env.stations.push(Station::default());
                        self.recalc_charging_stations();
                    }
                    if ui.button("Remove all").clicked() {
                        self.env.stations.clear();
                        self.recalc_charging_stations();
                    }
                });

                let mut to_remove: Option<usize> = None;

                for (i, station) in self.env.stations.iter_mut().enumerate() {
                    egui::CollapsingHeader::new({
                        let mut job = egui::text::LayoutJob::default();
                            job.append(
                                "⏺",
                                0.0,
                                egui::TextFormat {
                                    color: station.color,
                                    ..Default::default()
                                },
                            );
                            job.append(
                                format!(" {}", station.id).as_str(),
                                0.0,
                                egui::TextFormat::default(),
                            );
                            job
                    })
                    .default_open(false)
                    .show(ui, |ui| {
                        // LEFT_TOP_POS
                        let response = ui.add(
                            Slider::new(&mut station.position.x, 0.0..=10.0)
                                .text("Left top pos x")
                                .step_by(0.2)
                        );
                        if response.changed() {
                            //self.field.recalculate_crops()
                        }
                        let response = ui.add(
                            Slider::new(&mut station.position.y, 0.0..=10.0)
                                .text("Left top pos y")
                                .step_by(0.2)
                        );
                        if response.changed() {
                            //self.field.recalculate_crops()
                        }
                        let mut rgb = [
                            station.color.r(), 
                            station.color.g(), 
                            station.color.b(),
                        ];

                        // Use the color picker with the mutable reference to [u8; 3]
                        if ui.color_edit_button_srgb(&mut rgb).changed() {
                            // Update the Color32 from the RGB array
                            station.color = egui::Color32::from_rgb(rgb[0], rgb[1], rgb[2]);
                        }
                        if ui.button("Remove").clicked() {
                            to_remove = Some(i);
                        }
                    });
                }

                // Remove *after* iteration
                if let Some(index) = to_remove {
                    self.env.stations.remove(index);
                    self.recalc_charging_stations();
                }

            });
    }
    
}

impl EditorTool {

    pub fn recalc_charging_stations(&mut self) {
        let colors = generate_colors(self.env.stations.len(), 0.01);
        for (i, station) in self.env.stations.iter_mut().enumerate() {
            // Update the station with a new id, position, and color
            //*station = ChargingStation::new(i as u32, station.position, station.color);
            *station = Station::new(i as u32, station.position, station.queue_direction, station.waiting_offset, colors[i]);
        }
    }
}
