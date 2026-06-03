use crate::schema::page::Page;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Document {
    pub file_name: String,
    pub pages: Vec<Page>,
    pub metadata: HashMap<String, String>,
}

impl Document {
    pub fn new(file_name: String, pages: Vec<Page>) -> Self {
        Self {
            file_name,
            pages,
            metadata: HashMap::new(),
        }
    }

    pub fn find_block_with_context(&self, block_id: &str) -> Option<(Option<&crate::schema::block::Block>, &crate::schema::block::Block, Option<&crate::schema::block::Block>)> {
        for page in &self.pages {
            for (i, block) in page.blocks.iter().enumerate() {
                if block.id == block_id {
                    let prev = if i > 0 { page.blocks.get(i - 1) } else { None };
                    let next = page.blocks.get(i + 1);
                    return Some((prev, block, next));
                }
            }
        }
        None
    }

    pub fn update_block_text(&mut self, block_id: &str, new_text: &str) -> bool {
        for page in &mut self.pages {
            for block in &mut page.blocks {
                if block.id == block_id {
                    block.text = new_text.to_string();
                    return true;
                }
            }
        }
        false
    }
}
