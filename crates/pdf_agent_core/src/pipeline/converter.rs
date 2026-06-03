use crate::builders::document_builder::{DocumentBuilder, TextDocumentBuilder};
use crate::context::pipeline_context::PipelineContext;
use crate::error::{Error, Result};
use crate::pipeline::cancel_token::CancelToken;
use crate::pipeline::job_event::{JobEvent, JobProgress};
use crate::pipeline::stage::PipelineStage;
use crate::processors::line_merge_processor::LineMergeProcessor;
use crate::processors::heading_processor::HeadingProcessor;
use crate::processors::list_processor::ListProcessor;
use crate::processors::table_processor::TableProcessor;
use crate::processors::traits::DocumentProcessor;
use crate::providers::traits::DocumentProvider;
use crate::renderers::markdown_renderer::MarkdownRenderer;
use crate::renderers::traits::DocumentRenderer;
use tokio::sync::mpsc::UnboundedSender;

pub struct PdfConverter {
    processors: Vec<Box<dyn DocumentProcessor>>,
}

impl PdfConverter {
    pub fn new() -> Self {
        Self {
            processors: vec![
                Box::new(TableProcessor),
                Box::new(HeadingProcessor),
                Box::new(ListProcessor),
                Box::new(LineMergeProcessor),
            ],
        }
    }

    pub async fn convert(
        &self,
        job_id: &str,
        provider: &dyn DocumentProvider,
        ctx: &PipelineContext,
        cancel_token: &CancelToken,
        event_sender: Option<UnboundedSender<JobEvent>>,
    ) -> Result<(String, crate::schema::document::Document)> {
        let start_time = std::time::Instant::now();
        let total_pages = provider.page_count()?;

        let send_progress = |stage: PipelineStage, current_page: usize| {
            if let Some(ref sender) = event_sender {
                let progress = JobProgress {
                    job_id: job_id.to_string(),
                    stage,
                    current_page,
                    total_pages,
                    elapsed_seconds: start_time.elapsed().as_secs_f32(),
                };
                let _ = sender.send(JobEvent::Progress(progress));
            }
        };

        if cancel_token.is_cancelled() {
            return Err(Error::Cancelled);
        }

        // 1. Loading PDF and Building Document
        send_progress(PipelineStage::LoadingPdf, 0);
        let builder = TextDocumentBuilder;
        let mut document = builder.build(provider, ctx)?;

        if cancel_token.is_cancelled() {
            return Err(Error::Cancelled);
        }

        // 2. Running Processors
        send_progress(PipelineStage::RunningProcessors, 0);
        for processor in &self.processors {
            if cancel_token.is_cancelled() {
                return Err(Error::Cancelled);
            }
            processor.process(&mut document, ctx)?;
        }

        // 3. Rendering to Markdown
        send_progress(PipelineStage::Rendering, total_pages);
        if cancel_token.is_cancelled() {
            return Err(Error::Cancelled);
        }
        let renderer = MarkdownRenderer;
        let markdown = renderer.render(&document, ctx)?;

        Ok((markdown, document))
    }
}

impl Default for PdfConverter {
    fn default() -> Self {
        Self::new()
    }
}
