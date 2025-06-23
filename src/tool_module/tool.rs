use egui::Ui;

/// Trait representing a generic tool with update and rendering capabilities.
pub trait Tool {
    /// Update the tool's internal state.
    fn update(&mut self);

    /// Render the main UI content of the tool.
    fn render_main(&mut self, ui: &mut Ui);

    // Render side panel UI elements.
    fn render_ui(&mut self, ui: &mut Ui);
}
