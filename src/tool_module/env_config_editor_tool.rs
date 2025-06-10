use chrono::{NaiveDate, NaiveTime, Timelike};
use egui::{Slider, Ui};


use crate::cfg::{AGENT_CONFIGS_PATH, DEFAULT_ENV_CONFIG_PATH, ENV_CONFIGS_PATH, SCENE_CONFIGS_PATH};
use crate::environment::datetime::{DATE_FORMAT, TIME_FORMAT};
use crate::environment::env_module::env_config::EnvConfig;
use crate::environment::field_config::FieldConfig;
use crate::environment::scene_config::SceneConfig;
use crate::environment::spawn_area_module::spawn_area::SpawnArea;
use crate::logger::log_error_and_panic;
use crate::path_finding_module::visibility_graph::VisibilityGraph;
use crate::rendering::render::render_station;
use crate::task_module::task_manager::{ChargingStrat, ChooseStationStrat};
use crate::tool_module::has_camera::HasCamera;
use crate::tool_module::has_config_saving::HasConfigSaving;
use crate::utilities::utils::load_json_or_panic;
use crate::{
    environment::{
        station_module::station::Station
    }, rendering::{
        camera::Camera,
        render::{render_coordinate_system, render_field_config, render_grid, render_obstacles, render_spawn_area, render_visibility_graph},
    }, tool_module::{has_help::HasHelp, tool::Tool}, utilities::utils::get_json_files_in_folder
};


pub struct EnvConfigEditorTool {
    env_config: EnvConfig,
    scene_config: SceneConfig,
    field_config: FieldConfig,
    pub camera: Camera,
    save_file_name: String,
    pub current_env_config_path: String,
    pub help_open: bool,
}

impl Default for EnvConfigEditorTool {
    fn default() -> Self {
        let env_config: EnvConfig = load_json_or_panic(DEFAULT_ENV_CONFIG_PATH);
        let scene_config: SceneConfig = load_json_or_panic(env_config.scene_config_path.clone());
        let mut field_config: FieldConfig = load_json_or_panic(scene_config.field_config_path.clone());
        field_config.recalc_id_color();
        Self {
            env_config,
            scene_config,
            field_config,
            camera: Camera::default(),
            save_file_name: String::new(),
            current_env_config_path: DEFAULT_ENV_CONFIG_PATH.to_string(),
            help_open: false,
        }
    }
}

impl Tool for EnvConfigEditorTool {
    fn update(&mut self) {}

    fn render_main(&mut self, ui: &mut Ui) {
        self.camera.handle_events(ui);
        render_grid(ui, &self.camera);
        render_coordinate_system(ui, &self.camera);
        render_spawn_area(ui, &self.camera, &SpawnArea::from_config(self.scene_config.spawn_area_config.clone()));
        render_obstacles(ui, &self.camera, &self.field_config.get_obstacles());
        render_visibility_graph(ui, &self.camera, &VisibilityGraph::new(&self.field_config.get_graph_points(), self.field_config.get_obstacles()));
        render_field_config(ui, &self.camera, &self.field_config);
        for station_config in &self.scene_config.station_configs {
            render_station(ui, &self.camera, Station::from_config(0, egui::Color32::WHITE, station_config.clone()));
        }
    }

    fn render_ui(&mut self, ui: &mut Ui) {
        self.render_help_button(ui);
        ui.separator();

        self.ui_env_config_select(ui);
        
        let mut save_file_name = self.save_file_name.clone();
        self.draw_save_ui(ui, &mut save_file_name);
        self.save_file_name = save_file_name;
        ui.separator();
        
        self.ui_mouse_position(ui);
        ui.separator();
        

        ui.label(egui::RichText::new("Agents:").size(16.0));
        ui.add(Slider::new(&mut self.env_config.n_agents, 0..=20).text("n_agents").step_by(1.0));
        egui::ComboBox::from_label("agent_config_path")
            .selected_text(&self.env_config.agent_config_path)
            .show_ui(ui, |ui| {
                let json_files = get_json_files_in_folder(AGENT_CONFIGS_PATH);
                for json_file in json_files {
                    let whole_path = format!("{}{}", AGENT_CONFIGS_PATH, json_file.clone());
                    ui.selectable_value(&mut self.env_config.agent_config_path, whole_path.clone(), whole_path);
                }
            });
        
        ui.label(egui::RichText::new("Datetime:").size(16.0));
        ui.label(format!("{} {}", &self.env_config.datetime_config.date, &self.env_config.datetime_config.time));
                
        let mut date = NaiveDate::parse_from_str(&self.env_config.datetime_config.date, DATE_FORMAT)
            .unwrap_or_else(|e| {
                let msg = format!(
                    "Failed to parse date '{}' with format '{}': {}",
                    &self.env_config.datetime_config.date, DATE_FORMAT, e
                );
                log_error_and_panic(&msg)
            });
        if ui.add(egui_extras::DatePickerButton::new(&mut date)).changed() {
            self.env_config.datetime_config.date = date.format(DATE_FORMAT).to_string();
        }
        
        let time = NaiveTime::parse_from_str(&self.env_config.datetime_config.time, TIME_FORMAT)
            .unwrap_or_else(|e| {
                let msg = format!(
                    "Failed to parse time '{}' with format '{}': {}",
                    &self.env_config.datetime_config.time, TIME_FORMAT, e
                );
                log_error_and_panic(&msg);
            });
        let mut hours = time.hour();
        let mut minutes = time.minute();
        let mut seconds = time.second();
        let mut changed = false;
        ui.horizontal(|ui| {
            ui.label("Time:");
            changed |= ui.add(egui::Slider::new(&mut hours, 0..=23).text("h")).changed();
            changed |= ui.add(egui::Slider::new(&mut minutes, 0..=59).text("m")).changed();
            changed |= ui.add(egui::Slider::new(&mut seconds, 0..=59).text("s")).changed();
        });
        if changed {
            let combined = format!("{:02}:{:02}:{:02}", hours, minutes, seconds);
            self.env_config.datetime_config.time = combined;
        }

        self.ui_scene_config_select(ui);

        ui.label(egui::RichText::new("Task Manager:").size(16.0));
        egui::ComboBox::from_label("Choose Station Strategy")
            .selected_text(self.env_config.task_manager_config.choose_station_strat.to_string())
            .show_ui(ui, |ui| {
                let choose_station_options = ChooseStationStrat::variants();
                for strat in choose_station_options {
                    ui.selectable_value(&mut self.env_config.task_manager_config.choose_station_strat, strat.clone(), strat.clone().to_string());
                }
            });
        egui::ComboBox::from_label("Charging Strategy")
            .selected_text(self.env_config.task_manager_config.charging_strat.to_string())
            .show_ui(ui, |ui| {
                let charge_strat_options = ChargingStrat::variants();
                for strat in charge_strat_options {
                    ui.selectable_value(&mut self.env_config.task_manager_config.charging_strat, strat.clone(), strat.clone().to_string());
                }
            });
        
        
        self.render_help(ui);
    }
    
}

impl EnvConfigEditorTool {
    fn change_scene_config(&mut self) {
        let scene_config: SceneConfig = load_json_or_panic(self.env_config.scene_config_path.clone());
        self.scene_config = scene_config;
        self.field_config = load_json_or_panic(self.scene_config.field_config_path.clone());
        self.field_config.recalc_id_color();
    }
    fn change_env_config(&mut self) {
        let env_config: EnvConfig = load_json_or_panic(self.current_env_config_path.clone());
        self.env_config = env_config;
        self.scene_config = load_json_or_panic(self.env_config.scene_config_path.clone());
        self.field_config = load_json_or_panic(self.scene_config.field_config_path.clone());
        self.field_config.recalc_id_color();
    }

    fn ui_scene_config_select(&mut self, ui: &mut Ui) {
        ui.label(egui::RichText::new("Scene config:").size(16.0));
        egui::ComboBox::from_label(" ")
            .selected_text(format!("{:?}", self.env_config.scene_config_path))
            .show_ui(ui, |ui| {
                let json_files = get_json_files_in_folder(SCENE_CONFIGS_PATH);
                let previous_value = self.env_config.scene_config_path.clone();

                for json_file in json_files {
                    let new_value = format!("{}{}", SCENE_CONFIGS_PATH, json_file.clone());
                    ui.selectable_value(&mut self.env_config.scene_config_path, new_value.clone(), json_file);
                }

                if *self.env_config.scene_config_path != previous_value {
                    self.change_scene_config();
                }
            });
    }
    
    fn ui_env_config_select(&mut self, ui: &mut Ui) {
        ui.label(egui::RichText::new("Env config:").size(16.0));
        egui::ComboBox::from_label("  ")
            .selected_text(format!("{:?}", self.current_env_config_path))
            .show_ui(ui, |ui| {
                let json_files = get_json_files_in_folder(ENV_CONFIGS_PATH);
                let previous_value = self.current_env_config_path.clone();

                for json_file in json_files {
                    let new_value = format!("{}{}", ENV_CONFIGS_PATH, json_file.clone());
                    ui.selectable_value(&mut self.current_env_config_path, new_value.clone(), json_file);
                }

                if *self.current_env_config_path != previous_value {
                    self.change_env_config();
                }
            });
    }
}

impl HasConfigSaving for EnvConfigEditorTool {
    fn base_path() -> &'static str {
        ENV_CONFIGS_PATH
    }
    fn config(&self) -> impl serde::Serialize {
        self.env_config.clone()
    }
    fn update_current_path(&mut self, path: String) {
        self.current_env_config_path = path;
    }
    fn update_after_save(&mut self) {
        self.change_scene_config();
    }
}

impl HasHelp for EnvConfigEditorTool {
    fn help_modal(&self) -> egui::Modal {
        egui::Modal::new(egui::Id::new("Env Config Editor Tool Help"))
    }
    fn render_help_contents(&self, ui: &mut egui::Ui) {
        ui.heading("Env Config Editor Tool Help");
        ui.label("This is a Env Config Editor Tool where you can create, adjust and save env config.");
        ui.separator();

        ui.label("Env config:");
        ui.label("In dropdown you can select env config and save new config");
        ui.separator();

        ui.label("Mouse position:");
        ui.label("See where mouse is on screen and in env/scene.");
        ui.separator();

        ui.label("Agents:");
        ui.label("Set number of agents.");
        ui.label("Set agent config (see AgentConfigEditor).");
        ui.separator();

        ui.label("Datetime:");
        ui.label("Set date and time");
        ui.separator();

        ui.label("Scene config:");
        ui.label("Select scene config (see SceneConfigEditor)");
        ui.separator();

        ui.label("Task Manager:");
        ui.label("Select strategy for choosing station and charging");


    }
}