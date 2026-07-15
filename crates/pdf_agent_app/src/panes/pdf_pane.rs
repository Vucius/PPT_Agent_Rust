use crate::message::Message;
use crate::theme::BOLD_FONT;
use iced::widget::{button, column, row, text, Space};
use iced::{Color, Element, Length};

pub fn view<'a>(
    total_pages: usize,
    current_page: usize,
    is_loading_image: bool,
    image_error: Option<&'a str>,
    rendered_image: Option<&'a pdf_agent_core::providers::traits::PageImage>,
) -> Element<'a, Message> {
    if total_pages == 0 {
        return column![
            text("Source PDF Preview").size(20).font(BOLD_FONT),
            Space::with_height(20),
            text("Please click 'Open PDF' to select a document.")
        ]
        .width(Length::Fill)
        .into();
    }

    let preview_widget: Element<'a, Message> = if is_loading_image {
        column![Space::with_height(100), text("Rendering page...").size(16)]
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
        column![Space::with_height(100), text("Page not rendered.").size(16)]
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
}
