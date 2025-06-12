use chrono::{NaiveDate, NaiveTime, Timelike};

use crate::{
    cfg::{AGENT_CONFIGS_PATH, SCENE_CONFIGS_PATH}, environment::{
        datetime::{DATE_FORMAT, TIME_FORMAT}, env_module::{env::Env, env_config::EnvConfig},
    }, logger::log_error_and_panic, task_module::task_manager::{ChargingStrat, ChooseStationStrat}, utilities::utils::json_config_combo
};


pub trait HasEnv {
    fn set_running(&mut self, value: bool);
    fn set_tick(&mut self, value: u32);
    fn set_env(&mut self, value: Env);

    fn get_env_config(&self) -> &EnvConfig;
    fn get_mut_env_config(&mut self) -> &mut EnvConfig;

    fn rebuild_env(&mut self) {
        self.set_running(false);
        self.set_tick(0);
        self.set_env(Env::from_config(self.get_env_config().clone()))
    }
    fn ui_scene_config_select(&mut self, ui: &mut egui::Ui) {
        let mut new_value = self.get_env_config().scene_config_path.clone();

        if json_config_combo(ui, "", &mut new_value, SCENE_CONFIGS_PATH)
            && new_value != self.get_env_config().scene_config_path
        {
            self.get_mut_env_config().scene_config_path = new_value;
            self.rebuild_env();
        }
    }
    fn ui_agent_config_select(&mut self, ui: &mut egui::Ui) {
        let mut new_value = self.get_env_config().agent_config_path.clone();

        if json_config_combo(ui, " ", &mut new_value, AGENT_CONFIGS_PATH)
            && new_value != self.get_env_config().agent_config_path
        {
            self.get_mut_env_config().agent_config_path = new_value;
            self.rebuild_env();
        }
    }
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
    fn ui_task_manager_config_select(&mut self, ui: &mut egui::Ui) {
        let config = self.get_env_config().clone();
        ui.label("choose_station_strat");
        egui::ComboBox::from_id_salt("Choose Station Strategy")
            .selected_text(config.task_manager_config.choose_station_strat.to_string())
            .show_ui(ui, |ui| {
                let previous_value = config.task_manager_config.choose_station_strat.clone();
                for strat in ChooseStationStrat::variants() {
                    ui.selectable_value(&mut self.get_mut_env_config().task_manager_config.choose_station_strat, strat.clone(), strat.clone().to_string());
                }
                if self.get_env_config().task_manager_config.choose_station_strat != previous_value {
                    self.rebuild_env();
                }
            });
        ui.label("charging_strat");
        egui::ComboBox::from_id_salt("Charging Strategy")
            .selected_text(config.task_manager_config.charging_strat.to_string())
            .show_ui(ui, |ui| {
                let previous_value = config.task_manager_config.charging_strat.clone();
                for strat in ChargingStrat::variants() {
                    ui.selectable_value(&mut self.get_mut_env_config().task_manager_config.charging_strat, strat.clone(), strat.clone().to_string());
                }
                if self.get_env_config().task_manager_config.charging_strat != previous_value {
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
