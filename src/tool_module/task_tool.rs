use crate::{
    cfg::DEFAULT_ENV_CONFIG_PATH,
    environment::env_module::{
        env::Env,
        env_config::EnvConfig,
    },
    rendering::{
        camera::Camera,
        render::{render_agents, render_coordinate_system, render_grid, render_obstacles, render_spawn_area, render_stations, render_task_manager_on_field, render_visibility_graph, ui_render_agents, ui_render_mouse_screen_scene_pos, ui_render_stations, ui_render_task_manager},
    },
    tool_module::{
        env_tool::EnvTool, tool::Tool
    }
};

pub struct TaskTool {
    tick: u32,
    running: bool,
    pub env: Env,
    camera: Camera,
    pub current_env_config_string: String,
}

impl Default for TaskTool {
    fn default() -> Self {
        let env_config_string = DEFAULT_ENV_CONFIG_PATH.to_string();
        let env = Env::from_config(EnvConfig::from_json_file(&env_config_string).expect("Error"));
        let mut instance = Self {
            tick: 0,
            running: false,
            env,
            camera: Camera::default(),
            current_env_config_string: env_config_string,
        };
        instance.recalc_charging_stations();
        instance.recalc_field_config_on_add_remove();
        
        instance
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
        render_stations(ui, &self.camera, &self.env.stations);
        render_agents(ui, &self.camera, &self.env.agents);
    }
    fn render_ui(&mut self, ui: &mut egui::Ui) {
        self.config_select(ui);
        
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
        
        for i in 0..=self.env.agents.len()-1 {
            ui.horizontal(|ui| {
                if ui.button(format!("Agent {} ->  work", i)).clicked() {
                    let mut removed_from_station = false;
                    for agent in &mut self.env.agents {
                        if agent.id != i as u32 { continue; }
                        removed_from_station |= self.env.stations[0].release_agent(i as u32);
                        self.env.task_manager.assign_work_tasks_to_agent(agent);
                    }
                    if removed_from_station { self.env.task_manager.update_stations_on_agent_release(vec![0], &mut self.env.stations, &mut self.env.agents); }
                }
                if ui.button(format!("Agent {} ->  station 0", i)).clicked() {
                    for agent in &mut self.env.agents {
                        if agent.id != i as u32 { continue; }
                        if !self.env.stations[0].slots.contains(&agent.id) && !self.env.stations[0].queue.contains(&agent.id) {
                            self.env.task_manager.assign_station_tasks_to_agent(agent, &mut self.env.stations);
                        }
                    }
                }
                if ui.button(format!("Agent {} ->  spawn", i)).clicked() {
                    let mut removed_from_station = false;
                    for agent in &mut self.env.agents {
                        if agent.id != i as u32 { continue; }
                        removed_from_station |= self.env.stations[0].release_agent(i as u32);
                        self.env.task_manager.assign_idle_tasks_to_agent(agent);
                    }
                    if removed_from_station { self.env.task_manager.update_stations_on_agent_release(vec![0], &mut self.env.stations, &mut self.env.agents); }
                }
            });

        }
        ui.horizontal(|ui| {
            if ui.button("All -> work").clicked() {
                let mut removed_from_station = false;
                for agent in &mut self.env.agents {
                    removed_from_station |= self.env.stations[0].release_agent(agent.id);
                    self.env.task_manager.assign_work_tasks_to_agent(agent);
                }
                if removed_from_station { self.env.task_manager.update_stations_on_agent_release(vec![0], &mut self.env.stations, &mut self.env.agents); }
            }
            if ui.button("All -> station 0").clicked() {
                for agent in &mut self.env.agents {
                    if !self.env.stations[0].slots.contains(&agent.id) && !self.env.stations[0].queue.contains(&agent.id) {
                        self.env.task_manager.assign_station_tasks_to_agent(agent, &mut self.env.stations);
                    }
                }
            }
            if ui.button("All ->  spawn").clicked() {
                let mut removed_from_station = false;
                for agent in &mut self.env.agents {
                    removed_from_station |= self.env.stations[0].release_agent(agent.id);
                    self.env.task_manager.assign_idle_tasks_to_agent(agent);
                }
                if removed_from_station { self.env.task_manager.update_stations_on_agent_release(vec![0], &mut self.env.stations, &mut self.env.agents); }
            }
        });

        ui.separator();

        ui_render_agents(ui, &self.env.agents);
        ui.separator();
        ui_render_stations(ui, &self.env.stations);
        ui.separator();
        ui_render_task_manager(ui, &self.env.task_manager);
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

impl TaskTool {
    fn reset(&mut self) {
        self.tick = 0;
        self.running = false;
        self.env.reset();
    }
}