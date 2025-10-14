use crate::utilities::files::{get_folders_in_folder, get_json_files_in_folder};

/// Renders dropdown of all json config files in folder path.
pub fn json_config_combo(
    ui: &mut egui::Ui,
    id_salt: &str,
    current_value: &mut String,
    folder_path: &str,
) -> bool {
    let mut changed = false;

    egui::ComboBox::from_id_salt(id_salt)
        .selected_text(format!("{current_value:?}"))
        .show_ui(ui, |ui| {
            let json_files = get_json_files_in_folder(folder_path);
            let previous_value = current_value.clone();

            for json_file in json_files {
                let new_value = format!("{folder_path}{json_file}");
                ui.selectable_value(current_value, new_value.clone(), json_file);
            }

            if *current_value != previous_value {
                changed = true;
            }
        });

    changed
}

/// Renders dropdown of all folders in folder path.
pub fn folder_select_combo(
    ui: &mut egui::Ui,
    id_salt: &str,
    current_value: &mut String,
    base_path: &str,
) -> bool {
    let mut changed = false;

    egui::ComboBox::from_id_salt(id_salt)
        .selected_text(current_value.as_str())
        .show_ui(ui, |ui| {
            let options = get_folders_in_folder(base_path);
            let previous_value = current_value.clone();

            for option in options {
                let full_path = format!("{base_path}{option}");
                ui.selectable_value(current_value, full_path.clone(), full_path);
            }

            if *current_value != previous_value {
                changed = true;
            }
        });

    changed
}

/// Renders adjustable drag value and dropdown for unit.
pub fn value_with_unit_selector_ui<T: ToString + PartialEq + Copy + enum_iterator::Sequence>(
    ui: &mut egui::Ui,
    id_salt: &str,
    label: &str,
    value: &mut f32,
    unit: &mut T,
    min_value: Option<f32>,
    max_value: Option<f32>,
) {
    ui.horizontal(|ui| {
        ui.label(format!("        \"{label}\":"));
        ui.add(egui::DragValue::new(value).speed(0.1));
        if let Some(min) = min_value {
            if *value < min {
                *value = min;
            }
        }
        if let Some(max) = max_value {
            if *value > max {
                *value = max;
            }
        }

        egui::ComboBox::from_id_salt(id_salt)
            .selected_text(unit.to_string())
            .show_ui(ui, |ui| {
                for u in enum_iterator::all::<T>() {
                    ui.selectable_value(unit, u, u.to_string());
                }
            });
    });
}
