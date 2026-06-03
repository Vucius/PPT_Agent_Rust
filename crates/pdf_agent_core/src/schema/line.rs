use crate::schema::bbox::BBox;
use crate::schema::span::Span;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Line {
    pub spans: Vec<Span>,
    pub bbox: BBox,
}

impl Line {
    pub fn new(spans: Vec<Span>, bbox: BBox) -> Self {
        Self { spans, bbox }
    }

    pub fn text(&self) -> String {
        self.spans
            .iter()
            .map(|s| s.text.as_str())
            .collect::<Vec<&str>>()
            .join(" ")
    }
}
