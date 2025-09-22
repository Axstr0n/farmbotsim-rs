use std::{collections::{HashMap, VecDeque}, fs::File, io::{BufRead, BufReader}, path::Path};

use crate::{battery_module::{battery_config::BatteryConfig, battery_error::BatteryError, is_battery::IsBattery}, cfg::BATTERIES_PATH, logger::log_error_and_panic, units::{duration::Duration, energy::Energy, power::Power, voltage::Voltage}, utilities::utils::linear_interpolate};

/// Represents a rechargeable battery with energy capacity, voltage, and seasonal data models.
#[derive(Clone, Debug, PartialEq)]
pub struct Battery {
    pub voltage: Voltage,
    pub capacity: Energy,
    pub soc: f32,
    pub energy: Energy,
    pub jan_max_data: Vec<(u32, f32)>,
    pub jan_min_data: Vec<(u32, f32)>,
    pub jun_max_data: Vec<(u32, f32)>,
    pub start_index: HashMap<String, usize>,

    update_count: u32,
    pub soc_history: VecDeque<f32>,
}

impl IsBattery for Battery {
    /// Reduces battery energy based on power draw and elapsed time.
    fn discharge(&mut self, power: Power, duration: Duration) {
        if self.energy <= Energy::ZERO { return } // is empty
        let energy_removed = power * duration;
        let new_energy = self.energy - energy_removed;
        if new_energy < Energy::ZERO { self.energy = Energy::ZERO; }
        else { self.energy = new_energy; }
        self.soc = (self.energy / self.capacity) * 100.0;  // Update SoC

        self.update();
    }
    
    /// Increases battery energy using solar charge interpolation curves.
    fn charge(&mut self, duration: Duration, month: u32) {
        if self.energy >= self.capacity {
            return; // Battery is full
        }

        match self.get_morph_x_y(self.energy.to_watt_hour(), month, duration.to_base_unit() as u32) {
            Ok((_, new_energy)) => {
                let new_energy = Energy::watt_hours(new_energy);
                if new_energy < self.capacity { self.energy = new_energy; }
                else { self.energy = self.capacity; }
                self.soc = (self.energy / self.capacity) * 100.0;  // Update SoC
    
                self.update();
            }
            Err(e) => {
                eprintln!("⚠️ Failed to charge: {e}");
            }
        }
    }
    
    /// Returns the current battery state of charge.
    fn get_soc(&self) -> f32 {
        (self.energy / self.capacity) * 100.0
    }
    
    /// Updates the current energy based on state of charge.
    fn recalculate_energy(&mut self) {
        self.energy = self.capacity * self.soc * 0.01;
    }
}

impl Battery {
    /// Creates a battery from configuration and an initial SoC.
    pub fn from_config(config: BatteryConfig, initial_soc: f32) -> Self {
        let soc = initial_soc.clamp(0.0, 100.0);
        let path = format!("{}/{}/", BATTERIES_PATH, config.name);
        Self {
            voltage: config.voltage,
            capacity: config.capacity,
            soc,
            energy: (soc / 100.0) * config.capacity,
            jan_max_data: Self::get_month_data_points(format!("{}{}", path, config.jan_max)),
            jan_min_data: Self::get_month_data_points(format!("{}{}", path, config.jan_min)),
            jun_max_data: Self::get_month_data_points(format!("{}{}", path, config.jun_max)),
            start_index: [("jan".to_string(), 1), ("jun".to_string(), 1)]
                .iter().cloned().collect(),
            
            update_count: 0,
            soc_history: VecDeque::from(vec![soc; 100]),
        }
    }
    
    /// Parses charging data points from a whitespace-delimited file.
    fn get_month_data_points<P: AsRef<Path>>(file_path: P) -> Vec<(u32, f32)> {
        let path_ref = file_path.as_ref();
        let file = File::open(path_ref).unwrap_or_else(|e| {
            let msg = format!("Failed to open file {path_ref:?}: {e}");
            log_error_and_panic(&msg)
        });
        let reader = BufReader::new(file);
        let mut points = Vec::new();

        for line in reader.lines().skip(1).map_while(Result::ok) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if let (Some(t), Some(v)) = (parts.first(), parts.last()) {
                if let (Ok(t), Ok(v)) = (t.parse::<f32>(), v.parse::<f32>()) {
                    // Time (s) - Energy (Wh)
                    points.push(((t * 3600.0) as u32, v));
                }
            }
        }
        points
        
    }
    
    /// Periodically stores the latest SoC in the history.
    fn update(&mut self) {
        self.update_count += 1;
        if self.update_count % 300 == 0 {
            self.soc_history.pop_front();
            self.soc_history.push_back(self.soc);
        }
    }
    
    /// Finds interpolated energy output for a given input time in the month’s dataset.
    fn find_y_for_x_month(&mut self, month: &str, x: u32) -> Result<f32, BatteryError> {
        let data = match month {
            "jan" => &self.jan_min_data,
            "jun" => &self.jun_max_data,
            _ => return Err(BatteryError::UnsupportedMonth(month.to_string())),
        };

        let start = *self.start_index.get(month).unwrap_or(&1);
        for i in start..data.len() {
            let (x0, y0) = data[i - 1];
            let (x1, y1) = data[i];
            if x0 <= x && x <= x1 {
                self.start_index.insert(month.to_string(), std::cmp::max(1, i - 1));
                return Ok(linear_interpolate(x0 as f32, y0, x1 as f32, y1, x as f32));
            }
        }
        Err(BatteryError::NoYForX(x.to_string()))
    }
    
    /// Finds interpolated time needed to reach a given energy in the month’s dataset.
    fn find_x_for_y_month(&mut self, month: &str, y: f32) -> Result<u32, BatteryError> {
        let data = match month {
            "jan" => &self.jan_min_data,
            "jun" => &self.jun_max_data,
            _ => return  Err(BatteryError::UnsupportedMonth(month.to_string())),
        };

        let start = *self.start_index.get(month).unwrap_or(&1);
        for i in start..data.len() {
            let (x0, y0) = data[i - 1];
            let (x1, y1) = data[i];
            if y0 <= y && y <= y1 {
                self.start_index.insert(month.to_string(), std::cmp::max(1, i - 1));
                return Ok(linear_interpolate(y0, x0 as f32, y1, x1 as f32, y) as u32);
            }
        }
        Err(BatteryError::NoXForY(y.to_string()))
    }
    
    /// Calculates morphing time and energy for seasonal solar models.
    pub fn get_morph_x_y(&mut self, y: f32, month: u32, time: u32) -> Result<(u32, f32), BatteryError> {
        let jan_time = self.find_x_for_y_month("jan", y)?;
        let jun_time = self.find_x_for_y_month("jun", y)?;

        let jan_next_time = jan_time + time;
        let jun_next_time = jun_time + time;

        let jan_new_wh = self.find_y_for_x_month("jan", jan_next_time)?;
        let jun_new_wh = self.find_y_for_x_month("jun", jun_next_time)?;

        let month = month as f32;
        let weight1 = (1.0 + (std::f32::consts::PI * (month - 1.0) / 6.0).cos()) / 2.0;
        let weight2 = 1.0 - weight1;

        let new_time = weight1 * jan_next_time as f32 + weight2 * jun_next_time as f32;
        let new_wh = weight1 * jan_new_wh + weight2 * jun_new_wh;
        Ok((new_time as u32, new_wh))
    }
}