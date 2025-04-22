
use super::tool::Tool;
use crate::environment::env::Env;

use crate::environment::field_config::FieldConfig;
use crate::rendering::camera::Camera;
use crate::rendering::render::{render_agents, render_coordinate_system, render_crops, render_grid, render_obstacles, render_spawn_area, render_stations, render_visibility_graph};
use crate::rendering::render::{ui_render_agents_path, ui_render_mouse_screen_scene_pos};
use crate::task::task::Task;

pub struct PathTool {
    tick: u32,
    running: bool,
    env: Env,
    camera: Camera,
}

impl Default for PathTool {
    fn default() -> Self {
        Self {
            tick: 0,
            running: false,
            env: Env::new(1, Some(FieldConfig::default()), ""),
            camera: Camera::default(),
        }
    }
}

impl Tool for PathTool {
    fn render_main(&mut self, ui: &mut egui::Ui) {
        self.camera.handle_events(ui);
        self.assign_path_on_mouse_click(ui);
        render_grid(ui, &self.camera);
        render_coordinate_system(ui, &self.camera);
        render_spawn_area(ui, &self.camera, &self.env.spawn_area);
        render_visibility_graph(ui, &self.camera, &self.env.visibility_graph);
        render_obstacles(ui, &self.camera, &self.env.obstacles);
        render_crops(ui, &self.camera, &self.env.field.crops);
        render_stations(ui, &self.camera, &self.env.stations);
        render_agents(ui, &self.camera, &self.env.agents);
    }
    fn render_ui(&mut self, ui: &mut egui::Ui) {
        ui_render_mouse_screen_scene_pos(ui, &self.camera);
        
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

        ui.separator();
        ui_render_agents_path(ui, &self.env.agents);
    }
    fn update(&mut self) {
        if self.running {
            self.tick += 1;
            self.env.step();
        }
    }
}

impl PathTool {
    pub fn assign_path_on_mouse_click(&mut self, ui: &egui::Ui) {
        let response = ui.interact(ui.available_rect_before_wrap(), ui.id(), egui::Sense::click_and_drag());
        let mouse_position = response.hover_pos();
        if let Some(mouse_position) = mouse_position {
            if response.clicked_by(egui::PointerButton::Primary) {
                println!("Set path");
                let scene_pos = self.camera.screen_to_scene_pos(mouse_position);
                for agent in &mut self.env.agents {
                    let path = self.env.visibility_graph.find_path(agent.position, scene_pos);
                    if let Some(p) = path {
                        let task = Task::travel(p, 2.0, crate::task::task::Intent::Idle);
                        agent.current_task = Some(task);
                    }
                }
            }
        }
    
    }
}

