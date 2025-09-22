use std::str::FromStr;

use egui::Vec2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AngleUnit {
    Degrees,
    Radians,
}

impl AngleUnit {
    pub fn as_str(&self) -> &'static str {
        match self {
            AngleUnit::Degrees => "deg",
            AngleUnit::Radians => "rad",
        }
    }
}
impl FromStr for AngleUnit {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "deg" | "degree" | "degrees" => Ok(AngleUnit::Degrees),
            "rad" | "radian" | "radians" => Ok(AngleUnit::Radians),
            _ => Err(format!("Unknown AngleUnit: {s}")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Angle {
    pub value: f32,
    pub unit: AngleUnit,
}

impl Angle {
    pub const ZERO: Angle = Angle::radians(0.0);

    pub const fn new(value: f32, unit: AngleUnit) -> Self {
        Self { value, unit }
    }
    pub const fn degrees(value: f32) -> Self {
        Self::new(value, AngleUnit::Degrees)
    }
    pub const fn radians(value: f32) -> Self {
        Self::new(value, AngleUnit::Radians)
    }

    pub fn to_radians(self) -> f32 {
        match self.unit {
            AngleUnit::Degrees => self.value.to_radians(),
            AngleUnit::Radians => self.value,
        }
    }
    pub fn to_degrees(self) -> f32 {
        match self.unit {
            AngleUnit::Degrees => self.value,
            AngleUnit::Radians => self.value.to_degrees(),
        }
    }

    pub fn to_base_unit(self) -> f32 {
        match self.unit {
            AngleUnit::Degrees => self.value.to_radians(),
            AngleUnit::Radians => self.value,
        }
    }
    pub fn from_base_unit(value_in_radians: f32, unit: AngleUnit) -> Self {
        match unit {
            AngleUnit::Degrees => Self::degrees(value_in_radians.to_degrees()),
            AngleUnit::Radians => Self::radians(value_in_radians),
        }
    }

    pub fn to_vec2(&self) -> Vec2 {
        let radians = match self.unit {
            AngleUnit::Degrees => self.value.to_radians(),
            AngleUnit::Radians => self.value,
        };
        Vec2::new(radians.cos(), radians.sin())
    }
    pub fn is_close_to(&self, other: Angle, tolerance: Angle) -> bool {
        let diff_deg = (self.to_degrees() - other.to_degrees() + 180.0).rem_euclid(360.0) - 180.0;
        diff_deg.abs() <= tolerance.to_degrees()
    }
}