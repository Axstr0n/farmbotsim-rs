use serde::Serialize;

use crate::utilities::utils::save_as_json;

/// A trait to provide standardized configuration saving functionality.
pub trait HasConfigSaving{
    /// Returns the base directory path as a static string slice
    fn base_path() -> &'static str;

    /// Returns a serializable reference to the current configuration.
    fn config(&self) -> impl Serialize;

    /// Updates the current saved file path after a successful save.
    fn update_current_path(&mut self, path: String);

    /// Hook called after a successful save operation.
    fn update_after_save(&mut self) {

    }

    /// Draws a save UI panel.
    fn draw_save_ui(
        &mut self,
        ui: &mut egui::Ui,
        save_file_name: &mut String,
        editable_save_name: bool,
    ) {
        ui.horizontal(|ui| {
            if editable_save_name {
                ui.label(Self::base_path());
                ui.add(egui::TextEdit::singleline(save_file_name).desired_width(100.0));
                ui.label(".json");
            } else {
                ui.label(format!("{}{}.json", Self::base_path(), save_file_name));
            }

            if ui.button("Save config").clicked() && !save_file_name.is_empty() {
                let base_path = Self::base_path();
                let save_file_path = format!("{base_path}{save_file_name}.json");
                
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
                    Err(e) => eprintln!("Error saving config: {e}"),
                }
            }
        });
    }
}