use std::{collections::HashMap, fs};

use egui::Slider;
use egui_plot::{HLine, Legend, Line, Plot, PlotPoints};

use crate::agent::battery::{Battery, BatteryPack};

use super::tool::Tool;


#[derive(Debug)]
pub struct BatteryTool {
    selected: Option<String>,
    folder_names: Vec<String>,
    battery_map: HashMap<String, BatteryPack>,
    month: u32,
    morph_data: Option<Vec<(u32, f32)>>,
}

impl Default for BatteryTool {
    fn default() -> Self {
        let root = "batteries";
        let mut folders: Vec<String> = vec![];
        for entry in fs::read_dir(root).unwrap_or_else(|_| panic!("No folder named {}", root)) {
            let entry = entry.expect("No subfolders found");
            let path = entry.path();
            
            if path.is_dir() {
                if let Some(folder_name) = path.file_name() {
                    if let Some(name) = folder_name.to_str() {
                        folders.push(name.to_string());
                    }
                }
            }
        }
        Self {
            selected: None,
            folder_names: folders,
            battery_map: HashMap::new(),
            month: 1,
            morph_data: None,
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
                            let line = HLine::new("Current energy", battery.energy_wh);
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

        ui.label("Batteries");
        for folder in &self.folder_names {
            if ui.button(folder).clicked() {
                self.selected = Some(folder.clone());
                self.battery_map.entry(folder.clone()).or_insert_with(|| {
                    BatteryPack::from_config(folder.clone(), Some(70.0))
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
                ui.label(format!("Voltage (V): {}", battery.voltage));
                ui.label(format!("Capacity (Wh): {}", battery.capacity_wh));
                ui.label(format!("Energy (Wh): {}", battery.energy_wh));

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
                    while i <= battery.capacity_wh {
                        match battery.get_morph_x_y(i, self.month, 1) {
                            Ok((time, energy)) => {
                                data.push((time, energy));
                            }
                            Err(e) => {
                                eprintln!("⚠️ Failed to get morph x/y for i = {}: {}", i, e);
                            }
                        }
                        i += 5.0;
                    }
                    self.morph_data = Some(data);
                }
            }
        }
    }

    fn update(&mut self) {
        
    }
}

impl BatteryTool {
    
}