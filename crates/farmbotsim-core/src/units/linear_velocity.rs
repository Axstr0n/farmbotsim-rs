use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, enum_iterator::Sequence)]
pub enum LinearVelocityUnit {
    MetersPerSecond,
    KilometersPerHour,
}

impl LinearVelocityUnit {
    pub fn as_str(&self) -> &'static str {
        match self {
            LinearVelocityUnit::MetersPerSecond => "m/s",
            LinearVelocityUnit::KilometersPerHour => "km/h",
        }
    }
}
impl FromStr for LinearVelocityUnit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "m/s" | "ms" | "mps" | "meters_per_second" => Ok(LinearVelocityUnit::MetersPerSecond),
            "km/h" | "kmh" | "kph" | "kilometers_per_hour" => {
                Ok(LinearVelocityUnit::KilometersPerHour)
            }
            _ => Err(format!("Unknown LinearVelocityUnit: {s}")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LinearVelocity {
    pub value: f32,
    pub unit: LinearVelocityUnit,
}

impl LinearVelocity {
    pub const ZERO: LinearVelocity = LinearVelocity::meters_per_second(0.0);

    pub const fn new(value: f32, unit: LinearVelocityUnit) -> Self {
        Self { value, unit }
    }
    pub const fn meters_per_second(value: f32) -> Self {
        Self::new(value, LinearVelocityUnit::MetersPerSecond)
    }
    pub const fn kilometers_per_hour(value: f32) -> Self {
        Self::new(value, LinearVelocityUnit::KilometersPerHour)
    }

    pub fn to_base_unit(self) -> f32 {
        match self.unit {
            LinearVelocityUnit::MetersPerSecond => self.value,
            LinearVelocityUnit::KilometersPerHour => self.value / 3.6,
        }
    }
    pub fn from_base_unit(value: f32, unit: LinearVelocityUnit) -> Self {
        match unit {
            LinearVelocityUnit::MetersPerSecond => Self::new(value, unit),
            LinearVelocityUnit::KilometersPerHour => Self::new(value * 3.6, unit),
        }
    }
}
