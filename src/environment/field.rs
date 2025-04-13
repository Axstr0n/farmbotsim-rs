use super::{crop::Crop, row::Row};

#[derive(Default, PartialEq, Clone)]
pub struct Field {
    pub crops: Vec<Crop>,
    pub rows: Vec<Row>,
}