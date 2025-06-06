use std::{fs::{self, File}, io::Write};
use egui::Ui;

use crate::{
    agent_module::movement::Movement, cfg::{DEFAULT_ROMBA_MOVEMENT_CONFIG_PATH, MOVEMENT_CONFIGS_PATH}, tool_module::{has_help::HasHelp, tool::Tool}, utilities::utils::get_json_files_in_folder
};


pub struct MovementConfigEditorTool {
    content: String,
    save_file_name: String,
    pub current_movement_config_path: String,
    pub help_open: bool,
}

impl Default for MovementConfigEditorTool {
    fn default() -> Self {
        let file_path = DEFAULT_ROMBA_MOVEMENT_CONFIG_PATH;
        let json_str = fs::read_to_string(file_path).expect("File not found");
        Self {
            content: json_str,
            save_file_name: String::new(),
            current_movement_config_path: file_path.to_string(),
            help_open: false,
        }
    }
}

impl Tool for MovementConfigEditorTool {
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
            ui.label(MOVEMENT_CONFIGS_PATH);
            ui.add(egui::TextEdit::singleline(&mut self.save_file_name).desired_width(100.0));
            ui.label(".json");
            ui.spacing();
            if ui.button("Save movement config").clicked() && !self.save_file_name.is_empty() {
                let result = self.save_as_json(&self.save_file_name);
                match result {
                    Ok(_) => {
                        println!("File saved");
                        self.current_movement_config_path = format!("{}{}.json", MOVEMENT_CONFIGS_PATH, self.save_file_name.clone());
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

impl MovementConfigEditorTool {
    fn config_select(&mut self, ui: &mut Ui) {
        egui::ComboBox::from_label("")
            .selected_text(format!("{:?}", self.current_movement_config_path))
            .show_ui(ui, |ui| {
                let json_files = get_json_files_in_folder(MOVEMENT_CONFIGS_PATH).expect("Can't find json files");
                let previous_value = self.current_movement_config_path.clone();

                for json_file in json_files {
                    let new_value = format!("{}{}", MOVEMENT_CONFIGS_PATH, json_file.clone());
                    ui.selectable_value(&mut self.current_movement_config_path, new_value.clone(), json_file);
                }

                if *self.current_movement_config_path != previous_value {
                    let json_str = fs::read_to_string(self.current_movement_config_path.clone()).expect("File not found");
                    self.content = json_str;
                }
            });
    }

    fn save_as_json(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Check if it can be serialized
        let _: Movement = serde_json::from_str(&self.content)?;

        // Create file
        let mut file = File::create(format!("{}{}.json", MOVEMENT_CONFIGS_PATH, filename))?;
        
        // Write JSON to file
        file.write_all(self.content.as_bytes())?;
        
        println!("Successfully saved to {}", filename);
        Ok(())
    }
}

impl HasHelp for MovementConfigEditorTool {
    fn help_modal(&self) -> egui::Modal {
        egui::Modal::new(egui::Id::new("Movement Config Editor Tool Help"))
    }
    fn render_help_contents(&self, ui: &mut egui::Ui) {
        ui.heading("Movement Config Editor Tool Help");
        ui.label("This is a Movement Config Editor where you can see, change, create, save movement configs.");
        ui.separator();

        ui.label("There is 1 type of movement:");
        ui.monospace(
        r#"pub struct RombaMovement {
    pub max_velocity: LinearVelocity,
    pub max_angular_velocity: AngularVelocity,
    pub wheel_distance: Length,
    pub wheel_radius: Length
}"#,
    );
        ui.label("Where linear velocity, angular velocity, length can be specified as:");
        ui.label("Linear velocity: m/s-meters per second, km/h-kilometers per hour");
        ui.label("Angular velocity: rad/s-radians per second");
        ui.label("Length: m-meters, cm-centimeters, mm-millimeters");
    }
}