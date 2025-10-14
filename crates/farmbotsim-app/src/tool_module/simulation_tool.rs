use farmbotsim_core::environment::env_module::{env::Env, env_config::EnvConfig};

use crate::{
    rendering::{
        camera::Camera,
        render::{
            render_agents, render_coordinate_system, render_grid, render_obstacles,
            render_spawn_area, render_stations, render_task_manager_on_field, ui_render_agents,
            ui_render_datetime, ui_render_stations, ui_render_task_manager,
        },
    },
    tool_module::{
        has_env::HasEnv, has_env_controls::HasEnvControls, has_help::HasHelp, tool::Tool,
    },
};

/// A tool to set and view simulation in action.
pub struct SimulationTool {
    pub tick: u32,
    pub running: bool,
    pub env_config: EnvConfig,
    pub env: Env,
    pub camera: Camera,
    pub help_open: bool,
    pub show_battery_plot: bool,
}

impl Default for SimulationTool {
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
            show_battery_plot: false,
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

        self.ui_render_controls(ui);
        ui.separator();

        ui.checkbox(&mut self.show_battery_plot, "Battery plot");
        ui.label(egui::RichText::new("Env information:").size(16.0));
        ui_render_datetime(ui, &self.env.date_time_manager);
        ui_render_agents(ui, &self.env.agents, self.show_battery_plot);
        ui_render_stations(ui, &self.env.stations);
        ui_render_task_manager(ui, &self.env.task_manager);

        self.render_help(ui);
    }
    fn update(&mut self) {
        if self.running {
            self.tick += 1;
            self.env
                .task_manager
                .assign_tasks(&mut self.env.agents, &mut self.env.stations);
            self.env.step();
        }
    }
}

impl HasHelp for SimulationTool {
    fn help_modal(&self) -> egui::Modal {
        egui::Modal::new(egui::Id::new("Simulation Tool Help"))
    }
    fn render_help_contents(&self, ui: &mut egui::Ui) {
        ui.heading("Simulation Tool Help");
        ui.label("This is a simulation tool where you set and run simulation.");
        ui.separator();

        ui.label("Env config:");
        ui.label("Configure env config.");
        ui.separator();

        ui.label("Env controls:");
        ui.label("Then you have start/pause/resume/reset controls for env as well as current env step count.");
        ui.separator();

        ui.label("Env information:");
        ui.label("Date time to keep track of time progression.");
        ui.label("Agents are represented with table with their information.");
        ui.label("Stations are represented in table with information.");
        ui.label("Task manager with available, assigned, completed tasks");
    }
}
