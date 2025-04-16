
use egui::Pos2;

use super::tool::Tool;
use crate::environment::env::Env;

use crate::environment::field_config::{FieldConfig, LineFieldConfig, PointFieldConfig, VariantFieldConfig};
use crate::rendering::camera::Camera;
use crate::rendering::render::{render_agents, render_coordinate_system, render_crops, render_grid, render_obstacles, render_spawn_area, render_stations, render_visibility_graph};
use crate::rendering::render::{ui_render_agents, ui_render_mouse_screen_scene_pos, ui_render_task_manager};
use crate::task::task_manager::TaskManager;

pub struct TaskTool {
    tick: u32,
    running: bool,
    env: Env,
    camera: Camera,
    task_manager: TaskManager,
}

impl Default for TaskTool {
    fn default() -> Self {

        let cfg = FieldConfig::new(vec![
            VariantFieldConfig::Line(LineFieldConfig::new(Pos2::new(3.0, 4.0), 0.0, 3, 5.0, 0.5)),
            VariantFieldConfig::Point(PointFieldConfig::new(Pos2::new(7.0, 4.0), 0.0, 5, 4, 0.4, 0.3)),
        ]);
        Self {
            tick: 0,
            running: false,
            env: Env::new(3, Some(cfg.clone())),
            camera: Camera::default(),
            task_manager: TaskManager::from_field_config(cfg),
        }
    }
}

impl Tool for TaskTool {
    fn render_main(&mut self, ui: &mut egui::Ui) {
        self.camera.handle_events(ui);
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
            self.reset();
        }

        ui.separator();

        if ui.button("Agent 0 to task 0").clicked() {
            self.assign_task(vec![0], vec![0]);
        }
        if ui.button("Agent 0 to task 3 combine line").clicked() {
            self.assign_task(vec![0], vec![3]);
        }

        ui.separator();

        ui_render_agents(ui, &self.env.agents);
        ui_render_task_manager(ui, &self.task_manager);
    }
    fn update(&mut self) {
        if self.running {
            self.tick += 1;
            self.env.step();
            for agent in &mut self.env.agents {
                self.task_manager.update_completed_tasks(agent);
            }
        }
    }
}

impl TaskTool {
    fn assign_task(&mut self, agent_ids: Vec<u32>, task_ids: Vec<u32>) {
        let tasks: Vec<_> = self.task_manager
            .work_list
            .iter()
            .filter(|task| task_ids.contains(task.get_id()))
            .cloned()
            .collect();

        for agent in &mut self.env.agents {
            if !agent_ids.contains(&agent.id) {
                continue;
            }
            for task in &tasks {
                self.task_manager.assign_task_to_agent(agent, task.clone());
            }
        }
        self.task_manager.work_list.retain(|task| !task_ids.contains(task.get_id()));
    }
    fn reset(&mut self) {
        self.tick = 0;
        self.running = false;
        self.env.reset();
        self.task_manager.reset();
    }
}