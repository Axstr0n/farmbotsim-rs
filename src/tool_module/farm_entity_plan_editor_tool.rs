use egui::Ui;

use crate::{
    cfg::{DEFAULT_POINT_FARM_ENTITY_PLAN_PATH, FARM_ENTITY_PLANS_PATH},
    environment::farm_entity_module::{farm_entity_action::FarmEntityAction, farm_entity_plan::FarmEntityPlan},
    tool_module::{has_config_saving::HasConfigSaving, has_help::HasHelp, tool::Tool},
    utilities::utils::{json_config_combo, load_json_or_panic, value_with_unit_selector_ui}
};

/// A tool for defining, viewing, changing farm entity plan
pub struct FarmEntityPlanEditorTool {
    plan: FarmEntityPlan,
    save_file_name: String,
    pub current_farm_entity_plan_path: String,
    pub help_open: bool,
}

impl Default for FarmEntityPlanEditorTool {
    fn default() -> Self {
        let plan: FarmEntityPlan = load_json_or_panic(DEFAULT_POINT_FARM_ENTITY_PLAN_PATH);
        Self {
            plan,
            save_file_name: String::new(),
            current_farm_entity_plan_path: DEFAULT_POINT_FARM_ENTITY_PLAN_PATH.to_string(),
            help_open: false,
        }
    }
}

impl Tool for FarmEntityPlanEditorTool {
    fn render_main(&mut self, ui: &mut egui::Ui) {
        ui.label("{");
        ui.horizontal(|ui| {
            ui.label("    name:");
            ui.add(egui::TextEdit::singleline(&mut self.plan.crop_name).desired_width(100.0));
        });
        self.save_file_name = self.plan.crop_name.clone();
        // Type selection
        ui.horizontal(|ui| {
            ui.label("    type:");
            egui::ComboBox::from_id_salt("Type")
                .selected_text(self.plan.type_.clone())
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.plan.type_, "point".to_string(), "point");
                    ui.selectable_value(&mut self.plan.type_, "line".to_string(), "line");
                    let mut to_delete: Vec<usize> = vec![];
                    for (i, action) in self.plan.schedule.iter().enumerate() {
                        match action {
                            FarmEntityAction::Point { .. } => {
                                if self.plan.type_ != "point" {
                                    to_delete.push(i);
                                }
                            },
                            FarmEntityAction::Line { .. } => {
                                if self.plan.type_ != "line" {
                                    to_delete.push(i);
                                }
                            },
                            FarmEntityAction::Wait { .. } => {},
                        }
                    }
                    for &i in to_delete.iter().rev() {
                        self.plan.schedule.remove(i);
                    }
                    self.check_cycle();
                });
        });
        // Cycle
        ui.horizontal(|ui| {
            ui.label("    cycle:");
            ui.horizontal(|ui| {
                let is_none = self.plan.cycle.is_none();
                if ui.selectable_label(is_none, "None").clicked() {
                    self.plan.cycle = None;
                }

                for i in 0..self.plan.schedule.len() {
                    let label = format!("{i}");
                    let is_selected = self.plan.cycle == Some(i as u32);
                    if ui.selectable_label(is_selected, label).clicked() {
                        self.plan.cycle = Some(i as u32);
                    }
                }
            });
        });
        // Plan
        ui.horizontal(|ui| {
            ui.label("    plan: [");
            if self.plan.type_ == "point" && ui.button("Add point").clicked() { self.plan.schedule.push(FarmEntityAction::default_point()); }
            if self.plan.type_ == "line" && ui.button("Add line").clicked() { self.plan.schedule.push(FarmEntityAction::default_line()); }
            if ui.button("Add wait").clicked() { self.plan.schedule.push(FarmEntityAction::default_wait()); } 
            if ui.button("Remove all").clicked() { self.plan.schedule.clear(); }
            self.check_cycle();
        });
        let mut to_delete: Vec<usize> = vec![];
        for (i,action) in &mut self.plan.schedule.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                match action {
                    FarmEntityAction::Point{ action_name, duration, power } => {
                        ui.label(format!("        [{i}]"));
                        ui.label("{ \"Point\": {");
                        ui.label("\"action_name\":");
                        ui.add(egui::TextEdit::singleline(action_name).desired_width(100.0));
                        value_with_unit_selector_ui(ui, &format!("duration{i}"),"duration", &mut duration.value, &mut duration.unit, Some(0.0), None);
                        value_with_unit_selector_ui(ui, &format!("power{i}"),"power", &mut power.value, &mut power.unit, Some(0.0), None);
                        ui.label("} }");
                    },
                    FarmEntityAction::Line { action_name, velocity, power } => {
                        ui.label(format!("        [{i}]"));
                        ui.label("{ \"Line\": {");
                        ui.label("\"action_name\":");
                        ui.add(egui::TextEdit::singleline(action_name).desired_width(100.0));
                        value_with_unit_selector_ui(ui, &format!("velocity{i}"),"velocity", &mut velocity.value, &mut velocity.unit, Some(0.0), None);
                        value_with_unit_selector_ui(ui, &format!("power{i}"),"power", &mut power.value, &mut power.unit, Some(0.0), None);
                        ui.label("} }");
                    },
                    FarmEntityAction::Wait { action_name, duration } => {
                        ui.label(format!("        [{i}]"));
                        ui.label("{ \"Wait\": {");
                        ui.label("\"action_name\":");
                        ui.add(egui::TextEdit::singleline(action_name).desired_width(100.0));
                        value_with_unit_selector_ui(ui, &format!("duration{i}"),"duration", &mut duration.value, &mut duration.unit, Some(0.0), None);
                        ui.label("} }");
                    },
                }
                if ui.button("Remove").clicked() {
                    to_delete.push(i);
                }
            });
        }
        for &i in to_delete.iter().rev() {
            self.plan.schedule.remove(i);
        }
        self.check_cycle();
        ui.label("    ]");

        ui.label("}");
    }

    fn render_ui(&mut self, ui: &mut egui::Ui) {
        self.render_help_button(ui);
        ui.separator();

        let mut save_file_name = self.save_file_name.clone();
        self.draw_save_ui(ui, &mut save_file_name, false);
        self.save_file_name = save_file_name;

        self.ui_farm_entity_plan_select(ui);

        self.render_help(ui);
    }

    fn update(&mut self) {
        
    }
}

impl FarmEntityPlanEditorTool {
    /// Renders a dropdown to select a different farm entity plan configuration file.
    fn ui_farm_entity_plan_select(&mut self, ui: &mut Ui) {
        let mut new_value = self.current_farm_entity_plan_path.clone();

        if json_config_combo(ui, "", &mut new_value, FARM_ENTITY_PLANS_PATH)
            && new_value != self.current_farm_entity_plan_path
        {
            self.current_farm_entity_plan_path = new_value;
            let plan: FarmEntityPlan = load_json_or_panic(self.current_farm_entity_plan_path.clone());
            self.plan = plan;
        }
    }

    // Updates cycle paramater in plan so it is always valid
    fn check_cycle(&mut self) {
        if let Some(cycle) = self.plan.cycle {
            if self.plan.schedule.is_empty() { self.plan.cycle = None; }
            else if cycle >= self.plan.schedule.len() as u32 { self.plan.cycle = Some(self.plan.schedule.len() as u32-1) }
        }
    }

}

impl HasConfigSaving for FarmEntityPlanEditorTool {
    fn base_path() -> &'static str {
        FARM_ENTITY_PLANS_PATH
    }
    fn config(&self) -> impl serde::Serialize {
        self.plan.clone()
    }
    fn update_current_path(&mut self, path: String) {
        self.current_farm_entity_plan_path = path;
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

        ui.label("Type specifies if whole plan is point/stationary or line/moving.");
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
        ui.label("If type is point then only point and wait actions are available");
        ui.label("If type is line then only line and wait actions are available");
        ui.label("This actions will be converted to tasks where:");
        ui.label("  point - stationary task");
        ui.label("  line - moving task");
        ui.label("  wait - internal task in task manager where task manager waits duration before adding next task");
    }
}