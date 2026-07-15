use crate::message::Message;
use crate::state::main_state::MainState;
use crate::theme::BOLD_FONT;
use iced::widget::{button, column, row, text, Space};
use iced::{Color, Element, Length};
use crate::panes;
use crate::components;

pub fn view<'a>(
    state: &'a MainState,
    feedback_input: &'a str,
    current_page: usize,
    total_pages: usize,
    rendered_image: Option<&'a pdf_agent_core::providers::traits::PageImage>,
    is_loading_image: bool,
    image_error: Option<&'a str>,
    selected_block_id: Option<&'a str>,
    diff_mode: bool,
    patch_preview: Option<&'a str>,
    history_index: usize,
    history_len: usize,
    config: &'a pdf_agent_core::config::PipelineConfig,
) -> Element<'a, Message> {
    // 1. Build the left panel (PDF Preview & Navigation)
    let left_content = panes::pdf_pane::view(
        total_pages,
        current_page,
        is_loading_image,
        image_error,
        rendered_image,
    );
    let left = column![left_content]
        .width(Length::FillPortion(1))
        .padding(20);

    // Version History Controls (shared for right panel)
    let mut undo_btn = button("Undo");
    if history_index > 0 {
        undo_btn = undo_btn.on_press(Message::UndoClicked);
    }
    let mut redo_btn = button("Redo");
    if history_index + 1 < history_len {
        redo_btn = redo_btn.on_press(Message::RedoClicked);
    }
    let history_controls = row![
        undo_btn,
        Space::with_width(10),
        text(format!("Version {} / {}", history_index + 1, history_len)).size(14),
        Space::with_width(10),
        redo_btn,
    ].align_items(iced::Alignment::Center);

    // 2. Build the right panel based on App state
    let right: Element<'a, Message> = match state {
        MainState::Empty => column![
            text("Markdown Output").size(20).font(BOLD_FONT),
            Space::with_height(20),
            text("No document converted yet.")
        ]
        .width(Length::FillPortion(1))
        .padding(20)
        .into(),

        MainState::PdfLoaded { file_path, page_count } => column![
            text("Markdown Output").size(20).font(BOLD_FONT),
            Space::with_height(10),
            text(format!("Loaded: {:?}", file_path.file_name().unwrap_or_default())),
            text(format!("Total pages: {}", page_count)),
            Space::with_height(20),
            text("PDF loaded. Click 'Convert' to extract Markdown.")
        ]
        .width(Length::FillPortion(1))
        .padding(20)
        .into(),

        MainState::Converting { file_path, progress } => {
            components::progress_view::view(file_path, progress)
        }

        MainState::Converted { markdown, document, .. } => {
            if diff_mode && patch_preview.is_some() {
                panes::diff_pane::view(markdown, patch_preview.unwrap(), history_controls.into())
            } else {
                panes::markdown_pane::view(
                    markdown,
                    document,
                    current_page,
                    selected_block_id,
                    history_controls.into(),
                )
            }
        }

        MainState::Failed { error, .. } => column![
            text("Conversion Failed").size(20).font(BOLD_FONT),
            Space::with_height(20),
            text(error.clone()).style(Color::from_rgb(1.0, 0.3, 0.3))
        ]
        .width(Length::FillPortion(1))
        .padding(20)
        .into(),
    };

    let main_content = row![left, right].spacing(20).width(Length::Fill).height(Length::Fill);

    let is_pdf_loaded = !matches!(state, MainState::Empty | MainState::Converting { .. });
    let is_converting = matches!(state, MainState::Converting { .. });
    let is_converted = matches!(state, MainState::Converted { .. });

    let actions = components::toolbar::view(is_pdf_loaded, is_converting, is_converted);
    let feedback = components::feedback_box::view(feedback_input, selected_block_id);

    // Status bar at the very bottom (ui-interaction-spec §2.4)
    let status_bar = components::status_bar::view(
        state,
        &config.ocr_mode,
        &config.output_format,
        config.llm.daily_limit,
    );

    column![
        main_content,
        Space::with_height(10),
        row![actions, Space::with_width(20), feedback]
            .padding(10)
            .align_items(iced::Alignment::Center),
        status_bar
    ]
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}
