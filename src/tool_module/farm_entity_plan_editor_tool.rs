use std::{fs::{self, File}, io::Write};
use egui::Ui;

use crate::{
    cfg::{DEFAULT_POINT_FARM_ENTITY_PLAN_PATH, FARM_ENTITY_PLANS_PATH},
    environment::farm_entity_module::farm_entity_plan::FarmEntityPlan,
    tool_module::{has_help::HasHelp, tool::Tool},
    utilities::utils::get_json_files_in_folder
};


pub struct FarmEntityPlanEditorTool {
    content: String,
    save_file_name: String,
    pub current_farm_entity_plan_path: String,
    pub help_open: bool,
}

impl Default for FarmEntityPlanEditorTool {
    fn default() -> Self {
        let file_path = DEFAULT_POINT_FARM_ENTITY_PLAN_PATH;
        let json_str = fs::read_to_string(file_path).expect("File not found");
        Self {
            content: json_str,
            save_file_name: String::new(),
            current_farm_entity_plan_path: file_path.to_string(),
            help_open: false,
        }
    }
}

impl Tool for FarmEntityPlanEditorTool {
    fn render_main(&mut self, ui: &mut egui::Ui) {

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.add(
                egui::TextEdit::multiline(&mut self.content)
                    .font(egui::TextStyle::Monospace)
                    .code_editor()
                    .desired_rows(10)
                    .lock_focus(true)
                    .desired_width(f32::INFINITY)
            );
        });
    
    }

    fn render_ui(&mut self, ui: &mut egui::Ui) {
        self.render_help_button(ui);
        ui.separator();

        ui.horizontal(|ui| {
            ui.label(FARM_ENTITY_PLANS_PATH);
            ui.add(egui::TextEdit::singleline(&mut self.save_file_name).desired_width(100.0));
            ui.label(".json");
            ui.spacing();
            if ui.button("Save farm entity plan").clicked() && !self.save_file_name.is_empty() {
                let save_file_path = format!("{}{}.json", FARM_ENTITY_PLANS_PATH, self.save_file_name.clone());
                let result = self.save_as_json(&save_file_path);
                match result {
                    Ok(_) => {
                        println!("File saved");
                        self.current_farm_entity_plan_path = save_file_path;
                    },
                    Err(error) => eprintln!("{}", error)
                }
            }
        });
        self.config_select(ui);

        self.render_help(ui);
    }

    fn update(&mut self) {
        
    }
}

impl FarmEntityPlanEditorTool {
    fn config_select(&mut self, ui: &mut Ui) {
        egui::ComboBox::from_label("")
            .selected_text(format!("{:?}", self.current_farm_entity_plan_path))
            .show_ui(ui, |ui| {
                let json_files = get_json_files_in_folder(FARM_ENTITY_PLANS_PATH);
                let previous_value = self.current_farm_entity_plan_path.clone();

                for json_file in json_files {
                    let new_value = format!("{}{}", FARM_ENTITY_PLANS_PATH, json_file.clone());
                    ui.selectable_value(&mut self.current_farm_entity_plan_path, new_value.clone(), json_file);
                }

                if *self.current_farm_entity_plan_path != previous_value {
                    let json_str = fs::read_to_string(self.current_farm_entity_plan_path.clone()).expect("File not found");
                    self.content = json_str;
                }
            });
    }

    fn save_as_json(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Check if it can be serialized
        let _: FarmEntityPlan = serde_json::from_str(&self.content)?;

        // Create file
        let mut file = File::create(file_path)?;
        
        // Write JSON to file
        file.write_all(self.content.as_bytes())?;
        
        println!("Successfully saved to {}", file_path);
        Ok(())
    }
}

impl HasHelp for FarmEntityPlanEditorTool {
    fn help_modal(&self) -> egui::Modal {
        egui::Modal::new(egui::Id::new("Farm Entity Plan Editor Tool Help"))
    }
    fn render_help_contents(&self, ui: &mut egui::Ui) {
        ui.heading("Farm Entity Plan Editor Tool Help");
        ui.label("This is a Farm Entity Plan Editor where you can see, change, create, save plans.");
        ui.separator();

        ui.label("Type specifies if whole plan is point/stationary or line/moving. (point and line)");
        ui.label("Cycle parameter specify if after the last action the plan cycles and from which index");

        ui.label("There are 3 types of actions:");
        ui.monospace(
        r#"pub enum FarmEntityAction {
    Point {
        action_name: String,
        duration: Duration,
        power: Power,
    },
    Line {
        action_name: String,
        velocity: LinearVelocity,
        power: Power,
    },
    Wait {
        action_name: String,
        duration: Duration,
    }
}"#,
    );
        ui.label("Where power, duration and linear velocity can be specified as:");
        ui.label("Power: W-watt");
        ui.label("Duration: s-second, min-minute, h-hour, d-day");
        ui.label("Linear velocity: m/s-meters per second, km/h-kilometers per hour");
    }
}