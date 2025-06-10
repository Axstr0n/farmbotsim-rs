use serde::Serialize;

use crate::utilities::utils::save_as_json;

pub trait HasConfigSaving{
    fn base_path() -> &'static str;

    fn config(&self) -> impl Serialize;

    fn update_current_path(&mut self, path: String);

    fn update_after_save(&mut self) {

    }

    fn draw_save_ui(
        &mut self,
        ui: &mut egui::Ui,
        save_file_name: &mut String,
    ) {
        ui.horizontal(|ui| {
            ui.label(Self::base_path());
            ui.add(egui::TextEdit::singleline(save_file_name).desired_width(100.0));
            ui.label(".json");

            if ui.button("Save config").clicked() && !save_file_name.is_empty() {
                let base_path = Self::base_path();
                let save_file_path = format!("{}{}.json", base_path, save_file_name);
                
                let save_result = {
                    let config = self.config();
                    save_as_json(&config, &save_file_path)
                };

                match save_result {
                    Ok(_) => {
                        println!("File saved");
                        self.update_current_path(save_file_path);
                        self.update_after_save();
                    }
                    Err(e) => eprintln!("Error saving config: {}", e),
                }
            }
        });
    }
}