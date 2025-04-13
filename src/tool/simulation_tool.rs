
use super::tool::Tool;
use crate::environment::env::Env;

use crate::environment::field_config::FieldConfig;
use crate::rendering::camera::Camera;
use crate::rendering::render::{render_agents, render_coordinate_system, render_crops, render_grid, render_obstacles, render_stations, ui_render_stations};
use crate::rendering::render::ui_render_agents;

pub struct SimulationTool {
    tick: u32,
    running: bool,
    env: Env,
    camera: Camera,
}

impl Default for SimulationTool {
    fn default() -> Self {
        Self {
            tick: 0,
            running: false,
            env: Env::new(2, Some(FieldConfig::default())),
            camera: Camera::default(),
        }
    }
}


impl Tool for SimulationTool {
    fn render_main(&mut self, ui: &mut egui::Ui) {
        self.camera.handle_events(ui);
        render_grid(ui, &self.camera);
        render_coordinate_system(ui, &self.camera);
        render_obstacles(ui, &self.camera, &self.env.obstacles);
        render_crops(ui, &self.camera, &self.env.field.crops);
        render_stations(ui, &self.camera, &self.env.stations);
        render_agents(ui, &self.camera, &self.env.agents);
    }
    fn render_ui(&mut self, ui: &mut egui::Ui) {
        
        ui.label(format!("Running: {}", self.running));
        ui.label(format!("Env_step: {}", self.env.step_count));

        if !self.running {
            if ui.button("Start").clicked() {
                self.running = true;
            }
        } else if ui.button("Pause").clicked() {
            self.running = false;
        }
        if ui.button("Reset").clicked() {
            self.tick = 0;
            self.running = false;
            self.env.reset();
        }
        ui.label("NOT IMPLEMENTED !!!!!");
        ui.separator();
        ui_render_agents(ui, &self.env.agents);
        ui.separator();
        ui_render_stations(ui, &self.env.stations);
    }
    fn update(&mut self) {
        if self.running {
            self.tick += 1;
            self.env.step();
        }
    }
}
