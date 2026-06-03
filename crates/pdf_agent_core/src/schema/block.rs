use crate::schema::bbox::BBox;
use crate::schema::block_type::BlockType;
use crate::schema::line::Line;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Block {
    pub id: String,
    pub bbox: BBox,
    pub block_type: BlockType,
    pub text: String,
    pub lines: Vec<Line>,
}

impl Block {
    pub fn new(id: String, bbox: BBox, block_type: BlockType, text: String, lines: Vec<Line>) -> Self {
        Self {
            id,
            bbox,
            block_type,
            text,
            lines,
        }
    }
}
