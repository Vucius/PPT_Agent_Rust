use crate::message::Message;
use crate::theme::BOLD_FONT;
use iced::widget::{button, column, row, text, Space};
use iced::{Element, Length};

pub fn view<'a>(
    markdown: &'a str,
    document: &'a pdf_agent_core::schema::document::Document,
    current_page: usize,
    selected_block_id: Option<&'a str>,
    history_controls: Element<'a, Message>,
) -> Element<'a, Message> {
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
            iced::widget::scrollable(text(markdown.to_string()).size(14)).height(Length::Fill)
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
