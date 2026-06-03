use crate::context::pipeline_context::PipelineContext;
use crate::error::Result;
use crate::processors::traits::DocumentProcessor;
use crate::schema::block_type::BlockType;
use crate::schema::document::Document;

pub struct HeadingProcessor;

impl DocumentProcessor for HeadingProcessor {
    fn name(&self) -> &'static str {
        "HeadingProcessor"
    }

    fn process(&self, document: &mut Document, _ctx: &PipelineContext) -> Result<()> {
        for page in &mut document.pages {
            for block in &mut page.blocks {
                if block.block_type != BlockType::Text {
                    continue;
                }

                let mut total_size = 0.0;
                let mut count = 0;
                let mut has_bold = false;

                for line in &block.lines {
                    for span in &line.spans {
                        total_size += span.font_size;
                        count += 1;

                        let font_name_lower = span.font_name.to_lowercase();
                        if font_name_lower.contains("bold")
                            || font_name_lower.contains("bd")
                            || font_name_lower.contains("heavy")
                            || font_name_lower.contains("black")
                        {
                            has_bold = true;
                        }
                    }
                }

                let avg_font_size = if count > 0 {
                    total_size / count as f64
                } else {
                    10.0
                };

                let trimmed_text = block.text.trim();
                let word_count = trimmed_text.split_whitespace().count();

                // Structural criteria for headings: single line, relatively short text
                let is_short = word_count > 0 && word_count <= 12;
                let is_single_line = block.lines.len() <= 1;

                if is_short && is_single_line {
                    if avg_font_size >= 16.0 {
                        block.block_type = BlockType::Heading1;
                    } else if avg_font_size >= 13.0 {
                        block.block_type = BlockType::Heading2;
                    } else if avg_font_size >= 11.5 && has_bold {
                        block.block_type = BlockType::Heading3;
                    }
                }
            }
        }
        Ok(())
    }
}
