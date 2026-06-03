use crate::schema::block::Block;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Page {
    pub index: usize,
    pub width: f64,
    pub height: f64,
    pub blocks: Vec<Block>,
}

impl Page {
    pub fn new(index: usize, width: f64, height: f64, blocks: Vec<Block>) -> Self {
        Self {
            index,
            width,
            height,
            blocks,
        }
    }
}
