use std::fs;

use egui::Ui;

use crate::{cfg::ENV_CONFIGS_PATH, environment::{env::Env, env_config::EnvConfig, field_config::VariantFieldConfig, station::Station}, utilities::utils::generate_colors};


pub trait EnvTool {
    fn current_env_config_string(&self) -> &String;
    fn current_env_config_string_mut(&mut self) -> &mut String;

    fn env(&self) -> &Env;
    fn env_mut(&mut self) -> &mut Env;

    fn create_env(&mut self, new_config_file_path: String) {
        let new_env_config = EnvConfig::from_json_file(&new_config_file_path).expect("Err");
        *self.env_mut() = Env::from_config(new_env_config);
        self.recalc_charging_stations();
        self.recalc_field_config_on_add_remove();
        self.recalc_field_config_on_param_changed();
    }
    
    fn config_select(&mut self, ui: &mut Ui) {
        egui::ComboBox::from_label("")
            .selected_text(format!("{:?}", self.current_env_config_string()))
            .show_ui(ui, |ui| {
                let json_files = get_json_files(ENV_CONFIGS_PATH).expect("Can't find json files");
                let previous_value = self.current_env_config_string().clone();

                for json_file in json_files {
                    let new_value = format!("{}{}", ENV_CONFIGS_PATH, json_file.clone());
                    ui.selectable_value(self.current_env_config_string_mut(), new_value.clone(), json_file);
                }

                if *self.current_env_config_string() != previous_value {
                    // let new_env_config = EnvConfig::from_json_file(self.current_env_config_string()).expect("Err");
                    // *self.env_mut() = Env::from_config(new_env_config);
                    // self.recalc_charging_stations();
                    // self.recalc_field_config_on_add_remove();
                    // self.recalc_field_config_on_param_changed();
                    let new_config_file_path = self.current_env_config_string();
                    self.create_env(new_config_file_path.clone());
                }
            });
    }
    fn recalc_charging_stations(&mut self) {
        let colors = generate_colors(self.env().stations.len(), 0.01);
        for (i, station) in self.env_mut().stations.iter_mut().enumerate() {
            *station = Station::new(i as u32, station.position, station.queue_direction, station.waiting_offset, colors[i], station.n_slots);
        }
    }
    fn recalc_field_config_on_add_remove(&mut self) {
        let colors = generate_colors(self.env().field_config.configs.len(), 0.1);
        for (i, config_variant) in self.env_mut().field_config.configs.iter_mut().enumerate() {
            match config_variant {
                VariantFieldConfig::Line(config) => {
                    config.id = i as u32;
                    config.color = colors[i];
                },
                VariantFieldConfig::Point(config) => {
                    config.id = i as u32;
                    config.color = colors[i];
                },
            }
        }
        self.recalc_field_config_on_param_changed();
    }
    fn recalc_field_config_on_param_changed(&mut self) {
        let obstacles = self.env().field_config.get_obstacles();
        let graph_points = &self.env().field_config.get_graph_points();
        self.env_mut().obstacles = obstacles.clone();
        self.env_mut().visibility_graph.recalculate(graph_points, &obstacles);
    }

}

fn get_json_files(path: &str) -> Result<Vec<String>, std::io::Error> {
    let mut json_files = Vec::new();
    
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json") {
            if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                json_files.push(file_name.to_string());
            }
        }
    }
    
    Ok(json_files)
}


macro_rules! impl_env_tool {
    ($t:ty) => {
        impl EnvTool for $t {
            fn current_env_config_string(&self) -> &String {
                &self.current_env_config_string
            }
            fn current_env_config_string_mut(&mut self) -> &mut String {
                &mut self.current_env_config_string
            }
            fn env(&self) -> &Env {
                &self.env
            }
            fn env_mut(&mut self) -> &mut Env {
                &mut self.env
            }
        }
    };
}

impl_env_tool!(super::simulation_tool::SimulationTool);
impl_env_tool!(super::path_tool::PathTool);
impl_env_tool!(super::editor_tool::EditorTool);
impl_env_tool!(super::task_tool::TaskTool);
