use crate::schema::bbox::BBox;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Char {
    pub value: String,
    pub bbox: BBox,
}

impl Char {
    pub fn new(value: String, bbox: BBox) -> Self {
        Self { value, bbox }
    }
}
