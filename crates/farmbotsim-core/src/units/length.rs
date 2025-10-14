use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, enum_iterator::Sequence)]
pub enum LengthUnit {
    Millimeters,
    Centimeters,
    Meters,
}

impl LengthUnit {
    pub fn as_str(&self) -> &'static str {
        match self {
            LengthUnit::Millimeters => "mm",
            LengthUnit::Centimeters => "cm",
            LengthUnit::Meters => "m",
        }
    }
}
impl FromStr for LengthUnit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "mm" | "millimeter" | "millimeters" => Ok(LengthUnit::Millimeters),
            "cm" | "centimeter" | "centimeters" => Ok(LengthUnit::Centimeters),
            "m" | "meter" | "meters" => Ok(LengthUnit::Meters),
            _ => Err(format!("Unknown LengthUnit: {s}")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Length {
    pub value: f32,
    pub unit: LengthUnit,
}

impl Length {
    pub const ZERO: Length = Length::meters(0.0);

    pub const fn new(value: f32, unit: LengthUnit) -> Self {
        Self { value, unit }
    }
    pub const fn millimeters(value: f32) -> Self {
        Self::new(value, LengthUnit::Millimeters)
    }
    pub const fn centimeters(value: f32) -> Self {
        Self::new(value, LengthUnit::Centimeters)
    }
    pub const fn meters(value: f32) -> Self {
        Self::new(value, LengthUnit::Meters)
    }

    pub fn to_base_unit(self) -> f32 {
        match self.unit {
            LengthUnit::Millimeters => self.value / 1000.0,
            LengthUnit::Centimeters => self.value / 100.0,
            LengthUnit::Meters => self.value,
        }
    }
    pub fn from_base_unit(value: f32, unit: LengthUnit) -> Self {
        match unit {
            LengthUnit::Millimeters => Self::new(value * 1000.0, unit),
            LengthUnit::Centimeters => Self::new(value * 100.0, unit),
            LengthUnit::Meters => Self::new(value, unit),
        }
    }
}
