use std::{fs::{self, File}, io::Write};
use egui::Ui;

use crate::{cfg::{CROP_PLANS_PATH, DEFAULT_POINT_CROP_PLAN_PATH}, environment::crop_plan::CropPlan, utilities::utils::get_json_files_in_folder};

use super::tool::Tool;


pub struct CropPlanEditorTool {
    content: String,
    save_file_name: String,
    pub current_crop_plan_path: String,
}

impl Default for CropPlanEditorTool {
    fn default() -> Self {
        let file_path = DEFAULT_POINT_CROP_PLAN_PATH;
        let json_str = fs::read_to_string(file_path).expect("File not found");
        Self {
            content: json_str,
            save_file_name: String::new(),
            current_crop_plan_path: file_path.to_string(),
        }
    }
}

impl Tool for CropPlanEditorTool {
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
        ui.horizontal(|ui| {
            ui.label(CROP_PLANS_PATH);
            ui.add(egui::TextEdit::singleline(&mut self.save_file_name).desired_width(100.0));
            ui.label(".json");
            ui.spacing();
            if ui.button("Save crop plan").clicked() && !self.save_file_name.is_empty() {
                let result = self.save_as_json(&self.save_file_name);
                match result {
                    Ok(_) => println!("File saved"),
                    Err(error) => eprintln!("{}", error)
                }
                self.current_crop_plan_path = format!("{}{}.json", CROP_PLANS_PATH, self.save_file_name.clone());
            }
        });
        self.config_select(ui);
    }

    fn update(&mut self) {
        
    }
}

impl CropPlanEditorTool {
    fn config_select(&mut self, ui: &mut Ui) {
        egui::ComboBox::from_label("")
            .selected_text(format!("{:?}", self.current_crop_plan_path))
            .show_ui(ui, |ui| {
                let json_files = get_json_files_in_folder(CROP_PLANS_PATH).expect("Can't find json files");
                let previous_value = self.current_crop_plan_path.clone();

                for json_file in json_files {
                    let new_value = format!("{}{}", CROP_PLANS_PATH, json_file.clone());
                    ui.selectable_value(&mut self.current_crop_plan_path, new_value.clone(), json_file);
                }

                if *self.current_crop_plan_path != previous_value {
                    let json_str = fs::read_to_string(self.current_crop_plan_path.clone()).expect("File not found");
                    self.content = json_str;
                }
            });
    }

    fn save_as_json(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Check if it can be serialized
        let _: CropPlan = serde_json::from_str(&self.content)?;

        // Create file
        let mut file = File::create(format!("{}{}.json", CROP_PLANS_PATH, filename))?;
        
        // Write JSON to file
        file.write_all(self.content.as_bytes())?;
        
        println!("Successfully saved to {}", filename);
        Ok(())
    }
}