use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, enum_iterator::Sequence)]
pub enum PowerUnit {
    Watts,
    Kilowatts,
}

impl PowerUnit {
    pub fn as_str(&self) -> &'static str {
        match self {
            PowerUnit::Watts => "W",
            PowerUnit::Kilowatts => "kW",
        }
    }
}
impl FromStr for PowerUnit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "w" | "watt" | "watts" => Ok(PowerUnit::Watts),
            "kw" | "kilowatt" | "kilowatts" => Ok(PowerUnit::Kilowatts),
            _ => Err(format!("Unknown PowerUnit: {s}")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Power {
    pub value: f32,
    pub unit: PowerUnit,
}

impl Power {
    pub const ZERO: Power = Power::watts(0.0);

    pub const fn new(value: f32, unit: PowerUnit) -> Self {
        Self { value, unit }
    }
    pub const fn watts(value: f32) -> Self {
        Self::new(value, PowerUnit::Watts)
    }
    pub const fn kilowatts(value: f32) -> Self {
        Self::new(value, PowerUnit::Kilowatts)
    }

    pub fn to_base_unit(self) -> f32 {
        match self.unit {
            PowerUnit::Watts => self.value,
            PowerUnit::Kilowatts => self.value * 1000.0,
        }
    }
    pub fn from_base_unit(value: f32, unit: PowerUnit) -> Self {
        match unit {
            PowerUnit::Watts => Self::new(value, unit),
            PowerUnit::Kilowatts => Self::new(value / 1000.0, unit),
        }
    }
}
