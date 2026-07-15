use crate::context::pipeline_context::PipelineContext;
use crate::error::Result;
use crate::processors::traits::DocumentProcessor;
use crate::schema::block_type::BlockType;
use crate::schema::document::Document;

pub struct BlockRelabelProcessor;

impl DocumentProcessor for BlockRelabelProcessor {
    fn name(&self) -> &'static str {
        "BlockRelabelProcessor"
    }

    fn process(&self, document: &mut Document, _ctx: &PipelineContext) -> Result<()> {
        for page in &mut document.pages {
            // We use an index based approach to allow looking at prev/next if needed.
            for i in 0..page.blocks.len() {
                let text = page.blocks[i].text.trim().to_string();
                
                // If it's already a well-defined type, we might want to skip or refine.
                // We'll primarily focus on fixing blocks currently labeled as Text.
                if page.blocks[i].block_type == BlockType::Text {
                    if text.is_empty() {
                        // Could be an empty block, maybe we don't change type but it's a hint
                        continue;
                    }

                    // Heuristic: If it looks like a caption (starts with Fig., Figure, Table)
                    if text.starts_with("Fig.") || text.starts_with("Figure") {
                        // Though we don't have a Caption BlockType in our enum by default,
                        // we can leave it as Text or label appropriately if the enum expands.
                    }
                    
                    // Heuristic: If it's very short and ends with a dot, maybe a list item? 
                    // ListProcessor should handle this, so we leave it.

                    // For now, this acts as a placeholder for more advanced heuristic
                    // or LLM-based relabeling logic that runs after initial structured parsing.
                }
            }
        }
        Ok(())
    }
}
