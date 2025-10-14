use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EnergyUnit {
    Joules,
    Kilojoules,
    WattHours,
    KilowattHours,
}

impl EnergyUnit {
    pub fn as_str(&self) -> &'static str {
        match self {
            EnergyUnit::Joules => "J",
            EnergyUnit::Kilojoules => "kJ",
            EnergyUnit::WattHours => "Wh",
            EnergyUnit::KilowattHours => "kWh",
        }
    }
}
impl FromStr for EnergyUnit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "j" | "joule" | "joules" => Ok(EnergyUnit::Joules),
            "kj" | "kilojoule" | "kilojoules" => Ok(EnergyUnit::Kilojoules),
            "wh" | "watthour" | "watthours" => Ok(EnergyUnit::WattHours),
            "kwh" | "kilowatthour" | "kilowatthours" => Ok(EnergyUnit::KilowattHours),
            _ => Err(format!("Unknown EnergyUnit: {s}")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Energy {
    pub value: f32,
    pub unit: EnergyUnit,
}

impl Energy {
    pub const ZERO: Energy = Energy::joules(0.0);

    pub const fn new(value: f32, unit: EnergyUnit) -> Self {
        Self { value, unit }
    }
    pub const fn joules(value: f32) -> Self {
        Self {
            value,
            unit: EnergyUnit::Joules,
        }
    }
    pub const fn kilojoules(value: f32) -> Self {
        Self {
            value,
            unit: EnergyUnit::Kilojoules,
        }
    }
    pub const fn watt_hours(value: f32) -> Self {
        Self {
            value,
            unit: EnergyUnit::WattHours,
        }
    }
    pub const fn kilowatt_hours(value: f32) -> Self {
        Self {
            value,
            unit: EnergyUnit::KilowattHours,
        }
    }

    pub fn to_watt_hour(self) -> f32 {
        match self.unit {
            EnergyUnit::Joules => self.value / 3600.0,
            EnergyUnit::Kilojoules => (self.value * 1000.0) / 3600.0,
            EnergyUnit::WattHours => self.value,
            EnergyUnit::KilowattHours => self.value * 1000.0,
        }
    }

    pub fn to_base_unit(self) -> f32 {
        match self.unit {
            EnergyUnit::Joules => self.value,
            EnergyUnit::Kilojoules => self.value * 1000.0,
            EnergyUnit::WattHours => self.value * 3600.0,
            EnergyUnit::KilowattHours => self.value * 3_600_000.0,
        }
    }
    pub fn from_base_unit(value: f32, unit: EnergyUnit) -> Self {
        match unit {
            EnergyUnit::Joules => Self::joules(value),
            EnergyUnit::Kilojoules => Self::kilojoules(value / 1000.0),
            EnergyUnit::WattHours => Self::watt_hours(value / 3600.0),
            EnergyUnit::KilowattHours => Self::kilowatt_hours(value / 3_600_000.0),
        }
    }
}

impl FromStr for Energy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if let Some((num, unit)) = s.rsplit_once(char::is_whitespace) {
            let value: f32 = num.trim().parse().map_err(|_| "Invalid number")?;
            match unit.trim().to_ascii_lowercase().as_str() {
                "wh" => Ok(Energy {
                    value,
                    unit: EnergyUnit::WattHours,
                }),
                "kwh" => Ok(Energy {
                    value,
                    unit: EnergyUnit::KilowattHours,
                }),
                _ => Err(format!("Unknown energy unit: {unit}")),
            }
        } else {
            Err("Invalid format for Energy".to_string())
        }
    }
}
