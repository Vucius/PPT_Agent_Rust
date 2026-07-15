use crate::message::Message;
use iced::Command;
use std::path::PathBuf;

pub fn pick_file_cmd() -> Command<Message> {
    Command::perform(
        async {
            let file = rfd::AsyncFileDialog::new()
                .add_filter("PDF files", &["pdf"])
                .pick_file()
                .await;
            file.map(|f| f.path().to_path_buf())
        },
        |path_opt| match path_opt {
            Some(path) => match pdf_agent_pdf::PdfProvider::open(&path) {
                Ok(provider) => {
                    use pdf_agent_core::providers::traits::DocumentProvider;
                    let page_count = provider.page_count().unwrap_or(0);
                    Message::PdfLoaded { path, page_count }
                }
                Err(e) => Message::PdfLoadFailed(e.to_string()),
            },
            None => Message::PdfLoadFailed("No file selected".to_string()),
        },
    )
}

pub fn render_page_cmd(path: PathBuf, page_index: usize, dpi: u32) -> Command<Message> {
    Command::perform(
        async move {
            tokio::task::spawn_blocking(move || {
                let provider = pdf_agent_pdf::PdfProvider::open(&path)
                    .map_err(|e| pdf_agent_core::error::Error::Pdf(e.to_string()))?;
                use pdf_agent_core::providers::traits::DocumentProvider;
                provider.render_page(page_index, dpi)
            })
            .await
            .map_err(|e| e.to_string())?
            .map_err(|e| e.to_string())
        },
        move |result| match result {
            Ok(img) => Message::PageImageLoaded { page_index, image: img },
            Err(e) => Message::PageImageLoadFailed(e),
        }
    )
}

pub fn export_markdown_cmd(markdown: String, default_filename: String) -> Command<Message> {
    Command::perform(
        async move {
            let file = rfd::AsyncFileDialog::new()
                .set_file_name(&default_filename)
                .add_filter("Markdown files", &["md"])
                .save_file()
                .await;
            
            if let Some(file_handle) = file {
                let path = file_handle.path().to_path_buf();
                tokio::fs::write(&path, markdown).await.map_err(|e| e.to_string())?;
                
                // Open the folder
                if let Some(parent) = path.parent() {
                    let _ = opener::open(parent);
                }
                
                Ok(path.to_string_lossy().to_string())
            } else {
                Err("Export cancelled by user".to_string())
            }
        },
        |result| Message::ExportCompleted(result)
    )
}

pub fn export_json_cmd(document: pdf_agent_core::schema::document::Document, default_filename: String) -> Command<Message> {
    Command::perform(
        async move {
            let file = rfd::AsyncFileDialog::new()
                .set_file_name(&default_filename)
                .add_filter("JSON files", &["json"])
                .save_file()
                .await;
            
            if let Some(file_handle) = file {
                let path = file_handle.path().to_path_buf();
                let json = serde_json::to_string_pretty(&document).map_err(|e| e.to_string())?;
                tokio::fs::write(&path, json).await.map_err(|e| e.to_string())?;
                
                // Open the folder
                if let Some(parent) = path.parent() {
                    let _ = opener::open(parent);
                }
                
                Ok(path.to_string_lossy().to_string())
            } else {
                Err("Export cancelled by user".to_string())
            }
        },
        |result| Message::ExportCompleted(result)
    )
}
