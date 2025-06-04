use crate::{
    cfg::{DEFAULT_ENV_CONFIG_PATH, MAX_VELOCITY}, environment::env_module::{
        env::Env,
        env_config::EnvConfig,
    }, path_finding_module::path_finding::PathFinding, rendering::{
        camera::Camera,
        render::{render_agents, render_coordinate_system, render_grid, render_obstacles, render_spawn_area, render_stations, render_visibility_graph, ui_render_agents_path},
    }, task_module::task::{Intent, Task}, tool_module::{
        has_env::HasEnv, has_env_controls::HasEnvControls, has_help::HasHelp, tool::Tool
    }
};


pub struct PathTool {
    pub tick: u32,
    pub running: bool,
    pub env: Env,
    pub camera: Camera,
    pub current_env_config_string: String,
    pub help_open: bool,
}

impl Default for PathTool {
    fn default() -> Self {
        let env_config_string = DEFAULT_ENV_CONFIG_PATH.to_string();
        let env = Env::from_config(EnvConfig::from_json_file(&env_config_string).expect("Error"));
        let mut instance = Self {
            tick: 0,
            running: false,
            env,
            camera: Camera::default(),
            current_env_config_string: env_config_string,
            help_open: false,
        };
        instance.recalc_charging_stations();
        instance.recalc_field_config_on_add_remove();
        
        instance
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
        render_stations(ui, &self.camera, &self.env.stations);
        render_agents(ui, &self.camera, &self.env.agents);
    }
    fn render_ui(&mut self, ui: &mut egui::Ui) {
        self.render_help_button(ui);
        ui.separator();

        self.ui_config_select(ui);
        ui.separator();

        self.ui_mouse_position(ui);
        ui.separator();
        
        self.ui_render_controls(ui);
        ui.separator();

        ui.label(egui::RichText::new("Env information:").size(16.0));
        ui_render_agents_path(ui, &self.env.agents);

        self.render_help(ui);
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
                        let task = Task::travel(p, MAX_VELOCITY, Intent::Idle);
                        agent.current_task = Some(task);
                    }
                }
            }
        }
    
    }
}

impl HasHelp for PathTool {
    fn help_modal(&self) -> egui::Modal {
        egui::Modal::new(egui::Id::new("Path Tool Help"))
    }
    fn render_help_contents(&self, ui: &mut egui::Ui) {
        ui.heading("Path Tool Help");
        ui.label("This is a path tool where you can test pathfinding with clicking on screen(env).");
        ui.separator();

        ui.label("Env config:");
        ui.label("In dropdown you can select env config.");
        ui.separator();

        ui.label("Mouse position:");
        ui.label("See where mouse is on screen and in env/scene.");
        ui.separator();

        ui.label("Env controls:");
        ui.label("Then you have start/pause/resume/reset controls for env as well as current env step count.");
        ui.separator();

        ui.label("Env information:");
        ui.label("Agents with information.");
    }
}

