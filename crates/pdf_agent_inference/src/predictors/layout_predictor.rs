use pdf_agent_core::providers::traits::LayoutProvider;
use pdf_agent_core::schema::block::Block;

pub struct LayoutPredictor;

impl LayoutPredictor {
    pub fn new() -> Self {
        Self
    }
}

impl Default for LayoutPredictor {
    fn default() -> Self {
        Self::new()
    }
}

impl LayoutProvider for LayoutPredictor {
    fn detect_layout(&self, _page_image: &[u8], _width: u32, _height: u32) -> pdf_agent_core::Result<Vec<Block>> {
        // This is a stub for the LayoutLM / YOLO-based model.
        // It returns an empty list of detected layout blocks.
        Ok(vec![])
    }
}
