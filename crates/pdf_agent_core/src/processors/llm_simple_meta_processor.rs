use crate::context::pipeline_context::PipelineContext;
use crate::error::Result;
use crate::processors::traits::DocumentProcessor;
use crate::schema::document::Document;
// use crate::providers::traits::LlmService;

pub struct LlmSimpleMetaProcessor;

impl DocumentProcessor for LlmSimpleMetaProcessor {
    fn name(&self) -> &'static str {
        "LlmSimpleMetaProcessor"
    }

    fn process(&self, document: &mut Document, _ctx: &PipelineContext) -> Result<()> {
        // This is a stub for Phase C.
        // In the future, this processor would extract keywords or summaries using an LLM.
        // Example logic:
        // if let Some(llm_service) = _ctx.registry.get::<LlmService>() {
        //     let summary = llm_service.summarize(&document.file_name, &document.pages[0].blocks[0].text);
        //     document.metadata.insert("summary".to_string(), summary);
        // }
        
        // For now, just mark that it ran.
        document.metadata.insert("llm_meta_processed".to_string(), "true".to_string());
        
        Ok(())
    }
}
