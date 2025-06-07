use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    collections::{HashMap, VecDeque},
};

use crate::{
    cfg::BATTERIES_PATH, logger::log_error_and_panic, units::{
        duration::Duration,
        energy::Energy,
        power::Power,
        voltage::Voltage
    }, utilities::utils::load_json_or_panic
};

pub trait Battery {
    fn discharge(&mut self, power: Power, duration: Duration);
    fn charge(&mut self, duration: Duration, month: u32);
    fn get_soc(&self) -> f32;

    fn recalculate_energy(&mut self);
}

#[derive(Clone, Debug, PartialEq)]
pub struct BatteryPack {
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

impl BatteryPack {
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
    fn get_month_data_points<P: AsRef<Path>>(file_path: P) -> Vec<(u32, f32)> {
        let path_ref = file_path.as_ref();
        let file = File::open(path_ref).unwrap_or_else(|e| {
            let msg = format!("Failed to open file {:?}: {}", path_ref, e);
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

    fn update(&mut self) {
        self.update_count += 1;
        if self.update_count % 300 == 0 {
            self.soc_history.pop_front();
            self.soc_history.push_back(self.soc);
        }
    }

    fn linear_interpolate(&self, x0: f32, y0: f32, x1: f32, y1: f32, x: f32) -> f32 {
        if x1 == x0 {
            return y0;
        }
        y0 + (x - x0) * (y1 - y0) / (x1 - x0)
    }

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
                return Ok(self.linear_interpolate(x0 as f32, y0, x1 as f32, y1, x as f32));
            }
        }
        Err(BatteryError::NoYForX(x.to_string()))
    }

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
                return Ok(self.linear_interpolate(y0, x0 as f32, y1, x1 as f32, y) as u32);
            }
        }
        Err(BatteryError::NoXForY(y.to_string()))
    }

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

impl Battery for BatteryPack {
    fn discharge(&mut self, power: Power, duration: Duration) {
        if self.energy <= Energy::joules(0.0) { return } // is empty
        let energy_removed = power * duration;
        let new_energy = self.energy - energy_removed;
        if new_energy < Energy::joules(0.0) { self.energy = Energy::joules(0.0); }
        else { self.energy = new_energy; }
        self.soc = (self.energy / self.capacity) * 100.0;  // Update SoC

        self.update();
    }
    
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
                eprintln!("⚠️ Failed to charge: {}", e);
            }
        }
    }

    fn get_soc(&self) -> f32 {
        (self.energy / self.capacity) * 100.0
    }

    fn recalculate_energy(&mut self) {
        self.energy = self.capacity * self.soc * 0.01;
    }
}


#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct BatteryConfig {
    pub name: String,
    pub capacity: Energy,
    pub voltage: Voltage,
    pub jan_max: String,
    pub jan_min: String,
    pub jun_max: String,
}
impl BatteryConfig {
    pub fn from_json_file(folder_name: String) -> Self {
        let path_str = format!("{}/config.json", folder_name);
        load_json_or_panic(path_str)
    }
}


#[derive(Debug)]
pub enum BatteryError {
    UnsupportedMonth(String),
    NoXForY(String),
    NoYForX(String),
}

impl std::fmt::Display for BatteryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BatteryError::UnsupportedMonth(month) => {
                write!(f, "Unsupported month: {}", month)
            }
            BatteryError::NoXForY(y) => {
                write!(f, "No x found for y: {}", y)
            }
            BatteryError::NoYForX(x) => {
                write!(f, "No y found for x: {}", x)
            }
        }
    }
}
impl std::error::Error for BatteryError {}
