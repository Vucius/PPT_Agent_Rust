use crate::context::pipeline_context::PipelineContext;
use crate::error::Result;
use crate::providers::traits::DocumentProvider;
use crate::schema::block::Block;
use crate::schema::block_type::BlockType;
use crate::schema::document::Document;
use crate::schema::page::Page;

pub trait DocumentBuilder {
    fn build(&self, provider: &dyn DocumentProvider, ctx: &PipelineContext) -> Result<Document>;
}

pub struct TextDocumentBuilder;

impl DocumentBuilder for TextDocumentBuilder {
    fn build(&self, provider: &dyn DocumentProvider, ctx: &PipelineContext) -> Result<Document> {
        let page_count = provider.page_count()?;
        let mut pages = Vec::with_capacity(page_count);

        for page_idx in 0..page_count {
            let (width, height) = provider.page_size(page_idx)?;
            let mut lines = provider.extract_native_text(page_idx)?;

            // OCR Fallback: If no native text lines are extracted, try to perform OCR
            if lines.is_empty() {
                use crate::providers::traits::OcrService;
                if let Some(ocr_service) = ctx.registry.get::<OcrService>() {
                    if let Ok(page_image) = provider.render_page(page_idx, 150) {
                        if let Ok(ocr_lines) = ocr_service.recognize_text(&page_image.bytes, page_image.width, page_image.height) {
                            lines = ocr_lines;
                        }
                    }
                }
            }

            let mut blocks = Vec::new();
            for (idx, line) in lines.into_iter().enumerate() {
                let id = format!("p{}_b{}", page_idx, idx);
                let text = line.text();
                let bbox = line.bbox;
                let block = Block::new(id, bbox, BlockType::Text, text, vec![line]);
                blocks.push(block);
            }

            pages.push(Page::new(page_idx, width, height, blocks));
        }

        Ok(Document::new("extracted_document".to_string(), pages))
    }
}
