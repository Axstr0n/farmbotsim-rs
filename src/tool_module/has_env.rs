use chrono::{NaiveDate, NaiveTime, Timelike};

use crate::{
    cfg::{AGENT_CONFIGS_PATH, SCENE_CONFIGS_PATH}, environment::{
        datetime::{DATE_FORMAT, TIME_FORMAT}, env_module::{env::Env, env_config::EnvConfig},
    }, logger::log_error_and_panic, task_module::strategies::{ChargingStrategy, ChooseStationStrategy}, utilities::utils::json_config_combo
};

/// Trait for managing and interacting with an environment (`Env`) and its configuration (`EnvConfig`).
pub trait HasEnv {
    /// Set whether the environment is running.
    fn set_running(&mut self, value: bool);

    /// Set the current tick count.
    fn set_tick(&mut self, value: u32);

    /// Replace the current environment with a new instance.
    fn set_env(&mut self, value: Env);


    /// Get a reference to the current environment configuration.
    fn get_env_config(&self) -> &EnvConfig;

    /// Get a mutable reference to the current environment configuration.
    fn get_mut_env_config(&mut self) -> &mut EnvConfig;


    /// Rebuilds the environment from the current environment configuration.
    fn rebuild_env(&mut self) {
        self.set_running(false);
        self.set_tick(0);
        self.set_env(Env::from_config(self.get_env_config().clone()))
    }
    
    /// Render a UI widget to select the scene configuration file.
    fn ui_scene_config_select(&mut self, ui: &mut egui::Ui) {
        let mut new_value = self.get_env_config().scene_config_path.clone();

        if json_config_combo(ui, "", &mut new_value, SCENE_CONFIGS_PATH)
            && new_value != self.get_env_config().scene_config_path
        {
            self.get_mut_env_config().scene_config_path = new_value;
            self.rebuild_env();
        }
    }
    
    /// Render a UI widget to select the agent configuration file.
    fn ui_agent_config_select(&mut self, ui: &mut egui::Ui) {
        let mut new_value = self.get_env_config().agent_config_path.clone();

        if json_config_combo(ui, " ", &mut new_value, AGENT_CONFIGS_PATH)
            && new_value != self.get_env_config().agent_config_path
        {
            self.get_mut_env_config().agent_config_path = new_value;
            self.rebuild_env();
        }
    }
    
    /// Render UI controls to select and edit the environment date and time.
    fn ui_datetime_select(&mut self, ui: &mut egui::Ui) {
        let config = self.get_env_config().clone();
        ui.label(format!("{} {} |", config.datetime_config.date, config.datetime_config.time));

        let mut date = NaiveDate::parse_from_str(&config.datetime_config.date, DATE_FORMAT)
            .unwrap_or_else(|e| {
                let msg = format!(
                    "Failed to parse date '{}' with format '{}': {}",
                    &config.datetime_config.date, DATE_FORMAT, e
                );
                log_error_and_panic(&msg)
            });

        let mut changed = false;
        if ui.add(egui_extras::DatePickerButton::new(&mut date)).changed() {
            self.get_mut_env_config().datetime_config.date = date.format(DATE_FORMAT).to_string();
            changed = true;
        }

        ui.label("|");

        let time = NaiveTime::parse_from_str(&config.datetime_config.time, TIME_FORMAT)
            .unwrap_or_else(|e| {
                let msg = format!(
                    "Failed to parse time '{}' with format '{}': {}",
                    &config.datetime_config.time, TIME_FORMAT, e
                );
                log_error_and_panic(&msg);
            });

        let mut hours = time.hour();
        let mut minutes = time.minute();
        let mut seconds = time.second();

        ui.horizontal(|ui| {
            changed |= ui.add(egui::DragValue::new(&mut hours).range(0..=23)).changed();
            ui.label("h");
            changed |= ui.add(egui::DragValue::new(&mut minutes).range(0..=59)).changed();
            ui.label("m");
            changed |= ui.add(egui::DragValue::new(&mut seconds).range(0..=59)).changed();
            ui.label("s");
        });

        if changed {
            let combined = format!("{:02}:{:02}:{:02}", hours, minutes, seconds);
            self.get_mut_env_config().datetime_config.time = combined;
            self.rebuild_env();
        }
    }
    
    /// Render UI dropdowns to select task manager strategies:
    fn ui_task_manager_config_select(&mut self, ui: &mut egui::Ui) {
        let config = self.get_env_config().clone();
        ui.label("choose_station_strategy");
        egui::ComboBox::from_id_salt("Choose Station Strategy")
            .selected_text(config.task_manager_config.choose_station_strategy.to_string())
            .show_ui(ui, |ui| {
                let previous_value = config.task_manager_config.choose_station_strategy.clone();
                for strat in ChooseStationStrategy::variants() {
                    ui.selectable_value(&mut self.get_mut_env_config().task_manager_config.choose_station_strategy, strat.clone(), strat.clone().to_string());
                }
                if self.get_env_config().task_manager_config.choose_station_strategy != previous_value {
                    self.rebuild_env();
                }
            });
        ui.label("charging_strategy");
        egui::ComboBox::from_id_salt("Charging Strategy")
            .selected_text(config.task_manager_config.charging_strategy.to_string())
            .show_ui(ui, |ui| {
                let previous_value = config.task_manager_config.charging_strategy.clone();
                for strat in ChargingStrategy::variants() {
                    ui.selectable_value(&mut self.get_mut_env_config().task_manager_config.charging_strategy, strat.clone(), strat.clone().to_string());
                }
                if self.get_env_config().task_manager_config.charging_strategy != previous_value {
                    self.rebuild_env();
                }
            });
    }

}

macro_rules! impl_has_env {
    ($t:ty) => {
        impl HasEnv for $t {
            fn set_running(&mut self, value: bool) {
                self.running = value;
            }
            fn set_tick(&mut self, value: u32) {
                self.tick = value;
            }
            fn set_env(&mut self, value: Env) {
                self.env = value;
            }

            fn get_env_config(&self) -> &EnvConfig {
                &self.env_config
            }
            fn get_mut_env_config(&mut self) -> &mut EnvConfig {
                &mut self.env_config
            }
        }
    };
}
impl_has_env!(super::simulation_tool::SimulationTool);
impl_has_env!(super::path_tool::PathTool);
impl_has_env!(super::task_tool::TaskTool);
