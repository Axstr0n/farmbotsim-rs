use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, Copy)]
pub enum PowerUnit {
    Watts,
    Kilowatts,
}

impl PowerUnit {
    fn from_str(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "w" | "watt" | "watts" => Some(PowerUnit::Watts),
            "kw" | "kilowatt" | "kilowatts" => Some(PowerUnit::Kilowatts),
            _ => None,
        }
    }

    fn as_str(&self) -> &'static str {
        match self {
            PowerUnit::Watts => "W",
            PowerUnit::Kilowatts => "kW",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Power {
    pub value: f32,
    pub unit: PowerUnit,
}

impl Default for Power {
    fn default() -> Self {
        Power::watts(0.0)
    }
}

impl Power {
    /// Convert the power into total watts.
    fn to_watts(&self) -> f32 {
        match self.unit {
            PowerUnit::Watts => self.value,
            PowerUnit::Kilowatts => self.value * 1_000.0,
        }
    }

    /// Convenience constructors
    fn watts(val: f32) -> Self {
        Power { value: val, unit: PowerUnit::Watts }
    }

    fn kilowatts(val: f32) -> Self {
        Power { value: val, unit: PowerUnit::Kilowatts }
    }

}

// Serialize as "100 W"
impl Serialize for Power {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{} {}", self.value, self.unit.as_str()))
    }
}

// Deserialize from "100 W"
impl<'de> Deserialize<'de> for Power {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.len() != 2 {
            return Err(serde::de::Error::custom("Invalid power format"));
        }

        let value = parts[0].parse::<f32>().map_err(serde::de::Error::custom)?;
        let unit = PowerUnit::from_str(parts[1]).ok_or_else(|| {
            serde::de::Error::custom(format!("Unknown power unit: {}", parts[1]))
        })?;

        Ok(Power { value, unit })
    }
}