use egui::{Slider, Ui};

use crate::environment::station::Station;
use crate::tool::tool::Tool;
use crate::environment::env::Env;
use crate::environment::field_config::{FieldConfig, LineFieldConfig, PointFieldConfig, VariantFieldConfig};

use crate::rendering::camera::Camera;
use crate::rendering::render::{render_coordinate_system, render_crops, render_drag_points, render_grid, render_obstacles, render_spawn_area, render_stations, render_variant_field_configs, render_visibility_graph, ui_render_mouse_screen_scene_pos};
use crate::utilities::pos2::ExtendedPos2;
use crate::utilities::utils::generate_colors;


pub struct EditorTool {
    tick: u32,
    env: Env,
    camera: Camera,
}

impl Default for EditorTool {
    fn default() -> Self {
        let env = Env::new(0, Some(FieldConfig::default()));
        let mut instance = Self {
            tick: 0,
            env,
            camera: Camera::default(),
        };
        instance.recalc_charging_stations();
        instance.recalc_field_config_on_add_remove();
        
        instance
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
        render_obstacles(ui, &self.camera, &self.env.obstacles);
        render_visibility_graph(ui, &self.camera, &self.env.visibility_graph);
        render_crops(ui, &self.camera, &self.env.field.crops);
        render_variant_field_configs(ui, &self.camera, &self.env.field_config.configs);
        render_stations(ui, &self.camera, &self.env.stations);
        self.handle_dragging(ui);
    }

    fn render_ui(&mut self, ui: &mut Ui) {
        if ui.button("Save").clicked() {
            println!("Save not implemented");
        }
        
        ui_render_mouse_screen_scene_pos(ui, &self.camera);

        // ui.label(format!("Camera pos: {}", self.camera.position.fmt(2)));
        // ui.label(format!("Zoom: {}", self.camera.zoom_level));

        ui.separator();

        egui::CollapsingHeader::new("Fields")
            .default_open(true)
            .show(ui, |ui| {
                ui.horizontal_top(|ui| {
                    if ui.button("Add line").clicked() {
                        self.env.field_config.configs.push(VariantFieldConfig::Line(LineFieldConfig::default()));
                        self.recalc_field_config_on_add_remove();
                    }
                    if ui.button("Add point").clicked() {
                        self.env.field_config.configs.push(VariantFieldConfig::Point(PointFieldConfig::default()));
                        self.recalc_field_config_on_add_remove();
                    }
                    if ui.button("Remove all").clicked() {
                        self.env.field_config.configs.clear();
                        self.recalc_field_config_on_add_remove();
                    }
                });

                let mut to_remove: Option<usize> = None;
                let mut needs_recalc = false;

                for (i, config_variant) in self.env.field_config.configs.iter_mut().enumerate() {
                    match config_variant {
                        VariantFieldConfig::Line(config) => {
                            egui::CollapsingHeader::new({
                                let mut job = egui::text::LayoutJob::default();
                                    job.append(
                                        "⏺",
                                        0.0,
                                        egui::TextFormat {
                                            color: config.color,
                                            ..Default::default()
                                        },
                                    );
                                    job.append(
                                        format!(" Line {}", i).as_str(),
                                        0.0,
                                        egui::TextFormat::default(),
                                    );
                                    job
                            })
                                .default_open(false)
                                .show(ui, |ui| {
                                    ui.label(format!("Left top pos {}", config.left_top_pos.fmt(2)));
                                    let angle_response = ui.add(Slider::new(&mut config.angle, 0.0..=360.0).text("Angle").step_by(1.0));
                                    let n_lines_response = ui.add(Slider::new(&mut config.n_lines, 1..=10).text("N_lines").step_by(1.0));
                                    let line_spacing_response = ui.add(Slider::new(&mut config.line_spacing, 0.2..=0.8).text("Line spacing").step_by(0.05));
                                    let length_response = ui.add(Slider::new(&mut config.length, 1.0..=10.0).text("Length").step_by(0.05));
                                    if ui.button("Remove").clicked() {
                                        to_remove = Some(i);
                                    }
                                    if angle_response.changed() || 
                                        n_lines_response.changed() || 
                                        line_spacing_response.changed() || 
                                        length_response.changed() {
                                        needs_recalc = true;
                                    }
                            });

                        },
                        VariantFieldConfig::Point(config) => {
                            egui::CollapsingHeader::new({
                                let mut job = egui::text::LayoutJob::default();
                                    job.append(
                                        "⏺",
                                        0.0,
                                        egui::TextFormat {
                                            color: config.color,
                                            ..Default::default()
                                        },
                                    );
                                    job.append(
                                        format!(" Point {}", i).as_str(),
                                        0.0,
                                        egui::TextFormat::default(),
                                    );
                                    job
                            })
                                .default_open(false)
                                .show(ui, |ui| {
                                    ui.label(format!("Left top pos {}", config.left_top_pos.fmt(2)));
                                    let angle_response = ui.add(Slider::new(&mut config.angle, 0.0..=360.0).text("Angle").step_by(1.0));
                                    let n_lines_response = ui.add(Slider::new(&mut config.n_lines, 1..=10).text("N_lines").step_by(1.0));
                                    let line_spacing_response = ui.add(Slider::new(&mut config.line_spacing, 0.2..=0.8).text("Line spacing").step_by(0.05));
                                    let n_points_per_line_response = ui.add(Slider::new(&mut config.n_points_per_line, 1..=10).text("N points per line").step_by(1.0));
                                    let point_spacing_response = ui.add(Slider::new(&mut config.point_spacing, 0.2..=0.8).text("Point spacing").step_by(0.05));
                                    if ui.button("Remove").clicked() {
                                        to_remove = Some(i);
                                    }
                                    if angle_response.changed() || 
                                        n_lines_response.changed() || 
                                        line_spacing_response.changed() || 
                                        n_points_per_line_response.changed() ||
                                        point_spacing_response.changed() {
                                        needs_recalc = true;
                                    }
                            });
                        }
                    }

                }
                
                if let Some(index) = to_remove {
                    self.env.field_config.configs.remove(index);
                    self.recalc_field_config_on_add_remove();
                }
                
                if needs_recalc { self.recalc_field_config_on_param_changed(); }
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
                        ui.label(format!("Position {}", station.position.fmt(2)));
                        if ui.button("Remove").clicked() {
                            to_remove = Some(i);
                        }
                    });
                }

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
            *station = Station::new(i as u32, station.position, station.queue_direction, station.waiting_offset, colors[i], station.n_slots);
        }
    }
    pub fn recalc_field_config_on_add_remove(&mut self) {
        let colors = generate_colors(self.env.field_config.configs.len(), 0.1);
        for (i, config_variant) in self.env.field_config.configs.iter_mut().enumerate() {
            match config_variant {
                VariantFieldConfig::Line(config) => {
                    config.id = i as u32;
                    config.color = colors[i];
                },
                VariantFieldConfig::Point(config) => {
                    config.id = i as u32;
                    config.color = colors[i];
                },
            }
        }
        self.recalc_field_config_on_param_changed();
    }
    fn recalc_field_config_on_param_changed(&mut self) {
        self.env.obstacles = self.env.field_config.get_obstacles();
        self.env.visibility_graph.recalculate(&self.env.field_config.get_graph_points(), &self.env.obstacles);
    }

    fn handle_dragging(&mut self, ui: &mut Ui) {
        
        let mut pts = vec![];
        for station in &mut self.env.stations {
            let screen_pos = self.camera.scene_to_screen_pos(station.position);
            pts.push(screen_pos);
        }
        for config_variant in &mut self.env.field_config.configs {
            let left_top_pos = match config_variant {
                VariantFieldConfig::Line(config) => &config.left_top_pos,
                VariantFieldConfig::Point(config) => &config.left_top_pos,
            };
        
            let pos = self.camera.scene_to_screen_pos(*left_top_pos);
            pts.push(pos);
        }
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
        let mut needs_recalc = false;
        for (i, config_variant) in &mut self.env.field_config.configs.iter_mut().enumerate() {

            let left_top_pos = match config_variant {
                VariantFieldConfig::Line(config) => &mut config.left_top_pos,
                VariantFieldConfig::Point(config) => &mut config.left_top_pos,
            };
        
            let screen_pos = self.camera.scene_to_screen_pos(*left_top_pos);
            let rect = egui::Rect::from_center_size(screen_pos, egui::Vec2::splat(drag_point_size));
            let response = ui.interact(
                rect,
                ui.make_persistent_id(format!("field_drag_{}", i)),
                egui::Sense::click_and_drag(),
            );
        
            if response.dragged_by(egui::PointerButton::Primary) {
                let drag_delta = response.drag_delta();
                let new_screen_pos = screen_pos + drag_delta;
                let new_scene_pos = self.camera.screen_to_scene_pos(new_screen_pos);
                *left_top_pos = new_scene_pos;
                needs_recalc = true;
            }
        }
        if needs_recalc { self.recalc_field_config_on_param_changed(); }

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
