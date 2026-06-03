use crate::context::pipeline_context::PipelineContext;
use crate::error::Result;
use crate::processors::traits::DocumentProcessor;
use crate::schema::block_type::BlockType;
use crate::schema::document::Document;

pub struct ListProcessor;

impl DocumentProcessor for ListProcessor {
    fn name(&self) -> &'static str {
        "ListProcessor"
    }

    fn process(&self, document: &mut Document, _ctx: &PipelineContext) -> Result<()> {
        for page in &mut document.pages {
            for block in &mut page.blocks {
                if block.block_type != BlockType::Text {
                    continue;
                }

                let trimmed = block.text.trim_start();
                let mut is_list_item = false;
                let mut prefix_len = 0;

                // Check common bullets
                for bullet in &["* ", "- ", "• ", "+ ", "◦ ", "▪ "] {
                    if trimmed.starts_with(bullet) {
                        is_list_item = true;
                        prefix_len = bullet.len();
                        break;
                    }
                }

                // Check numbered list patterns (e.g., "1. ", "12. ")
                if !is_list_item {
                    let chars: Vec<char> = trimmed.chars().collect();
                    if !chars.is_empty() && chars[0].is_ascii_digit() {
                        let mut i = 0;
                        while i < chars.len() && chars[i].is_ascii_digit() {
                            i += 1;
                        }
                        if i < chars.len() && chars[i] == '.' {
                            if i + 1 < chars.len() && chars[i+1].is_whitespace() {
                                is_list_item = true;
                                prefix_len = trimmed.char_indices().nth(i + 2).map(|(idx, _)| idx).unwrap_or(trimmed.len());
                            }
                        }
                    }
                }

                if is_list_item {
                    block.block_type = BlockType::ListItem;
                    let clean_text = trimmed[prefix_len..].to_string();
                    block.text = clean_text;
                }
            }
        }
        Ok(())
    }
}
