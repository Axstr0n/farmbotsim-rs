/// Represents errors that can occur in battery processing.
#[derive(Debug)]
pub enum BatteryError {
    UnsupportedMonth(String),
    NoXForY(String),
    NoYForX(String),
}

impl std::fmt::Display for BatteryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BatteryError::UnsupportedMonth(month) => {
                write!(f, "Unsupported month: {month}")
            }
            BatteryError::NoXForY(y) => {
                write!(f, "No x found for y: {y}")
            }
            BatteryError::NoYForX(x) => {
                write!(f, "No y found for x: {x}")
            }
        }
    }
}
impl std::error::Error for BatteryError {}