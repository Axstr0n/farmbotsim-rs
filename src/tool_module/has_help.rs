use crate::tool_module::{agent_config_editor_tool::AgentConfigEditorTool, battery_tool::BatteryTool, editor_tool::EditorTool, farm_entity_plan_editor_tool::FarmEntityPlanEditorTool, movement_config_editor_tool::MovementConfigEditorTool, path_tool::PathTool, simulation_tool::SimulationTool, task_tool::TaskTool};

pub trait HasHelpCore {
    fn is_help_open(&self) -> bool;
    fn set_help_open(&mut self, open: bool);
}

pub trait HasHelp: HasHelpCore {
    fn render_help_button(&mut self, ui: &mut egui::Ui) {
        if ui.button("Help").clicked() {
            self.set_help_open(true);
        }
    }

    fn render_help(&mut self, ui: &mut egui::Ui) {
        if self.is_help_open() {
            let modal = self.help_modal();

            let modal_response = modal.show(ui.ctx(), |ui| {
                ui.set_width(550.0);
                self.render_help_contents(ui);
            });

            if modal_response.should_close() {
                self.set_help_open(false);
            }
        }
    }

    fn help_modal(&self) -> egui::Modal;
    fn render_help_contents(&self, ui: &mut egui::Ui);
}



macro_rules! impl_help_core {
    ($t:ty) => {
        impl HasHelpCore for $t {
            fn is_help_open(&self) -> bool {
                self.help_open
            }

            fn set_help_open(&mut self, open: bool) {
                self.help_open = open;
            }
        }
    };
}

impl_help_core!(SimulationTool);
impl_help_core!(PathTool);
impl_help_core!(TaskTool);
impl_help_core!(EditorTool);
impl_help_core!(BatteryTool);
impl_help_core!(FarmEntityPlanEditorTool);
impl_help_core!(MovementConfigEditorTool);
impl_help_core!(AgentConfigEditorTool);