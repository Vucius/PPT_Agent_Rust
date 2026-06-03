use std::path::Path;
use pdfium_render::prelude::*;

pub fn get_pdfium() -> std::result::Result<Pdfium, String> {
    // Try system library first
    match Pdfium::bind_to_system_library() {
        Ok(bindings) => Ok(Pdfium::new(bindings)),
        Err(system_err) => {
            // If system binding fails, try local library
            let lib_name = Pdfium::pdfium_platform_library_name_at_path(Path::new("./"));
            match Pdfium::bind_to_library(lib_name) {
                Ok(bindings) => Ok(Pdfium::new(bindings)),
                Err(local_err) => Err(format!(
                    "Could not bind to PDFium. System error: {:?}. Local error: {:?}",
                    system_err, local_err
                )),
            }
        }
    }
}
