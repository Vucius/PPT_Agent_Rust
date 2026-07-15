use crate::message::Message;
use iced::widget::{button, row};
use iced::{Alignment, Element};

pub fn view(
    is_pdf_loaded: bool,
    is_converting: bool,
    is_converted: bool,
) -> Element<'static, Message> {
    let mut actions = row![].spacing(10).align_items(Alignment::Center);

    if is_converting {
        actions = actions.push(button("Cancel").on_press(Message::CancelClicked));
    } else {
        actions = actions.push(button("Open PDF").on_press(Message::OpenFileClicked));
        if is_pdf_loaded {
            actions = actions.push(button("Convert").on_press(Message::ConvertClicked));
        }
    }

    if is_converted && !is_converting {
        actions = actions.push(iced::widget::Space::with_width(20));
        actions = actions.push(button("Export MD").on_press(Message::ExportMarkdownClicked));
        actions = actions.push(button("Export JSON").on_press(Message::ExportJsonClicked));
    }

    actions.into()
}
