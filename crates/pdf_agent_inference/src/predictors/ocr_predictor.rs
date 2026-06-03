use pdf_agent_core::providers::traits::OcrProvider;
use pdf_agent_core::schema::line::Line;

pub struct OcrPredictor;

impl OcrPredictor {
    pub fn new() -> Self {
        Self
    }
}

impl Default for OcrPredictor {
    fn default() -> Self {
        Self::new()
    }
}

impl OcrProvider for OcrPredictor {
    fn recognize_text(&self, _page_image: &[u8], _width: u32, _height: u32) -> pdf_agent_core::Result<Vec<Line>> {
        // This is a stub for the PaddleOCR / ONNX-based model.
        // It provides default recognized text lines to prove the OCR fallback pipeline triggers and executes successfully.
        use pdf_agent_core::schema::bbox::BBox;
        use pdf_agent_core::schema::span::Span;

        let mock_span = Span::new(
            "Scanned page content recognized via OCR pipeline".to_string(),
            "Courier-Bold".to_string(),
            12.0,
            BBox::new(50.0, 100.0, 450.0, 115.0),
            vec![]
        );
        let mock_line = Line::new(vec![mock_span], BBox::new(50.0, 100.0, 450.0, 115.0));
        Ok(vec![mock_line])
    }
}
