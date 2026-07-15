use crate::context::pipeline_context::PipelineContext;
use crate::error::Result;
use crate::processors::traits::DocumentProcessor;
use crate::schema::block_type::BlockType;
use crate::schema::document::Document;

pub struct TableMergeProcessor;

impl DocumentProcessor for TableMergeProcessor {
    fn name(&self) -> &'static str {
        "TableMergeProcessor"
    }

    fn process(&self, document: &mut Document, _ctx: &PipelineContext) -> Result<()> {
        let page_count = document.pages.len();
        if page_count < 2 {
            return Ok(());
        }

        // We iterate through pages and look at page i and page i+1
        for i in 0..page_count - 1 {
            // Check if page i ends with a table
            let mut has_ending_table = false;
            if let Some(last_block) = document.pages[i].blocks.last() {
                if last_block.block_type == BlockType::Table {
                    has_ending_table = true;
                }
            }

            // Check if page i+1 starts with a table
            let mut has_starting_table = false;
            if let Some(first_block) = document.pages[i + 1].blocks.first() {
                if first_block.block_type == BlockType::Table {
                    has_starting_table = true;
                }
            }

            if has_ending_table && has_starting_table {
                // Heuristic pass: merge them
                // We'll take the text from page i+1's first block and append it to page i's last block
                let next_text = document.pages[i + 1].blocks[0].text.clone();
                let mut next_lines = document.pages[i + 1].blocks[0].lines.clone();

                let last_idx_prev_page = document.pages[i].blocks.len() - 1;
                let prev_block = &mut document.pages[i].blocks[last_idx_prev_page];

                prev_block.text.push_str("\n");
                prev_block.text.push_str(&next_text);
                prev_block.lines.append(&mut next_lines);

                // Update bounding box to cover the new area (naively merge the bboxes)
                // Though they are on different pages, bounding box merging is mostly conceptual here
                // to encompass all spans.
                
                // Clear the merged table from the next page so it's not rendered twice.
                // We'll just change its type to Unknown and clear its text.
                let next_block = &mut document.pages[i + 1].blocks[0];
                next_block.block_type = BlockType::Unknown;
                next_block.text.clear();
                next_block.lines.clear();
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::{bbox::BBox, block::Block, page::Page};
    use crate::context::ServiceRegistry;

    fn make_block(id: &str, text: &str, btype: BlockType) -> Block {
        Block::new(id.to_string(), BBox::new(0.0, 0.0, 10.0, 10.0), btype, text.to_string(), vec![])
    }

    #[test]
    fn test_table_merge_processor() {
        let b1 = make_block("1", "Text", BlockType::Text);
        let b2 = make_block("2", "Table Part 1", BlockType::Table);
        let page1 = Page::new(0, 800.0, 600.0, vec![b1, b2]);

        let b3 = make_block("3", "Table Part 2", BlockType::Table);
        let b4 = make_block("4", "More Text", BlockType::Text);
        let page2 = Page::new(1, 800.0, 600.0, vec![b3, b4]);

        let mut doc = Document::new("test.pdf".to_string(), vec![page1, page2]);
        let ctx = PipelineContext::new(crate::config::PipelineConfig::default(), ServiceRegistry::new());

        let processor = TableMergeProcessor;
        processor.process(&mut doc, &ctx).unwrap();

        // Check page 1
        assert_eq!(doc.pages[0].blocks[1].text, "Table Part 1\nTable Part 2");
        assert_eq!(doc.pages[0].blocks[1].block_type, BlockType::Table);

        // Check page 2
        assert_eq!(doc.pages[1].blocks[0].text, "");
        assert_eq!(doc.pages[1].blocks[0].block_type, BlockType::Unknown);
    }
}
