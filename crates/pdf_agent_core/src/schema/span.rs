use crate::schema::bbox::BBox;
use crate::schema::char::Char;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Span {
    pub text: String,
    pub font_name: String,
    pub font_size: f64,
    pub bbox: BBox,
    pub chars: Vec<Char>,
}

impl Span {
    pub fn new(text: String, font_name: String, font_size: f64, bbox: BBox, chars: Vec<Char>) -> Self {
        Self {
            text,
            font_name,
            font_size,
            bbox,
            chars,
        }
    }
}
