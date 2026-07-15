use crate::context::pipeline_context::PipelineContext;
use crate::error::Result;
use crate::processors::traits::DocumentProcessor;
use crate::schema::document::Document;

pub struct OrderProcessor;

impl DocumentProcessor for OrderProcessor {
    fn name(&self) -> &'static str {
        "OrderProcessor"
    }

    fn process(&self, document: &mut Document, _ctx: &PipelineContext) -> Result<()> {
        for page in &mut document.pages {
            page.blocks.sort_by(|a, b| {
                let y_tolerance = 5.0;
                let a_y = a.bbox.y0;
                let b_y = b.bbox.y0;
                
                if (a_y - b_y).abs() < y_tolerance {
                    a.bbox.x0.partial_cmp(&b.bbox.x0).unwrap_or(std::cmp::Ordering::Equal)
                } else {
                    a_y.partial_cmp(&b_y).unwrap_or(std::cmp::Ordering::Equal)
                }
            });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::{bbox::BBox, block::Block, block_type::BlockType, page::Page};
    use crate::context::pipeline_context::PipelineContext;

    fn make_block(id: &str, x0: f64, y0: f64) -> Block {
        Block::new(
            id.to_string(),
            BBox::new(x0, y0, x0 + 10.0, y0 + 10.0),
            BlockType::Text,
            "test".to_string(),
            vec![],
        )
    }

    #[test]
    fn test_order_processor() {
        let b1 = make_block("b1", 100.0, 50.0); // Right, Top
        let b2 = make_block("b2", 10.0, 50.0);  // Left, Top
        let b3 = make_block("b3", 10.0, 100.0); // Left, Bottom
        let b4 = make_block("b4", 50.0, 52.0);  // Middle, Top (within tolerance)

        let page = Page::new(0, 800.0, 600.0, vec![b1, b2, b3, b4]);
        let mut doc = Document::new("test.pdf".to_string(), vec![page]);
        let ctx = PipelineContext::new(
            crate::config::PipelineConfig::default(),
            crate::context::ServiceRegistry::new(),
        );

        let processor = OrderProcessor;
        processor.process(&mut doc, &ctx).unwrap();

        let ordered_ids: Vec<String> = doc.pages[0].blocks.iter().map(|b| b.id.clone()).collect();
        // Top line y≈50: x0=10 (b2), x0=50 (b4), x0=100 (b1)
        // Bottom line y=100: x0=10 (b3)
        assert_eq!(ordered_ids, vec!["b2", "b4", "b1", "b3"]);
    }
}
