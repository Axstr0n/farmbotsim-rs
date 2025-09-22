use crate::{
    cfg::{DEFAULT_TASK_MANAGER_CONFIG_PATH, TASK_MANAGER_CONFIGS_PATH}, task_module::{strategies::{ChargingStrategy, ChooseStationStrategy}, task_manager_config::TaskManagerConfig}, tool_module::{has_config_saving::HasConfigSaving, has_help::HasHelp, tool::Tool}, utilities::utils::{json_config_combo, load_json_or_panic}
};


/// A tool to edit, change, view TaskManager configurations
pub struct TaskManagerConfigEditorTool {
    config: TaskManagerConfig,
    save_file_name: String,
    pub current_config_path: String,
    pub help_open: bool,
}

impl Default for TaskManagerConfigEditorTool {
    fn default() -> Self {
        let config = load_json_or_panic(DEFAULT_TASK_MANAGER_CONFIG_PATH);
        Self {
            config,
            save_file_name: String::new(),
            current_config_path: DEFAULT_TASK_MANAGER_CONFIG_PATH.to_string(),
            help_open: false,
        }
    }
}

impl Tool for TaskManagerConfigEditorTool {
    fn render_main(&mut self, ui: &mut egui::Ui) {
        self.task_manager_ui(ui);
    }

    fn render_ui(&mut self, ui: &mut egui::Ui) {
        self.render_help_button(ui);
        ui.separator();

        let mut save_file_name = self.save_file_name.clone();
        self.draw_save_ui(ui, &mut save_file_name, true);
        self.save_file_name = save_file_name;

        self.ui_config_select(ui);

        self.render_help(ui);
    }

    fn update(&mut self) {}
}

impl TaskManagerConfigEditorTool {
    /// Renders dropdown to select TaskManager configuration file
    fn ui_config_select(&mut self, ui: &mut egui::Ui) {
        let mut new_path = self.current_config_path.clone();

        if json_config_combo(ui, "", &mut new_path, TASK_MANAGER_CONFIGS_PATH)
            && new_path != self.current_config_path
        {
            self.current_config_path = new_path;
            let config = load_json_or_panic(self.current_config_path.clone());
            self.config = config;
        }
    }

    /// Renders json-like structure with editable values
    fn task_manager_ui(&mut self, ui: &mut egui::Ui) {
        ui.label("{");

        // ChargingStrategy dropdown with sliders
        ui.horizontal(|ui| {
            ui.label("    \"charging_strategy\":");

            egui::ComboBox::from_id_salt("ChargingStrategy")
                .selected_text(match self.config.charging_strategy {
                    ChargingStrategy::CriticalOnly(_) => "CriticalOnly",
                    ChargingStrategy::ThresholdWithLimit(_, _) => "ThresholdWithLimit",
                })
                .show_ui(ui, |ui| {
                    if ui.selectable_label(
                        matches!(self.config.charging_strategy, ChargingStrategy::CriticalOnly(_)),
                        "CriticalOnly",
                    )
                    .clicked()
                    {
                        self.config.charging_strategy = ChargingStrategy::CriticalOnly(50.0);
                    }

                    if ui.selectable_label(
                        matches!(self.config.charging_strategy, ChargingStrategy::ThresholdWithLimit(_, _)),
                        "ThresholdWithLimit",
                    )
                    .clicked()
                    {
                        self.config.charging_strategy = ChargingStrategy::ThresholdWithLimit(60.0, 45.0);
                    }
                });
        });

        // ChargingStrategy inner sliders
        match &mut self.config.charging_strategy {
            ChargingStrategy::CriticalOnly(c) => {
                ui.horizontal(|ui| {
                    ui.label("        critical:");
                    ui.add(egui::Slider::new(c, 0.0..=100.0).text(" %"));
                });
            }
            ChargingStrategy::ThresholdWithLimit(t, c) => {
                ui.horizontal(|ui| {
                    ui.label("        threshold:");
                    ui.add(egui::Slider::new(t, 0.0..=100.0).text(" %"));
                });
                ui.horizontal(|ui| {
                    ui.label("        critical:");
                    ui.add(egui::Slider::new(c, 0.0..=100.0).text(" %"));
                });
            }
        }

        // ChooseStationStrategy dropdown
        ui.horizontal(|ui| {
            ui.label("    \"choose_station_strategy\":");

            egui::ComboBox::from_id_salt("ChooseStationStrategy")
                .selected_text(match &self.config.choose_station_strategy {
                    ChooseStationStrategy::Manhattan(_) => "Manhattan",
                    ChooseStationStrategy::Path(_) => "Path",
                })
                .show_ui(ui, |ui| {
                    if ui
                        .selectable_label(matches!(self.config.choose_station_strategy, ChooseStationStrategy::Manhattan(_)), "Manhattan")
                        .clicked()
                    {
                        // Keep old factor if possible, otherwise set default
                        let factor = match self.config.choose_station_strategy {
                            ChooseStationStrategy::Manhattan(f) => f,
                            _ => 0.5,
                        };
                        self.config.choose_station_strategy = ChooseStationStrategy::Manhattan(factor);
                    }
                    if ui
                        .selectable_label(matches!(self.config.choose_station_strategy, ChooseStationStrategy::Path(_)), "Path")
                        .clicked()
                    {
                        let factor = match self.config.choose_station_strategy {
                            ChooseStationStrategy::Path(f) => f,
                            _ => 0.5,
                        };
                        self.config.choose_station_strategy = ChooseStationStrategy::Path(factor);
                    }
                });
        });

        // Slider for factor
        match &mut self.config.choose_station_strategy {
            ChooseStationStrategy::Manhattan(factor) | ChooseStationStrategy::Path(factor) => {
                ui.horizontal(|ui| {
                    ui.label("        factor:");
                    ui.add(egui::Slider::new(factor, 0.0..=1.0).step_by(0.01));
                });
            }
        }

        ui.label("}");
    }

}

impl HasConfigSaving for TaskManagerConfigEditorTool {
    fn base_path() -> &'static str {
        TASK_MANAGER_CONFIGS_PATH
    }
    fn config(&self) -> impl serde::Serialize {
        self.config.clone()
    }
    fn update_current_path(&mut self, path: String) {
        self.current_config_path = path;
    }
}

impl HasHelp for TaskManagerConfigEditorTool {
    fn help_modal(&self) -> egui::Modal {
        egui::Modal::new(egui::Id::new("TaskManager Config Editor Tool Help"))
    }
    fn render_help_contents(&self, ui: &mut egui::Ui) {
        ui.heading("TaskManager Config Editor Tool Help");
        ui.label("This is a TaskManager Config Editor where you can view, change, create, and save TaskManager configs.");
        ui.separator();

        ui.label("ChargingStrategy options:");
        ui.monospace(
r#"CriticalOnly(f32)
    - 'f32' = critical battery level (0.0 - 100.0)
    - Robot will charge only if battery falls below this level.

ThresholdWithLimit(f32, f32)
    - First 'f32' = threshold battery level (0.0 - 100.0)
        * Robot will try to charge if battery is below this and a station is available
    - Second 'f32' = critical battery level (0.0 - 100.0)
        * Robot will always charge if battery drops below this level."#,
        );

        ui.separator();
        ui.label("ChooseStationStrategy options:");
        ui.monospace(
r#"Manhattan(f32)
    - 'f32' = factor (0.0 - 1.0)
    - Interpolates between:
        * 0.0 → Choose station with smallest Manhattan distance
        * 1.0 → Choose station with the smallest queue
        * Values in between balance distance and queue.

Path(f32)
    - 'f32' = factor (0.0 - 1.0)
    - Same as above, but distance is calculated along the path instead of Manhattan distance."#,
        );
    }


}
