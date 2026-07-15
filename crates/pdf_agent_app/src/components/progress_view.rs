use crate::message::Message;
use crate::theme::BOLD_FONT;
use iced::widget::{column, text, Space};
use iced::{Element, Length};

pub fn view<'a>(
    file_path: &'a std::path::Path,
    progress: &'a pdf_agent_core::pipeline::job_event::JobProgress,
) -> Element<'a, Message> {
    column![
        text("Converting...").size(22).font(BOLD_FONT),
        Space::with_height(10),
        text(format!("Processing: {:?}", file_path.file_name().unwrap_or_default())),
        Space::with_height(20),
        text(format!("Stage: {:?}", progress.stage)),
        text(format!("Page: {} / {}", progress.current_page, progress.total_pages)),
        text(format!("Elapsed: {:.1}s", progress.elapsed_seconds))
    ]
    .width(Length::FillPortion(1))
    .padding(20)
    .into()
}
