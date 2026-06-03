use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    pub provider: String,
    pub model_name: String,
    pub base_url: String,
    pub daily_limit: i64,
}

impl Default for LlmConfig {
    fn default() -> Self {
        Self {
            provider: "mock".to_string(),
            model_name: "gpt-4o-mini".to_string(),
            base_url: "https://api.openai.com/v1".to_string(),
            daily_limit: 50000,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineConfig {
    pub ocr_mode: String,
    pub output_format: String,
    pub llm: LlmConfig,
}

impl Default for PipelineConfig {
    fn default() -> Self {
        Self {
            ocr_mode: "auto".to_string(),
            output_format: "markdown".to_string(),
            llm: LlmConfig::default(),
        }
    }
}
