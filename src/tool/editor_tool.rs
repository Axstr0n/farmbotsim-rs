use egui::{Slider, Ui};

use crate::environment::station::Station;
use crate::tool::tool::Tool;
use crate::environment::env::Env;
use crate::environment::field_config::FieldConfig;

use crate::rendering::camera::Camera;
use crate::rendering::render::{render_coordinate_system, render_crops, render_drag_points, render_grid, render_obstacles, render_spawn_area, render_stations, render_visibility_graph};
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
        render_spawn_area(ui, &self.camera, &self.env.spawn_area);
        render_obstacles(ui, &self.camera, &self.env.field.obstacles);
        render_visibility_graph(ui, &self.camera, &self.env.visibility_graph);
        render_crops(ui, &self.camera, &self.env.field.crops);
        render_stations(ui, &self.camera, &self.env.stations);
        self.handle_dragging(ui);
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

        // ui.label(format!("Camera pos: {}", self.camera.position.fmt(2)));
        // ui.label(format!("Zoom: {}", self.camera.zoom_level));

        ui.separator();

        egui::CollapsingHeader::new("Field")
            .default_open(true)
            .show(ui, |ui| {
                ui.label(format!("Left top pos {}", self.env.field.config.left_top_pos.fmt(2)));

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
                    1..=20,
                    "N rows",
                    1.0,
                    &mut self.env.field,
                    &mut self.env.visibility_graph
                );

                // N_CROPS_PER_ROW
                add_slider!(
                    ui,
                    &mut self.env.field.config.n_crops_per_row,
                    1..=20,
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
        
        egui::CollapsingHeader::new("Spawn area")
            .default_open(true)
            .show(ui, |ui| {
                ui.label(format!("Left top pos {}", self.env.spawn_area.left_top_pos.fmt(2)));

                ui.add(Slider::new(
                    &mut self.env.spawn_area.angle,
                    0.0..=360.0)
                    .text("Angle")
                    .step_by(1.0)
                );
                ui.add(Slider::new(
                    &mut self.env.spawn_area.length,
                    1.0..=10.0)
                    .text("Length")
                    .step_by(0.1)
                );
                ui.add(Slider::new(
                    &mut self.env.spawn_area.width,
                    1.0..=10.0)
                    .text("Width")
                    .step_by(0.1)
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
                        // let response = ui.add(
                        //     Slider::new(&mut station.position.x, 0.0..=10.0)
                        //         .text("Left top pos x")
                        //         .step_by(0.2)
                        // );
                        // if response.changed() {
                        //     //self.field.recalculate_crops()
                        // }
                        // let response = ui.add(
                        //     Slider::new(&mut station.position.y, 0.0..=10.0)
                        //         .text("Left top pos y")
                        //         .step_by(0.2)
                        // );
                        // if response.changed() {
                        //     //self.field.recalculate_crops()
                        // }
                        ui.label(format!("Position {}", station.position.fmt(2)));
                        // let mut rgb = [
                        //     station.color.r(), 
                        //     station.color.g(), 
                        //     station.color.b(),
                        // ];

                        // // Use the color picker with the mutable reference to [u8; 3]
                        // if ui.color_edit_button_srgb(&mut rgb).changed() {
                        //     // Update the Color32 from the RGB array
                        //     station.color = egui::Color32::from_rgb(rgb[0], rgb[1], rgb[2]);
                        // }
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

    fn handle_dragging(&mut self, ui: &mut Ui) {
        
        // Draw drag points
        let mut pts = vec![];
        for station in &mut self.env.stations {
            let screen_pos = self.camera.scene_to_screen_pos(station.position);
            pts.push(screen_pos);
        }
        let field_pos = self.camera.scene_to_screen_pos(self.env.field.crops[0].position);
        pts.push(field_pos);
        let spawn_pos = self.camera.scene_to_screen_pos(self.env.spawn_area.left_top_pos);
        pts.push(spawn_pos);

        render_drag_points(ui, &self.camera, &pts);

        let drag_point_size = self.camera.scene_to_screen_val(0.1);
        // Drag stations
        for station in &mut self.env.stations {
            let screen_pos = self.camera.scene_to_screen_pos(station.position);
            let rect = egui::Rect::from_center_size(screen_pos, egui::Vec2::splat(drag_point_size));
            let response = ui.interact(rect, ui.make_persistent_id(format!("station_drag_{}", station.id)), egui::Sense::click_and_drag());
            
            if response.dragged() {
                let drag_delta = response.drag_delta();
                let new_screen_pos = screen_pos + drag_delta;
                let new_scene_pos = self.camera.screen_to_scene_pos(new_screen_pos);
                station.position = new_scene_pos;
            }
        }
        // Drag field
        let screen_pos = self.camera.scene_to_screen_pos(self.env.field.crops[0].position);
        let rect = egui::Rect::from_center_size(screen_pos, egui::Vec2::splat(drag_point_size));
        let response = ui.interact(rect, ui.make_persistent_id("field_drag"), egui::Sense::click_and_drag());

        if response.dragged() {
            let drag_delta = response.drag_delta();
            let new_screen_pos = screen_pos + drag_delta;
            let new_scene_pos = self.camera.screen_to_scene_pos(new_screen_pos);
            self.env.field.config.left_top_pos = new_scene_pos;
            self.env.field.recalculate_field();
            self.env.visibility_graph.recalculate(&self.env.field.get_graph_points(), &self.env.field.obstacles);
        }

        // Drag spawn
        let screen_pos = self.camera.scene_to_screen_pos(self.env.spawn_area.left_top_pos);
        let rect = egui::Rect::from_center_size(screen_pos, egui::Vec2::splat(drag_point_size));
        let response = ui.interact(rect, ui.make_persistent_id("spawn_drag"), egui::Sense::click_and_drag());

        if response.dragged() {
            let drag_delta = response.drag_delta();
            let new_screen_pos = screen_pos + drag_delta;
            let new_scene_pos = self.camera.screen_to_scene_pos(new_screen_pos);
            self.env.spawn_area.left_top_pos = new_scene_pos;
        }

    }
}
