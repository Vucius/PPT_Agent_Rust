use crate::context::pipeline_context::PipelineContext;
use crate::error::Result;
use crate::processors::traits::DocumentProcessor;
use crate::schema::document::Document;

pub struct DebugProcessor;

impl DocumentProcessor for DebugProcessor {
    fn name(&self) -> &'static str {
        "DebugProcessor"
    }

    fn process(&self, document: &mut Document, _ctx: &PipelineContext) -> Result<()> {
        println!("[DEBUG] Document '{}' has {} pages", document.file_name, document.pages.len());
        for page in &document.pages {
            println!("  - Page {} has {} blocks", page.index, page.blocks.len());
        }
        Ok(())
    }
}
