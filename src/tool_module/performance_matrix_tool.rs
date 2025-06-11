use std::{fs, io::Write};

use chrono::{NaiveDate, NaiveTime, Timelike};
use egui::DragValue;
use serde::{Deserialize, Serialize};

use crate::{cfg::{AGENT_CONFIGS_PATH, DEFAULT_AGENT_CONFIG_PATH, DEFAULT_SCENE_CONFIG_PATH, PERFORMANCE_MATRIX_PATH, SCENE_CONFIGS_PATH}, environment::{datetime::{DateTimeConfig, DATE_FORMAT, TIME_FORMAT}, env_module::{env::Env, env_config::EnvConfig}, field_config::FieldConfig, scene_config::SceneConfig}, logger::log_error_and_panic, task_module::task_manager::{ChargingStrat, ChooseStationStrat}, tool_module::{has_help::HasHelp, tool::Tool}, units::duration::{average_duration, format_duration, Duration}, utilities::utils::{get_json_files_in_folder, load_json_or_panic}};


#[derive(Debug, Clone, PartialEq)]
pub enum TerminationCondition {
    AllTasksCompleted, // only if there are no cycles in farm entity plans
    NumberCompletedTasks(u32),
    EnvDuration(Duration),
}

#[derive(Serialize, Deserialize)]
struct SimulationSummary {
    start_datetime: chrono::DateTime<chrono::Local>,
    evaluation_duration: std::time::Duration,
    n_episodes: usize,
    scene_config_path: String,
    results: Vec<EpisodeResult>,
}

#[derive(Serialize, Deserialize)]
struct EpisodeResult {
    env_config: EnvConfig,
    #[serde(rename = "n_completed_tasks(min,avg,max)")]
    n_completed_tasks: String,
    #[serde(rename = "env_duration_(min,avg,max)")]
    env_duration: String,
}

pub struct PerformanceMatrixTool {
    current_pm_path: Option<String>,
    current_content: Option<String>,
    pub running: bool,
    pub n_episodes: u32,
    pub scene_config_path: String,
    pub agent_config_path: String,
    pub datetime_config: DateTimeConfig,
    pub env_configs: Vec<EnvConfig>,
    pub env_durations: Vec<Vec<Duration>>,
    pub env_n_completed_tasks: Vec<Vec<u32>>,
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
            env_durations: vec![],
            env_n_completed_tasks: vec![],
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
        // if let Some(pm_content) = &self.current_content {
        //     egui::ScrollArea::vertical().show(ui, |ui| {
        //         ui.add(
        //             egui::TextEdit::multiline(&mut pm_content.clone())
        //                 .font(egui::TextStyle::Monospace)
        //                 .desired_rows(20)
        //                 .lock_focus(true)
        //                 .desired_width(f32::INFINITY)
        //                 .interactive(false),
        //             );
        //         });
        // }

        //if let Ok(raw) = fs::read_to_string("configs/scene_config/default.json") {
        if let Some(raw) = &self.current_content {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(raw) {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    // Force the content to fill the full available width
                    ui.set_min_width(ui.available_width());

                    show_json_value(ui, &json, "root");
                });
            } else {
                ui.label("❌ Failed to parse JSON.");
            }
        }

    }
    fn render_ui(&mut self, ui: &mut egui::Ui) {
        self.render_help_button(ui);
        ui.separator();

        self.ui_summary_select(ui);
        ui.separator();

        ui.horizontal(|ui| {
            ui.label("n_episodes: ");
            ui.add(egui::DragValue::new(&mut self.n_episodes).speed(10).range(10..=10000));
        });
        ui.horizontal(|ui| {
            ui.label("scene_config: ");
            self.ui_scene_config_select(ui);
        });
        // agent_config_path
        ui.horizontal(|ui| {
            ui.label("agent_config_path");
            egui::ComboBox::from_id_salt("agent_config_path")
                .selected_text(&self.agent_config_path)
                .show_ui(ui, |ui| {
                    let json_files = get_json_files_in_folder(AGENT_CONFIGS_PATH);
                    let previous_value = self.agent_config_path.clone();
                    for json_file in json_files {
                        let whole_path = format!("{}{}", AGENT_CONFIGS_PATH, json_file.clone());
                        ui.selectable_value(&mut self.agent_config_path, whole_path.clone(), whole_path);
                    }
                    if *self.agent_config_path != previous_value {
                        for env_config in self.env_configs.iter_mut() {
                            env_config.agent_config_path = self.agent_config_path.clone();
                        }
                    }
                });
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
                let combined = format!("{:02}:{:02}:{:02}", hours, minutes, seconds);
                self.datetime_config.time = combined;
                for env_config in self.env_configs.iter_mut() {
                    env_config.datetime_config = self.datetime_config.clone();
                }
            }
        });

        ui.label(egui::RichText::new(format!("Env configs ({}):", self.env_configs.len())).size(16.0));
        ui.horizontal(|ui| {
            if ui.button("Add").clicked() {
                self.env_configs.push(EnvConfig {
                    scene_config_path: self.scene_config_path.clone(),
                    ..Default::default()
                });
                self.env_durations.push(vec![]);
                self.env_n_completed_tasks.push(vec![]);
            }
            if ui.button("Remove all").clicked() {
                self.env_configs.clear();
                self.env_durations.clear();
                self.env_n_completed_tasks.clear();
            }
        });
        let mut to_remove: Option<usize> = None;
        for (i, config) in self.env_configs.iter_mut().enumerate() {
            egui::CollapsingHeader::new(format!("Config {}", i))
            .default_open(true)
            .show(ui, |ui| {
                // n_agents
                ui.horizontal(|ui| {
                    ui.label("n_agents: ");
                    ui.add(egui::DragValue::new(&mut config.n_agents).speed(1).range(1..=10));
                });
                // // agent_config_path
                // ui.horizontal(|ui| {
                //     ui.label("agent_config_path");
                //     ui.label(config.agent_config_path.to_string());
                // });
                // // datetime
                // ui.horizontal(|ui| {
                //     ui.label("datetime:");
                //     ui.label(format!("{} {}", config.datetime_config.date, config.datetime_config.time));
                // });
                // // scene config path
                // ui.horizontal(|ui| {
                //     ui.label("scene_config_path:");
                //     ui.label(config.scene_config_path.to_string());
                // });
                //taskmanager
                ui.horizontal(|ui| {
                    ui.label("task_manager_config:");
                    ui.label("choose_station_strat");
                    egui::ComboBox::from_id_salt("Choose Station Strategy")
                        .selected_text(config.task_manager_config.choose_station_strat.to_string())
                        .show_ui(ui, |ui| {
                            let choose_station_options = ChooseStationStrat::variants();
                            for strat in choose_station_options {
                                ui.selectable_value(&mut config.task_manager_config.choose_station_strat, strat.clone(), strat.clone().to_string());
                            }
                        });
                    ui.label("charging_strat");
                    egui::ComboBox::from_id_salt("Charging Strategy")
                        .selected_text(config.task_manager_config.charging_strat.to_string())
                        .show_ui(ui, |ui| {
                            let charge_strat_options = ChargingStrat::variants();
                            for strat in charge_strat_options {
                                ui.selectable_value(&mut config.task_manager_config.charging_strat, strat.clone(), strat.clone().to_string());
                            }
                        });
    
                    });
                if ui.button("Remove").clicked() {
                    to_remove = Some(i);
                }
            });
        }
        if let Some(index) = to_remove {
            self.env_configs.remove(index);
            self.env_durations.remove(index);
            self.env_n_completed_tasks.remove(index);
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
            for data in self.env_durations.iter_mut() {
                data.clear();
            }
            for data in self.env_n_completed_tasks.iter_mut() {
                data.clear();
            }
            self.env_index = 0;
            self.env_episode = 0;
        }
        if self.running {
            ui.label("Evaluating...");
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
            println!("Set first env");
        }
        if let Some(env) = &mut self.env {
            env.task_manager.assign_tasks(&mut env.agents, &mut env.stations);
            env.step();

            let (finished, n_completed_tasks, env_duration) = match self.termination_condition {
                TerminationCondition::AllTasksCompleted => {
                    let scene_config: SceneConfig = load_json_or_panic(self.scene_config_path.clone());
                    let field_config: FieldConfig = load_json_or_panic(scene_config.field_config_path);
                    if let Some(n_actions) = field_config.number_of_actions() {
                        if env.task_manager.completed_tasks.len() as u32 == n_actions {
                            (true, env.task_manager.completed_tasks.len(), env.duration)
                        } else {
                            (false, 0, Duration::seconds(0.0))
                        }
                    } else {
                        (false, 0, Duration::seconds(0.0))
                    }
                },
                TerminationCondition::EnvDuration(duration) => {
                    if env.duration >= duration {
                        (true, env.task_manager.completed_tasks.len(), env.duration)
                    } else {
                        (false, 0, Duration::seconds(0.0))
                    }
                },
                TerminationCondition::NumberCompletedTasks(n_tasks) => {
                    if env.task_manager.completed_tasks.len() as u32 == n_tasks {
                        (true, env.task_manager.completed_tasks.len(), env.duration)
                    } else {
                        (false, 0, Duration::seconds(0.0))
                    }
                }
            };
            if finished {
                self.increment_update_data(n_completed_tasks as u32, env_duration);
            }
        }
    }
}

impl PerformanceMatrixTool {
    fn ui_summary_select(&mut self, ui: &mut egui::Ui) {
        ui.label(egui::RichText::new("Summary:").size(16.0));
        egui::ComboBox::from_label("  ")
            .selected_text(
                self.current_pm_path
                    .as_deref()
                    .unwrap_or("Select file...")
            )
            .show_ui(ui, |ui| {
                let json_files = get_json_files_in_folder(PERFORMANCE_MATRIX_PATH);
                let previous_value = self.current_pm_path.clone();

                for json_file in json_files {
                    let new_value = format!("{}{}", PERFORMANCE_MATRIX_PATH, json_file.clone());
                    ui.selectable_value(&mut self.current_pm_path, Some(new_value.clone()), json_file);
                }

                if self.current_pm_path != previous_value {
                    if let Some(path) = &self.current_pm_path {
                        let json_str = fs::read_to_string(path.clone()).expect("File not found");
                        self.current_content = Some(json_str);
                    }
                }
            });
    }
    fn ui_scene_config_select(&mut self, ui: &mut egui::Ui) {
        egui::ComboBox::from_label(" ")
            .selected_text(format!("{:?}", self.scene_config_path))
            .show_ui(ui, |ui| {
                let json_files = get_json_files_in_folder(SCENE_CONFIGS_PATH);
                let previous_value = self.scene_config_path.clone();

                for json_file in json_files {
                    let new_value = format!("{}{}", SCENE_CONFIGS_PATH, json_file.clone());
                    ui.selectable_value(&mut self.scene_config_path, new_value.clone(), json_file);
                }
                if *self.scene_config_path != previous_value {
                    for env_config in self.env_configs.iter_mut() {
                        env_config.scene_config_path = self.scene_config_path.clone();
                    }
                }
            });
    }

    fn has_cycle_plan(scene_path: String) -> bool {
        let scene_config: SceneConfig = load_json_or_panic(scene_path);
        let field_config: FieldConfig = load_json_or_panic(scene_config.field_config_path);
        field_config.has_cycle_farm_entity_plan()
    }

    fn increment_update_data(&mut self, n_completed_tasks: u32, duration: Duration) {
        // write data
        println!("Finished {} {}", self.env_index, self.env_episode);
        self.env_durations[self.env_index].push(duration);
        self.env_n_completed_tasks[self.env_index].push(n_completed_tasks);
        // change env
        self.env_episode += 1;
        if self.env_episode < self.n_episodes { // same env next episode
            self.env = Some(Env::from_config(self.env_configs[self.env_index].clone()));
        } else {
            self.env_episode = 0;
            self.env_index += 1;
            if self.env_index < self.env_configs.len() { // new env first episode
                self.env = Some(Env::from_config(self.env_configs[self.env_index].clone()));
            } else {
                // no more env configs
                let evaluation_duration = self.start_time
                    .map(|start| start.elapsed())
                    .unwrap_or_default();
                let start_datetime: chrono::DateTime<chrono::Local> = self.start_datetime.unwrap_or_else(chrono::Local::now);
                let summary = SimulationSummary {
                    start_datetime,
                    evaluation_duration,
                    n_episodes: self.n_episodes as usize,
                    scene_config_path: self.scene_config_path.clone(),
                    results: self.env_configs.iter().enumerate().map(|(i, config)| {
                        let durations = &self.env_durations[i];
                        let tasks = &self.env_n_completed_tasks[i];

                        let n_completed_tasks = format!("({:?}, {:?}, {:?})",
                            tasks.iter().min().unwrap_or(&0),
                            tasks.iter().sum::<u32>() / tasks.len() as u32,
                            tasks.iter().max().unwrap_or(&0),
                        );
                        let env_duration = format!("({}, {}, {})",
                            format_duration(durations.iter().min().unwrap_or(&Duration::seconds(0.0))),
                            format_duration(&average_duration(durations)),
                            format_duration(durations.iter().max().unwrap_or(&Duration::seconds(0.0)))
                        );
                        EpisodeResult {
                            env_config: config.clone(),
                            n_completed_tasks,
                            env_duration,
                        }
                    }).collect(),
                };
                let json = serde_json::to_string_pretty(&summary).expect("");
                let path = format!("{}{}.json", PERFORMANCE_MATRIX_PATH, self.save_file_name);
                let mut file = std::fs::File::create(path.clone()).expect("");
                file.write_all(json.as_bytes()).expect("");
                
                self.current_content = Some(json);
                self.current_pm_path = Some(path);

                self.running = false;
            }
        }
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

        ui.label("Env config:");
        ui.label("In dropdown you can select env config.");
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
                        show_json_value(ui, item, &format!("[{}]", i));
                    }
                });
        }
        _ => {
            ui.horizontal(|ui| {
                ui.label(format!("{}:", label));
                ui.label(format!("{}", value));
            });
        }
    }
}