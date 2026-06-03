use crate::error::Result;
use crate::schema::line::Line;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone)]
pub struct PageImage {
    pub width: u32,
    pub height: u32,
    pub bytes: Vec<u8>,
}

pub trait DocumentProvider: Send + Sync {
    fn page_count(&self) -> Result<usize>;
    fn page_size(&self, page_index: usize) -> Result<(f64, f64)>;
    fn render_page(&self, page_index: usize, dpi: u32) -> Result<PageImage>;
    fn extract_native_text(&self, page_index: usize) -> Result<Vec<Line>>;
}

use crate::schema::block::Block;
use std::sync::Arc;

pub trait OcrProvider: Send + Sync {
    fn recognize_text(&self, page_image: &[u8], width: u32, height: u32) -> Result<Vec<Line>>;
}

pub struct OcrService {
    provider: Arc<dyn OcrProvider>,
}

impl OcrService {
    pub fn new(provider: Arc<dyn OcrProvider>) -> Self {
        Self { provider }
    }
    pub fn recognize_text(&self, page_image: &[u8], width: u32, height: u32) -> Result<Vec<Line>> {
        self.provider.recognize_text(page_image, width, height)
    }
}

pub trait LayoutProvider: Send + Sync {
    fn detect_layout(&self, page_image: &[u8], width: u32, height: u32) -> Result<Vec<Block>>;
}

pub struct LayoutService {
    provider: Arc<dyn LayoutProvider>,
}

impl LayoutService {
    pub fn new(provider: Arc<dyn LayoutProvider>) -> Self {
        Self { provider }
    }
    pub fn detect_layout(&self, page_image: &[u8], width: u32, height: u32) -> Result<Vec<Block>> {
        self.provider.detect_layout(page_image, width, height)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmRequest {
    pub system_prompt: Option<String>,
    pub user_prompt: String,
    pub temperature: Option<f32>,
    pub max_tokens: Option<usize>,
    pub json_mode: bool,
}

#[async_trait::async_trait]
pub trait LlmService: Send + Sync {
    async fn complete(&self, request: LlmRequest) -> Result<String>;
    async fn complete_json(&self, request: LlmRequest) -> Result<serde_json::Value>;
}

pub struct LlmServiceWrapper {
    pub service: Arc<dyn LlmService>,
}

impl LlmServiceWrapper {
    pub fn new(service: Arc<dyn LlmService>) -> Self {
        Self { service }
    }
}
