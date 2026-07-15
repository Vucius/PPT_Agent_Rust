use crate::message::Message;
use iced::Command;
use pdf_agent_core::config::PipelineConfig;
use pdf_agent_core::schema::document::Document;

pub fn submit_feedback_cmd(
    document: Document,
    block_id: String,
    feedback: String,
    config: PipelineConfig,
    api_key: String,
) -> Command<Message> {
    if let Some((prev, target, next)) = document.find_block_with_context(&block_id) {
        let mut document_clone = document.clone();
        let prev_text = prev.map(|b| b.text.clone()).unwrap_or_else(|| "[None]".to_string());
        let target_text = target.text.clone();
        let next_text = next.map(|b| b.text.clone()).unwrap_or_else(|| "[None]".to_string());

        Command::perform(
            async move {
                let system_prompt = "You are an assistant correcting OCR and layout parsing errors in a document block. Output only the corrected text for the target block. Do not wrap in markdown code blocks unless the block itself is a code block.";
                let user_prompt = format!(
                    "Context:\n- Previous Block: {}\n- Target Block: {}\n- Next Block: {}\n\nUser Feedback: {}\n\nCorrected Target Block text:",
                    prev_text, target_text, next_text, feedback
                );
                
                let llm_service: std::sync::Arc<dyn pdf_agent_core::providers::traits::LlmService> = if config.llm.provider == "mock" {
                    std::sync::Arc::new(pdf_agent_llm::MockLlmService)
                } else {
                    std::sync::Arc::new(pdf_agent_llm::OpenAiLlmService::new(
                        api_key,
                        config.llm.base_url.clone(),
                        config.llm.model_name.clone(),
                    ))
                };
                
                let request = pdf_agent_core::providers::traits::LlmRequest {
                    system_prompt: Some(system_prompt.to_string()),
                    user_prompt,
                    temperature: Some(0.2),
                    max_tokens: Some(1024),
                    json_mode: false,
                };
                
                let result = llm_service.complete(request).await;
                match result {
                    Ok(new_text) => {
                        document_clone.update_block_text(&block_id, &new_text);
                        let renderer = pdf_agent_core::renderers::markdown_renderer::MarkdownRenderer;
                        let ctx = pdf_agent_core::context::PipelineContext::new(
                            config,
                            pdf_agent_core::context::ServiceRegistry::new()
                        );
                        use pdf_agent_core::renderers::traits::DocumentRenderer;
                        let new_markdown = renderer.render(&document_clone, &ctx)
                            .map_err(|e| e.to_string())?;
                            
                        if let Ok(db) = pdf_agent_storage::DbConnection::open("pdf_agent.db") {
                            let today = std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .map(|d| format!("day_{}", d.as_secs() / 86400))
                                .unwrap_or_else(|_| "today".to_string());
                            let estimated_tokens = (new_text.len() + system_prompt.len()) as i64 / 4;
                            let _ = db.increment_tokens_used(&today, estimated_tokens, 50000);
                        }
                        
                        Ok((new_markdown, document_clone))
                    }
                    Err(e) => Err(e.to_string())
                }
            },
            |res| Message::LlmFeedbackResult(res)
        )
    } else {
        Command::none()
    }
}
