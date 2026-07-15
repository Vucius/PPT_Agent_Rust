use crate::message::Message;
use iced::Command;
use pdf_agent_core::config::PipelineConfig;
use pdf_agent_core::runtime::JobManager;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedReceiver;
use pdf_agent_core::pipeline::job_event::JobEvent;

pub fn start_conversion_cmd(
    job_id: String,
    file_path: PathBuf,
    config: PipelineConfig,
    api_key: String,
    job_manager: Arc<JobManager>,
    event_receiver: Arc<tokio::sync::Mutex<Option<UnboundedReceiver<JobEvent>>>>,
) -> Command<Message> {
    match pdf_agent_pdf::PdfProvider::open(&file_path) {
        Ok(provider) => {
            let provider_arc = Arc::new(provider);
            let mut registry = pdf_agent_core::context::ServiceRegistry::new();
            
            let ocr_predictor = Arc::new(pdf_agent_inference::predictors::OcrPredictor::new());
            let ocr_service = Arc::new(pdf_agent_core::providers::traits::OcrService::new(ocr_predictor));
            registry.register(ocr_service);

            let layout_predictor = Arc::new(pdf_agent_inference::predictors::LayoutPredictor::new());
            let layout_service = Arc::new(pdf_agent_core::providers::traits::LayoutService::new(layout_predictor));
            registry.register(layout_service);

            let llm_service: Arc<dyn pdf_agent_core::providers::traits::LlmService> = if config.llm.provider == "mock" {
                Arc::new(pdf_agent_llm::MockLlmService)
            } else {
                Arc::new(pdf_agent_llm::OpenAiLlmService::new(
                    api_key,
                    config.llm.base_url.clone(),
                    config.llm.model_name.clone(),
                ))
            };
            let llm_service_wrapper = Arc::new(pdf_agent_core::providers::traits::LlmServiceWrapper::new(llm_service));
            registry.register(llm_service_wrapper);

            let ctx = Arc::new(pdf_agent_core::context::PipelineContext::new(config, registry));
            let rx = job_manager.start_job(job_id.clone(), provider_arc, ctx);

            Command::perform(
                async move {
                    let mut opt = event_receiver.lock().await;
                    *opt = Some(rx);
                },
                |_| Message::FeedbackInputChanged(String::new()),
            )
        }
        Err(e) => {
            let error_msg = e.to_string();
            Command::perform(
                async move { error_msg },
                move |err| Message::JobFailed { job_id: job_id.clone(), error: err }
            )
        }
    }
}
