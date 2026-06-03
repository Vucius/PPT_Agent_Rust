use crate::context::pipeline_context::PipelineContext;
use crate::error::Result;
use crate::processors::traits::DocumentProcessor;
use crate::schema::block::Block;
use crate::schema::block_type::BlockType;
use crate::schema::document::Document;

pub struct LineMergeProcessor;

impl DocumentProcessor for LineMergeProcessor {
    fn name(&self) -> &'static str {
        "LineMergeProcessor"
    }

    fn process(&self, document: &mut Document, _ctx: &PipelineContext) -> Result<()> {
        for page in &mut document.pages {
            if page.blocks.is_empty() {
                continue;
            }

            let mut merged_blocks = Vec::new();
            let mut current_block: Option<Block> = None;

            for block in std::mem::take(&mut page.blocks) {
                if let Some(mut curr) = current_block {
                    let y_gap = (curr.bbox.y0 - block.bbox.y1).abs();
                    let overlap_x = curr.bbox.x0 < block.bbox.x1 && curr.bbox.x1 > block.bbox.x0;

                    if curr.block_type == block.block_type
                        && (curr.block_type == BlockType::Text || curr.block_type == BlockType::Table)
                        && y_gap < 18.0
                        && overlap_x
                    {
                        if curr.block_type == BlockType::Table {
                            let lines_count = curr.text.lines().count();
                            if lines_count == 1 {
                                let col_count = curr.text.matches('|').count().saturating_sub(1);
                                let mut separator = String::from("|");
                                for _ in 0..col_count {
                                    separator.push_str(" --- |");
                                }
                                curr.text = format!("{}\n{}\n{}", curr.text.trim(), separator, block.text.trim());
                            } else {
                                curr.text = format!("{}\n{}", curr.text.trim(), block.text.trim());
                            }
                        } else {
                            curr.text = format!("{} {}", curr.text.trim(), block.text.trim());
                        }
                        curr.bbox = curr.bbox.merge(&block.bbox);
                        curr.lines.extend(block.lines);
                        current_block = Some(curr);
                    } else {
                        merged_blocks.push(curr);
                        current_block = Some(block);
                    }
                } else {
                    current_block = Some(block);
                }
            }

            if let Some(curr) = current_block {
                merged_blocks.push(curr);
            }

            page.blocks = merged_blocks;
        }

        Ok(())
    }
}
