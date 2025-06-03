use std::fs::File;
use std::io::Write;
use chrono::{NaiveDate, NaiveTime, Timelike};
use serde_json::to_string_pretty;
use egui::{Slider, Ui};


use super::env_tool::EnvTool;
use crate::{
    environment::{
        datetime::{DATE_FORMAT, TIME_FORMAT},
        env_module::{
            env::Env,
            env_config::EnvConfig,
        },
        station_module::station::Station,
        field_config::{LineFieldConfig, PointFieldConfig, VariantFieldConfig},
    },
    rendering::{
        camera::Camera,
        render::{render_coordinate_system, render_drag_points, render_grid, render_obstacles, render_spawn_area, render_stations, render_field_config, render_visibility_graph},
        render::{ui_render_mouse_screen_scene_pos},
    },
    tool_module::tool::Tool,
    utilities::pos2::ExtendedPos2,
    cfg::{DEFAULT_ENV_CONFIG_PATH, ENV_CONFIGS_PATH},
};


pub struct EditorTool {
    tick: u32,
    pub env: Env,
    camera: Camera,
    save_file_name: String,
    pub current_env_config_string: String,
}

impl Default for EditorTool {
    fn default() -> Self {
        let env_config_string = DEFAULT_ENV_CONFIG_PATH.to_string();
        let env = Env::from_config(EnvConfig::from_json_file(&env_config_string).expect("Error"));
        let mut instance = Self {
            tick: 0,
            env,
            camera: Camera::default(),
            save_file_name: String::new(),
            current_env_config_string: env_config_string,
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
        render_field_config(ui, &self.camera, &self.env.field_config);
        render_stations(ui, &self.camera, &self.env.stations);
        self.handle_dragging(ui);
    }

    fn render_ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(ENV_CONFIGS_PATH);
            ui.add(egui::TextEdit::singleline(&mut self.save_file_name).desired_width(100.0));
            ui.label(".json");
            ui.spacing();
            if ui.button("Save env config").clicked() && !self.save_file_name.is_empty() {
                let _ = self.save_as_json(&self.save_file_name);
                self.current_env_config_string = format!("{}{}.json", ENV_CONFIGS_PATH, self.save_file_name.clone());
                self.create_env(self.current_env_config_string.clone());
            }
        });

        self.config_select(ui);

        ui.separator();
        
        ui_render_mouse_screen_scene_pos(ui, &self.camera);

        // ui.label(format!("Camera pos: {}", self.camera.position.fmt(2)));
        // ui.label(format!("Zoom: {}", self.camera.zoom_level));

        ui.separator();

        
        egui::CollapsingHeader::new("n_agents")
        .default_open(true)
        .show(ui, |ui| {
                ui.add(Slider::new(&mut self.env.n_agents, 0..=20).text("n_agents").step_by(1.0));
        });
        
        egui::CollapsingHeader::new("Datetime")
            .default_open(true)
            .show(ui, |ui| {
                ui.label(format!("{} {}", &self.env.datetime_config.date, &self.env.datetime_config.time));
                let mut date = NaiveDate::parse_from_str(&self.env.datetime_config.date, DATE_FORMAT).expect("");
                if ui.add(egui_extras::DatePickerButton::new(&mut date)).changed() {
                    self.env.datetime_config.date = date.format(DATE_FORMAT).to_string();
                }
                
                let time = NaiveTime::parse_from_str(&self.env.datetime_config.time, TIME_FORMAT).expect("Invalid time format");
                let mut hours = time.hour();
                let mut minutes = time.minute();
                let mut seconds = time.second();
                let mut changed = false;
                ui.horizontal(|ui| {
                    ui.label("Time:");
                    changed |= ui.add(egui::Slider::new(&mut hours, 0..=23).text("h")).changed();
                    changed |= ui.add(egui::Slider::new(&mut minutes, 0..=59).text("m")).changed();
                    changed |= ui.add(egui::Slider::new(&mut seconds, 0..=59).text("s")).changed();
                });
                if changed {
                    let combined = format!("{:02}:{:02}:{:02}", hours, minutes, seconds);
                    self.env.datetime_config.time = combined;
                }
            });

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
                                    let angle_response = ui.add(Slider::new(&mut config.angle.value, 0.0..=360.0).text(format!("Angle [{}]", config.angle.unit)).step_by(1.0));
                                    let n_lines_response = ui.add(Slider::new(&mut config.n_lines, 1..=10).text("N_lines").step_by(1.0));
                                    let line_spacing_response = ui.add(Slider::new(&mut config.line_spacing.value, 0.2..=0.8).text(format!("Line spacing [{}]", config.line_spacing.unit)).step_by(0.05));
                                    let length_response = ui.add(Slider::new(&mut config.length.value, 1.0..=10.0).text(format!("Length [{}]", config.length.unit)).step_by(0.05));
                                    ui.label(&config.crop_plan.crop_name);
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
                                    let angle_response = ui.add(Slider::new(&mut config.angle.value, 0.0..=360.0).text(format!("Angle [{}]", config.angle.unit)).step_by(1.0));
                                    let n_lines_response = ui.add(Slider::new(&mut config.n_lines, 1..=10).text("N_lines").step_by(1.0));
                                    let line_spacing_response = ui.add(Slider::new(&mut config.line_spacing.value, 0.2..=0.8).text(format!("Line spacing [{}]", config.line_spacing.unit)).step_by(0.05));
                                    let n_points_per_line_response = ui.add(Slider::new(&mut config.n_points_per_line, 1..=10).text("N points per line").step_by(1.0));
                                    let point_spacing_response = ui.add(Slider::new(&mut config.point_spacing.value, 0.2..=0.8).text(format!("Point spacing [{}]", config.point_spacing.unit)).step_by(0.05));
                                    ui.label(&config.crop_plan.crop_name);
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
                    &mut self.env.spawn_area.angle.value,
                    0.0..=360.0)
                    .text(format!("Angle [{}]", self.env.spawn_area.angle.unit))
                    .step_by(1.0)
                );
                ui.add(Slider::new(
                    &mut self.env.spawn_area.height.value,
                    1.0..=10.0)
                    .text(format!("Height [{}]", self.env.spawn_area.height.unit))
                    .step_by(0.1)
                );
                ui.add(Slider::new(
                    &mut self.env.spawn_area.width.value,
                    1.0..=10.0)
                    .text(format!("Width [{}]", self.env.spawn_area.width.unit))
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

    fn save_as_json(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let env_config = self.env.to_config();
        // Serialize data to pretty-printed JSON
        let json = to_string_pretty(&env_config)?;
        
        // Create file
        let mut file = File::create(format!("{}{}.json", ENV_CONFIGS_PATH, filename))?;
        
        // Write JSON to file
        file.write_all(json.as_bytes())?;
        
        println!("Successfully saved to {}", filename);
        Ok(())
    }
}
