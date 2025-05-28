use std::ops::{Add, Div, Mul, Neg, Sub};
use egui::Vec2;

use super::{
    angle::Angle,
    angular_velocity::AngularVelocity,
    duration::Duration,
    energy::{Energy, EnergyUnit},
    length::Length,
    linear_velocity::LinearVelocity,
    power::Power,
    voltage::Voltage
};

macro_rules! impl_same_type_ops {
    ($type:ty) => {
        impl Add for $type {
            type Output = Self;
            fn add(self, other: Self) -> Self {
                let result_base = self.to_base_unit() + other.to_base_unit();
                Self::from_base_unit(result_base, self.unit)
            }
        }

        impl Sub for $type {
            type Output = Self;
            fn sub(self, other: Self) -> Self {
                let result_base = self.to_base_unit() - other.to_base_unit();
                Self::from_base_unit(result_base, self.unit)
            }
        }

        impl Mul<f32> for $type {
            type Output = Self;
            fn mul(self, scalar: f32) -> Self {
                Self::new(self.value * scalar, self.unit)
            }
        }

        impl Mul<$type> for f32 {
            type Output = $type;
            fn mul(self, measurement: $type) -> $type {
                measurement * self
            }
        }

        impl Div<f32> for $type {
            type Output = Self;
            fn div(self, scalar: f32) -> Self {
                Self::new(self.value / scalar, self.unit)
            }
        }

        impl Div for $type {
            type Output = f32;
            fn div(self, other: Self) -> f32 {
                self.to_base_unit() / other.to_base_unit()
            }
        }
    };
}

impl_same_type_ops!(Angle);
impl_same_type_ops!(AngularVelocity);
impl_same_type_ops!(Duration);
impl_same_type_ops!(Energy);
impl_same_type_ops!(Length);
impl_same_type_ops!(LinearVelocity);
impl_same_type_ops!(Power);
impl_same_type_ops!(Voltage);


// ===================== CROSS-TYPE OPERATIONS =====================


// Length / Duration = LinearVelocity (m/s)
impl Div<Duration> for Length {
    type Output = LinearVelocity;
    fn div(self, duration: Duration) -> LinearVelocity {
        let length_m = self.to_base_unit();
        let duration_s = duration.to_base_unit();
        LinearVelocity::meters_per_second(length_m / duration_s)
    }
}

// LinearVelocity * Duration = Length (m)
impl Mul<Duration> for LinearVelocity {
    type Output = Length;
    fn mul(self, duration: Duration) -> Length {
        let velocity_ms = self.to_base_unit();
        let duration_s = duration.to_base_unit();
        Length::meters(velocity_ms * duration_s)
    }
}

impl Mul<LinearVelocity> for Duration {
    type Output = Length;
    fn mul(self, velocity: LinearVelocity) -> Length {
        velocity * self
    }
}

// AngularVelocity * Length = LinearVelocity (for radius)
impl Mul<Length> for AngularVelocity {
    type Output = LinearVelocity;
    fn mul(self, radius: Length) -> LinearVelocity {
        let angular_rads = self.to_base_unit();
        let radius_m = radius.to_base_unit();
        LinearVelocity::meters_per_second(angular_rads * radius_m)
    }
}

impl Mul<AngularVelocity> for Length {
    type Output = LinearVelocity;
    fn mul(self, angular_vel: AngularVelocity) -> LinearVelocity {
        angular_vel * self
    }
}

// LinearVelocity / Length = AngularVelocity (for radius)
impl Div<Length> for LinearVelocity {
    type Output = AngularVelocity;
    fn div(self, radius: Length) -> AngularVelocity {
        let velocity_ms = self.to_base_unit();
        let radius_m = radius.to_base_unit();
        AngularVelocity::radians_per_second(velocity_ms / radius_m)
    }
}

// Power * Duration = Energy (J)
impl Mul<Duration> for Power {
    type Output = Energy;
    fn mul(self, duration: Duration) -> Energy {
        let power = self.to_base_unit();
        let duration = duration.to_base_unit();
        Energy { 
            value: power * duration,
            unit: EnergyUnit::Joules
        }
    }
}

impl Mul<Power> for Duration {
    type Output = Energy;
    fn mul(self, power: Power) -> Energy {
        power * self
    }
}

// Duration * AngularVelocity = Angle (rad)
impl Mul<Duration> for AngularVelocity {
    type Output = Angle;

    fn mul(self, rhs: Duration) -> Angle {
        Angle::radians(self.value * rhs.to_base_unit())
    }
}

// Length * Vec2 = Vec2
impl Mul<Length> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: Length) -> Vec2 {
        self * rhs.to_base_unit()
    }
}


// ===================== OTHER OPERATIONS =====================

impl Neg for Length {
    type Output = Length;

    fn neg(self) -> Length {
        Length {
            value: -self.value,
            unit: self.unit,
        }
    }
}
