use crate::context::pipeline_context::PipelineContext;
use crate::error::Result;
use crate::schema::document::Document;

pub trait DocumentRenderer {
    type Output;

    fn render(&self, document: &Document, ctx: &PipelineContext) -> Result<Self::Output>;
}
