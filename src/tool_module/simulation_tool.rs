use crate::{
    environment::env_module::{
        env::Env,
        env_config::EnvConfig,
    },
    rendering::{
        camera::Camera,
        render::{render_agents, render_coordinate_system, render_grid, render_obstacles, render_spawn_area, render_stations, render_task_manager_on_field},
        render::{ui_render_agents, ui_render_datetime, ui_render_stations, ui_render_task_manager},
    },
    tool_module::{
        env_tool::EnvTool, tool::Tool
    },
    cfg::DEFAULT_ENV_CONFIG_PATH,
};

pub struct SimulationTool {
    tick: u32,
    running: bool,
    pub env: Env,
    camera: Camera,
    pub current_env_config_string: String,
}

impl Default for SimulationTool {
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


impl Tool for SimulationTool {
    fn render_main(&mut self, ui: &mut egui::Ui) {
        self.camera.handle_events(ui);
        render_grid(ui, &self.camera);
        render_coordinate_system(ui, &self.camera);
        render_spawn_area(ui, &self.camera, &self.env.spawn_area);
        render_obstacles(ui, &self.camera, &self.env.obstacles);
        render_task_manager_on_field(ui, &self.camera, &self.env.task_manager);
        render_stations(ui, &self.camera, &self.env.stations);
        render_agents(ui, &self.camera, &self.env.agents);
    }
    fn render_ui(&mut self, ui: &mut egui::Ui) {

        self.config_select(ui);
        
        ui.label(format!("Running: {}", self.running));
        ui.label(format!("Env_step: {}", self.env.step_count));

        if !self.running {
            if self.tick == 0 {
                if ui.button("Start").clicked() {
                    self.running = true;
                } 
            } else if ui.button("Resume").clicked() {
                self.running = true;
            }
        } else if ui.button("Pause").clicked() {
            self.running = false;
        }
        if ui.button("Reset").clicked() {
            self.reset();
        }
        ui.separator();
        ui_render_datetime(ui, &self.env.date_time_manager);
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
            self.env.task_manager.assign_tasks(&mut self.env.agents, &mut self.env.stations);
            self.env.step();
        }
    }
}
impl SimulationTool {
    fn reset(&mut self) {
        self.tick = 0;
        self.running = false;
        self.env.reset();
    }
}
