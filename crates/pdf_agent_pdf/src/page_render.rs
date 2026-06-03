use crate::error::{Error, Result};
use crate::pdfium_backend::get_pdfium;
use pdf_agent_core::providers::traits::PageImage;
use pdfium_render::prelude::*;
use std::path::Path;

pub fn render_pdf_page<P: AsRef<Path>>(
    file_path: P,
    page_index: usize,
    dpi: u32,
) -> Result<PageImage> {
    let pdfium = get_pdfium().map_err(|e| Error::Processing(e))?;
    
    let path_str = file_path.as_ref().to_str().ok_or_else(|| {
        Error::Processing("Invalid path character encoding".to_string())
    })?;
    
    let document = pdfium
        .load_pdf_from_file(path_str, None)
        .map_err(|e| Error::Processing(format!("PDFium load error: {:?}", e)))?;

    let pages = document.pages();
    if page_index >= pages.len() as usize {
        return Err(Error::PageOutOfBounds(page_index));
    }

    let page = pages
        .get(page_index as u16)
        .map_err(|e| Error::Processing(format!("Failed to get page: {:?}", e)))?;

    // Compute target width based on DPI
    // PDF coordinates are in points (1/72 of an inch)
    let width_pts = page.width().value;
    let target_width = ((width_pts as f64 * dpi as f64) / 72.0) as u16;

    let render_config = PdfRenderConfig::new()
        .set_target_width(target_width as i32)
        .render_form_data(true)
        .render_annotations(true);

    let bitmap = page
        .render_with_config(&render_config)
        .map_err(|e| Error::Processing(format!("PDFium render error: {:?}", e)))?;

    let dynamic_image = bitmap.as_image();

    let rgba_image = dynamic_image.into_rgba8();
    let width = rgba_image.width();
    let height = rgba_image.height();
    let bytes = rgba_image.into_raw();

    Ok(PageImage {
        width,
        height,
        bytes,
    })
}
