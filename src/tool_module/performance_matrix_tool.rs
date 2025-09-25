use std::io::Write;
use chrono::{NaiveDate, NaiveTime, Timelike};
use egui::DragValue;

use crate::{cfg::{AGENT_CONFIGS_PATH, DEFAULT_AGENT_CONFIG_PATH, DEFAULT_SCENE_CONFIG_PATH, PERFORMANCE_MATRIX_PATH, SCENE_CONFIGS_PATH, TASK_MANAGER_CONFIGS_PATH}, environment::{datetime::{DateTimeConfig, DATE_FORMAT, TIME_FORMAT}, env_module::{env::Env, env_config::EnvConfig}, field_config::FieldConfig, scene_config::SceneConfig}, logger::log_error_and_panic, statistics::{EnvEpisodeStats, EnvResult, PerformanceMatrixResult}, tool_module::{has_help::HasHelp, tool::Tool}, units::duration::Duration, utilities::utils::{json_config_combo, load_json_or_panic}};

/// Defines conditions under which the simulation terminates.
#[derive(Debug, Clone, PartialEq)]
pub enum TerminationCondition {
    /// Terminates when all tasks in the environment are completed.
    /// Only valid if farm entity plans have no cycle.
    AllTasksCompleted,
    /// Terminates after a specified number of tasks are completed.
    NumberCompletedTasks(u32),
    /// Terminates after a specified duration in simulation time.
    EnvDuration(Duration),
}

/// A tool for running and analyzing multiple environment configurations.
pub struct PerformanceMatrixTool {
    current_pm_path: Option<String>,
    current_content: Option<String>,
    pub running: bool,
    pub n_episodes: u32,
    pub scene_config_path: String,
    pub agent_config_path: String,
    pub datetime_config: DateTimeConfig,
    pub env_configs: Vec<EnvConfig>,
    pub env_episode_stats: Vec<Vec<EnvEpisodeStats>>,
    pub termination_condition: TerminationCondition,
    pub env: Option<Env>,
    env_index: usize,
    env_episode: u32,
    save_file_name: String,
    start_datetime: Option<chrono::DateTime<chrono::Local>>,
    start_time: Option<std::time::Instant>,
    pub help_open: bool,
}

impl Default for PerformanceMatrixTool {
    fn default() -> Self {
        let termination_condition = if Self::has_cycle_plan(DEFAULT_SCENE_CONFIG_PATH.to_string()) {
            TerminationCondition::EnvDuration(Duration::days(1.0))
        } else {TerminationCondition::AllTasksCompleted};
        Self {
            current_pm_path: None,
            current_content: None,
            running: false,
            n_episodes: 10,
            scene_config_path: DEFAULT_SCENE_CONFIG_PATH.to_string(),
            agent_config_path: DEFAULT_AGENT_CONFIG_PATH.to_string(),
            datetime_config: DateTimeConfig::from_string("01.01.2025 00:00:00".to_string()),
            env_configs: vec![],
            env_episode_stats: vec![],
            termination_condition,
            env: None,
            env_index: 0,
            env_episode: 0,
            save_file_name: String::new(),
            start_datetime: None,
            start_time: None,
            help_open: false,
        }
    }
}

impl Tool for PerformanceMatrixTool {
    fn render_main(&mut self, ui: &mut egui::Ui) {
        if let Some(raw) = &self.current_content {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(raw) {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.set_min_width(ui.available_width());

                    show_json_value(ui, &json, "root");
                });
            } else {
                ui.label("Failed to parse JSON.");
            }
        }

    }
    fn render_ui(&mut self, ui: &mut egui::Ui) {
        self.render_help_button(ui);
        ui.separator();

        self.ui_result_select(ui);
        ui.separator();

        // n_episodes
        ui.horizontal(|ui| {
            ui.label("n_episodes: ");
            ui.add(egui::DragValue::new(&mut self.n_episodes).speed(10).range(10..=10000));
        });
        // scene_config
        ui.horizontal(|ui| {
            ui.label("scene_config: ");
            self.ui_scene_config_select(ui);
        });
        // agent_config_path
        ui.horizontal(|ui| {
            ui.label("agent_config_path");
            self.ui_agent_config_select(ui);
        });
        // datetime
        ui.horizontal(|ui| {
            ui.label("datetime:");
            ui.label(format!("{} {} |", self.datetime_config.date, self.datetime_config.time));
                    
            let mut date = NaiveDate::parse_from_str(&self.datetime_config.date, DATE_FORMAT)
                .unwrap_or_else(|e| {
                    let msg = format!(
                        "Failed to parse date '{}' with format '{}': {}",
                        &self.datetime_config.date, DATE_FORMAT, e
                    );
                    log_error_and_panic(&msg)
                });
            let mut changed = false;
            if ui.add(egui_extras::DatePickerButton::new(&mut date)).changed() {
                self.datetime_config.date = date.format(DATE_FORMAT).to_string();
                changed = true;
            }
            ui.label("|");
            
            let time = NaiveTime::parse_from_str(&self.datetime_config.time, TIME_FORMAT)
                .unwrap_or_else(|e| {
                    let msg = format!(
                        "Failed to parse time '{}' with format '{}': {}",
                        &self.datetime_config.time, TIME_FORMAT, e
                    );
                    log_error_and_panic(&msg);
                });
            let mut hours = time.hour();
            let mut minutes = time.minute();
            let mut seconds = time.second();
            //let mut changed = false;
            ui.horizontal(|ui| {
                changed |= ui.add(egui::DragValue::new(&mut hours).range(0..=23)).changed();
                ui.label("h");
                changed |= ui.add(egui::DragValue::new(&mut minutes).range(0..=59)).changed();
                ui.label("m");
                changed |= ui.add(egui::DragValue::new(&mut seconds).range(0..=59)).changed();
                ui.label("s");
            });
            if changed {
                let combined = format!("{hours:02}:{minutes:02}:{seconds:02}");
                self.datetime_config.time = combined;
                for env_config in self.env_configs.iter_mut() {
                    env_config.datetime_config = self.datetime_config.clone();
                }
            }
        });
        // env configs
        ui.label(egui::RichText::new(format!("Env configs ({}):", self.env_configs.len())).size(16.0));
        ui.horizontal(|ui| {
            if ui.button("Add").clicked() {
                self.env_configs.push(EnvConfig {
                    scene_config_path: self.scene_config_path.clone(),
                    ..Default::default()
                });
                // self.env_durations.push(vec![]);
                // self.env_n_completed_tasks.push(vec![]);
                self.env_episode_stats.push(vec![]);
            }
            if ui.button("Remove all").clicked() {
                self.env_configs.clear();
                // self.env_durations.clear();
                // self.env_n_completed_tasks.clear();
                self.env_episode_stats.clear();
            }
        });
        let mut to_remove: Option<usize> = None;
        for (i, config) in self.env_configs.iter_mut().enumerate() {
            egui::CollapsingHeader::new(format!("Config {i}"))
            .default_open(true)
            .show(ui, |ui| {
                // n_agents
                ui.horizontal(|ui| {
                    ui.label("n_agents: ");
                    ui.add(egui::DragValue::new(&mut config.n_agents).speed(1).range(1..=10));
                });
                //taskmanager
                ui.horizontal(|ui| {
                    ui.label("task_manager_config_path:");

                    let mut new_config_path = config.task_manager_config_path.clone();

                    if json_config_combo(ui, "", &mut new_config_path, TASK_MANAGER_CONFIGS_PATH)
                        && new_config_path != config.task_manager_config_path
                    {
                        config.task_manager_config_path = new_config_path.clone();
                    }
                });
                
                if ui.button("Remove").clicked() {
                    to_remove = Some(i);
                }
            });
        }
        if let Some(index) = to_remove {
            self.env_configs.remove(index);
            // self.env_durations.remove(index);
            // self.env_n_completed_tasks.remove(index);
            self.env_episode_stats.remove(index);
        }

        ui.separator();
        // termination condition
        ui.horizontal(|ui| {
            ui.label("termination_condition:");
            let mut selected_kind = match self.termination_condition {
                TerminationCondition::AllTasksCompleted => "AllTasksCompleted",
                TerminationCondition::NumberCompletedTasks(_) => "NumberCompletedTasks",
                TerminationCondition::EnvDuration(_) => "EnvDuration",
            };

            egui::ComboBox::from_id_salt("Termination Condition")
                .selected_text(selected_kind)
                .show_ui(ui, |ui| {
                    if !Self::has_cycle_plan(self.scene_config_path.clone()) && ui.selectable_label(selected_kind == "AllTasksCompleted", "AllTasksCompleted").clicked() {
                            self.termination_condition = TerminationCondition::AllTasksCompleted;
                            selected_kind = "AllTasksCompleted";
                    }
                    if ui.selectable_label(selected_kind == "NumberCompletedTasks", "NumberCompletedTasks").clicked() {
                        self.termination_condition = TerminationCondition::NumberCompletedTasks(1); // default value
                        selected_kind = "NumberCompletedTasks";
                    }
                    if ui.selectable_label(selected_kind == "EnvDuration", "EnvDuration").clicked() {
                        self.termination_condition = TerminationCondition::EnvDuration(Duration::days(1.0)); // default
                        selected_kind = "EnvDuration";
                    }
                });
            match &mut self.termination_condition {
                TerminationCondition::NumberCompletedTasks(val) => {
                    let mut min_n_actions = 10000;
                    let scene_config: SceneConfig = load_json_or_panic(self.scene_config_path.clone());
                    let field_config: FieldConfig = load_json_or_panic(scene_config.field_config_path);
                    if let Some(n_actions) = field_config.number_of_actions() {
                        if n_actions < min_n_actions {
                            min_n_actions = n_actions;
                        }
                    }
                    ui.horizontal(|ui| {
                        ui.label("Completed Tasks:");
                        ui.add(DragValue::new(val).speed(1).range(1..=min_n_actions));
                    });
                }
                TerminationCondition::EnvDuration(duration) => {
                    let mut hours = duration.to_hour() as u32;
                    ui.horizontal(|ui| {
                        ui.label("Duration (h):");
                        if ui.add(DragValue::new(&mut hours)).changed() {
                            *duration = Duration::hours(hours as f32);
                        }
                    });
                }
                _ => {}
            }
        });
        ui.separator();
        ui.horizontal(|ui| {
            ui.label(PERFORMANCE_MATRIX_PATH);
            ui.add(egui::TextEdit::singleline(&mut self.save_file_name).desired_width(100.0));
            ui.label(".json");
        });
        if !self.running && ui.button("Evaluate").clicked() && !self.save_file_name.is_empty() {
            self.running = true;
            self.start_datetime = Some(chrono::Local::now());
            self.start_time = Some(std::time::Instant::now());
            self.env_index = 0;
            self.env_episode = 0;
        }
        if ui.button("Reset").clicked() {
            self.running = false;
            // for data in self.env_durations.iter_mut() {
            //     data.clear();
            // }
            // for data in self.env_n_completed_tasks.iter_mut() {
            //     data.clear();
            // }
            for data in self.env_episode_stats.iter_mut() {
                data.clear();
            }
            self.env_index = 0;
            self.env_episode = 0;
        }
        if self.running {
            ui.label(egui::RichText::new("Make sure you set tps/fps ratio high in settings").color(egui::Color32::RED));
            ui.label("Evaluating...");
            let progress = (self.env_index as u32*self.n_episodes+self.env_episode) as f32 / (self.env_configs.len() as u32*self.n_episodes) as f32;
            ui.add(egui::ProgressBar::new(progress).show_percentage());
        }


        self.render_help(ui);
    }
    fn update(&mut self) {
        if !self.running { return; }
        if self.env_configs.is_empty() {
            self.running = false;
            return;
        }

        if self.env.is_none() {
            self.env = Some(Env::from_config(self.env_configs[self.env_index].clone()));
        }
        if let Some(env) = &mut self.env {
            env.task_manager.assign_tasks(&mut env.agents, &mut env.stations);
            env.step();

            // Check termination condition
            let finished = match self.termination_condition {
                TerminationCondition::AllTasksCompleted => {
                    let scene_config: SceneConfig = load_json_or_panic(self.scene_config_path.clone());
                    let field_config: FieldConfig = load_json_or_panic(scene_config.field_config_path);
                    if let Some(n_actions) = field_config.number_of_actions() {
                        env.task_manager.completed_tasks.len() as u32 >= n_actions
                    } else {
                        false
                    }
                },
                TerminationCondition::EnvDuration(duration) => env.duration >= duration,
                TerminationCondition::NumberCompletedTasks(n_tasks) => {
                    env.task_manager.completed_tasks.len() as u32 >= n_tasks
                },
            };

            // If episode finished, compute stats and increment results
            if finished {
                let episode_stats = env.get_env_episode_stats();
                self.increment_env_episode(episode_stats);
            }
        }
    }
}

impl PerformanceMatrixTool {
    /// Renders dropdown to select result file.
    fn ui_result_select(&mut self, ui: &mut egui::Ui) {
        ui.label(egui::RichText::new("Result:").size(16.0));

        let mut selected_path = self.current_pm_path.clone().unwrap_or("Select file...".to_string());

        if json_config_combo(ui, "  ", &mut selected_path, PERFORMANCE_MATRIX_PATH)
            && Some(selected_path.clone()) != self.current_pm_path
        {
            self.current_pm_path = Some(selected_path.clone());

            if let Ok(json_str) = std::fs::read_to_string(&selected_path) {
                self.current_content = Some(json_str);
            } else {
                self.current_content = None;
            }
        }
    }

    /// Renders dropdown to select scene configuration.
    fn ui_scene_config_select(&mut self, ui: &mut egui::Ui) {
        let mut new_value = self.scene_config_path.clone();

        if json_config_combo(ui, "", &mut new_value, SCENE_CONFIGS_PATH)
            && new_value != self.scene_config_path
        {
            self.scene_config_path = new_value;
            for env_config in self.env_configs.iter_mut() {
                env_config.scene_config_path = self.scene_config_path.clone();
            }
        }
    }
    
    /// Renders dropdown to select agent configuration.
    fn ui_agent_config_select(&mut self, ui: &mut egui::Ui) {
        let mut new_value = self.agent_config_path.clone();

        if json_config_combo(ui, " ", &mut new_value, AGENT_CONFIGS_PATH)
            && new_value != self.agent_config_path
        {
            self.agent_config_path = new_value;
            for env_config in self.env_configs.iter_mut() {
                env_config.agent_config_path = self.agent_config_path.clone();
            }
        }
    }

    /// Checks if field configuration has any plan with cycle.
    fn has_cycle_plan(scene_path: String) -> bool {
        let scene_config: SceneConfig = load_json_or_panic(scene_path);
        let field_config: FieldConfig = load_json_or_panic(scene_config.field_config_path);
        field_config.has_cycle_farm_entity_plan()
    }

    /// Increment episode/env counters and store episode-level data
    fn increment_env_episode(&mut self, episode_stats: EnvEpisodeStats) {
        // Store episode data
        self.env_episode_stats[self.env_index].push(episode_stats);

        // Advance episode counter
        self.env_episode += 1;

        if self.env_episode < self.n_episodes {
            // Same environment, next episode
            self.env = Some(Env::from_config(self.env_configs[self.env_index].clone()));
        } else {
            // Completed all episodes for current environment
            self.env_episode = 0;
            self.env_index += 1;

            if self.env_index < self.env_configs.len() {
                // New environment, first episode
                self.env = Some(Env::from_config(self.env_configs[self.env_index].clone()));
            } else {
                // All environments completed → finalize
                self.finalize_result();
            }
        }
    }

    /// Aggregate all environment results and write PerformanceMatrixResult to JSON
    fn finalize_result(&mut self) {
        let evaluation_duration = self.start_time.map(|s| s.elapsed()).unwrap_or_default();
        let start_datetime: chrono::DateTime<chrono::Local> =
            self.start_datetime.unwrap_or_else(chrono::Local::now);

        let env_results: Vec<EnvResult> = self.env_configs.iter().enumerate().map(|(i, config)| {
            let episodes = &self.env_episode_stats[i];
            EnvResult::from_episodes(config.clone(), episodes.to_vec())
        }).collect();

        let result = PerformanceMatrixResult {
            start_datetime,
            evaluation_duration,
            n_episodes: self.n_episodes as usize,
            scene_config_path: self.scene_config_path.clone(),
            env_results,
        };

        // Serialize to JSON
        let json = serde_json::to_string_pretty(&result).unwrap_or_else(|e| {
            let msg = format!("Failed to serialize result: {e}");
            log_error_and_panic(&msg);
        });

        let path = format!("{}{}.json", PERFORMANCE_MATRIX_PATH, self.save_file_name);
        let mut file = std::fs::File::create(&path).unwrap_or_else(|e| {
            let msg = format!("Failed to create file {path:?}: {e}");
            log_error_and_panic(&msg);
        });

        file.write_all(json.as_bytes()).unwrap_or_else(|e| {
            let msg = format!("Failed to write result JSON: {e}");
            log_error_and_panic(&msg);
        });

        self.current_content = Some(json);
        self.current_pm_path = Some(path);
        self.running = false;
    }
}

impl HasHelp for PerformanceMatrixTool {
    fn help_modal(&self) -> egui::Modal {
        egui::Modal::new(egui::Id::new("Performance Matrix Tool Help"))
    }
    fn render_help_contents(&self, ui: &mut egui::Ui) {
        ui.heading("Performance Matrix Tool Help");
        ui.label("This is a Performance Matrix Tool where you set and run selected env configs and get evaluations.");
        ui.separator();

        ui.label("Result:");
        ui.label("In dropdown you can select result.");
        ui.separator();

        ui.label("Env settings:");
        ui.label("Set common settings for all env configs.");
        ui.separator();

        ui.label("Env configs:");
        ui.label("Add or remove env configs.");
        ui.label("Set number of agents and task manager config for each env config");
        ui.separator();

        ui.label("Set condition when env stops");
        ui.separator();

        ui.label("Saving");
        ui.label("Name file and start evaluation.");
    }
}

pub fn show_json_value(ui: &mut egui::Ui, value: &serde_json::Value, label: &str) {
    match value {
        serde_json::Value::Object(map) => {
            egui::CollapsingHeader::new(label)
                .default_open(true)
                .show(ui, |ui| {
                    for (k, v) in map {
                        show_json_value(ui, v, k);
                    }
                });
        }
        serde_json::Value::Array(arr) => {
            egui::CollapsingHeader::new(format!("{} [{}]", label, arr.len()))
                .default_open(false)
                .show(ui, |ui| {
                    for (i, item) in arr.iter().enumerate() {
                        show_json_value(ui, item, &format!("[{i}]"));
                    }
                });
        }
        _ => {
            ui.horizontal(|ui| {
                ui.label(format!("{label}:"));
                ui.label(format!("{value}"));
            });
        }
    }
}