use crate::tool_module::{agent_config_editor_tool::AgentConfigEditorTool, battery_tool::BatteryTool, farm_entity_plan_editor_tool::FarmEntityPlanEditorTool, field_config_editor_tool::FieldConfigEditorTool, movement_config_editor_tool::MovementConfigEditorTool, path_tool::PathTool, performance_matrix_tool::PerformanceMatrixTool, scene_config_editor_tool::SceneConfigEditorTool, simulation_tool::SimulationTool, task_tool::TaskTool};

/// Core trait for types that support toggling a help UI modal.
pub trait HasHelpCore {
    /// Returns `true` if the help modal is open.
    fn is_help_open(&self) -> bool;

    /// Sets the help modal open or closed.
    fn set_help_open(&mut self, open: bool);
}

/// Trait that builds upon [`HasHelpCore`] to render help UI elements.
pub trait HasHelp: HasHelpCore {
    /// Renders a "Help" button.
    fn render_help_button(&mut self, ui: &mut egui::Ui) {
        if ui.button("Help").clicked() {
            self.set_help_open(true);
        }
    }

    /// Renders the help modal window, if open.
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

    /// Returns an `egui::Modal` used to render the help popup window.
    fn help_modal(&self) -> egui::Modal;

    /// Renders the contents of the help modal.
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
impl_help_core!(BatteryTool);
impl_help_core!(FarmEntityPlanEditorTool);
impl_help_core!(MovementConfigEditorTool);
impl_help_core!(AgentConfigEditorTool);
impl_help_core!(FieldConfigEditorTool);
impl_help_core!(SceneConfigEditorTool);
impl_help_core!(PerformanceMatrixTool);