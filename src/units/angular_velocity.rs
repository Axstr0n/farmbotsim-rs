use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, enum_iterator::Sequence)]
pub enum AngularVelocityUnit {
    RadiansPerSecond,
    DegreesPerSecond,
}

impl AngularVelocityUnit {
    pub fn as_str(&self) -> &'static str {
        match self {
            AngularVelocityUnit::RadiansPerSecond => "rad/s",
            AngularVelocityUnit::DegreesPerSecond => "deg/s",
        }
    }
}
impl FromStr for AngularVelocityUnit {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "rad/s" | "rads" | "radians_per_second" => Ok(AngularVelocityUnit::RadiansPerSecond),
            "deg/s" | "degs" | "degrees_per_second" => Ok(AngularVelocityUnit::DegreesPerSecond),
            _ => Err(format!("Unknown AngularVelocityUnit: {s}")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AngularVelocity {
    pub value: f32,
    pub unit: AngularVelocityUnit,
}

impl AngularVelocity {
    pub const ZERO: AngularVelocity = AngularVelocity::radians_per_second(0.0);

    pub const fn new(value: f32, unit: AngularVelocityUnit) -> Self {
        Self { value, unit }
    }
    pub const fn radians_per_second(value: f32) -> Self {
        Self::new(value, AngularVelocityUnit::RadiansPerSecond)
    }
    pub const fn degrees_per_second(value: f32) -> Self {
        Self::new(value, AngularVelocityUnit::DegreesPerSecond)
    }

    pub fn to_base_unit(self) -> f32 {
        use std::f32::consts::PI;
        match self.unit {
            AngularVelocityUnit::RadiansPerSecond => self.value,
            AngularVelocityUnit::DegreesPerSecond => self.value * PI / 180.0,
        }
    }
    pub fn from_base_unit(value: f32, unit: AngularVelocityUnit) -> Self {
        use std::f32::consts::PI;
        match unit {
            AngularVelocityUnit::RadiansPerSecond => Self::new(value, unit),
            AngularVelocityUnit::DegreesPerSecond => Self::new(value * 180.0 / PI, unit),
        }
    }
}