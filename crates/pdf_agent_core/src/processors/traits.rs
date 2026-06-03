use crate::context::pipeline_context::PipelineContext;
use crate::error::Result;
use crate::schema::document::Document;

pub trait DocumentProcessor: Send + Sync {
    fn name(&self) -> &'static str;
    fn process(&self, document: &mut Document, ctx: &PipelineContext) -> Result<()>;
}
