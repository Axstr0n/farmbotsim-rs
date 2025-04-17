

pub trait Battery {
    fn discharge(&mut self, power: f32, time: u32);
    fn get_soc(&self) -> f32;
}

#[derive(Clone, Debug, PartialEq)]
pub struct BatteryPack {
    voltage: f32,
    capacity_wh: f32,
    soc: f32,
    energy_wh: f32
}

impl BatteryPack {
    pub fn new(voltage: f32, capacity_wh: f32, initial_soc: f32) -> Self {
        Self {
            voltage,
            capacity_wh,
            soc: initial_soc,
            energy_wh: (initial_soc / 100.0) * capacity_wh
        }
    }
}

impl Battery for BatteryPack {
    fn discharge(&mut self, power_w: f32, time_s: u32) {
        if self.energy_wh <= 0.0 { return } // is empty
        let energy_removed_wh = (power_w * time_s as f32) / 3600.0;  // Convert W to Wh
        self.energy_wh = 0_f32.max(self.energy_wh - energy_removed_wh);
        self.soc = (self.energy_wh / self.capacity_wh) * 100.0  // Update SoC
    }
    fn get_soc(&self) -> f32 {
        (self.energy_wh / self.capacity_wh) * 100.0
    }
}
