use crate::context::pipeline_context::PipelineContext;
use crate::error::Result;
use crate::processors::traits::DocumentProcessor;
use crate::schema::document::Document;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TocEntry {
    pub level: usize,
    pub title: String,
    pub page_index: usize,
}

pub struct TocProcessor;

impl DocumentProcessor for TocProcessor {
    fn name(&self) -> &'static str {
        "TocProcessor"
    }

    fn process(&self, document: &mut Document, _ctx: &PipelineContext) -> Result<()> {
        let mut toc = Vec::new();

        for (page_idx, page) in document.pages.iter().enumerate() {
            for block in &page.blocks {
                if block.block_type.is_heading() {
                    let level = match block.block_type {
                        crate::schema::block_type::BlockType::Heading1 => 1,
                        crate::schema::block_type::BlockType::Heading2 => 2,
                        crate::schema::block_type::BlockType::Heading3 => 3,
                        crate::schema::block_type::BlockType::Heading4 => 4,
                        crate::schema::block_type::BlockType::Heading5 => 5,
                        crate::schema::block_type::BlockType::Heading6 => 6,
                        _ => 1,
                    };

                    toc.push(TocEntry {
                        level,
                        title: block.text.trim().to_string(),
                        page_index: page_idx,
                    });
                }
            }
        }

        if !toc.is_empty() {
            if let Ok(toc_json) = serde_json::to_string(&toc) {
                document.metadata.insert("toc_json".to_string(), toc_json);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::{bbox::BBox, block::Block, block_type::BlockType, page::Page};
    use crate::context::ServiceRegistry;

    fn make_block(id: &str, text: &str, btype: BlockType) -> Block {
        Block::new(id.to_string(), BBox::new(0.0, 0.0, 10.0, 10.0), btype, text.to_string(), vec![])
    }

    #[test]
    fn test_toc_processor() {
        let b1 = make_block("1", "Introduction", BlockType::Heading1);
        let b2 = make_block("2", "Background", BlockType::Heading2);
        let b3 = make_block("3", "Some text", BlockType::Text);
        
        let page1 = Page::new(0, 800.0, 600.0, vec![b1, b2, b3]);
        let page2 = Page::new(1, 800.0, 600.0, vec![make_block("4", "Conclusion", BlockType::Heading1)]);
        
        let mut doc = Document::new("test.pdf".to_string(), vec![page1, page2]);
        let ctx = PipelineContext::new(crate::config::PipelineConfig::default(), ServiceRegistry::new());

        let processor = TocProcessor;
        processor.process(&mut doc, &ctx).unwrap();

        assert!(doc.metadata.contains_key("toc_json"));
        let toc_json = &doc.metadata["toc_json"];
        let parsed: Vec<TocEntry> = serde_json::from_str(toc_json).unwrap();
        
        assert_eq!(parsed.len(), 3);
        assert_eq!(parsed[0].title, "Introduction");
        assert_eq!(parsed[0].level, 1);
        assert_eq!(parsed[0].page_index, 0);
        
        assert_eq!(parsed[1].title, "Background");
        assert_eq!(parsed[1].level, 2);
        
        assert_eq!(parsed[2].title, "Conclusion");
        assert_eq!(parsed[2].level, 1);
        assert_eq!(parsed[2].page_index, 1);
    }
}
