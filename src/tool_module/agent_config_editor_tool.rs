use egui::Ui;

use crate::{
    agent_module::agent_config::AgentConfig, cfg::{AGENT_CONFIGS_PATH, BATTERIES_PATH, DEFAULT_AGENT_CONFIG_PATH, MOVEMENT_CONFIGS_PATH}, tool_module::{has_config_saving::HasConfigSaving, has_help::HasHelp, tool::Tool}, utilities::utils::{folder_select_combo, json_config_combo, load_json_or_panic}
};


pub struct AgentConfigEditorTool {
    save_file_name: String,
    pub current_agent_config_path: String,
    pub current_movement_path: String,
    pub current_battery_path: String,
    pub current_battery_soc: f32,
    pub help_open: bool,
}

impl Default for AgentConfigEditorTool {
    fn default() -> Self {
        let file_path = DEFAULT_AGENT_CONFIG_PATH;
        let agent_config: AgentConfig = AgentConfig::from_json_file(file_path);

        Self {
            save_file_name: String::new(),
            current_agent_config_path: file_path.to_string(),
            current_movement_path: agent_config.movement,
            current_battery_path: agent_config.battery,
            current_battery_soc: agent_config.battery_soc,
            help_open: false,
        }
    }
}

impl Tool for AgentConfigEditorTool {
    fn render_main(&mut self, ui: &mut egui::Ui) {
        ui.label(r#"{"#);

        ui.horizontal(|ui| {
            ui.label(r#"   "movement":"#);
            self.ui_movement_select(ui)
        });

        ui.horizontal(|ui| {
            ui.label(r#"   "battery":"#);
            self.ui_battery_select(ui);
        });

        ui.horizontal(|ui| {
            ui.label(r#"   "battery_soc":"#);
            ui.add(egui::DragValue::new(&mut self.current_battery_soc).range(0.0..=100.0));
        });

        ui.label(r#"}"#);
    
    }

    fn render_ui(&mut self, ui: &mut egui::Ui) {
        self.render_help_button(ui);
        ui.separator();

        
        let mut save_file_name = self.save_file_name.clone();
        self.draw_save_ui(ui, &mut save_file_name);
        self.save_file_name = save_file_name;

        self.ui_agent_config_select(ui);

        self.render_help(ui);
    }

    fn update(&mut self) {}
}

impl AgentConfigEditorTool {
    fn ui_agent_config_select(&mut self, ui: &mut Ui) {
        let mut new_value = self.current_agent_config_path.clone();

        if json_config_combo(ui, "", &mut new_value, AGENT_CONFIGS_PATH)
            && new_value != self.current_agent_config_path
        {
            self.current_agent_config_path = new_value;
            let agent_config: AgentConfig = load_json_or_panic(self.current_agent_config_path.clone());
            self.current_movement_path = agent_config.movement;
            self.current_battery_path = agent_config.battery;
            self.current_battery_soc = agent_config.battery_soc;
        }
    }
    fn ui_movement_select(&mut self, ui: &mut egui::Ui) {
        let mut new_path = self.current_movement_path.clone();

        if json_config_combo(ui, "", &mut new_path, MOVEMENT_CONFIGS_PATH)
            && new_path != self.current_movement_path
        {
            self.current_movement_path = new_path;
        }
    }
    fn ui_battery_select(&mut self, ui: &mut egui::Ui) {
        let mut new_path = self.current_battery_path.clone();

        if folder_select_combo(
            ui,
            "battery_select",
            &mut new_path,
            BATTERIES_PATH,
        ) && new_path != self.current_battery_path
        {
            self.current_battery_path = new_path;
        }
    }
}

impl HasConfigSaving for AgentConfigEditorTool {
    fn base_path() -> &'static str {
        AGENT_CONFIGS_PATH
    }
    fn config(&self) -> impl serde::Serialize {
        AgentConfig::new(self.current_movement_path.clone(), self.current_battery_path.clone(), self.current_battery_soc)
    }
    fn update_current_path(&mut self, path: String) {
        self.current_agent_config_path = path;
    }
}

impl HasHelp for AgentConfigEditorTool {
    fn help_modal(&self) -> egui::Modal {
        egui::Modal::new(egui::Id::new("Agent Config Editor Tool Help"))
    }
    fn render_help_contents(&self, ui: &mut egui::Ui) {
        ui.heading("Agent Config Editor Tool Help");
        ui.label("This is a Agent Config Editor where you can see, change, create, save agent configs.");
        ui.separator();

        ui.label("movement: path to movement config (see MovementConfigEditor)");
        ui.label("battery: select what is available");
        ui.label("battery_soc: initial percent of charge in [%]");
    }
}