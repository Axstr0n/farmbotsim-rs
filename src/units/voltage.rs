use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VoltageUnit {
    Volts,
    Millivolts,
}

impl VoltageUnit {
    pub fn as_str(&self) -> &'static str {
        match self {
            VoltageUnit::Volts => "V",
            VoltageUnit::Millivolts => "mV",
        }
    }
}
impl FromStr for VoltageUnit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "v" | "volt" | "volts" => Ok(VoltageUnit::Volts),
            "mv" | "millivolt" | "millivolts" => Ok(VoltageUnit::Millivolts),
            _ => Err(format!("Unknown VoltageUnit: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Voltage {
    pub value: f32,
    pub unit: VoltageUnit,
}

impl Voltage {
    pub const ZERO: Voltage = Voltage::volts(0.0);

    pub const fn new(value: f32, unit: VoltageUnit) -> Self {
        Self { value, unit }
    }
    pub const fn volts(value: f32) -> Self {
        Self { value, unit: VoltageUnit::Volts }
    }
    pub const fn millivolts(value: f32) -> Self {
        Self { value, unit: VoltageUnit::Millivolts }
    }

    pub fn to_base_unit(self) -> f32 {
        match self.unit {
            VoltageUnit::Volts => self.value,
            VoltageUnit::Millivolts => self.value / 1000.0,
        }
    }
    pub fn from_base_unit(value: f32, unit: VoltageUnit) -> Self {
        match unit {
            VoltageUnit::Volts => Self::volts(value),
            VoltageUnit::Millivolts => Self::millivolts(value * 1000.0),
        }
    }
}

impl FromStr for Voltage {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if let Some((num, unit)) = s.rsplit_once(char::is_whitespace) {
            let value: f32 = num.trim().parse().map_err(|_| "Invalid number")?;
            match unit.trim().to_ascii_lowercase().as_str() {
                "v" => Ok(Voltage::volts(value)),
                _ => Err(format!("Unknown voltage unit: {}", unit)),
            }
        } else {
            Err("Invalid format for Voltage".to_string())
        }
    }
}