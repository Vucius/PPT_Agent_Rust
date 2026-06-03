use crate::context::pipeline_context::PipelineContext;
use crate::error::Result;
use crate::processors::traits::DocumentProcessor;
use crate::schema::block_type::BlockType;
use crate::schema::document::Document;

pub struct TableProcessor;

impl DocumentProcessor for TableProcessor {
    fn name(&self) -> &'static str {
        "TableProcessor"
    }

    fn process(&self, document: &mut Document, _ctx: &PipelineContext) -> Result<()> {
        for page in &mut document.pages {
            for block in &mut page.blocks {
                if block.block_type != BlockType::Text {
                    continue;
                }

                if block.lines.is_empty() {
                    continue;
                }

                let line = &block.lines[0];
                let spans = &line.spans;

                if spans.len() >= 2 {
                    let mut is_table_row = true;
                    for i in 0..spans.len() - 1 {
                        let gap = spans[i+1].bbox.x0 - spans[i].bbox.x1;
                        if gap < 8.0 {
                            is_table_row = false;
                            break;
                        }
                    }

                    if is_table_row {
                        block.block_type = BlockType::Table;
                        let mut table_row = String::from("| ");
                        for span in spans {
                            table_row.push_str(&span.text.trim());
                            table_row.push_str(" | ");
                        }
                        block.text = table_row.trim_end().to_string();
                    }
                }
            }
        }
        Ok(())
    }
}
