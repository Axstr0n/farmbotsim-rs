use std::collections::HashSet;

use crate::{
    rendering::{
        camera::Camera,
        render::{
            render_agents, render_coordinate_system, render_grid, render_obstacles,
            render_spawn_area, render_stations, render_task_manager_on_field,
            render_visibility_graph, ui_render_agents, ui_render_datetime, ui_render_stations,
            ui_render_task_manager,
        },
    },
    tool_module::{
        has_camera::HasCamera, has_env::HasEnv, has_env_controls::HasEnvControls,
        has_help::HasHelp, tool::Tool,
    },
};
use farmbotsim_core::prelude::*;

/// A tool to set environment and test task assignment.
pub struct TaskTool {
    pub tick: u32,
    pub running: bool,
    pub env_config: EnvConfig,
    pub env: Env,
    pub camera: Camera,
    pub help_open: bool,
}

impl Default for TaskTool {
    fn default() -> Self {
        let env_config = EnvConfig::default();
        let env = Env::from_config(env_config.clone());
        Self {
            tick: 0,
            running: false,
            env_config,
            env,
            camera: Camera::default(),
            help_open: false,
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
        render_task_manager_on_field(ui, &self.camera, &self.env.task_manager);
        render_stations(ui, &self.camera, &self.env.stations, false);
        render_agents(ui, &self.camera, &self.env.agents);
    }
    fn render_ui(&mut self, ui: &mut egui::Ui) {
        self.render_help_button(ui);
        ui.separator();

        ui.label(egui::RichText::new("Env config:").size(16.0));
        // n_agents
        ui.horizontal(|ui| {
            ui.label("n_agents:");
            if ui
                .add(
                    egui::DragValue::new(&mut self.env_config.n_agents)
                        .speed(1)
                        .range(1..=10),
                )
                .changed()
            {
                self.rebuild_env();
            };
        });
        // agent_config_path
        ui.horizontal(|ui| {
            ui.label("agent_config_path:");
            self.ui_agent_config_select(ui);
        });
        // datetime
        ui.horizontal(|ui| {
            ui.label("datetime:");
            self.ui_datetime_select(ui);
        });
        // scene_config
        ui.horizontal(|ui| {
            ui.label("scene_config: ");
            self.ui_scene_config_select(ui);
        });
        //taskmanager
        ui.horizontal(|ui| {
            ui.label("task_manager_config_path:");
            self.ui_task_manager_config_select(ui);
        });
        ui.separator();

        self.ui_mouse_position(ui);
        ui.separator();

        self.ui_render_controls(ui);
        ui.separator();

        ui.label(egui::RichText::new("Manual task assignment:").size(16.0));
        let mut agent_ids_updated = HashSet::new();
        for i in 0..=self.env.agents.len() - 1 {
            ui.horizontal(|ui| {
                if ui.button(format!("Agent {i} ->  work")).clicked() {
                    let mut removed_from_station = false;
                    let mut station_ids_updated = HashSet::new();
                    for agent in &mut self.env.agents {
                        if agent.id != AgentId::new(i as u32) {
                            continue;
                        }
                        for station in &mut self.env.stations {
                            let removed = station.release_agent(agent.id);
                            removed_from_station |= removed;
                            if removed {
                                station_ids_updated.insert(station.id);
                            }
                        }
                        self.env.task_manager.assign_work_tasks_to_agent(agent);
                    }
                    if removed_from_station {
                        self.env.task_manager.update_stations_on_agent_release(
                            station_ids_updated,
                            &mut agent_ids_updated,
                            &mut self.env.stations,
                            &mut self.env.agents,
                        );
                    }
                }
                if ui.button(format!("Agent {i} ->  station")).clicked() {
                    for agent in &mut self.env.agents {
                        if agent.id != AgentId::new(i as u32) {
                            continue;
                        }
                        if let Some(task) = &agent.current_task {
                            if *task.get_intent() != Intent::Queue
                                && *task.get_intent() != Intent::Charge
                            {
                                self.env
                                    .task_manager
                                    .assign_station_tasks_to_agent(agent, &mut self.env.stations);
                            }
                        } else {
                            self.env
                                .task_manager
                                .assign_station_tasks_to_agent(agent, &mut self.env.stations);
                        }
                    }
                }
                if ui.button(format!("Agent {i} ->  spawn")).clicked() {
                    let mut station_ids_updated = HashSet::new();
                    let mut removed_from_station = false;
                    for agent in &mut self.env.agents {
                        if agent.id != AgentId::new(i as u32) {
                            continue;
                        }
                        for station in &mut self.env.stations {
                            let removed = station.release_agent(agent.id);
                            removed_from_station |= removed;
                            if removed {
                                station_ids_updated.insert(station.id);
                            }
                        }
                        self.env.task_manager.assign_idle_tasks_to_agent(agent);
                    }
                    if removed_from_station {
                        self.env.task_manager.update_stations_on_agent_release(
                            station_ids_updated,
                            &mut agent_ids_updated,
                            &mut self.env.stations,
                            &mut self.env.agents,
                        );
                    }
                }
            });
        }
        ui.horizontal(|ui| {
            if ui.button("All -> work").clicked() {
                let mut removed_from_station = false;
                let mut station_ids_updated = HashSet::new();
                for agent in &mut self.env.agents {
                    for station in &mut self.env.stations {
                        let removed = station.release_agent(agent.id);
                        removed_from_station |= removed;
                        if removed {
                            station_ids_updated.insert(station.id);
                        }
                    }
                    self.env.task_manager.assign_work_tasks_to_agent(agent);
                }
                if removed_from_station {
                    self.env.task_manager.update_stations_on_agent_release(
                        station_ids_updated,
                        &mut agent_ids_updated,
                        &mut self.env.stations,
                        &mut self.env.agents,
                    );
                }
            }
            if ui.button("All -> station").clicked() {
                for agent in &mut self.env.agents {
                    if let Some(task) = &agent.current_task {
                        if *task.get_intent() != Intent::Queue
                            && *task.get_intent() != Intent::Charge
                        {
                            self.env
                                .task_manager
                                .assign_station_tasks_to_agent(agent, &mut self.env.stations);
                        }
                    } else {
                        self.env
                            .task_manager
                            .assign_station_tasks_to_agent(agent, &mut self.env.stations);
                    }
                }
            }
            if ui.button("All ->  spawn").clicked() {
                let mut station_ids_updated = HashSet::new();
                let mut removed_from_station = false;
                for agent in &mut self.env.agents {
                    for station in &mut self.env.stations {
                        let removed = station.release_agent(agent.id);
                        removed_from_station |= removed;
                        if removed {
                            station_ids_updated.insert(station.id);
                        }
                    }
                    self.env.task_manager.assign_idle_tasks_to_agent(agent);
                }
                if removed_from_station {
                    self.env.task_manager.update_stations_on_agent_release(
                        station_ids_updated,
                        &mut agent_ids_updated,
                        &mut self.env.stations,
                        &mut self.env.agents,
                    );
                }
            }
        });

        ui.separator();

        ui.label(egui::RichText::new("Env information:").size(16.0));
        ui_render_datetime(ui, &self.env.date_time_manager);
        ui_render_agents(ui, &self.env.agents, true);
        ui_render_stations(ui, &self.env.stations);
        ui_render_task_manager(ui, &self.env.task_manager);
        ui.separator();

        self.render_help(ui);
    }
    fn update(&mut self) {
        if self.running {
            self.tick += 1;
            self.env.step();
            for agent in &mut self.env.agents {
                self.env.task_manager.update_completed_tasks(agent);
            }
        }
    }
}

impl HasHelp for TaskTool {
    fn help_modal(&self) -> egui::Modal {
        egui::Modal::new(egui::Id::new("Task Tool Help"))
    }
    fn render_help_contents(&self, ui: &mut egui::Ui) {
        ui.heading("Task Tool Help");
        ui.label("This is a Task tool where you can manually control what agents do.");
        ui.separator();

        ui.label("Env config:");
        ui.label("Configure env config.");
        ui.separator();

        ui.label("Env controls:");
        ui.label("Then you have start/pause/resume/reset controls for env as well as current env step count.");
        ui.separator();

        ui.label("Manual task assignment:");
        ui.label("For each agent you can send him to spawn, work or charge.");
        ui.separator();

        ui.label("Env information:");
        ui.label("Date time to keep track of time progression.");
        ui.label("Agents are represented with table with their information.");
        ui.label("Stations are represented in table with information.");
        ui.label("Task manager with available, assigned, completed tasks");
    }
}
