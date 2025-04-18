
use super::tool::Tool;
use crate::environment::env::Env;

use crate::environment::field_config::FieldConfig;
use crate::rendering::camera::Camera;
use crate::rendering::render::{render_agents, render_coordinate_system, render_crops, render_grid, render_obstacles, render_spawn_area, render_stations, render_tasks_on_field};
use crate::rendering::render::{ui_render_agents, ui_render_stations, ui_render_task_manager};
use crate::task::task_manager::TaskManager;

pub struct SimulationTool {
    tick: u32,
    running: bool,
    env: Env,
    camera: Camera,
    task_manager: TaskManager,
}

impl Default for SimulationTool {
    fn default() -> Self {
        let field_config = FieldConfig::default();
        Self {
            tick: 0,
            running: false,
            env: Env::new(4, Some(field_config.clone())),
            camera: Camera::default(),
            task_manager: TaskManager::from_field_config(field_config)
        }
    }
}


impl Tool for SimulationTool {
    fn render_main(&mut self, ui: &mut egui::Ui) {
        self.camera.handle_events(ui);
        render_grid(ui, &self.camera);
        render_coordinate_system(ui, &self.camera);
        render_spawn_area(ui, &self.camera, &self.env.spawn_area);
        render_obstacles(ui, &self.camera, &self.env.obstacles);
        render_crops(ui, &self.camera, &self.env.field.crops);
        render_tasks_on_field(ui, &self.camera, &self.task_manager.all_tasks);
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
            self.reset();
        }
        ui.separator();
        ui_render_agents(ui, &self.env.agents);
        ui.separator();
        ui_render_stations(ui, &self.env.stations);
        ui.separator();
        ui_render_task_manager(ui, &self.task_manager);
    }
    fn update(&mut self) {
        if self.running {
            self.tick += 1;
            self.task_manager.assign_tasks(&mut self.env.agents, &mut self.env.stations);
            self.env.step();
        }
    }
}
impl SimulationTool {
    fn reset(&mut self) {
        self.tick = 0;
        self.running = false;
        self.env.reset();
        self.task_manager.reset();
    }
}
