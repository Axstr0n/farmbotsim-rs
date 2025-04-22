use super::{crop::Crop, row::Row};

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Field {
    pub crops: Vec<Crop>,
    pub rows: Vec<Row>,
}