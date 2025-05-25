use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::collections::{HashMap, VecDeque};

pub trait Battery {
    fn discharge(&mut self, power: f32, time: u32);
    fn charge(&mut self, time: u32, month: u32);
    fn get_soc(&self) -> f32;

    fn recalculate_energy(&mut self);
}

#[derive(Clone, Debug, PartialEq)]
pub struct BatteryPack {
    pub voltage: f32,
    pub capacity_wh: f32,
    pub soc: f32,
    pub energy_wh: f32,
    pub jan_max_data: Vec<(u32, f32)>,
    pub jan_min_data: Vec<(u32, f32)>,
    pub jun_max_data: Vec<(u32, f32)>,
    pub start_index: HashMap<String, usize>,

    update_count: u32,
    pub soc_history: VecDeque<f32>,
}

impl BatteryPack {
    pub fn from_config(folder_path: String, state_of_charge: Option<f32>) -> Self {
        let loader = BatteryLoader::new(folder_path);
        let soc = state_of_charge.unwrap_or(100.0);
        Self {
            voltage: loader.voltage,
            capacity_wh: loader.capacity_wh,
            soc,
            energy_wh: (soc / 100.0) * loader.capacity_wh,
            jan_max_data: loader.jan_max_data,
            jan_min_data: loader.jan_min_data,
            jun_max_data: loader.jun_max_data,
            start_index: [("jan".to_string(), 1), ("jun".to_string(), 1)]
                .iter().cloned().collect(),
            
            update_count: 0,
            soc_history: VecDeque::from(vec![soc; 100]),
        }
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
    fn discharge(&mut self, power: f32, time: u32) {
        if self.energy_wh <= 0.0 { return } // is empty
        let energy_removed_wh = (power * time as f32) / 3600.0;  // Convert W to Wh
        self.energy_wh = 0_f32.max(self.energy_wh - energy_removed_wh);
        self.soc = (self.energy_wh / self.capacity_wh) * 100.0;  // Update SoC

        self.update();
    }
    
    fn charge(&mut self, time: u32, month: u32) {
        if self.energy_wh >= self.capacity_wh {
            return; // Battery is full
        }

        match self.get_morph_x_y(self.energy_wh, month, time) {
            Ok((_, new_wh)) => {
                self.energy_wh = self.capacity_wh.min(new_wh);
                self.soc = (self.energy_wh / self.capacity_wh) * 100.0;  // Update SoC
    
                self.update();
            }
            Err(e) => {
                eprintln!("⚠️ Failed to charge: {}", e);
            }
        }
    }

    fn get_soc(&self) -> f32 {
        (self.energy_wh / self.capacity_wh) * 100.0
    }

    fn recalculate_energy(&mut self) {
        self.energy_wh = self.capacity_wh * self.soc * 0.01;
    }
}

pub struct BatteryLoader {
    pub folder_path: String,
    pub capacity_wh: f32,
    pub voltage: f32,
    pub jan_max_data: Vec<(u32, f32)>,
    pub jan_min_data: Vec<(u32, f32)>,
    pub jun_max_data: Vec<(u32, f32)>,
}

impl BatteryLoader {
    pub fn new(folder_name: String) -> Self {
        let mut loader = BatteryLoader {
            folder_path: format!("batteries/{}", folder_name),
            capacity_wh: 0.0,
            voltage: 0.0,
            jan_max_data: Vec::new(),
            jan_min_data: Vec::new(),
            jun_max_data: Vec::new(),
        };
        loader.initialize_battery_params();
        loader
    }

    fn initialize_battery_params(&mut self) {
        let config_path = format!("{}/config.txt", self.folder_path);
        let file = File::open(&config_path).unwrap_or_else(|_| panic!("Failed to open {}", config_path));
        let reader = BufReader::new(file);

        for line in reader.lines().map_while(Result::ok) {
            let parts: Vec<&str> = line.split(": ").collect();
            if parts.len() != 2 {
                continue;
            }
            let key = parts[0];
            let value = parts[1].trim();

            match key {
                "capacity_wh" => self.capacity_wh = value.parse().unwrap_or(0.0),
                "voltage" => self.voltage = value.parse().unwrap_or(0.0),
                "jan_max" => self.jan_max_data = Self::get_month_data_points(format!("{}/{}", self.folder_path, value)),
                "jan_min" => self.jan_min_data = Self::get_month_data_points(format!("{}/{}", self.folder_path, value)),
                "jun_max" => self.jun_max_data = Self::get_month_data_points(format!("{}/{}", self.folder_path, value)),
                _ => {}
            }
        }
    }

    fn get_month_data_points<P: AsRef<Path>>(file_path: P) -> Vec<(u32, f32)> {
        let file = File::open(file_path).expect("Failed to open month data file");
        let reader = BufReader::new(file);
        let mut points = Vec::new();

        for line in reader.lines().skip(1).map_while(Result::ok) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if let (Some(t), Some(v)) = (parts.first(), parts.last()) {
                if let (Ok(t), Ok(v)) = (t.parse::<f32>(), v.parse::<f32>()) {
                    points.push(((t * 3600.0) as u32, v));
                }
            }
        }
        points
        
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
