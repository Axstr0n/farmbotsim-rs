use std::cmp::Ordering;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use std::fmt;

use crate::units::{
    angle::{Angle, AngleUnit},
    angular_velocity::{AngularVelocity, AngularVelocityUnit},
    duration::{Duration, DurationUnit},
    energy::{Energy, EnergyUnit},
    length::{Length, LengthUnit},
    linear_velocity::{LinearVelocity, LinearVelocityUnit},
    power::{Power, PowerUnit},
    voltage::{Voltage, VoltageUnit},
};

macro_rules! impl_ordering {
    ($type:ty) => {
        impl PartialOrd for $type {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                self.to_base_unit().partial_cmp(&other.to_base_unit())
            }
        }
    };
}

impl_ordering!(Angle);
impl_ordering!(AngularVelocity);
impl_ordering!(Duration);
impl_ordering!(Energy);
impl_ordering!(Length);
impl_ordering!(LinearVelocity);
impl_ordering!(Power);
impl_ordering!(Voltage);


macro_rules! impl_ser_deser {
    ($type:ident, $unit_enum:ident) => {
        impl Serialize for $type {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_str(&format!("{:.3} {}", self.value, self.unit.as_str()))
            }
        }
        impl<'de> ::serde::Deserialize<'de> for $type {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                use std::str::FromStr;

                let s = <String as Deserialize>::deserialize(deserializer)?;
                let parts: Vec<&str> = s.split_whitespace().collect();

                if parts.len() != 2 {
                    return Err(::serde::de::Error::custom("Expected format: '<value> <unit>'"));
                }

                let value = parts[0]
                    .parse::<f32>()
                    .map_err(::serde::de::Error::custom)?;

                let unit = <$unit_enum>::from_str(parts[1])
                    .map_err(::serde::de::Error::custom)?;

                Ok($type { value, unit })
            }
        }
    };
}

impl_ser_deser!(Angle, AngleUnit);
impl_ser_deser!(AngularVelocity, AngularVelocityUnit);
impl_ser_deser!(Duration, DurationUnit);
impl_ser_deser!(Energy, EnergyUnit);
impl_ser_deser!(Length, LengthUnit);
impl_ser_deser!(LinearVelocity, LinearVelocityUnit);
impl_ser_deser!(Power, PowerUnit);
impl_ser_deser!(Voltage, VoltageUnit);


macro_rules! impl_display {
    ($type:ident, $unit_enum:ident) => {
        impl fmt::Display for $unit_enum {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.as_str())
            }
        }
        impl fmt::Display for $type {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{} {}", self.value, self.unit)
            }
        }
    };
}

impl_display!(Angle, AngleUnit);
impl_display!(AngularVelocity, AngularVelocityUnit);
impl_display!(Duration, DurationUnit);
impl_display!(Energy, EnergyUnit);
impl_display!(Length, LengthUnit);
impl_display!(LinearVelocity, LinearVelocityUnit);
impl_display!(Power, PowerUnit);
impl_display!(Voltage, VoltageUnit);