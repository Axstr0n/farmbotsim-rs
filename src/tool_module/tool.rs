use egui::Ui;

pub trait Tool {
    fn update(&mut self);

    fn render_main(&mut self, ui: &mut Ui);
    fn render_ui(&mut self, ui: &mut Ui);
}
