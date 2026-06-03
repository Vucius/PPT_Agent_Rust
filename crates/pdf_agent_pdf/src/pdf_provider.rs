use crate::error::Result;
use crate::native_text::NativeTextExtractor;
use lopdf::Document as LopdfDoc;
use pdf_agent_core::providers::traits::{DocumentProvider, PageImage};
use pdf_agent_core::schema::line::Line;
use std::path::Path;

pub struct PdfProvider {
    doc: LopdfDoc,
    file_path: std::path::PathBuf,
}

impl PdfProvider {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let doc = LopdfDoc::load(path.as_ref())?;
        Ok(Self {
            doc,
            file_path: path.as_ref().to_path_buf(),
        })
    }
}

impl DocumentProvider for PdfProvider {
    fn page_count(&self) -> pdf_agent_core::Result<usize> {
        Ok(self.doc.get_pages().len())
    }

    fn page_size(&self, page_index: usize) -> pdf_agent_core::Result<(f64, f64)> {
        let page_num = (page_index + 1) as u32;
        let page_id = self.doc.page_iter().nth(page_index).ok_or_else(|| {
            pdf_agent_core::error::Error::Pdf(format!("Page {} not found", page_num))
        })?;

        let page_dict = self
            .doc
            .get_object(page_id)
            .map_err(|e| pdf_agent_core::error::Error::Pdf(e.to_string()))?
            .as_dict()
            .map_err(|e| pdf_agent_core::error::Error::Pdf(e.to_string()))?;

        let media_box = page_dict
            .get(b"MediaBox")
            .ok()
            .and_then(|obj| obj.as_array().ok())
            .map(|arr| {
                let x0 = arr[0].as_float().map(|f| f as f64).unwrap_or(0.0);
                let y0 = arr[1].as_float().map(|f| f as f64).unwrap_or(0.0);
                let x1 = arr[2].as_float().map(|f| f as f64).unwrap_or(595.0);
                let y1 = arr[3].as_float().map(|f| f as f64).unwrap_or(842.0);
                (x1 - x0, y1 - y0)
            })
            .unwrap_or((595.0, 842.0));

        Ok(media_box)
    }

    fn render_page(&self, page_index: usize, dpi: u32) -> pdf_agent_core::Result<PageImage> {
        crate::page_render::render_pdf_page(&self.file_path, page_index, dpi)
            .map_err(|e| pdf_agent_core::error::Error::Pdf(e.to_string()))
    }

    fn extract_native_text(&self, page_index: usize) -> pdf_agent_core::Result<Vec<Line>> {
        let page_num = (page_index + 1) as u32;
        let (_, height) = self.page_size(page_index)?;
        let lines = NativeTextExtractor::extract_page_text(&self.doc, page_num, height)
            .map_err(|e| pdf_agent_core::error::Error::Pdf(e.to_string()))?;
        Ok(lines)
    }
}
