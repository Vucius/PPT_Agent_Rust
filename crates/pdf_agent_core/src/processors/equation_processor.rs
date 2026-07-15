use crate::context::pipeline_context::PipelineContext;
use crate::error::Result;
use crate::processors::traits::DocumentProcessor;
use crate::schema::block_type::BlockType;
use crate::schema::document::Document;

pub struct EquationProcessor;

impl DocumentProcessor for EquationProcessor {
    fn name(&self) -> &'static str {
        "EquationProcessor"
    }

    fn process(&self, document: &mut Document, _ctx: &PipelineContext) -> Result<()> {
        for page in &mut document.pages {
            for block in &mut page.blocks {
                let text = block.text.trim();
                
                if (text.starts_with("$$") && text.ends_with("$$") && text.len() > 4) ||
                   (text.starts_with("\\[") && text.ends_with("\\]") && text.len() > 4) 
                {
                    block.block_type = BlockType::Equation;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::{bbox::BBox, block::Block, page::Page};
    use crate::context::pipeline_context::PipelineContext;

    fn make_block(id: &str, text: &str) -> Block {
        Block::new(
            id.to_string(),
            BBox::new(0.0, 0.0, 10.0, 10.0),
            BlockType::Text,
            text.to_string(),
            vec![],
        )
    }

    #[test]
    fn test_equation_processor() {
        let b1 = make_block("1", "$$ x^2 + y^2 = r^2 $$");
        let b2 = make_block("2", "\\[ E = mc^2 \\]");
        let b3 = make_block("3", "This is normal text $$ inline $$ test.");
        
        let page = Page::new(0, 800.0, 600.0, vec![b1, b2, b3]);
        let mut doc = Document::new("test.pdf".to_string(), vec![page]);
        let ctx = PipelineContext::new(
            crate::config::PipelineConfig::default(),
            crate::context::ServiceRegistry::new(),
        );

        let processor = EquationProcessor;
        processor.process(&mut doc, &ctx).unwrap();

        assert_eq!(doc.pages[0].blocks[0].block_type, BlockType::Equation);
        assert_eq!(doc.pages[0].blocks[1].block_type, BlockType::Equation);
        assert_eq!(doc.pages[0].blocks[2].block_type, BlockType::Text); // Inline shouldn't match block rule
    }
}
