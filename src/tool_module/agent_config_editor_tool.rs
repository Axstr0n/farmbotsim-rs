use egui::Ui;

use crate::{
    agent_module::agent_config::AgentConfig, cfg::{AGENT_CONFIGS_PATH, BATTERIES_PATH, DEFAULT_AGENT_CONFIG_PATH, MOVEMENT_CONFIGS_PATH}, tool_module::{has_config_saving::HasConfigSaving, has_help::HasHelp, tool::Tool}, utilities::utils::{get_folders_in_folder, get_json_files_in_folder, load_json_or_panic}
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
            egui::ComboBox::from_id_salt("movement_dropdown")
                .selected_text(&self.current_movement_path)
                .show_ui(ui, |ui| {
                    let movement_options = get_json_files_in_folder(MOVEMENT_CONFIGS_PATH);
                    for movement in movement_options {
                        let whole_path = format!("{}{}", MOVEMENT_CONFIGS_PATH, movement);
                        ui.selectable_value(&mut self.current_movement_path, whole_path.clone(), whole_path);
                    }
                });
        });

        ui.horizontal(|ui| {
            ui.label(r#"   "battery":"#);
            egui::ComboBox::from_id_salt("battery_dropdown")
                .selected_text(&self.current_battery_path)
                .show_ui(ui, |ui| {
                    let battery_options = get_folders_in_folder(BATTERIES_PATH);
                    for battery in battery_options {
                        let whole_path = format!("{}{}", BATTERIES_PATH, battery);
                        ui.selectable_value(&mut self.current_battery_path, whole_path.clone(), whole_path);
                    }
                });
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

        self.config_select(ui);

        self.render_help(ui);
    }

    fn update(&mut self) {
        
    }
}

impl AgentConfigEditorTool {
    fn config_select(&mut self, ui: &mut Ui) {
        egui::ComboBox::from_label("")
            .selected_text(format!("{:?}", self.current_agent_config_path))
            .show_ui(ui, |ui| {
                let json_files = get_json_files_in_folder(AGENT_CONFIGS_PATH);
                let previous_value = self.current_agent_config_path.clone();

                for json_file in json_files {
                    let new_value = format!("{}{}", AGENT_CONFIGS_PATH, json_file.clone());
                    ui.selectable_value(&mut self.current_agent_config_path, new_value.clone(), json_file);
                }

                if *self.current_agent_config_path != previous_value {
                    let agent_config: AgentConfig = load_json_or_panic(self.current_agent_config_path.clone());
                    self.current_movement_path = agent_config.movement;
                    self.current_battery_path = agent_config.battery;
                    self.current_battery_soc = agent_config.battery_soc;
                }
            });
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