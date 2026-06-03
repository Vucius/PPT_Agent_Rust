use crate::message::Message;
use crate::state::main_state::MainState;
use crate::theme::BOLD_FONT;
use iced::widget::{button, column, row, text, text_input, Space};
use iced::{Color, Element, Length};

pub fn view(
    state: &MainState,
    feedback_input: &str,
    current_page: usize,
    total_pages: usize,
    rendered_image: Option<&pdf_agent_core::providers::traits::PageImage>,
    is_loading_image: bool,
    image_error: Option<&str>,
    selected_block_id: Option<&str>,
    diff_mode: bool,
    patch_preview: Option<&str>,
    history_index: usize,
    history_len: usize,
) -> Element<'static, Message> {
    // 1. Build the left panel (PDF Preview & Navigation)
    let left_content: Element<'static, Message> = if total_pages == 0 {
        column![
            text("Source PDF Preview").size(20).font(BOLD_FONT),
            Space::with_height(20),
            text("Please click 'Open PDF' to select a document.")
        ]
        .width(Length::Fill)
        .into()
    } else {
        let preview_widget: Element<'static, Message> = if is_loading_image {
            column![
                Space::with_height(100),
                text("Rendering page...").size(16),
            ]
            .align_items(iced::Alignment::Center)
            .width(Length::Fill)
            .into()
        } else if let Some(err) = image_error {
            column![
                Space::with_height(100),
                text(format!("Failed to render page: {}", err))
                    .style(Color::from_rgb(1.0, 0.3, 0.3))
                    .size(16),
            ]
            .align_items(iced::Alignment::Center)
            .width(Length::Fill)
            .into()
        } else if let Some(img) = rendered_image {
            let handle = iced::widget::image::Handle::from_pixels(img.width, img.height, img.bytes.clone());
            let image_widget = iced::widget::Image::new(handle).width(Length::Fill);
            iced::widget::scrollable(image_widget).height(Length::Fill).into()
        } else {
            column![
                Space::with_height(100),
                text("Page not rendered.").size(16),
            ]
            .align_items(iced::Alignment::Center)
            .width(Length::Fill)
            .into()
        };

        let prev_btn = if current_page > 0 {
            button("Previous").on_press(Message::PageChanged(current_page - 1))
        } else {
            button("Previous")
        };

        let next_btn = if current_page + 1 < total_pages {
            button("Next").on_press(Message::PageChanged(current_page + 1))
        } else {
            button("Next")
        };

        let page_controls = row![
            prev_btn,
            Space::with_width(15),
            text(format!("Page {} of {}", current_page + 1, total_pages)).size(16),
            Space::with_width(15),
            next_btn,
        ]
        .align_items(iced::Alignment::Center);

        column![
            text("Source PDF Preview").size(20).font(BOLD_FONT),
            Space::with_height(10),
            preview_widget,
            Space::with_height(10),
            page_controls,
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    };

    let left = column![left_content]
        .width(Length::FillPortion(1))
        .padding(20);

    // 2. Build the right panel based on App state
    let right: Element<'static, Message> = match state {
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

        MainState::Converting { file_path, progress } => column![
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
        .into(),

        MainState::Converted { markdown, document, .. } => {
            // Version History Controls
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

            if diff_mode && patch_preview.is_some() {
                // Diff View Mode
                let diff_content = patch_preview.unwrap().to_string();
                column![
                    row![
                        text("Proposed LLM Patch (Diff Preview)").size(20).font(BOLD_FONT),
                        Space::with_width(20),
                        history_controls,
                    ].align_items(iced::Alignment::Center),
                    Space::with_height(15),
                    row![
                        button("Accept Patch").on_press(Message::AcceptPatchClicked),
                        Space::with_width(10),
                        button("Reject Patch").on_press(Message::RejectPatchClicked),
                    ],
                    Space::with_height(15),
                    iced::widget::scrollable(
                        column![
                            text("Proposed Updated Output:").size(14).font(BOLD_FONT),
                            Space::with_height(10),
                            text(diff_content).size(14),
                        ]
                    ).height(Length::Fill)
                ]
                .width(Length::FillPortion(1))
                .padding(20)
                .into()
            } else {
                // Normal Preview Mode with Page Block List
                let empty_blocks = Vec::new();
                let blocks = document.pages.get(current_page).map(|p| &p.blocks).unwrap_or(&empty_blocks);
                
                 let block_list_view = column(
                     blocks.iter().map(|block| {
                         let is_selected = selected_block_id == Some(block.id.as_str());
                         let label = format!("[{:?}] {}", block.block_type, if block.text.len() > 80 { format!("{}...", &block.text[..80].replace('\n', " ")) } else { block.text.clone().replace('\n', " ") });

                         let block_btn = button(
                             row![
                                 text(label).size(13).width(Length::Fill),
                                 text(if is_selected { "SELECTED" } else { "SELECT" }).size(11).font(BOLD_FONT),
                             ].spacing(10).align_items(iced::Alignment::Center)
                         )
                         .width(Length::Fill)
                         .padding(8)
                         .on_press(Message::BlockSelected(if is_selected { None } else { Some(block.id.clone()) }));

                         block_btn.into()
                     }).collect::<Vec<_>>()
                 ).spacing(10);

                let workspace_tab = row![
                    column![
                        text("Converted Markdown").font(BOLD_FONT),
                        Space::with_height(10),
                        iced::widget::scrollable(text(markdown.clone()).size(14)).height(Length::Fill)
                    ].width(Length::FillPortion(1)),
                    Space::with_width(15),
                    column![
                        text("Document Blocks (Page)").font(BOLD_FONT),
                        Space::with_height(10),
                        iced::widget::scrollable(block_list_view).height(Length::Fill)
                    ].width(Length::FillPortion(1)),
                ].spacing(10).height(Length::Fill);

                column![
                    row![
                        text("Converted Document").size(20).font(BOLD_FONT),
                        Space::with_width(20),
                        history_controls,
                    ].align_items(iced::Alignment::Center),
                    Space::with_height(10),
                    workspace_tab
                ]
                .width(Length::FillPortion(1))
                .padding(20)
                .into()
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

    let mut actions = row![].spacing(10).align_items(iced::Alignment::Center);

    if is_converting {
        actions = actions.push(button("Cancel").on_press(Message::CancelClicked));
    } else {
        actions = actions.push(button("Open PDF").on_press(Message::OpenFileClicked));
        if is_pdf_loaded {
            actions = actions.push(button("Convert").on_press(Message::ConvertClicked));
        }
    }

    let is_block_selected = selected_block_id.is_some();
    let mut submit_btn = button("Submit");
    if is_block_selected && !feedback_input.trim().is_empty() {
        submit_btn = submit_btn.on_press(Message::SubmitFeedbackClicked);
    }

    let feedback_label = if let Some(id) = selected_block_id {
        format!("Feedback on Block (ID: {})...", &id[..id.len().min(8)])
    } else {
        "Select a block above to request LLM alignment...".to_string()
    };

    let feedback = row![
        text_input(&feedback_label, feedback_input)
            .on_input(Message::FeedbackInputChanged),
        submit_btn
    ]
    .spacing(10)
    .width(Length::Fill);

    column![
        main_content,
        Space::with_height(10),
        row![actions, Space::with_width(20), feedback]
            .padding(10)
            .align_items(iced::Alignment::Center)
    ]
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}
