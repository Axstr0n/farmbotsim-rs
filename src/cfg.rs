pub const TOLERANCE_DISTANCE: f32 = 0.005;
pub const TOLERANCE_ANGLE: f32 = 0.1;

pub const POWER_CONSUMPTION_WAIT: f32 = 10.0; // W/s
pub const POWER_CONSUMPTION_TRAVEL: f32 = 2.0*350.0; // W/s

pub const MAX_VELOCITY: f32 = 10.0 * 0.277_777_8; // m/s
pub const MAX_VELOCITY_BETWEEN_POINTS: f32 = 3.0 * 0.277_777_8; // m/s // between crops


pub const ENV_CONFIGS_PATH: &str = "configs/env_configs/";
pub const DEFAULT_ENV_CONFIG_PATH: &str = "configs/env_configs/default.json";

pub const CROP_PLANS_PATH: &str = "configs/crop_plans/";
pub const DEFAULT_POINT_CROP_PLAN_PATH: &str = "configs/crop_plans/default_point.json";
pub const DEFAULT_LINE_CROP_PLAN_PATH: &str = "configs/crop_plans/default_line.json";