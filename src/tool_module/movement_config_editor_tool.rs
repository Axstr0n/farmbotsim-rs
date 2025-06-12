use crate::{
    agent_module::movement::{Movement, RombaMovement}, cfg::{DEFAULT_ROMBA_MOVEMENT_CONFIG_PATH, MOVEMENT_CONFIGS_PATH}, tool_module::{has_config_saving::HasConfigSaving, has_help::HasHelp, tool::Tool}, utilities::utils::{json_config_combo, load_json_or_panic}
};


pub struct MovementConfigEditorTool {
    movement: Movement,
    save_file_name: String,
    pub current_movement_config_path: String,
    pub help_open: bool,
}

impl Default for MovementConfigEditorTool {
    fn default() -> Self {
        let movement = load_json_or_panic(DEFAULT_ROMBA_MOVEMENT_CONFIG_PATH);
        Self {
            movement,
            save_file_name: String::new(),
            current_movement_config_path: DEFAULT_ROMBA_MOVEMENT_CONFIG_PATH.to_string(),
            help_open: false,
        }
    }
}

impl Tool for MovementConfigEditorTool {
    fn render_main(&mut self, ui: &mut egui::Ui) {
        self.movement_ui(ui);
    }

    fn render_ui(&mut self, ui: &mut egui::Ui) {
        self.render_help_button(ui);
        ui.separator();

        let mut save_file_name = self.save_file_name.clone();
        self.draw_save_ui(ui, &mut save_file_name);
        self.save_file_name = save_file_name;

        self.ui_movement_select(ui);

        self.render_help(ui);
    }

    fn update(&mut self) {
        
    }
}

impl MovementConfigEditorTool {
    fn ui_movement_select(&mut self, ui: &mut egui::Ui) {
        let mut new_path = self.current_movement_config_path.clone();

        if json_config_combo(ui, "", &mut new_path, MOVEMENT_CONFIGS_PATH)
            && new_path != self.current_movement_config_path
        {
            self.current_movement_config_path = new_path;
            let movement = load_json_or_panic(self.current_movement_config_path.clone());
            self.movement = movement;
        }
    }

    fn movement_ui(&mut self, ui: &mut egui::Ui) {
        ui.label("{");
        ui.horizontal(|ui| {
            ui.label("    \"type\":");

            // Type selection
            egui::ComboBox::from_id_salt("Type")
                .selected_text(match self.movement {
                    Movement::RombaMovement(_) => "RombaMovement",
                })
                .show_ui(ui, |ui| {
                    if ui.selectable_label(matches!(self.movement, Movement::RombaMovement(_)), "RombaMovement").clicked() {
                        self.movement = Movement::RombaMovement(RombaMovement::default())
                    }
                    // More variants ...
                });
        });

        // Param fields
        match &mut self.movement {
            Movement::RombaMovement(params) => {
                ui.label("    params: {");

                // Max velocity
                Self::param_with_unit_ui(ui, "max_velocity", &mut params.max_velocity.value, &mut params.max_velocity.unit);

                // Angular velocity
                Self::param_with_unit_ui(ui, "max_angular_velocity", &mut params.max_angular_velocity.value, &mut params.max_angular_velocity.unit);

                // Wheel distance
                Self::param_with_unit_ui(ui, "wheel_distance", &mut params.wheel_distance.value, &mut params.wheel_distance.unit);

                // Wheel radius
                Self::param_with_unit_ui(ui, "wheel_radius", &mut params.wheel_radius.value, &mut params.wheel_radius.unit);

                ui.label("    }");
            }
        }
        ui.label("}");
    }
    fn param_with_unit_ui<T: ToString + PartialEq + Copy + enum_iterator::Sequence>(
        ui: &mut egui::Ui,
        label: &str,
        value: &mut f32,
        unit: &mut T,
    ) {
        ui.horizontal(|ui| {
            ui.label(format!("        \"{}\":", label));
            ui.add(egui::DragValue::new(value).speed(0.1));

            egui::ComboBox::from_id_salt(label)
                .selected_text(unit.to_string())
                .show_ui(ui, |ui| {
                    for u in enum_iterator::all::<T>() {
                        ui.selectable_value(unit, u, u.to_string());
                    }
                });
        });
    }

}

impl HasConfigSaving for MovementConfigEditorTool {
    fn base_path() -> &'static str {
        MOVEMENT_CONFIGS_PATH
    }
    fn config(&self) -> impl serde::Serialize {
        self.movement.clone()
    }
    fn update_current_path(&mut self, path: String) {
        self.current_movement_config_path = path;
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