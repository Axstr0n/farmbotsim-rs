use crate::units::{angle::Angle, length::Length, linear_velocity::LinearVelocity, power::Power};

pub const TOLERANCE_DISTANCE: Length = Length::meters(0.005);
pub const TOLERANCE_ANGLE: Angle = Angle::degrees(0.1);

pub const POWER_CONSUMPTION_WAIT: Power = Power::watts(10.0); // W/s
pub const POWER_CONSUMPTION_TRAVEL: Power = Power::watts(2.0*350.0); // W/s

pub const MAX_VELOCITY: LinearVelocity = LinearVelocity::kilometers_per_hour(10.0);
pub const MAX_VELOCITY_BETWEEN_POINTS: LinearVelocity = LinearVelocity::kilometers_per_hour(3.0); // between crops


pub const ENV_CONFIGS_PATH: &str = "configs/env_configs/";
pub const DEFAULT_ENV_CONFIG_PATH: &str = "configs/env_configs/default.json";

pub const CROP_PLANS_PATH: &str = "configs/crop_plans/";
pub const DEFAULT_POINT_CROP_PLAN_PATH: &str = "configs/crop_plans/default_point.json";
pub const DEFAULT_LINE_CROP_PLAN_PATH: &str = "configs/crop_plans/default_line.json";

pub const BATTERIES_PATH: &str = "configs/batteries/";