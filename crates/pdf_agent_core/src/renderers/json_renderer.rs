use crate::context::pipeline_context::PipelineContext;
use crate::error::Result;
use crate::renderers::traits::DocumentRenderer;
use crate::schema::document::Document;
use serde_json::Value;

pub struct JsonRenderer;

impl DocumentRenderer for JsonRenderer {
    type Output = Value;

    fn render(&self, document: &Document, _ctx: &PipelineContext) -> Result<Self::Output> {
        let val = serde_json::to_value(document)?;
        Ok(val)
    }
}
