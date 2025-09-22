use std::collections::HashMap;
use egui::Slider;
use egui_plot::{HLine, Legend, Line, Plot, PlotPoints};

use crate::{
    battery_module::{battery::Battery, battery_config::BatteryConfig, is_battery::IsBattery}, cfg::BATTERIES_PATH, tool_module::{has_help::HasHelp, tool::Tool}, utilities::utils::get_folders_in_folder
};

/// A tool for inspecting and interacting with battery configuration data.
#[derive(Debug)]
pub struct BatteryTool {
    selected: Option<String>,
    folder_names: Vec<String>,
    battery_map: HashMap<String, Battery>,
    month: u32,
    morph_data: Option<Vec<(u32, f32)>>,
    pub help_open: bool,
}

impl Default for BatteryTool {
    fn default() -> Self {
        let folders = get_folders_in_folder(BATTERIES_PATH);
        Self {
            selected: None,
            folder_names: folders,
            battery_map: HashMap::new(),
            month: 1,
            morph_data: None,
            help_open: false,
        }
    }
}


impl Tool for BatteryTool {
    fn render_main(&mut self, ui: &mut egui::Ui) {
        match &self.selected {
            None => {}
            Some(selected) => {
                if let Some(battery) = self.battery_map.get(selected) {

                    let jan_max: PlotPoints = battery.jan_max_data
                        .iter()
                        .map(|(x, y)| [f64::from(*x), f64::from(*y)])
                        .collect::<Vec<_>>()
                        .into();
                    let line_jan_max = Line::new("January Max", jan_max);
            
                    let jan_min: PlotPoints = battery.jan_min_data
                        .iter()
                        .map(|(x, y)| [f64::from(*x), f64::from(*y)])
                        .collect::<Vec<_>>()
                        .into();
                    let line_jan_min = Line::new("January Min", jan_min);
            
                    let jun_max: PlotPoints = battery.jun_max_data
                        .iter()
                        .map(|(x, y)| [f64::from(*x), f64::from(*y)])
                        .collect::<Vec<_>>()
                        .into();
                    let line_jun_max = Line::new("June Max", jun_max);

                    let line_morph = match &self.morph_data {
                        Some(data) => {
                            let morph: PlotPoints = data
                                .iter()
                                .map(|(x, y)| [f64::from(*x), f64::from(*y)])
                                .collect::<Vec<_>>()
                                .into();
                            Line::new("Morph", morph)
                        }
                        None => Line::new("Morph", vec![]), // fallback empty line
                    };
            
            
                    Plot::new("battery_plot")
                        .legend(Legend::default())
                        .auto_bounds(true)
                        .x_axis_label("Time (s)")
                        .y_axis_label("Energy (Wh)")
                        .show(ui, |plot_ui| {
                            let line = HLine::new("Current energy", battery.energy.value);
                            plot_ui.hline(line);
                            plot_ui.line(line_jan_max);
                            plot_ui.line(line_jan_min);
                            plot_ui.line(line_jun_max);
                            plot_ui.line(line_morph);
                        });
                }
        

            }
        }
    }

    fn render_ui(&mut self, ui: &mut egui::Ui) {
        self.render_help_button(ui);
        ui.separator();

        ui.label("Batteries");
        for folder in &self.folder_names {
            let whole_path = format!("{}{}", BATTERIES_PATH, folder.clone());
            if ui.button(whole_path.clone()).clicked() {
                self.selected = Some(whole_path.clone());
                self.battery_map.entry(whole_path.clone()).or_insert_with(|| {
                    Battery::from_config(BatteryConfig::from_json_file(whole_path), 70.0)
                });
            }
        }
        if ui.button("Deselect").clicked() {
            self.selected = None;
        }

        ui.separator();

        if let Some(selected) = &self.selected {
            if let Some(battery) = self.battery_map.get_mut(selected) {
                ui.label("Double click on plot to reset");
                ui.spacing();
                ui.heading(selected);
                ui.label(format!("Voltage: {}", battery.voltage));
                ui.label(format!("Capacity: {}", battery.capacity));
                ui.label(format!("Energy: {}", battery.energy));

                let response = ui.add(Slider::new(
                    &mut battery.soc,
                    0.0..=100.0)
                    .text("SoC [%]")
                    .step_by(1.0)
                );
                if response.changed() {
                    battery.recalculate_energy();
                }
                let response = ui.add(Slider::new(
                    &mut self.month,
                    1..=12)
                    .text("Month")
                    .step_by(1.0)
                );
                if response.changed() || response.enabled() {
                    let mut data = vec![];
                    let mut i = 26.0;
                    battery.start_index.insert("jan".to_string(), 1);
                    battery.start_index.insert("jun".to_string(), 1);
                    while i <= battery.capacity.value {
                        match battery.get_morph_x_y(i, self.month, 1) {
                            Ok((time, energy)) => {
                                data.push((time, energy));
                            }
                            Err(e) => {
                                eprintln!("⚠️ Failed to get morph x/y for i = {i}: {e}");
                            }
                        }
                        i += 5.0;
                    }
                    self.morph_data = Some(data);
                }
            }
        }
        
        self.render_help(ui);
    }

    fn update(&mut self) {
        
    }
}

impl HasHelp for BatteryTool {
    fn help_modal(&self) -> egui::Modal {
        egui::Modal::new(egui::Id::new("Battery Tool Help"))
    }
    fn render_help_contents(&self, ui: &mut egui::Ui) {
        ui.heading("Battery Tool Help");
        ui.label("This is a battery tool where you can see selected battery charging characteristics and parameters.");
        ui.separator();

        ui.label("When battery is selected you can see morphed characteristics between jan and jun data.");
    }
}